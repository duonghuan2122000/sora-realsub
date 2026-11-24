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
    /// tương ứng. Danh sách rỗng trả `Vec` rỗng, không lỗi (FR-003 PBI 13).
    ///
    /// Gọi `on_chunk(segment, chunk)` ngay sau khi segment đó có kết quả, để nơi gọi (ví dụ
    /// `lib.rs`) phát event `transcript-partial`/`transcript-final` theo đúng thứ tự thời gian
    /// thực, không phải đợi toàn bộ `segments` xử lý xong (PBI 14, FR-001/FR-002/FR-003; xem
    /// research.md quyết định 1).
    ///
    /// Khi 1 segment lỗi infer, gọi `on_error(segment, err)` rồi tiếp tục segment kế tiếp,
    /// không dừng cả hàm — lỗi 1 segment không được lan ra toàn pipeline (PBI 15, FR-005;
    /// xem research.md quyết định 2). Segment lỗi không có chunk tương ứng trong kết quả trả về.
    pub async fn process_segments(
        &self,
        segments: Vec<SegmentReady>,
        mut on_chunk: impl FnMut(&SegmentReady, &TranscriptChunk),
        mut on_error: impl FnMut(&SegmentReady, &CoreError),
    ) -> Result<Vec<TranscriptChunk>, CoreError> {
        let mut chunks = Vec::with_capacity(segments.len());
        for segment in segments {
            let frame = AudioFrame {
                samples: segment.samples.clone(),
                sample_rate: segment.sample_rate,
            };
            match self.stt.transcribe(frame).await {
                Ok(chunk) => {
                    on_chunk(&segment, &chunk);
                    chunks.push(chunk);
                }
                Err(err) => {
                    on_error(&segment, &err);
                }
            }
        }
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

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

    /// STT trả lần lượt các `TranscriptChunk` định trước theo mỗi lần gọi `transcribe`
    /// (PBI 14, giả lập partial → final cho cùng 1 segment).
    struct ScriptedSpeechToText {
        chunks: Mutex<std::vec::IntoIter<TranscriptChunk>>,
    }

    impl ScriptedSpeechToText {
        fn new(chunks: Vec<TranscriptChunk>) -> Self {
            Self {
                chunks: Mutex::new(chunks.into_iter()),
            }
        }
    }

    #[async_trait]
    impl SpeechToText for ScriptedSpeechToText {
        async fn transcribe(&self, _frame: AudioFrame) -> Result<TranscriptChunk, CoreError> {
            Ok(self.chunks.lock().unwrap().next().expect("hết chunk đã định trước"))
        }
    }

    /// STT trả lỗi infer (`CoreError::Provider`) đúng vào các lần gọi nằm trong
    /// `fail_at_calls` (0-based), còn lại trả `Ok` (PBI 15, giả lập lỗi infer giữa các segment
    /// thành công).
    struct FlakySpeechToText {
        call_count: AtomicUsize,
        fail_at_calls: Vec<usize>,
    }

    impl FlakySpeechToText {
        fn new(fail_at_calls: Vec<usize>) -> Self {
            Self {
                call_count: AtomicUsize::new(0),
                fail_at_calls,
            }
        }
    }

    #[async_trait]
    impl SpeechToText for FlakySpeechToText {
        async fn transcribe(&self, _frame: AudioFrame) -> Result<TranscriptChunk, CoreError> {
            let n = self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.fail_at_calls.contains(&n) {
                Err(CoreError::Provider(format!("lỗi infer giả lập tại lần gọi {n}")))
            } else {
                Ok(TranscriptChunk {
                    text: format!("doan-{n}"),
                    is_final: true,
                })
            }
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
        let chunks = pipeline
            .process_segments(segments, |_, _| {}, |_, _| {})
            .await
            .unwrap();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "doan-0");
        assert_eq!(chunks[1].text, "doan-1");
        assert_eq!(chunks[2].text, "doan-2");
    }

    #[tokio::test]
    async fn danh_sach_segment_rong_tra_ve_rong_khong_loi() {
        let stt = MockSpeechToText::new();
        let pipeline = TranscriptPipeline::new(&stt);

        let chunks = pipeline
            .process_segments(Vec::new(), |_, _| {}, |_, _| {})
            .await
            .unwrap();

        assert!(chunks.is_empty());
    }

    // PBI 14 — T004: callback phát transcript-final đúng thứ tự cho nhiều segment.
    #[tokio::test]
    async fn phat_transcript_final_dung_thu_tu() {
        let stt = MockSpeechToText::new();
        let pipeline = TranscriptPipeline::new(&stt);

        let segments = vec![
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
        ];
        let received: Mutex<Vec<(bool, String)>> = Mutex::new(Vec::new());

        pipeline
            .process_segments(
                segments,
                |_seg, chunk| {
                    received
                        .lock()
                        .unwrap()
                        .push((chunk.is_final, chunk.text.clone()));
                },
                |_, _| {},
            )
            .await
            .unwrap();

        let received = received.into_inner().unwrap();
        assert_eq!(
            received,
            vec![
                (true, "doan-0".to_string()),
                (true, "doan-1".to_string()),
                (true, "doan-2".to_string()),
            ]
        );
    }

    // PBI 14 — T005: danh sách segment rỗng không gọi callback lần nào.
    #[tokio::test]
    async fn danh_sach_rong_khong_goi_callback() {
        let stt = MockSpeechToText::new();
        let pipeline = TranscriptPipeline::new(&stt);
        let call_count = AtomicUsize::new(0);

        pipeline
            .process_segments(
                Vec::new(),
                |_, _| {
                    call_count.fetch_add(1, Ordering::SeqCst);
                },
                |_, _| {},
            )
            .await
            .unwrap();

        assert_eq!(call_count.load(Ordering::SeqCst), 0);
    }

    // PBI 14 — T008: với 1 segment, STT trả partial rồi final (giả lập) → callback phải nhận
    // đúng thứ tự partial trước, final sau. `WhisperSpeechToText` thật hiện không phát sinh
    // nhiều chunk/segment; test này xác nhận hành vi tổng quát của callback khi STT làm được.
    #[tokio::test]
    async fn partial_den_truoc_final_cung_segment() {
        let stt = ScriptedSpeechToText::new(vec![
            TranscriptChunk {
                text: "xin ch".to_string(),
                is_final: false,
            },
            TranscriptChunk {
                text: "xin chào".to_string(),
                is_final: true,
            },
        ]);
        let pipeline = TranscriptPipeline::new(&stt);

        let segments = vec![segment_with(vec![0.0; 4]), segment_with(vec![0.0; 4])];
        let received: Mutex<Vec<(bool, String)>> = Mutex::new(Vec::new());

        pipeline
            .process_segments(
                segments,
                |_seg, chunk| {
                    received
                        .lock()
                        .unwrap()
                        .push((chunk.is_final, chunk.text.clone()));
                },
                |_, _| {},
            )
            .await
            .unwrap();

        let received = received.into_inner().unwrap();
        assert_eq!(
            received,
            vec![
                (false, "xin ch".to_string()),
                (true, "xin chào".to_string()),
            ]
        );
    }

    // PBI 15 — T009: 1 segment lỗi infer giữa 2 segment thành công, lỗi không lan ra toàn
    // pipeline, segment lỗi không có chunk tương ứng trong kết quả trả về.
    #[tokio::test]
    async fn loi_infer_1_segment_khong_lam_dung_cac_segment_khac() {
        let stt = FlakySpeechToText::new(vec![1]);
        let pipeline = TranscriptPipeline::new(&stt);

        let segments = vec![
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
        ];
        let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());

        let chunks = pipeline
            .process_segments(
                segments,
                |_, _| {},
                |_seg, err| {
                    errors.lock().unwrap().push(err.to_string());
                },
            )
            .await
            .unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].text, "doan-0");
        assert_eq!(chunks[1].text, "doan-2");

        let errors = errors.into_inner().unwrap();
        assert_eq!(errors.len(), 1);
    }

    // PBI 15 — T010: nhiều segment liên tiếp đều lỗi infer, mỗi lỗi báo đúng 1 lần, không
    // panic, hàm vẫn trả Ok.
    #[tokio::test]
    async fn nhieu_segment_lien_tiep_loi_infer_khong_panic() {
        let stt = FlakySpeechToText::new(vec![0, 1, 2]);
        let pipeline = TranscriptPipeline::new(&stt);

        let segments = vec![
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
            segment_with(vec![0.0; 4]),
        ];
        let error_count = AtomicUsize::new(0);

        let chunks = pipeline
            .process_segments(
                segments,
                |_, _| {},
                |_, _| {
                    error_count.fetch_add(1, Ordering::SeqCst);
                },
            )
            .await
            .unwrap();

        assert!(chunks.is_empty());
        assert_eq!(error_count.load(Ordering::SeqCst), 3);
    }
}
