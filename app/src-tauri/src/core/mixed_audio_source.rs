use super::audio_source::AudioSource;
use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MIX_TICK_MS: u64 = 15;

/// Cài đặt `AudioSource` trộn 2 nguồn con (ví dụ mic + loopback) thành 1 luồng.
/// Mỗi nguồn con chạy độc lập (thread riêng của chính nó); một thread trộn đọc
/// theo chu kỳ cố định từ 2 buffer trung gian, cộng mẫu và giới hạn biên độ.
pub struct MixedAudioSource {
    source_a: Box<dyn AudioSource>,
    source_b: Box<dyn AudioSource>,
}

impl MixedAudioSource {
    pub fn new(source_a: Box<dyn AudioSource>, source_b: Box<dyn AudioSource>) -> Self {
        Self { source_a, source_b }
    }
}

/// Resample đơn giản (linear interpolation) để quy 1 khối mẫu về `target_rate`.
pub fn resample_linear(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate || samples.is_empty() {
        return samples.to_vec();
    }
    let ratio = to_rate as f64 / from_rate as f64;
    let out_len = ((samples.len() as f64) * ratio).round() as usize;
    (0..out_len)
        .map(|i| {
            let src_pos = i as f64 / ratio;
            let idx = src_pos.floor() as usize;
            let frac = (src_pos - idx as f64) as f32;
            let s0 = samples.get(idx).copied().unwrap_or(0.0);
            let s1 = samples.get(idx + 1).copied().unwrap_or(s0);
            s0 + (s1 - s0) * frac
        })
        .collect()
}

/// Buffer trung gian cho 1 nguồn con: giữ mẫu đã được quy về `target_rate` (khi đã biết).
struct SourceBuffer {
    samples: VecDeque<f32>,
    target_rate: Arc<Mutex<Option<u32>>>,
}

impl SourceBuffer {
    fn push(&mut self, frame: AudioFrame) {
        let mut target = self.target_rate.lock().unwrap();
        let rate = *target.get_or_insert(frame.sample_rate);
        drop(target);
        let resampled = resample_linear(&frame.samples, frame.sample_rate, rate);
        self.samples.extend(resampled);
    }

    fn take(&mut self, n: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.samples.pop_front().unwrap_or(0.0));
        }
        out
    }
}

impl AudioSource for MixedAudioSource {
    fn start(
        &self,
        mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<StreamHandle, CoreError> {
        let target_rate = Arc::new(Mutex::new(None::<u32>));

        let buf_a = Arc::new(Mutex::new(SourceBuffer {
            samples: VecDeque::new(),
            target_rate: target_rate.clone(),
        }));
        let buf_b = Arc::new(Mutex::new(SourceBuffer {
            samples: VecDeque::new(),
            target_rate: target_rate.clone(),
        }));

        let buf_a_cb = buf_a.clone();
        let handle_a = self.source_a.start(Box::new(move |frame| {
            buf_a_cb.lock().unwrap().push(frame);
        }))?;

        let buf_b_cb = buf_b.clone();
        let handle_b = match self.source_b.start(Box::new(move |frame| {
            buf_b_cb.lock().unwrap().push(frame);
        })) {
            Ok(h) => h,
            Err(e) => {
                handle_a.stop();
                return Err(e);
            }
        };

        let stop_flag = Arc::new(Mutex::new(false));
        let stop_flag_mixer = stop_flag.clone();
        let mixer = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(MIX_TICK_MS));
                if *stop_flag_mixer.lock().unwrap() {
                    break;
                }
                let Some(rate) = *target_rate.lock().unwrap() else {
                    continue;
                };
                let n = (rate as u64 * MIX_TICK_MS / 1000).max(1) as usize;
                let a = buf_a.lock().unwrap().take(n);
                let b = buf_b.lock().unwrap().take(n);
                let mixed: Vec<f32> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(x, y)| (x + y).clamp(-1.0, 1.0))
                    .collect();
                on_frame(AudioFrame {
                    samples: mixed,
                    sample_rate: rate,
                });
            }
        });

        Ok(StreamHandle::new(move || {
            *stop_flag.lock().unwrap() = true;
            let _ = mixer.join();
            handle_a.stop();
            handle_b.stop();
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct MockSource {
        samples: Vec<f32>,
        sample_rate: u32,
    }

    impl AudioSource for MockSource {
        fn start(
            &self,
            mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
        ) -> Result<StreamHandle, CoreError> {
            on_frame(AudioFrame {
                samples: self.samples.clone(),
                sample_rate: self.sample_rate,
            });
            Ok(StreamHandle::new(|| {}))
        }
    }

    struct SilentSource {
        sample_rate: u32,
    }

    impl AudioSource for SilentSource {
        fn start(
            &self,
            _on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
        ) -> Result<StreamHandle, CoreError> {
            Ok(StreamHandle::new(|| {}))
        }
    }

    fn collect_for(
        handle_builder: impl FnOnce(Box<dyn FnMut(AudioFrame) + Send>) -> StreamHandle,
        wait_ms: u64,
    ) -> Vec<f32> {
        let collected = Arc::new(Mutex::new(Vec::new()));
        let collected_cb = collected.clone();
        let handle = handle_builder(Box::new(move |frame| {
            collected_cb.lock().unwrap().extend(frame.samples);
        }));
        thread::sleep(Duration::from_millis(wait_ms));
        handle.stop();
        Arc::try_unwrap(collected).unwrap().into_inner().unwrap()
    }

    #[test]
    fn tron_hai_nguon_cung_co_tieng_cho_ra_tong_co_clamp() {
        let a = Box::new(MockSource {
            samples: vec![0.6; 32],
            sample_rate: 16_000,
        });
        let b = Box::new(MockSource {
            samples: vec![0.6; 32],
            sample_rate: 16_000,
        });
        let mixer = MixedAudioSource::new(a, b);
        let out = collect_for(|cb| mixer.start(cb).expect("start"), 60);
        assert!(out.len() >= 32);
        assert!(out[..32].iter().all(|s| (*s - 1.0).abs() < 1e-6));
    }

    #[test]
    fn mot_nguon_im_lang_output_khop_nguon_con_lai() {
        let a = Box::new(MockSource {
            samples: vec![0.3; 32],
            sample_rate: 16_000,
        });
        let b = Box::new(SilentSource {
            sample_rate: 16_000,
        });
        let mixer = MixedAudioSource::new(a, b);
        let out = collect_for(|cb| mixer.start(cb).expect("start"), 60);
        assert!(out.len() >= 32);
        assert!(out[..32].iter().all(|s| (*s - 0.3).abs() < 1e-6));
    }

    #[test]
    fn stop_khong_panic_va_dung_nhan_mau_moi() {
        let a = Box::new(SilentSource {
            sample_rate: 16_000,
        });
        let b = Box::new(SilentSource {
            sample_rate: 16_000,
        });
        let mixer = MixedAudioSource::new(a, b);
        let handle = mixer.start(Box::new(|_frame| {})).expect("start");
        handle.stop();
    }

    #[test]
    fn resample_linear_giu_nguyen_khi_cung_rate() {
        let samples = vec![0.1, 0.2, 0.3];
        assert_eq!(resample_linear(&samples, 16_000, 16_000), samples);
    }

    #[test]
    fn resample_linear_doi_do_dai_khi_rate_khac_nhau() {
        let samples = vec![0.0, 1.0, 0.0, 1.0];
        let out = resample_linear(&samples, 8_000, 16_000);
        assert_eq!(out.len(), 8);
    }
}
