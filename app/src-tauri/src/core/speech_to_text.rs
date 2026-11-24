use super::error::CoreError;
use super::types::{AudioFrame, TranscriptChunk};
use async_trait::async_trait;

/// Nhận dạng giọng nói: nhận một khối âm thanh, trả về văn bản (tạm hoặc chốt).
#[async_trait]
pub trait SpeechToText: Send + Sync {
    async fn transcribe(&self, frame: AudioFrame) -> Result<TranscriptChunk, CoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Cài đặt giả: lần gọi đầu trả partial, lần sau trả final.
    struct MockSpeechToText {
        call_count: AtomicUsize,
    }

    impl MockSpeechToText {
        fn new() -> Self {
            Self {
                call_count: AtomicUsize::new(0),
            }
        }
    }

    #[async_trait]
    impl SpeechToText for MockSpeechToText {
        async fn transcribe(&self, _frame: AudioFrame) -> Result<TranscriptChunk, CoreError> {
            let n = self.call_count.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                Ok(TranscriptChunk {
                    text: "xin".into(),
                    is_final: false,
                })
            } else {
                Ok(TranscriptChunk {
                    text: "xin chào".into(),
                    is_final: true,
                })
            }
        }
    }

    fn dummy_frame() -> AudioFrame {
        AudioFrame {
            samples: vec![0.0; 4],
            sample_rate: 16_000,
        }
    }

    #[tokio::test]
    async fn tra_ve_partial_roi_final() {
        let stt = MockSpeechToText::new();

        let first = stt.transcribe(dummy_frame()).await.unwrap();
        assert!(!first.is_final);
        assert_eq!(first.text, "xin");

        let second = stt.transcribe(dummy_frame()).await.unwrap();
        assert!(second.is_final);
        assert_eq!(second.text, "xin chào");
    }

    #[tokio::test]
    async fn goi_qua_trait_object_khong_biet_cai_dat_cu_the() {
        let stt: Box<dyn SpeechToText> = Box::new(MockSpeechToText::new());
        let result = stt.transcribe(dummy_frame()).await.unwrap();
        assert!(!result.is_final);
    }
}
