//! Cài đặt `LoopbackAudioSource` cho Windows (WASAPI loopback). Chỉ build
//! trên Windows — bản Linux xem `loopback_audio_source_linux.rs`.
#![cfg(windows)]

use super::audio_source::AudioSource;
use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};
use std::sync::mpsc;
use std::thread;
use wasapi::{DeviceEnumerator, Direction, SampleType, StreamMode, initialize_mta};

/// Cài đặt `AudioSource` thật, capture loopback từ thiết bị output mặc định
/// của hệ thống (Windows WASAPI loopback), qua crate `wasapi`.
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

fn decode_samples(bytes: &[u8], sample_type: SampleType, bytes_per_sample: usize) -> Vec<f32> {
    match (sample_type, bytes_per_sample) {
        (SampleType::Float, 4) => bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect(),
        (SampleType::Int, 4) => bytes
            .chunks_exact(4)
            .map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f32 / i32::MAX as f32)
            .collect(),
        (SampleType::Int, 2) => bytes
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / i16::MAX as f32)
            .collect(),
        _ => Vec::new(),
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

impl AudioSource for LoopbackAudioSource {
    fn start(
        &self,
        mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<StreamHandle, CoreError> {
        // `AudioClient`/`AudioCaptureClient` của `wasapi` không phải `Send`, nên
        // phải tạo và giữ sống chúng trên 1 thread riêng, giống `MicAudioSource`.
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), String>>();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();

        let worker = thread::spawn(move || {
            let setup = (|| -> Result<_, String> {
                initialize_mta().ok().map_err(|e| e.to_string())?;

                let enumerator = DeviceEnumerator::new().map_err(|e| e.to_string())?;
                let device = enumerator
                    .get_default_device(&Direction::Render)
                    .map_err(|_| "không có thiết bị output mặc định".to_string())?;
                let format = device.get_device_format().map_err(|e| e.to_string())?;
                let sample_type = format.get_subformat().map_err(|e| e.to_string())?;
                let bits_per_sample = format.get_bitspersample();
                if !matches!(
                    (sample_type, bits_per_sample),
                    (SampleType::Float, 32) | (SampleType::Int, 16) | (SampleType::Int, 32)
                ) {
                    return Err(format!(
                        "định dạng mẫu không hỗ trợ: {sample_type:?} {bits_per_sample} bit"
                    ));
                }
                let sample_rate = format.get_samplespersec();
                let channels = format.get_nchannels() as usize;

                let mut audio_client = device.get_iaudioclient().map_err(|e| e.to_string())?;
                let (_def_time, min_time) = audio_client
                    .get_device_period()
                    .map_err(|e| e.to_string())?;
                let mode = StreamMode::EventsShared {
                    autoconvert: true,
                    buffer_duration_hns: min_time,
                };
                audio_client
                    .initialize_client(&format, &Direction::Capture, &mode)
                    .map_err(|e| e.to_string())?;
                let event_handle = audio_client
                    .set_get_eventhandle()
                    .map_err(|e| e.to_string())?;
                let capture_client = audio_client
                    .get_audiocaptureclient()
                    .map_err(|e| e.to_string())?;
                audio_client.start_stream().map_err(|e| e.to_string())?;

                Ok((
                    audio_client,
                    capture_client,
                    event_handle,
                    sample_rate,
                    channels,
                    sample_type,
                    bits_per_sample,
                ))
            })();

            match setup {
                Ok((
                    audio_client,
                    capture_client,
                    event_handle,
                    sample_rate,
                    channels,
                    sample_type,
                    bits_per_sample,
                )) => {
                    let _ = ready_tx.send(Ok(()));
                    let bytes_per_sample = (bits_per_sample / 8) as usize;
                    let mut byte_buf: std::collections::VecDeque<u8> =
                        std::collections::VecDeque::new();
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        if capture_client
                            .read_from_device_to_deque(&mut byte_buf)
                            .is_err()
                        {
                            break;
                        }
                        let frame_bytes = channels * bytes_per_sample;
                        let whole_frames = byte_buf.len() / frame_bytes;
                        if whole_frames > 0 {
                            let n = whole_frames * frame_bytes;
                            let bytes: Vec<u8> = byte_buf.drain(..n).collect();
                            let samples: Vec<f32> =
                                decode_samples(&bytes, sample_type, bytes_per_sample);
                            on_frame(AudioFrame {
                                samples: to_mono(&samples, channels),
                                sample_rate,
                            });
                        }
                        if event_handle.wait_for_event(1000).is_err() {
                            break;
                        }
                    }
                    let _ = audio_client.stop_stream();
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
        // Chỉ chạy nếu máy test có thiết bị output mặc định; bỏ qua nếu không có (CI headless).
        let source = LoopbackAudioSource::new();
        let Ok(handle) = source.start(Box::new(|_frame| {})) else {
            return;
        };
        handle.stop();
    }
}
