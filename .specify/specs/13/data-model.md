# Data model: Ghép STT vào luồng VAD → Transcript (PBI 13)

## `TranscriptPipeline` (mới, `core/transcript_pipeline.rs`)

Bộ điều phối nối `VadPipeline` (PBI 11) và `SpeechToText` (PBI 12) thành một luồng.

- **Trường**: không giữ state ngoài một `Arc<dyn SpeechToText>`/`&dyn SpeechToText` được truyền vào lúc tạo.
- **Hành vi chính**: `async fn process_segments(&self, segments: Vec<SegmentReady>) -> Result<Vec<TranscriptChunk>, CoreError>`
  - Với mỗi `SegmentReady` (theo thứ tự trong `Vec`):
    1. Chuyển `samples` + `sample_rate` thành `AudioFrame` (đã có ở `core::types`).
    2. Gọi `SpeechToText::transcribe(frame)`.
    3. Đẩy `TranscriptChunk` trả về vào kết quả.
  - Trả `Vec<TranscriptChunk>` đúng thứ tự thời gian các segment (FR-002).
  - Nếu danh sách `segments` rỗng (không có đoạn thoại) → trả `Vec` rỗng, không lỗi (FR-003).

## Thực thể đã có, tái dùng không đổi

- **`SegmentReady`** (`core/vad_pipeline.rs`, PBI 11): `start_frame_index`, `end_frame_index`, `samples: Vec<f32>`, `sample_rate: u32`.
- **`AudioFrame`** (`core/types.rs`): `samples: Vec<f32>`, `sample_rate: u32`.
- **`TranscriptChunk`** (`core/types.rs`, PBI 12): `text: String`, `is_final: bool`.
- **`SpeechToText`** trait (`core/speech_to_text.rs`, PBI 12): `async fn transcribe(&self, frame: AudioFrame) -> Result<TranscriptChunk, CoreError>`.

Không có thực thể dữ liệu lưu trữ lâu dài nào ở PBI này — toàn bộ chạy trong một lần gọi, không persist.
