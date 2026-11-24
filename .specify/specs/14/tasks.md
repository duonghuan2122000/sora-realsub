# Danh sách Task: Event transcript-partial/transcript-final cho UI

**Mã PBI**: 14
**Nguồn**: plan.md, spec.md, data-model.md, contracts/transcript-events.md, research.md

## Pha 1: Setup

- [X] T001 Đọc lại `app/src-tauri/src/core/transcript_pipeline.rs`, `app/src-tauri/src/core/types.rs`, `app/src-tauri/src/lib.rs` để xác nhận điểm nối callback (không tạo file mới ở pha này).

## Pha 2: Foundational

- [X] T002 Sửa `app/src-tauri/src/core/transcript_pipeline.rs`: đổi `TranscriptPipeline::process_segments` nhận thêm tham số `on_chunk: impl FnMut(&SegmentReady, &TranscriptChunk)` (hoặc kiểu tương đương), gọi `on_chunk` ngay sau mỗi `self.stt.transcribe(...)` thành công, giữ nguyên việc trả `Vec<TranscriptChunk>` để không phá test PBI 13 hiện có.
- [X] T003 Cập nhật các test hiện có trong `app/src-tauri/src/core/transcript_pipeline.rs` (mục `#[cfg(test)] mod tests`) cho khớp signature mới của `process_segments` (truyền callback rỗng `|_, _| {}` nếu test không quan tâm callback).

## Pha 3: User Story 1 - Phát transcript-final đúng thứ tự (Ưu tiên: P1)

**Mục tiêu**: Mỗi segment thoại được STT xử lý xong phải phát ra đúng 1 event `transcript-final` (khi `is_final == true`), đúng thứ tự thời gian, audio im lặng không phát event nào.
**Tiêu chí kiểm thử độc lập**: Chạy `cargo test` trong `app/src-tauri` cho `TranscriptPipeline` với `MockSpeechToText` trả `is_final: true` — callback nhận đủ số lần, đúng thứ tự, nội dung khớp; danh sách segment rỗng → callback không bị gọi.

- [X] T004 [US1] Trong `app/src-tauri/src/core/transcript_pipeline.rs`, thêm test `phat_transcript_final_dung_thu_tu` dùng `MockSpeechToText` (đã có) để xác nhận callback nhận `(segment, chunk)` theo đúng thứ tự `segments` đầu vào (FR-001, FR-003, SC-001).
- [X] T005 [US1] Trong `app/src-tauri/src/core/transcript_pipeline.rs`, thêm test `danh_sach_rong_khong_goi_callback` xác nhận `segments = vec![]` → callback không được gọi lần nào, không lỗi (FR-005).
- [X] T006 [US1] Trong `app/src-tauri/src/lib.rs`: thêm struct `TranscriptEventPayload` (serde `rename_all = "camelCase"`, field `start_frame_index`, `end_frame_index`, `text`, `is_final`) theo `data-model.md`/`contracts/transcript-events.md`.
- [X] T007 [US1] Trong `app/src-tauri/src/lib.rs`, hàm `start_capture`: sau khi `VadPipeline` trả `ready_segments`, với mỗi segment gọi `TranscriptPipeline::process_segments` (hoặc gọi `stt.transcribe` trực tiếp qua pipeline mới) kèm callback emit `app_handle.emit("transcript-final", TranscriptEventPayload::from(...))` khi `chunk.is_final == true`, và `app_handle.emit("transcript-partial", ...)` khi `chunk.is_final == false`, theo đúng tên event trong `contracts/transcript-events.md`.

## Pha 4: User Story 2 - Thứ tự partial → final cho cùng 1 segment (Ưu tiên: P2)

**Mục tiêu**: Khi STT có kết quả tạm trước khi chốt, UI phải nhận `transcript-partial` trước `transcript-final` của cùng segment.
**Tiêu chí kiểm thử độc lập**: Test mock STT trả về nhiều `TranscriptChunk` cho cùng 1 segment (partial rồi final) — callback phải nhận đúng thứ tự partial → final.

- [X] T008 [US2] Trong `app/src-tauri/src/core/transcript_pipeline.rs`, thêm test `partial_den_truoc_final_cung_segment` dùng mock STT giả lập 2 lần gọi cho cùng segment (lần 1 `is_final: false`, lần 2 `is_final: true`), xác nhận callback nhận đúng thứ tự (FR-002, SC-002). Nếu `SpeechToText::transcribe` hiện tại chỉ trả 1 lần/segment (không hỗ trợ multi-chunk), ghi rõ trong test/comment rằng test này xác nhận hành vi callback tổng quát (nhận nhiều chunk cho 1 segment nếu STT cung cấp), không thay đổi `WhisperSpeechToText` ở PBI này — khớp giả định trong spec.md.

## Pha cuối: Polish & Cross-cutting

- [X] T009 Chạy `cargo test` toàn bộ trong `app/src-tauri`, xác nhận không có test nào (PBI 11, 12, 13 cũ) bị hỏng do thay đổi signature `process_segments`.
- [X] T010 Chụp ảnh/log kết quả `cargo test` (toàn bộ test transcript_pipeline pass) làm bằng chứng, lưu vào `.specify/specs/14/evidence/`.

## Sơ đồ phụ thuộc

- Pha 1 (Setup) → Pha 2 (Foundational: đổi signature `process_segments`) → Pha 3 (US1: phát `transcript-final`, bắt buộc để có MVP) → Pha 4 (US2: thứ tự partial → final, phụ thuộc cơ chế callback đã có ở Pha 2/3) → Pha cuối (Polish).
- US2 phụ thuộc US1 (cùng dùng callback vừa thêm ở T002/T007), không độc lập hoàn toàn nhưng chỉ thêm test, không sửa lại code đã viết ở US1.

## Ví dụ chạy song song

- T004 và T005 có thể làm song song (khác test function, cùng file nhưng không phụ thuộc nhau về logic) — không đánh `[P]` vì cùng sửa 1 file `transcript_pipeline.rs`, nên gộp làm tuần tự trong cùng 1 lần sửa để tránh conflict khi áp dụng diff.
- T006 (lib.rs, định nghĩa payload) có thể làm trước/song song với T004/T005 (khác file `lib.rs` vs `transcript_pipeline.rs`) — đánh dấu `[P]` nếu triển khai bởi 2 người/luồng khác nhau.

## Chiến lược triển khai

- MVP đề xuất: User Story 1 (phát `transcript-final` đúng thứ tự, audio im lặng không event) — đủ để UI có thể bắt đầu hiển thị phụ đề thật ở M5.
- Thứ tự giao hàng tăng dần: hoàn thành Setup + Foundational + US1 trước, verify bằng `cargo test`; sau đó thêm US2 (partial → final) nếu cần chuẩn bị sớm cho STT streaming thật trong tương lai; Polish ở cuối.
