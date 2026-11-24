# Data Model: Xử lý lỗi runtime model STT (PBI 15)

## ModelErrorKind (enum, Core)

Phân loại nguyên nhân lỗi runtime model, dùng nội bộ Core và map sang payload event cho UI.

- `Load` — lỗi khi nạp model (thiếu file, file hỏng, lỗi tạo context whisper).
- `Infer` — lỗi khi infer một segment thoại cụ thể.

## ModelErrorEvent (payload Tauri event `model-error`)

Gửi sang UI qua `app_handle.emit("model-error", ModelErrorEvent { .. })`.

| Trường | Kiểu | Ghi chú |
|---|---|---|
| `kind` | `"load" \| "infer"` (camelCase string, giống pattern `TranscriptEventPayload` ở PBI 14) | Loại lỗi, map từ `ModelErrorKind` |
| `message` | `String` | Mô tả ngắn, dễ hiểu cho người dùng cuối (không phải chuỗi lỗi kỹ thuật thô từ `whisper-rs`) |
| `segmentStartFrameIndex` | `Option<u64>` (hoặc kiểu tương ứng `SegmentReady.start_frame_index`) | Chỉ có khi `kind == "infer"`, xác định segment nào lỗi; `None` khi `kind == "load"` |

Quan hệ: `ModelErrorEvent` độc lập với `TranscriptEvent` (PBI 14) — khi 1 segment lỗi infer, Core phát `model-error` cho segment đó thay cho `transcript-final`, KHÔNG phát cả hai cho cùng 1 segment.

## Trạng thái STT trong 1 phiên capture

- `Some(stt)` — model nạp thành công, pipeline STT hoạt động bình thường (như PBI 14).
- `None` — model nạp lỗi (đã phát `model-error` loại `load` đúng 1 lần) hoặc không cấu hình `WHISPER_MODEL_PATH`; phiên vẫn chạy VAD, không phát `transcript-*`/`model-error` lặp lại.

Không có thực thể dữ liệu bền (không lưu DB/file) — toàn bộ chỉ tồn tại trong vòng đời 1 phiên capture, giống các PBI 11-14.
