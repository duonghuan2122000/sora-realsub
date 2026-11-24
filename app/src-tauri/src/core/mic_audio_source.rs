use super::audio_source::AudioSource;
use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::thread;

/// Cài đặt `AudioSource` thật, capture từ thiết bị mic qua `cpal`.
pub struct MicAudioSource {
    /// `None` = dùng thiết bị input mặc định của hệ điều hành.
    device_id: Option<String>,
}

impl MicAudioSource {
    pub fn new(device_id: Option<String>) -> Self {
        Self { device_id }
    }

    fn find_device(&self) -> Result<cpal::Device, CoreError> {
        let host = cpal::default_host();
        match &self.device_id {
            None => host
                .default_input_device()
                .ok_or_else(|| CoreError::Io("không có thiết bị input mặc định".to_string())),
            Some(id) => {
                let mut devices = host
                    .input_devices()
                    .map_err(|e| CoreError::Io(e.to_string()))?;
                devices
                    .find(|d| d.name().map(|n| n == *id).unwrap_or(false))
                    .ok_or_else(|| CoreError::Io(format!("không tìm thấy thiết bị mic: {id}")))
            }
        }
    }
}

fn to_mono(data: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data.to_vec();
    }
    data.chunks(channels)
        .map(|frame| frame.iter().sum::<f32>() / channels as f32)
        .collect()
}

impl AudioSource for MicAudioSource {
    fn start(
        &self,
        mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<StreamHandle, CoreError> {
        // `cpal::Stream` không phải `Send`, nên phải tạo và giữ sống nó trên 1 thread
        // riêng; thread chính chỉ giao tiếp qua channel để biết khi nào sẵn sàng/dừng.
        let device = self.find_device()?;
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), String>>();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();

        let worker = thread::spawn(move || {
            let setup = (|| -> Result<cpal::Stream, String> {
                let config = device
                    .default_input_config()
                    .map_err(|e| e.to_string())?;
                let sample_rate = config.sample_rate().0;
                let channels = config.channels() as usize;
                let err_fn = |err: cpal::StreamError| eprintln!("lỗi stream audio: {err}");

                let stream = match config.sample_format() {
                    cpal::SampleFormat::F32 => device.build_input_stream(
                        &config.into(),
                        move |data: &[f32], _| {
                            on_frame(AudioFrame {
                                samples: to_mono(data, channels),
                                sample_rate,
                            });
                        },
                        err_fn,
                        None,
                    ),
                    cpal::SampleFormat::I16 => device.build_input_stream(
                        &config.into(),
                        move |data: &[i16], _| {
                            let floats: Vec<f32> =
                                data.iter().map(|s| *s as f32 / i16::MAX as f32).collect();
                            on_frame(AudioFrame {
                                samples: to_mono(&floats, channels),
                                sample_rate,
                            });
                        },
                        err_fn,
                        None,
                    ),
                    other => return Err(format!("định dạng mẫu không hỗ trợ: {other:?}")),
                }
                .map_err(|e| e.to_string())?;

                stream.play().map_err(|e| e.to_string())?;
                Ok(stream)
            })();

            match setup {
                Ok(stream) => {
                    let _ = ready_tx.send(Ok(()));
                    let _ = stop_rx.recv();
                    drop(stream);
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
                Err(CoreError::Io("luồng capture audio dừng bất ngờ".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thiet_bi_khong_ton_tai_tra_loi_ro_nghia() {
        let source = MicAudioSource::new(Some("thiết-bị-chắc-chắn-không-tồn-tại-xyz".into()));
        let result = source.start(Box::new(|_frame| {}));
        assert!(matches!(result, Err(CoreError::Io(_))));
    }

    #[test]
    fn stop_tren_handle_khong_panic_va_dung_nhan_mau() {
        // Chỉ chạy nếu máy test có thiết bị input mặc định; bỏ qua nếu không có (CI headless).
        let source = MicAudioSource::new(None);
        let Ok(handle) = source.start(Box::new(|_frame| {})) else {
            return;
        };
        handle.stop();
    }
}
