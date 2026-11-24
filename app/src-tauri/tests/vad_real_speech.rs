//! Test tích hợp PBI 9: chạy VAD trên file ghi âm thật (mic, 8 giây: im lặng → nói → im lặng),
//! xem `tests/fixtures/vad_sample.wav`. Không có timestamp nói/lặng gán tay chính xác (phụ thuộc
//! phản ứng người ghi âm), nên dùng RMS năng lượng mỗi frame làm "ground truth" năng lượng độc lập
//! để đối chiếu với nhãn VAD, thay cho so khớp mốc thời gian cứng.
use app_lib::core::{AudioFrame, detect};
use hound::{SampleFormat, WavReader};

const FRAME_SIZE: usize = 512;

fn load_fixture_frame() -> AudioFrame {
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

fn frame_rms(chunk: &[f32]) -> f32 {
    let sum_sq: f32 = chunk.iter().map(|s| s * s).sum();
    (sum_sq / chunk.len() as f32).sqrt()
}

#[test]
fn vad_phan_biet_duoc_doan_co_nang_luong_cao_va_thap_tren_ghi_am_thuc() {
    let frame = load_fixture_frame();
    let labels = detect(&[frame.clone()]).expect("vad chạy thành công trên fixture thật");
    assert!(
        !labels.is_empty(),
        "fixture phải đủ dài để sinh ít nhất 1 frame VAD"
    );

    // Dùng lại đúng pipeline resample 16kHz mà `detect()` dùng nội bộ, để RMS tính trên
    // cùng dữ liệu đã đưa vào model (so sánh ngang hàng).
    let resampled = app_lib::core::resample_linear(&frame.samples, frame.sample_rate, 16_000);
    let rms_per_frame: Vec<f32> = resampled.chunks_exact(FRAME_SIZE).map(frame_rms).collect();
    assert_eq!(rms_per_frame.len(), labels.len());

    let speech_count = labels.iter().filter(|l| l.is_speech).count();
    let silence_count = labels.len() - speech_count;
    assert!(
        speech_count > 0 && silence_count > 0,
        "kỳ vọng ghi âm 8s (im lặng-nói-im lặng) có cả nhãn speech và silence, thực tế speech={speech_count} silence={silence_count}/{}",
        labels.len()
    );

    // Đối chiếu nhãn VAD với RMS năng lượng độc lập: frame RMS cao hơn trung vị toàn cục
    // nên nghiêng về speech, frame RMS thấp hơn trung vị nên nghiêng về silence.
    let mut sorted_rms = rms_per_frame.clone();
    sorted_rms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_rms = sorted_rms[sorted_rms.len() / 2];

    let mut agree = 0usize;
    for (label, &rms) in labels.iter().zip(rms_per_frame.iter()) {
        let energy_says_speech = rms > median_rms;
        if energy_says_speech == label.is_speech {
            agree += 1;
        }
    }
    let agreement_ratio = agree as f32 / labels.len() as f32;
    assert!(
        agreement_ratio >= 0.6,
        "VAD nên khớp hướng với năng lượng RMS ở ít nhất 60% frame, thực tế {:.2}",
        agreement_ratio
    );

    // Thứ tự nhãn trả về phải đúng thứ tự thời gian của fixture gốc.
    let indices: Vec<usize> = labels.iter().map(|l| l.frame_index).collect();
    let expected: Vec<usize> = (0..labels.len()).collect();
    assert_eq!(
        indices, expected,
        "nhãn VAD phải giữ đúng thứ tự frame_index theo thời gian"
    );
}
