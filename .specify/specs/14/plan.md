# Kế hoạch triển khai: Event transcript-partial/transcript-final cho UI

**Mã PBI**: 14
**Liên kết spec**: .specify/specs/14/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Core, Tauri 2), async qua `tokio`/`async-trait` đã dùng sẵn trong `TranscriptPipeline`.
- Framework / Thư viện chính: Tauri 2 (`tauri::Emitter::emit`), `serde` (camelCase payload), kế thừa `core::vad_pipeline::SegmentReady`, `core::types::TranscriptChunk`, `core::transcript_pipeline::TranscriptPipeline`.
- Lưu trữ dữ liệu: không có — chỉ phát event tức thời, không lưu transcript (thuộc M7).
- Kiểm thử: `cargo test` trong `app/src-tauri` (unit + integration test mới cho `TranscriptPipeline` với callback), test mock `SpeechToText` theo pattern đã có ở PBI 13.
- Nền tảng triển khai: không đổi — vẫn trong app Tauri desktop hiện có (Windows).
- Ràng buộc hiệu năng: không yêu cầu mới; giữ nguyên tốc độ pipeline VAD → STT đã có từ PBI 13.
- Ràng buộc khác: không đổi logic `SpeechToText`/VAD hiện tại; chỉ thêm cơ chế phát event, giữ khả năng mở rộng khi STT hỗ trợ partial thật sau này (xem research.md quyết định 2).

## Kiểm tra theo hiến pháp dự án

Dự án chưa có `.specify/memory/constitution.md`. Đối chiếu với nguyên tắc nêu trong `CLAUDE.md` (Core/UI tách biệt qua Tauri IPC):

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) và UI (Vue) tách biệt, không đưa logic nghiệp vụ vào UI | ✅ | Toàn bộ logic quyết định partial/final nằm trong `TranscriptPipeline` (Core); UI chỉ lắng nghe event, không tính toán |
| Giao tiếp Core ↔ UI qua Tauri IPC commands/events | ✅ | Dùng Tauri event `transcript-partial`/`transcript-final`, đúng pattern đã có với `vad-segment` (PBI 11) |

Không có vi phạm cần ngoại lệ.

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`:
1. Thêm callback `on_chunk` vào `TranscriptPipeline::process_segments` để phát ngay khi có chunk, thay vì gom hết rồi trả `Vec`.
2. `transcript-partial` chỉ phát khi `TranscriptChunk.is_final == false`; với `WhisperSpeechToText` hiện tại (luôn `is_final: true`), mỗi segment chỉ phát `transcript-final` — đúng giả định đã ghi trong spec.md, không đổi STT.
3. Payload event dùng `startFrameIndex`/`endFrameIndex` của segment gốc để UI định danh, nhất quán với contract `vad-segment`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` — không đổi `TranscriptChunk`, thêm `TranscriptEventPayload` (lib.rs).
- Hợp đồng giao diện: xem `contracts/transcript-events.md` — 2 event Tauri `transcript-partial`/`transcript-final`, payload chung 1 schema.
- Kịch bản khởi động nhanh: xem `quickstart.md` — test tích hợp bằng mock STT, không cần UI/mic thật; có hướng dẫn kiểm tra tay qua UI (tuỳ chọn).

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/transcript_pipeline.rs   # sửa: process_segments nhận thêm callback on_chunk, gọi ngay mỗi khi có TranscriptChunk
app/src-tauri/src/lib.rs                        # sửa: start_capture nối TranscriptPipeline vào luồng VAD, thêm TranscriptEventPayload, emit transcript-partial/transcript-final
app/src-tauri/tests/ (hoặc trong transcript_pipeline.rs) # thêm: test case callback nhận đúng thứ tự, partial trước final, danh sách rỗng không gọi callback
.specify/specs/14/                               # spec.md, plan.md, research.md, data-model.md, contracts/, quickstart.md (đã tạo)
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: `WhisperSpeechToText` không có khả năng phát partial thật → nhánh `transcript-partial` trong code sẽ không có test tích hợp thật với model whisper (chỉ test được bằng mock). Chấp nhận được vì đã nêu rõ trong spec.md (giả định + trường hợp biên), không phải lỗi che giấu.
- Không có ngoại lệ vi phạm nguyên tắc dự án cần biện minh.
