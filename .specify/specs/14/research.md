# Research: Event transcript-partial/transcript-final

**Mã PBI**: 14

## Quyết định 1 — Cơ chế phát event từ `TranscriptPipeline`

**Quyết định**: Thêm callback `on_chunk: FnMut(TranscriptChunk)` (tương tự `VadPipeline::push_frame` trả `Vec<SegmentReady>`) vào `TranscriptPipeline::process_segments`, gọi callback ngay sau mỗi segment được STT xử lý xong — thay vì gom hết rồi trả `Vec` một lần như PBI 13.
**Lý do**: Giữ nguyên interface hiện có càng nhiều càng tốt, nhưng cần phát từng chunk ngay khi có để UI nhận đúng thứ tự thời gian thực (FR-003), không phải đợi toàn bộ segment xử lý xong. Pattern giống `start_capture` đang emit `vad-segment` ngay trong callback của `VadPipeline`.
**Phương án khác đã xem xét**: Trả `Vec<TranscriptChunk>` như cũ rồi lib.rs emit lần lượt sau khi `await` toàn bộ — bị loại vì làm mất tính "ngay khi có kết quả", không khớp luồng streaming thực tế khi nối vào capture session.

## Quyết định 2 — `transcript-partial` khi `SpeechToText` hiện tại không hỗ trợ streaming thật

**Quyết định**: `WhisperSpeechToText::transcribe` hiện trả đúng 1 `TranscriptChunk { is_final: true }` cho mỗi segment (không có bước tạm). Theo giả định đã ghi trong spec.md, PBI này KHÔNG bắt buộc đổi whisper sang chế độ streaming thật. `TranscriptPipeline` chỉ cần: nếu `chunk.is_final == false` → phát `transcript-partial`; nếu `true` → phát `transcript-final`. Với STT hiện tại luôn `is_final = true`, pipeline chỉ phát `transcript-final` cho mỗi segment — thỏa đúng FR-001/FR-005, bỏ qua `transcript-partial` cho segment đó (đã liệt kê trong "Trường hợp biên" của spec.md).
**Lý do**: Tách rời đúng phạm vi PBI — đổi whisper sang streaming thật là thay đổi lớn hơn, không phải mục tiêu của "phát event cho UI". Giữ cổng mở: field `is_final` đã có sẵn trong `TranscriptChunk`, khi sau này STT hỗ trợ partial thật, chỉ cần set `is_final = false` ở lần gọi trung gian, pipeline tự động phát đúng loại event, không cần sửa `TranscriptPipeline`/lib.rs.
**Phương án khác đã xem xét**: Giả lập `transcript-partial` giả (ví dụ emit text rỗng trước mỗi final) — bị loại vì tạo dữ liệu giả không phản ánh trạng thái thật, dễ gây hiểu sai khi test E2E kiểm tra nội dung.

## Quyết định 3 — Định danh segment trong payload event

**Quyết định**: Payload `TranscriptEventPayload` mang `startFrameIndex`/`endFrameIndex` (lấy từ `SegmentReady` gốc, giống cách `vad-segment` đã làm), cộng `text` và `isFinal`.
**Lý do**: Cho UI xác định event nào thuộc cùng 1 segment (ghép partial → final), nhất quán với contract `vad-segment` đã có ở PBI 11, không cần sinh thêm ID mới.
**Phương án khác đã xem xét**: Sinh UUID riêng cho mỗi segment — bị loại vì thêm độ phức tạp không cần thiết khi `startFrameIndex`/`endFrameIndex` đã đủ duy nhất trong 1 phiên capture.
