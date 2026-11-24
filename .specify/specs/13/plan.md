# Kế hoạch triển khai: Ghép STT vào luồng VAD → Transcript

**Mã PBI**: 13
**Liên kết spec**: .specify/specs/13/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core), async qua `tokio` + `async_trait` (đã dùng ở PBI 12).
- Framework / Thư viện chính: tái dùng `VadPipeline` (PBI 11) và `SpeechToText`/`WhisperSpeechToText` (PBI 12), không thêm dependency mới.
- Lưu trữ dữ liệu: không có, toàn bộ xử lý trong bộ nhớ, một lần gọi.
- Kiểm thử: test tích hợp Rust (`#[tokio::test]`) với file audio mẫu thật, theo mẫu `vad_pipeline_real_speech.rs` (PBI 11) và test STT (PBI 12).
- Nền tảng triển khai: không đổi, vẫn core Rust nội bộ app Tauri, chưa expose qua IPC (PBI sau mới làm event Tauri).
- Ràng buộc hiệu năng: không yêu cầu đo số cụ thể ở PBI này (theo spec).
- Ràng buộc khác: không gửi audio ra ngoài mạng (giữ nguyên ràng buộc local từ PBI 12).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Toàn bộ nằm trong `core/`, không đụng tầng UI (Vue). |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: ghép qua bộ điều phối mới `TranscriptPipeline`, xử lý tuần tự giữ đúng thứ tự, test bằng file audio mẫu (chưa cần capture thời gian thực).

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` — `TranscriptPipeline` mới, tái dùng `SegmentReady`, `AudioFrame`, `TranscriptChunk`, `SpeechToText` đã có.
- Hợp đồng giao diện: không áp dụng — pipeline nội bộ core, chưa expose API/IPC ở PBI này.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  transcript_pipeline.rs   (mới) — TranscriptPipeline::process_segments
  mod.rs                   (sửa) — khai báo module mới
app/src-tauri/tests/
  transcript_pipeline_real_speech.rs   (mới) — test end-to-end VAD → STT bằng audio thật
```

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc dự án.
- Rủi ro: thời gian chạy test tích hợp tăng do phải load model whisper thật (đã chấp nhận từ PBI 12, không phát sinh mới).
