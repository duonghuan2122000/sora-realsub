# Mô hình dữ liệu: PBI 11 — Phát sự kiện đoạn VAD cho UI

## `SegmentReady` (Core, `core::vad_pipeline`)

Đại diện cho 1 đoạn audio vừa được xác định đóng, sẵn sàng phát ra ngoài.

| Trường | Kiểu | Ý nghĩa |
|---|---|---|
| `start_frame_index` | `usize` | Chỉ số frame VAD (32ms/frame, 16kHz) bắt đầu đoạn, theo thứ tự thời gian tích lũy từ lúc phiên bắt đầu |
| `end_frame_index` | `usize` | Chỉ số frame VAD kết thúc đoạn (bao gồm) |
| `samples` | `Vec<f32>` | Mẫu âm thanh (16kHz mono, đã resample) của đúng đoạn `[start_frame_index, end_frame_index]` |
| `sample_rate` | `u32` | Luôn `16_000`, giữ tường minh để payload tự mô tả, không ngầm định |

Quan hệ: tương ứng 1:1 với 1 `Segment` (đã có ở `core::segment`) tại thời điểm nó vừa đóng lần đầu — không có trạng thái chuyển đổi, không cập nhật lại sau khi đã phát.

## Payload sự kiện Tauri `vad-segment` (biên giới Core ↔ UI)

Serialize từ `SegmentReady` sang JSON khi `lib.rs` gọi `app_handle.emit("vad-segment", payload)`.

| Trường JSON | Kiểu | Ghi chú |
|---|---|---|
| `startFrameIndex` | number | = `start_frame_index` |
| `endFrameIndex` | number | = `end_frame_index` |
| `samples` | number[] | = `samples` |
| `sampleRate` | number | = `sample_rate` |

Không có thực thể lưu trữ lâu dài — sự kiện phát rồi không giữ lại ở Core (ngoài phạm vi spec, xem "Ngoài phạm vi").
