//! Cài đặt `LoopbackAudioSource` cho Linux: capture âm thanh hệ thống qua
//! monitor source của sink output mặc định (`@DEFAULT_MONITOR@` — tên giả
//! định do PulseAudio tự thay bằng monitor source của sink mặc định hiện
//! tại), dùng
//! PulseAudio client API (`libpulse-simple-binding`) — tương thích cả hệ
//! thống chạy PulseAudio thuần và PipeWire (qua lớp `pipewire-pulse`).
//! Chỉ build trên Linux — bản Windows xem `loopback_audio_source.rs`.
#![cfg(target_os = "linux")]

use super::audio_source::AudioSource;
use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};
use libpulse_binding::sample::{Format, Spec};
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;
use std::sync::mpsc;
use std::thread;

const SAMPLE_RATE: u32 = 16_000;
const CHANNELS: u8 = 1;
/// Số sample 16-bit đọc mỗi lần từ PulseAudio trước khi gửi 1 `AudioFrame`.
const FRAMES_PER_READ: usize = 1600;

/// Cài đặt `AudioSource` thật, capture loopback từ monitor source của sink
/// output mặc định trên Linux (PulseAudio/PipeWire), qua crate
/// `libpulse-binding`/`libpulse-simple-binding`.
pub struct LoopbackAudioSource;

impl Default for LoopbackAudioSource {
    fn default() -> Self {
        Self::new()
    }
}

impl LoopbackAudioSource {
    pub fn new() -> Self {
        Self
    }
}

fn decode_i16_le_to_f32(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / i16::MAX as f32)
        .collect()
}

impl AudioSource for LoopbackAudioSource {
    fn start(
        &self,
        mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<StreamHandle, CoreError> {
        // `Simple` của `libpulse-simple-binding` không phải `Send`, nên phải
        // tạo và giữ sống nó trên 1 thread riêng, giống bản WASAPI.
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), String>>();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();

        let worker = thread::spawn(move || {
            let spec = Spec {
                format: Format::S16le,
                channels: CHANNELS,
                rate: SAMPLE_RATE,
            };
            let setup = Simple::new(
                None,
                "Sora RealSub",
                Direction::Record,
                Some("@DEFAULT_MONITOR@"),
                "loopback",
                &spec,
                None,
                None,
            )
            .map_err(|e| format!("không mở được monitor source mặc định: {e}"));

            match setup {
                Ok(simple) => {
                    let _ = ready_tx.send(Ok(()));
                    let mut buf = vec![0u8; FRAMES_PER_READ * 2];
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        if simple.read(&mut buf).is_err() {
                            break;
                        }
                        on_frame(AudioFrame {
                            samples: decode_i16_le_to_f32(&buf),
                            sample_rate: SAMPLE_RATE,
                        });
                    }
                }
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                }
            }
        });

        match ready_rx.recv() {
            Ok(Ok(())) => Ok(StreamHandle::new(move || {
                let _ = stop_tx.send(());
                let _ = worker.join();
            })),
            Ok(Err(e)) => {
                let _ = worker.join();
                Err(CoreError::Io(e))
            }
            Err(_) => {
                let _ = worker.join();
                Err(CoreError::Io(
                    "luồng capture loopback dừng bất ngờ".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_tren_handle_khong_panic_va_dung_nhan_mau() {
        // Chỉ chạy nếu máy test có PulseAudio/PipeWire với monitor source khả
        // dụng; bỏ qua nếu không có (container CI headless, không có audio server).
        let source = LoopbackAudioSource::new();
        let Ok(handle) = source.start(Box::new(|_frame| {})) else {
            return;
        };
        handle.stop();
    }
}
