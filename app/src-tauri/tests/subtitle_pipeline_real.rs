//! Test tích hợp PBI 19 (T014): nối STT final (PBI 12/13) → dịch có ngữ cảnh (PBI 16/17) →
//! sự kiện phụ đề, chạy với model STT + sidecar dịch thật. Cần `WHISPER_MODEL_PATH`,
//! `SORA_TRANSLATE_SIDECAR_PATH`, `SORA_TRANSLATE_MODEL_PATH` — xem quickstart.md PBI 19.
//! Đánh dấu `#[ignore]` giống mẫu PBI 16/17 vì model/sidecar không commit vào repo.
use app_lib::core::{RecentTranscriptContext, SidecarTranslator, Translator, WhisperSpeechToText};

#[tokio::test]
#[ignore = "cần WHISPER_MODEL_PATH + sidecar dịch/model local — xem quickstart.md PBI 19"]
async fn noi_stt_final_qua_dich_co_ngu_canh_khop_pbi_17() {
    let whisper_model_path =
        std::env::var("WHISPER_MODEL_PATH").expect("đặt WHISPER_MODEL_PATH tới model STT cục bộ");
    let sidecar_path = std::env::var("SORA_TRANSLATE_SIDECAR_PATH")
        .expect("đặt SORA_TRANSLATE_SIDECAR_PATH tới executable dịch cục bộ");
    let model_dir = std::env::var("SORA_TRANSLATE_MODEL_PATH")
        .expect("đặt SORA_TRANSLATE_MODEL_PATH tới thư mục model dịch cục bộ");

    // Chỉ cần xác nhận model STT nạp được — PBI này dùng trực tiếp text cố định cho bước STT để
    // tách biệt khỏi độ chính xác nhận diện giọng nói (đã xác nhận riêng ở PBI 12/13).
    WhisperSpeechToText::new(&whisper_model_path).expect("nạp model STT thất bại");
    let translator = SidecarTranslator::new(&sidecar_path, &model_dir).unwrap();

    // Mô phỏng 2 câu transcript final liên tiếp trong 1 phiên — câu 2 có đại từ phụ thuộc câu 1
    // (giống kịch bản PBI 17 T016), xác nhận SC-002: dịch qua luồng nối (có ngữ cảnh từ
    // RecentTranscriptContext) khớp bản dịch có ngữ cảnh đã xác nhận ở PBI 17.
    let mut recent_ctx = RecentTranscriptContext::new();

    let first_text = "John lost his keys.";
    let first_translated = translator
        .translate(first_text, &recent_ctx.recent())
        .await
        .expect("dịch câu 1 thất bại");
    recent_ctx.push(first_text);

    let second_text = "He was very upset.";
    let with_context = translator
        .translate(second_text, &recent_ctx.recent())
        .await
        .expect("dịch câu 2 (có ngữ cảnh) thất bại");
    let without_context = translator
        .translate(second_text, &[])
        .await
        .expect("dịch câu 2 (không ngữ cảnh) thất bại");

    println!("Câu 1 dịch: {first_translated}");
    println!("Câu 2 không ngữ cảnh: {without_context}");
    println!("Câu 2 có ngữ cảnh (qua luồng nối): {with_context}");

    assert!(!with_context.is_empty());
    // Bằng chứng SC-002: kết quả qua luồng nối (RecentTranscriptContext -> Translator) phải là
    // kết quả dịch CÓ ngữ cảnh, không lẫn/không rơi về bản dịch không ngữ cảnh.
    assert_eq!(
        with_context,
        translator
            .translate(second_text, &["John lost his keys.".to_string()])
            .await
            .expect("dịch đối chiếu thất bại"),
        "ngữ cảnh lấy từ RecentTranscriptContext phải khớp đúng ngữ cảnh truyền tay (PBI 17)"
    );
}
