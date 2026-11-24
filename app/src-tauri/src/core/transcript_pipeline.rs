use super::error::CoreError;
use super::speech_to_text::SpeechToText;
use super::types::{AudioFrame, TranscriptChunk};
use super::vad_pipeline::SegmentReady;

/// Ghép segment thoại do `VadPipeline` cắt ra qua `SpeechToText`, giữ đúng thứ tự thời gian
/// (PBI 13, FR-001/FR-002/FR-004; xem data-model.md).
pub struct TranscriptPipeline<'a> {
    stt: &'a dyn SpeechToText,
}

impl<'a> TranscriptPipeline<'a> {
    pub fn new(stt: &'a dyn SpeechToText) -> Self {
        Self { stt }
    }

    /// Đưa từng segment qua STT theo đúng thứ tự trong `segments`, trả về `TranscriptChunk`
    /// tương ứng. Danh sách rỗng trả `Vec` rỗng, không lỗi (FR-003).
    pub async fn process_segments(
        &self,
        segments: Vec<SegmentReady>,
    ) -> Result<Vec<TranscriptChunk>, CoreError> {
        let mut chunks = Vec::with_capacity(segments.len());
        for segment in segments {
            let frame = AudioFrame {
                samples: segment.samples,
                sample_rate: segment.sample_rate,
            };
            chunks.push(self.stt.transcribe(frame).await?);
        }
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
            Ok(TranscriptChunk {
                text: format!("doan-{n}"),
                is_final: true,
            })
        }
    }

    fn segment_with(samples: Vec<f32>) -> SegmentReady {
        SegmentReady {
            start_frame_index: 0,
            end_frame_index: 0,
            samples,
            sample_rate: 16_000,
        }
    }

    #[tokio::test]
    async fn nhieu_segment_tra_ve_text_dung_thu_tu() {
        let stt = MockSpeechToText::new();
        let pipeline = TranscriptPipeline::new(&stt);

        let segments = vec![
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
        ];
        let chunks = pipeline.process_segments(segments).await.unwrap();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "doan-0");
        assert_eq!(chunks[1].text, "doan-1");
        assert_eq!(chunks[2].text, "doan-2");
    }

    #[tokio::test]
    async fn danh_sach_segment_rong_tra_ve_rong_khong_loi() {
        let stt = MockSpeechToText::new();
        let pipeline = TranscriptPipeline::new(&stt);

        let chunks = pipeline.process_segments(Vec::new()).await.unwrap();

        assert!(chunks.is_empty());
    }
}
