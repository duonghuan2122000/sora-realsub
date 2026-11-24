use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};

/// Nguồn cung cấp dữ liệu âm thanh liên tục (mic, loopback hệ thống, ...).
pub trait AudioSource: Send + Sync {
    /// Bắt đầu capture; gọi `on_frame` mỗi khi có một khối mẫu mới.
    fn start(
        &self,
        on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<StreamHandle, CoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Cài đặt giả: phát ra im lặng (toàn số 0) một lần rồi dừng.
    struct MockAudioSourceSilent;

    impl AudioSource for MockAudioSourceSilent {
        fn start(
            &self,
            mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
        ) -> Result<StreamHandle, CoreError> {
            on_frame(AudioFrame {
                samples: vec![0.0; 4],
                sample_rate: 16_000,
            });
            Ok(StreamHandle::new(|| {}))
        }
    }

    /// Cài đặt giả: phát ra nhiễu (giá trị cố định khác 0) một lần rồi dừng.
    struct MockAudioSourceNoise;

    impl AudioSource for MockAudioSourceNoise {
        fn start(
            &self,
            mut on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
        ) -> Result<StreamHandle, CoreError> {
            on_frame(AudioFrame {
                samples: vec![0.5; 4],
                sample_rate: 16_000,
            });
            Ok(StreamHandle::new(|| {}))
        }
    }

    fn run_and_collect_first_sample(source: &dyn AudioSource) -> f32 {
        let received = Arc::new(Mutex::new(0.0f32));
        let received_clone = received.clone();
        let handle = source
            .start(Box::new(move |frame| {
                *received_clone.lock().unwrap() = frame.samples[0];
            }))
            .expect("start phải thành công");
        handle.stop();
        *received.lock().unwrap()
    }

    #[test]
    fn mock_silent_phat_ra_im_lang() {
        assert_eq!(run_and_collect_first_sample(&MockAudioSourceSilent), 0.0);
    }

    #[test]
    fn mock_noise_phat_ra_nhieu() {
        assert_eq!(run_and_collect_first_sample(&MockAudioSourceNoise), 0.5);
    }

    #[test]
    fn doi_cai_dat_khong_can_sua_code_goi() {
        let sources: Vec<Box<dyn AudioSource>> = vec![
            Box::new(MockAudioSourceSilent),
            Box::new(MockAudioSourceNoise),
        ];
        let results: Vec<f32> = sources
            .iter()
            .map(|s| run_and_collect_first_sample(s.as_ref()))
            .collect();
        assert_eq!(results, vec![0.0, 0.5]);
    }
}
