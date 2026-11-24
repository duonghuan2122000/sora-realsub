# Danh sách Task: Ghép STT vào luồng VAD → Transcript

**Mã PBI**: 13
**Nguồn**: plan.md, spec.md, data-model.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Khai báo module `transcript_pipeline` trong `app/src-tauri/src/core/mod.rs`

## Pha 2: Foundational

*(Không có — PBI này tái dùng nguyên trạng `VadPipeline` và `SpeechToText` đã có từ PBI 11-12, không cần hạ tầng mới trước khi làm User Story)*

## Pha 3: User Story 1 - Ghép segment VAD qua STT giữ đúng thứ tự (Ưu tiên: P1)

**Mục tiêu**: Mỗi `SegmentReady` từ `VadPipeline` được đưa qua `SpeechToText`, kết quả `TranscriptChunk` trả về đúng thứ tự thời gian (FR-001, FR-002, FR-004).
**Tiêu chí kiểm thử độc lập**: Chạy file audio mẫu nhiều đoạn nói xen lặng qua `VadPipeline` rồi `TranscriptPipeline`, nhận đúng số lượng + đúng thứ tự + đúng nội dung từng đoạn text.

- [X] T002 [US1] Cài đặt `struct TranscriptPipeline` + `async fn process_segments(&self, segments: Vec<SegmentReady>) -> Result<Vec<TranscriptChunk>, CoreError>` tại `app/src-tauri/src/core/transcript_pipeline.rs` (nhận `&dyn SpeechToText`, chuyển từng `SegmentReady` thành `AudioFrame`, gọi tuần tự `transcribe`, gom kết quả theo thứ tự — xem data-model.md)
- [X] T003 [US1] Viết unit test (mock `SpeechToText`) tại `app/src-tauri/src/core/transcript_pipeline.rs` xác nhận: nhiều segment trả đúng thứ tự text; `Vec` segment rỗng trả `Vec` kết quả rỗng không lỗi (FR-003)
- [X] T004 [US1] Viết test tích hợp `app/src-tauri/tests/transcript_pipeline_real_speech.rs` dùng file audio mẫu thật + `WhisperSpeechToText` (PBI 12): nạp `.wav` → `AudioFrame` → `VadPipeline::push_frame` → `TranscriptPipeline::process_segments`, theo kịch bản quickstart.md (nhiều đoạn, một đoạn, toàn im lặng)
- [X] T005 [US1] Chạy `cargo test` xác nhận toàn bộ test T003-T004 pass, không crash; chụp/lưu bằng chứng kết quả test theo yêu cầu `/sora-implement`

## Pha cuối: Polish & Cross-cutting

- [X] T006 Rà soát doc-comment tiếng Việt trên `TranscriptPipeline` và `process_segments` tham chiếu đúng FR/data-model.md (nhất quán style với `vad_pipeline.rs`, `speech_to_text.rs`)

## Sơ đồ phụ thuộc

- T001 → T002 (cần khai báo module trước khi file tồn tại trong build).
- T002 → T003, T004 (cần cài đặt xong mới test được).
- T003, T004 → T005 (chạy test sau khi đã viết).
- T005 → T006 (polish sau khi test pass).
- Chỉ 1 User Story (US1) — không có story song song khác ở PBI này.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (T002-T005) — đây là toàn bộ phạm vi PBI 13.
- Thứ tự giao hàng: T001 → T002 → (T003, T004 có thể viết song song vì khác file test, nhưng cả hai phụ thuộc T002 đã xong) → T005 → T006.
