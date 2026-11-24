//! Test tích hợp PBI 11: đưa file ghi âm thật (`tests/fixtures/vad_sample.wav`, tái dùng từ
//! PBI 9-10) qua `VadPipeline::push_frame` theo từng khối nhỏ (giống cách capture thật gửi
//! từng `AudioFrame`), xác nhận các segment trả về đúng thứ tự thời gian và không trùng/thiếu.
use app_lib::core::{AudioFrame, VadPipeline};
use hound::{SampleFormat, WavReader};

fn load_fixture() -> (Vec<f32>, u32) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/vad_sample.wav");
    let mut reader = WavReader::open(path).expect("mở file fixture vad_sample.wav");
    let spec = reader.spec();
    assert_eq!(spec.sample_format, SampleFormat::Float, "fixture phải là PCM float32");
    let samples: Vec<f32> = reader
        .samples::<f32>()
        .map(|s| s.expect("đọc mẫu float"))
        .collect();
    (samples, spec.sample_rate)
}

#[test]
fn streaming_tung_khoi_nho_phat_segment_dung_thu_tu_khong_trung() {
    let (samples, sample_rate) = load_fixture();
    let mut pipeline = VadPipeline::new();
    let mut ready = Vec::new();

    // Đưa audio vào theo khối 2048 mẫu/lần, mô phỏng từng AudioFrame nhận liên tục từ capture
    // (khác test fixture gốc ở vad.rs/segment.rs vốn đưa nguyên file vào 1 lần).
    for chunk in samples.chunks(2048) {
        let frame = AudioFrame {
            samples: chunk.to_vec(),
            sample_rate,
        };
        ready.extend(pipeline.push_frame(frame).expect("push_frame chạy thành công"));
    }
    // Đẩy thêm lặng dài để chắc chắn đóng segment cuối, dù fixture gốc không đủ lặng ở cuối.
    let trailing_silence = AudioFrame {
        samples: vec![0.0; sample_rate as usize * 2],
        sample_rate,
    };
    ready.extend(
        pipeline
            .push_frame(trailing_silence)
            .expect("push_frame xử lý lặng cuối thành công"),
    );

    assert!(
        !ready.is_empty(),
        "ghi âm có đoạn nói thật phải phát ít nhất 1 sự kiện segment"
    );

    for segment in &ready {
        assert!(!segment.samples.is_empty(), "segment phát ra phải có dữ liệu audio");
        assert_eq!(segment.sample_rate, 16_000);
        assert!(segment.end_frame_index >= segment.start_frame_index);
    }

    // Không trùng/thiếu, đúng thứ tự thời gian (FR-003, FR-004): segment sau phải bắt đầu
    // sau khi segment trước đã kết thúc.
    for i in 1..ready.len() {
        assert!(
            ready[i].start_frame_index > ready[i - 1].end_frame_index,
            "segment {i} phải bắt đầu sau khi segment trước kết thúc, không trùng/chồng lấp"
        );
    }
}

#[test]
fn sau_khi_het_audio_push_frame_khong_phat_them_segment_trung() {
    let (samples, sample_rate) = load_fixture();
    let mut pipeline = VadPipeline::new();

    for chunk in samples.chunks(2048) {
        let frame = AudioFrame {
            samples: chunk.to_vec(),
            sample_rate,
        };
        pipeline.push_frame(frame).expect("push_frame chạy thành công");
    }
    let trailing_silence = AudioFrame {
        samples: vec![0.0; sample_rate as usize * 2],
        sample_rate,
    };
    pipeline
        .push_frame(trailing_silence.clone())
        .expect("push_frame xử lý lặng cuối thành công");

    // Phiên "dừng" ở đây tương ứng với không còn frame nào được gửi tiếp (FR-005: Core không
    // phát thêm vad-segment sau khi dừng phiên) — gọi thêm lặng lần 2 không được tạo segment mới.
    let second_flush = pipeline
        .push_frame(trailing_silence)
        .expect("push_frame lần 2 chạy thành công");

    assert!(
        second_flush.is_empty(),
        "không được phát lại/phát thêm segment đã đóng từ trước, thực tế: {:?}",
        second_flush
    );
}
