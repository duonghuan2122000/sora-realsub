//! Bin thủ công (PBI 5): capture mic trong N giây, ghi ra file .wav để nghe lại kiểm thử.
//! Dùng: `cargo run --bin capture_mic_to_wav -- <số giây> [device_id]`

use app_lib::core::{AudioSource, MicAudioSource};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type CollectedFrames = Arc<Mutex<Vec<(Vec<f32>, u32)>>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let seconds: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5);
    let device_id = args.get(2).cloned();

    let source = MicAudioSource::new(device_id.clone());
    let frames: CollectedFrames = Arc::new(Mutex::new(Vec::new()));
    let frames_clone = frames.clone();

    let handle = match source.start(Box::new(move |frame| {
        frames_clone
            .lock()
            .unwrap()
            .push((frame.samples, frame.sample_rate));
    })) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Lỗi capture mic (device_id={device_id:?}): {e}");
            std::process::exit(1);
        }
    };

    println!("Đang capture {seconds} giây từ mic (device_id={device_id:?})...");
    thread::sleep(Duration::from_secs(seconds));
    handle.stop();

    let collected = frames.lock().unwrap();
    let sample_rate = collected.first().map(|(_, sr)| *sr).unwrap_or(16_000);
    let out_path = "capture_mic_to_wav_output.wav";

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(out_path, spec).expect("tạo file wav thất bại");
    for (samples, _) in collected.iter() {
        for s in samples {
            writer.write_sample(*s).expect("ghi mẫu thất bại");
        }
    }
    writer.finalize().expect("ghi file wav thất bại");

    println!("Đã ghi {} khối mẫu ra file: {out_path}", collected.len());
}
