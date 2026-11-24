use super::error::CoreError;
use super::mixed_audio_source::resample_linear;
use super::types::{AudioFrame, TranscriptChunk};
use async_trait::async_trait;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Nhận dạng giọng nói: nhận một khối âm thanh, trả về văn bản (tạm hoặc chốt).
#[async_trait]
pub trait SpeechToText: Send + Sync {
    async fn transcribe(&self, frame: AudioFrame) -> Result<TranscriptChunk, CoreError>;
}

/// Sample rate whisper.cpp yêu cầu: 16kHz mono f32 (xem research.md PBI 12).
const WHISPER_SAMPLE_RATE: u32 = 16_000;

/// Cài đặt `SpeechToText` bằng whisper.cpp (binding `whisper-rs`), chạy hoàn toàn local.
pub struct WhisperSpeechToText {
    ctx: WhisperContext,
}

impl WhisperSpeechToText {
    /// Nạp model ggml/gguf từ đường dẫn file cục bộ.
    pub fn new(model_path: &str) -> Result<Self, CoreError> {
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .map_err(|e| CoreError::Provider(format!("không nạp được model whisper: {e}")))?;
        Ok(Self { ctx })
    }
}

#[async_trait]
impl SpeechToText for WhisperSpeechToText {
    async fn transcribe(&self, frame: AudioFrame) -> Result<TranscriptChunk, CoreError> {
        let samples = resample_linear(&frame.samples, frame.sample_rate, WHISPER_SAMPLE_RATE);

        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| CoreError::Provider(format!("không tạo được state whisper: {e}")))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_progress(false);
        params.set_print_special(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        state
            .full(params, &samples)
            .map_err(|e| CoreError::Provider(format!("lỗi infer whisper: {e}")))?;

        let num_segments = state.full_n_segments();

        let mut text = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i)
                && let Ok(segment_text) = segment.to_str()
            {
                let segment_text = segment_text.trim();
                // whisper.cpp trả tag dạng "[BLANK_AUDIO]"/"[SILENCE]" thay vì rỗng khi
                // không có giọng nói — không phải nội dung đã nói, bỏ qua (FR-003).
                let is_non_speech_tag =
                    segment_text.starts_with('[') && segment_text.ends_with(']');
                if segment_text.is_empty() || is_non_speech_tag {
                    continue;
                }
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(segment_text);
            }
        }

        Ok(TranscriptChunk {
            text,
            is_final: true,
        })
    }
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
