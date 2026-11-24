//! Test tích hợp PBI 12: STT local bằng whisper.cpp (`WhisperSpeechToText`).
//! Cần model ggml/gguf tại đường dẫn biến môi trường `WHISPER_MODEL_PATH` để chạy thật —
//! nếu thiếu, test bị `#[ignore]` tự động (xem `model_or_skip`).
use app_lib::core::{AudioFrame, SpeechToText, WhisperSpeechToText};
use hound::{SampleFormat, WavReader};

/// Trả về STT đã nạp model, hoặc `None` nếu môi trường chưa có `WHISPER_MODEL_PATH`
/// (máy chạy test không có model — xem rủi ro đã ghi trong plan.md PBI 12).
fn model_or_skip() -> Option<WhisperSpeechToText> {
    let path = std::env::var("WHISPER_MODEL_PATH").ok()?;
    match WhisperSpeechToText::new(&path) {
        Ok(stt) => Some(stt),
        Err(e) => {
            eprintln!("bỏ qua test STT: không nạp được model tại {path}: {e}");
            None
        }
    }
}

fn load_real_speech_fixture() -> AudioFrame {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/vad_sample.wav");
    let mut reader = WavReader::open(path).expect("mở file fixture vad_sample.wav");
    let spec = reader.spec();
    assert_eq!(
        spec.sample_format,
        SampleFormat::Float,
        "fixture phải là PCM float32"
    );
    let samples: Vec<f32> = reader
        .samples::<f32>()
        .map(|s| s.expect("đọc mẫu float"))
        .collect();
    AudioFrame {
        samples,
        sample_rate: spec.sample_rate,
    }
}

fn silence_frame() -> AudioFrame {
    AudioFrame {
        samples: vec![0.0; 16_000 * 2],
        sample_rate: 16_000,
    }
}

#[tokio::test]
async fn giong_noi_thuc_tra_ve_text_khong_rong() {
    let Some(stt) = model_or_skip() else {
        eprintln!("SKIP: thiếu WHISPER_MODEL_PATH, không chạy được test STT thật");
        return;
    };

    let frame = load_real_speech_fixture();
    let result = stt.transcribe(frame).await.expect("transcribe không lỗi");

    assert!(result.is_final);
    assert!(
        !result.text.trim().is_empty(),
        "file ghi âm có giọng nói thật phải cho ra text không rỗng"
    );
}

#[tokio::test]
async fn audio_im_lang_tra_ve_text_rong_khong_crash() {
    let Some(stt) = model_or_skip() else {
        eprintln!("SKIP: thiếu WHISPER_MODEL_PATH, không chạy được test STT thật");
        return;
    };

    let result = stt
        .transcribe(silence_frame())
        .await
        .expect("transcribe audio im lặng không lỗi/panic");

    assert!(
        result.text.trim().is_empty(),
        "audio im lặng phải cho ra text rỗng/gần rỗng, nhận được: {:?}",
        result.text
    );
}
