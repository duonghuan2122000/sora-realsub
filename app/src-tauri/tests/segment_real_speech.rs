//! Test tích hợp PBI 10: ghép 2 lượt nói thật (tái dùng `vad_sample.wav` từ PBI 9) cách nhau
//! bởi 1 khoảng lặng dài, rồi chạy qua `vad::detect()` -> `segment::segment()`, xác nhận số
//! segment trả về đúng với số lượt nói đã biết trước (2) và không bị cắt gãy bởi khoảng lặng
//! ngắn vốn có trong chính file ghi âm gốc.
use app_lib::core::{AudioFrame, detect, segment};
use hound::{SampleFormat, WavReader};

fn load_fixture_samples() -> (Vec<f32>, u32) {
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

#[test]
fn hai_luot_noi_tach_biet_sinh_dung_hai_segment() {
    let (speech_turn, sample_rate) = load_fixture_samples();
    // Khoảng lặng dài (2s) để chắc chắn vượt ngưỡng min_silence_frames mặc định.
    let long_silence = vec![0.0f32; sample_rate as usize * 2];

    let mut combined = speech_turn.clone();
    combined.extend(long_silence);
    combined.extend(speech_turn);

    let frame = AudioFrame {
        samples: combined,
        sample_rate,
    };

    let labels = detect(&[frame]).expect("vad chạy thành công trên dữ liệu ghép");
    let segments = segment(&labels);

    assert_eq!(
        segments.len(),
        2,
        "kỳ vọng đúng 2 segment cho 2 lượt nói tách biệt, thực tế: {:?}",
        segments
    );
    // Mỗi segment phải nằm đúng thứ tự thời gian: segment 1 kết thúc trước khi segment 2 bắt đầu.
    assert!(segments[0].end_frame_index < segments[1].start_frame_index);
}
