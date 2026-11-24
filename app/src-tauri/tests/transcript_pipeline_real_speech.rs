//! Test tích hợp PBI 13: ghép `VadPipeline` (PBI 11) → `TranscriptPipeline` (PBI 13) →
//! `WhisperSpeechToText` (PBI 12), chạy trên file ghi âm thật. Cần model ggml/gguf tại
//! `WHISPER_MODEL_PATH` để chạy thật — nếu thiếu, test tự bỏ qua (xem `model_or_skip`).
use app_lib::core::{AudioFrame, TranscriptPipeline, VadPipeline, WhisperSpeechToText};
use hound::{SampleFormat, WavReader};

fn model_or_skip() -> Option<WhisperSpeechToText> {
    let path = std::env::var("WHISPER_MODEL_PATH").ok()?;
    match WhisperSpeechToText::new(&path) {
        Ok(stt) => Some(stt),
        Err(e) => {
            eprintln!("bỏ qua test transcript pipeline: không nạp được model tại {path}: {e}");
            None
        }
    }
}

fn load_fixture() -> (Vec<f32>, u32) {
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
    (samples, spec.sample_rate)
}

/// Đưa toàn bộ fixture qua `VadPipeline` theo khối nhỏ (mô phỏng capture liên tục), cộng thêm
/// lặng cuối để chắc chắn đóng segment cuối — giống cách `vad_pipeline_real_speech.rs` làm.
fn segments_from_fixture() -> Vec<app_lib::core::SegmentReady> {
    let (samples, sample_rate) = load_fixture();
    let mut pipeline = VadPipeline::new();
    let mut ready = Vec::new();

    for chunk in samples.chunks(2048) {
        let frame = AudioFrame {
            samples: chunk.to_vec(),
            sample_rate,
        };
        ready.extend(pipeline.push_frame(frame).expect("push_frame thành công"));
    }
    let trailing_silence = AudioFrame {
        samples: vec![0.0; sample_rate as usize * 2],
        sample_rate,
    };
    ready.extend(
        pipeline
            .push_frame(trailing_silence)
            .expect("push_frame lặng cuối thành công"),
    );
    ready
}

#[tokio::test]
async fn pipeline_vad_stt_tra_ve_text_dung_so_luong_va_thu_tu() {
    let Some(stt) = model_or_skip() else {
        eprintln!("SKIP: thiếu WHISPER_MODEL_PATH, không chạy được test pipeline thật");
        return;
    };

    let segments = segments_from_fixture();
    assert!(
        !segments.is_empty(),
        "fixture có giọng nói thật phải sinh ít nhất 1 segment"
    );
    let num_segments = segments.len();

    let pipeline = TranscriptPipeline::new(&stt);
    let chunks = pipeline
        .process_segments(segments)
        .await
        .expect("process_segments không lỗi");

    assert_eq!(
        chunks.len(),
        num_segments,
        "số đoạn text phải khớp số segment VAD cắt ra (FR-001)"
    );
    assert!(
        chunks.iter().any(|c| !c.text.trim().is_empty()),
        "phải có ít nhất 1 đoạn text không rỗng vì fixture có giọng nói thật"
    );
}

#[tokio::test]
async fn audio_toan_im_lang_khong_sinh_text_nao_khong_loi() {
    let Some(stt) = model_or_skip() else {
        eprintln!("SKIP: thiếu WHISPER_MODEL_PATH, không chạy được test pipeline thật");
        return;
    };

    let sample_rate = 16_000u32;
    let mut vad_pipeline = VadPipeline::new();
    let silence = AudioFrame {
        samples: vec![0.0; sample_rate as usize * 3],
        sample_rate,
    };
    let segments = vad_pipeline
        .push_frame(silence)
        .expect("push_frame lặng thành công");

    assert!(
        segments.is_empty(),
        "audio toàn im lặng không được sinh segment nào (FR-003)"
    );

    let pipeline = TranscriptPipeline::new(&stt);
    let chunks = pipeline
        .process_segments(segments)
        .await
        .expect("process_segments không lỗi với danh sách rỗng");

    assert!(chunks.is_empty(), "không có segment thì không có text nào");
}
