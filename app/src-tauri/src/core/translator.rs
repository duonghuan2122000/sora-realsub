use super::error::CoreError;
use async_trait::async_trait;
use std::process::Command;

/// Dịch văn bản, có thể kèm ngữ cảnh (1-2 câu trước) để dịch chính xác hơn.
#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError>;
}

/// Cặp ngôn ngữ mặc định đã chốt ở M0 (English → Vietnamese). Model Marian-MT hiện dùng
/// (opus-mt-en-vi) là model 1 cặp ngôn ngữ cố định nên sidecar không thực sự dùng 2 mã này để
/// chọn hướng dịch, nhưng vẫn truyền qua CLI để hợp đồng sidecar tổng quát cho model khác sau
/// này (ví dụ NLLB, cần mã ngôn ngữ FLORES-200 dạng "eng_Latn"/"vie_Latn").
const SOURCE_LANG: &str = "en";
const TARGET_LANG: &str = "vi";

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

#[async_trait]
impl Translator for SidecarTranslator {
    async fn translate(&self, text: &str, _context: &[String]) -> Result<String, CoreError> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let output = Command::new(&self.executable_path)
            .arg("--model-dir")
            .arg(&self.model_dir)
            .arg("--source-lang")
            .arg(SOURCE_LANG)
            .arg("--target-lang")
            .arg(TARGET_LANG)
            .arg("--text")
            .arg(text)
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

        let translated = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(translated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
