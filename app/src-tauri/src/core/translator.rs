use super::error::CoreError;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::process::Command;

/// Dịch văn bản, có thể kèm ngữ cảnh (1-2 câu trước) để dịch chính xác hơn.
#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError>;
}

/// Số câu ngữ cảnh tối đa dùng để dịch câu hiện tại (PBI 17, FR-003).
const MAX_CONTEXT_SENTENCES: usize = 2;

/// Giữ tối đa 2 câu transcript final gần nhất trong phiên hiện tại, dùng làm ngữ cảnh khi dịch
/// câu kế tiếp (PBI 17; xem data-model.md). Chỉ sống trong bộ nhớ của một phiên, không persist.
/// PBI này chỉ cung cấp struct ở mức đơn vị — chưa nối vào `TranscriptPipeline` thật (ngoài phạm
/// vi PBI 17, xem research.md Quyết định 3).
#[derive(Default)]
pub struct RecentTranscriptContext {
    sentences: VecDeque<String>,
}

impl RecentTranscriptContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Thêm câu mới vào cuối; bỏ qua câu rỗng/khoảng trắng; loại câu cũ nhất khi đã đầy
    /// `MAX_CONTEXT_SENTENCES`.
    pub fn push(&mut self, sentence: &str) {
        if sentence.trim().is_empty() {
            return;
        }
        if self.sentences.len() >= MAX_CONTEXT_SENTENCES {
            self.sentences.pop_front();
        }
        self.sentences.push_back(sentence.to_string());
    }

    /// Trả về các câu hiện có, theo thứ tự cũ → mới, tối đa `MAX_CONTEXT_SENTENCES` câu.
    pub fn recent(&self) -> Vec<String> {
        self.sentences.iter().cloned().collect()
    }
}

/// Lọc bỏ câu rỗng/khoảng trắng trong `context`, chỉ giữ tối đa `MAX_CONTEXT_SENTENCES` câu gần
/// câu hiện tại nhất (các phần tử cuối slice) — FR-003, FR-005.
fn normalize_context(context: &[String]) -> Vec<String> {
    let filtered: Vec<String> = context
        .iter()
        .filter(|s| !s.trim().is_empty())
        .cloned()
        .collect();
    let start = filtered.len().saturating_sub(MAX_CONTEXT_SENTENCES);
    filtered[start..].to_vec()
}

/// Tách `text` thành các câu theo dấu kết câu (`.`, `!`, `?`), giữ lại dấu câu ở cuối mỗi đoạn.
/// Dùng để tách bản dịch của câu hiện tại ra khỏi bản dịch ghép ngữ cảnh+hiện tại (research.md
/// Quyết định 2). Phần còn lại sau dấu kết câu cuối cùng (nếu có nội dung) được tính là 1 đoạn.
fn split_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?') {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                sentences.push(trimmed);
            }
            current.clear();
        }
    }
    let trailing = current.trim().to_string();
    if !trailing.is_empty() {
        sentences.push(trailing);
    }
    sentences
}

/// Từ bản dịch ghép (ngữ cảnh + câu hiện tại) `combined_translation`, trả về bản dịch của riêng
/// câu hiện tại nếu tách được đúng `expected_sentence_count` đoạn (đoạn cuối cùng); trả `None`
/// khi tách thất bại (số đoạn không khớp) để nơi gọi áp dụng phương án dự phòng — research.md
/// Quyết định 2 (FR-002).
fn extract_current_translation(
    combined_translation: &str,
    expected_sentence_count: usize,
) -> Option<String> {
    let split = split_sentences(combined_translation);
    if split.len() == expected_sentence_count {
        split.into_iter().last()
    } else {
        None
    }
}

/// Cặp ngôn ngữ mặc định đã chốt ở M0 (English → Vietnamese). Model Marian-MT hiện dùng
/// (opus-mt-en-vi) là model 1 cặp ngôn ngữ cố định nên sidecar không thực sự dùng 2 mã này để
/// chọn hướng dịch, nhưng vẫn truyền qua CLI để hợp đồng sidecar tổng quát cho model khác sau
/// này (ví dụ NLLB, cần mã ngôn ngữ FLORES-200 dạng "eng_Latn"/"vie_Latn").
const SOURCE_LANG: &str = "en";
const TARGET_LANG: &str = "vi";

/// Một cặp ngôn ngữ nguồn-đích hệ thống hỗ trợ chọn từ UI (PBI 29). Danh sách tĩnh, không
/// persist — khi có model dịch mới chỉ cần thêm phần tử vào `SUPPORTED_LANGUAGE_PAIRS`
/// (research.md Quyết định 1).
pub struct LanguagePair {
    pub id: &'static str,
    pub source_lang: &'static str,
    pub target_lang: &'static str,
    pub label: &'static str,
}

/// Danh sách cặp ngôn ngữ hệ thống hỗ trợ. Hiện chỉ có 1 phần tử, khớp model dịch đã có
/// (PBI 16). Mọi phần tử PHẢI có `source_lang != target_lang` (đảm bảo FR-003 bằng cấu trúc
/// dữ liệu, xem test `moi_cap_ngon_ngu_nguon_khac_dich`).
pub const SUPPORTED_LANGUAGE_PAIRS: &[LanguagePair] = &[LanguagePair {
    id: "en-vi",
    source_lang: SOURCE_LANG,
    target_lang: TARGET_LANG,
    label: "Anh → Việt",
}];

/// Cài đặt `Translator` bằng model dịch local open-source (Marian-MT, Apache-2.0 — xem
/// research.md PBI 16), chạy qua một **tiến trình ngoài (sidecar)** thay vì binding native
/// in-process — tránh xung đột linker CRT tĩnh/động giữa thư viện binding dịch và `ort`
/// (onnxruntime) đang dùng cho VAD.
///
/// `executable_path` là đường dẫn tới chương trình dịch cục bộ (CLI), `model_dir` là thư mục
/// chứa model đã nạp sẵn cho sidecar đó. Không gọi tiến trình khi khởi tạo — chỉ lưu đường dẫn;
/// lỗi (thiếu executable, model lỗi) chỉ phát hiện khi gọi `translate()` (FR-002).
pub struct SidecarTranslator {
    executable_path: String,
    model_dir: String,
}

impl SidecarTranslator {
    pub fn new(executable_path: &str, model_dir: &str) -> Result<Self, CoreError> {
        Ok(Self {
            executable_path: executable_path.to_string(),
            model_dir: model_dir.to_string(),
        })
    }
}

impl SidecarTranslator {
    /// Gọi sidecar dịch cho đúng 1 chuỗi `input` (có thể là 1 câu hoặc nhiều câu ghép), trả về
    /// nguyên văn bản dịch sidecar in ra (chưa tách câu).
    fn translate_raw(&self, input: &str) -> Result<String, CoreError> {
        let output = Command::new(&self.executable_path)
            .arg("--model-dir")
            .arg(&self.model_dir)
            .arg("--source-lang")
            .arg(SOURCE_LANG)
            .arg("--target-lang")
            .arg(TARGET_LANG)
            .arg("--text")
            .arg(input)
            .output()
            .map_err(|e| {
                CoreError::Provider(format!(
                    "không chạy được sidecar dịch '{}': {e}",
                    self.executable_path
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CoreError::Provider(format!(
                "sidecar dịch trả lỗi (exit code {:?}): {}",
                output.status.code(),
                stderr.trim()
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl Translator for SidecarTranslator {
    async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let context = normalize_context(context);
        if context.is_empty() {
            // Không có ngữ cảnh hợp lệ → dịch câu hiện tại một mình, giữ nguyên hành vi PBI 16
            // (FR-004).
            return self.translate_raw(text);
        }

        // Có ngữ cảnh: ghép ngữ cảnh (cũ → mới) + câu hiện tại thành 1 input, gửi sidecar hiện
        // có (không đổi CLI sidecar) — research.md Quyết định 1 (FR-001).
        let mut combined_parts = context.clone();
        combined_parts.push(text.to_string());
        let combined_input = combined_parts.join(" ");

        let combined_translation = self.translate_raw(&combined_input)?;

        // Tách bản dịch câu hiện tại ra khỏi bản dịch ghép — research.md Quyết định 2 (FR-002).
        match extract_current_translation(&combined_translation, combined_parts.len()) {
            Some(current_translation) => Ok(current_translation),
            None => {
                // Tách thất bại (số đoạn không khớp số câu đầu vào) → dự phòng: dịch lại câu
                // hiện tại không kèm ngữ cảnh, đảm bảo luôn trả đúng bản dịch câu hiện tại,
                // không crash, không lẫn ngữ cảnh (FR-002, SC-002, SC-003).
                self.translate_raw(text)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PBI 29 — T002: danh sách cặp ngôn ngữ hỗ trợ không rỗng và mọi cặp có nguồn != đích.
    #[test]
    fn danh_sach_cap_ngon_ngu_khong_rong_va_moi_cap_nguon_khac_dich() {
        assert!(!SUPPORTED_LANGUAGE_PAIRS.is_empty());
        for pair in SUPPORTED_LANGUAGE_PAIRS {
            assert_ne!(pair.source_lang, pair.target_lang);
        }
    }

    /// Cài đặt giả: ghép tiền tố "[dịch]" + số câu ngữ cảnh nhận được.
    struct MockTranslator;

    #[async_trait]
    impl Translator for MockTranslator {
        async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError> {
            Ok(format!("[dịch:{}] {}", context.len(), text))
        }
    }

    #[tokio::test]
    async fn dich_kem_ngu_canh() {
        let translator = MockTranslator;
        let context = vec!["câu trước".to_string()];
        let result = translator.translate("hello", &context).await.unwrap();
        assert_eq!(result, "[dịch:1] hello");
    }

    #[tokio::test]
    async fn goi_qua_trait_object() {
        let translator: Box<dyn Translator> = Box::new(MockTranslator);
        let result = translator.translate("hi", &[]).await.unwrap();
        assert_eq!(result, "[dịch:0] hi");
    }

    // PBI 16 — T005: sidecar executable không tồn tại phải trả lỗi rõ ràng, không panic.
    #[tokio::test]
    async fn sidecar_khong_ton_tai_tra_loi() {
        let translator = SidecarTranslator::new("duong-dan-khong-ton-tai.exe", "model-dir").unwrap();
        let result = translator.translate("Hello", &[]).await;
        match result {
            Err(CoreError::Provider(msg)) => assert!(!msg.is_empty()),
            Err(other) => panic!("kỳ vọng CoreError::Provider, nhận lỗi khác: {other}"),
            Ok(_) => panic!("kỳ vọng Err khi sidecar không tồn tại"),
        }
    }

    // PBI 16 — T006: input rỗng/khoảng trắng không spawn tiến trình, trả rỗng, không lỗi.
    #[tokio::test]
    async fn input_rong_tra_ve_rong_khong_spawn_tien_trinh() {
        // Dùng executable chắc chắn không tồn tại — nếu input rỗng mà vẫn trả Ok("") thì
        // chứng minh translate() đã return sớm, không thử spawn (FR-003).
        let translator = SidecarTranslator::new("duong-dan-khong-ton-tai.exe", "model-dir").unwrap();
        assert_eq!(translator.translate("", &[]).await.unwrap(), "");
        assert_eq!(translator.translate("   ", &[]).await.unwrap(), "");
    }

    // PBI 16 — T007: test tích hợp với sidecar dịch thật, cần chuẩn bị executable + model cục bộ
    // (xem quickstart.md). Đánh dấu #[ignore] vì sidecar/model không commit vào repo; chạy thủ
    // công bằng `cargo test -p app translator -- --ignored` khi đã có sẵn trên máy dev.
    #[tokio::test]
    #[ignore = "cần sidecar dịch + model local open-source (Marian-MT opus-mt-en-vi) cục bộ — xem quickstart.md"]
    async fn dich_cau_co_dinh_dung_nghia_va_on_dinh() {
        let executable_path = std::env::var("SORA_TRANSLATE_SIDECAR_PATH")
            .expect("đặt SORA_TRANSLATE_SIDECAR_PATH tới executable dịch cục bộ");
        let model_dir = std::env::var("SORA_TRANSLATE_MODEL_PATH")
            .expect("đặt SORA_TRANSLATE_MODEL_PATH tới thư mục model dịch cục bộ");
        let translator = SidecarTranslator::new(&executable_path, &model_dir).unwrap();

        let first = translator
            .translate("Hello, how are you?", &[])
            .await
            .expect("dịch lần 1 thất bại");
        // Bản dịch tham chiếu lấy từ chạy thật sidecar (model gaudi/opus-mt-en-vi-ctranslate2,
        // Apache-2.0, xem quickstart.md) — không phải câu "chuẩn" duy nhất, chỉ cần khớp đúng
        // model/sidecar đang dùng.
        assert_eq!(first, "Xin chào, anh khỏe không?");

        let second = translator
            .translate("Hello, how are you?", &[])
            .await
            .expect("dịch lần 2 thất bại");
        assert_eq!(
            first, second,
            "cùng câu đầu vào phải cho kết quả ổn định (FR-004)"
        );
    }

    /// Translator giả dùng cho test PBI 17: luôn "dịch" bằng cách thêm tiền tố "[vi] " trước
    /// mỗi câu trong input (giữ nguyên số câu, dấu kết câu) — mô phỏng hành vi sidecar dịch cả
    /// đoạn ghép ngữ cảnh+hiện tại thành 1 chuỗi.
    struct EchoPrefixSidecar;

    impl EchoPrefixSidecar {
        fn translate_combined(&self, input: &str) -> String {
            split_sentences(input)
                .iter()
                .map(|s| format!("[vi] {s}"))
                .collect::<Vec<_>>()
                .join(" ")
        }
    }

    // PBI 17 — T010: dịch có 1 câu ngữ cảnh → input gửi đi gồm cả ngữ cảnh + câu hiện tại, kết
    // quả trả về chỉ là phần câu hiện tại (không lẫn bản dịch câu ngữ cảnh) — FR-001, FR-002.
    #[test]
    fn ghep_ngu_canh_va_tach_dung_cau_hien_tai() {
        let echo = EchoPrefixSidecar;
        let combined = "John lost his keys. He was very upset.";
        let translated_combined = echo.translate_combined(combined);
        let expected_count = split_sentences(combined).len();
        let split = split_sentences(&translated_combined);
        assert_eq!(split.len(), expected_count);
        assert_eq!(split.last().unwrap(), "[vi] He was very upset.");
        assert_ne!(
            split.last().unwrap(),
            &translated_combined,
            "kết quả không được lẫn cả bản dịch ngữ cảnh"
        );
    }

    // PBI 17 — T011: normalize_context với danh sách rỗng → trả rỗng, tương đương không có
    // ngữ cảnh (FR-004: hành vi giống PBI 16 khi context rỗng).
    #[test]
    fn normalize_context_rong_tra_ve_rong() {
        assert!(normalize_context(&[]).is_empty());
    }

    // PBI 17 — T012: câu rỗng/khoảng trắng trong context bị loại trước khi dùng (FR-005).
    #[test]
    fn normalize_context_loai_cau_rong() {
        let context = vec![
            "".to_string(),
            "John lost his keys.".to_string(),
            "   ".to_string(),
        ];
        assert_eq!(normalize_context(&context), vec!["John lost his keys."]);
    }

    // PBI 17 — T013: context có hơn 2 câu → chỉ 2 câu gần câu hiện tại nhất (cuối danh sách)
    // được giữ lại (FR-003).
    #[test]
    fn normalize_context_gioi_han_toi_da_2_cau() {
        let context = vec![
            "Câu 1.".to_string(),
            "Câu 2.".to_string(),
            "Câu 3.".to_string(),
        ];
        assert_eq!(normalize_context(&context), vec!["Câu 2.", "Câu 3."]);
    }

    // PBI 17 — RecentTranscriptContext: push bỏ qua câu rỗng, chỉ giữ tối đa 2 câu gần nhất,
    // đúng thứ tự cũ → mới.
    #[test]
    fn recent_transcript_context_giu_toi_da_2_cau_gan_nhat() {
        let mut ctx = RecentTranscriptContext::new();
        ctx.push("   ");
        ctx.push("Câu 1.");
        ctx.push("Câu 2.");
        ctx.push("Câu 3.");
        assert_eq!(ctx.recent(), vec!["Câu 2.", "Câu 3."]);
    }

    // PBI 17 — T014: nếu tách câu không khớp số câu đầu vào (sidecar gộp câu lại, không giữ
    // đúng dấu kết câu), `extract_current_translation()` phải trả `None` (báo hiệu nơi gọi
    // dùng phương án dự phòng dịch lại không ngữ cảnh) — không trả về rác/cả đoạn ghép.
    #[test]
    fn tach_cau_khong_khop_tra_ve_none_de_fallback() {
        // Sidecar giả gộp toàn bộ ngữ cảnh+hiện tại thành 1 câu duy nhất (không giữ 2 dấu kết
        // câu như đầu vào) → số đoạn tách được (1) không khớp số câu đầu vào (2).
        let combined_translation = "mot cau gop khong tach duoc";
        assert_eq!(
            extract_current_translation(combined_translation, 2),
            None
        );
    }

    // PBI 17 — T014 (tiếp): khi tách đúng số câu, trả về đúng đoạn cuối (câu hiện tại).
    #[test]
    fn tach_cau_khop_tra_ve_dung_cau_cuoi() {
        let combined_translation = "[vi] John lost his keys. [vi] He was very upset.";
        assert_eq!(
            extract_current_translation(combined_translation, 2),
            Some("[vi] He was very upset.".to_string())
        );
    }

    // PBI 17 — T015: cùng câu hiện tại + cùng context, gọi `normalize_context` nhiều lần cho
    // cùng kết quả (hàm thuần, không trạng thái) — giữ FR-006.
    #[test]
    fn normalize_context_on_dinh_giua_cac_lan_goi() {
        let context = vec!["Câu 1.".to_string(), "Câu 2.".to_string()];
        assert_eq!(normalize_context(&context), normalize_context(&context));
    }

    // PBI 17 — T016: test tích hợp với sidecar + model thật, xác nhận SC-001 — câu có đại từ
    // dịch nhất quán hơn khi có ngữ cảnh. Đánh dấu #[ignore] giống T007 (cần sidecar/model cục
    // bộ, xem quickstart.md).
    #[tokio::test]
    #[ignore = "cần sidecar dịch + model local open-source (Marian-MT opus-mt-en-vi) cục bộ — xem quickstart.md"]
    async fn dich_co_ngu_canh_dai_tu_dich_dung_hon() {
        let executable_path = std::env::var("SORA_TRANSLATE_SIDECAR_PATH")
            .expect("đặt SORA_TRANSLATE_SIDECAR_PATH tới executable dịch cục bộ");
        let model_dir = std::env::var("SORA_TRANSLATE_MODEL_PATH")
            .expect("đặt SORA_TRANSLATE_MODEL_PATH tới thư mục model dịch cục bộ");
        let translator = SidecarTranslator::new(&executable_path, &model_dir).unwrap();

        let context = vec!["John lost his keys.".to_string()];
        let with_context = translator
            .translate("He was very upset.", &context)
            .await
            .expect("dịch kèm ngữ cảnh thất bại");
        let without_context = translator
            .translate("He was very upset.", &[])
            .await
            .expect("dịch không ngữ cảnh thất bại");

        // Ghi lại cả 2 kết quả làm bằng chứng SC-001 (xem evidence/T016-testcase.md) — không
        // assert_eq cứng vào 1 câu "chuẩn" vì bản dịch tham chiếu phụ thuộc model cụ thể đang
        // dùng, chỉ cần quan sát và ghi nhận sự khác biệt/đúng đắn hơn khi có ngữ cảnh.
        println!("Không ngữ cảnh: {without_context}");
        println!("Có ngữ cảnh:    {with_context}");
        assert!(!with_context.is_empty());
    }
}
