# Mô hình dữ liệu: Hợp đồng giao diện Core (PBI 3)

Các kiểu dữ liệu tối thiểu để 4 trait biên dịch được. Chi tiết trường sẽ mở rộng ở các PBI cài đặt thật (M1–M5), không cố định quá sớm ở đây.

## `CoreError`

Lỗi chung cho mọi hợp đồng Core.

| Biến thể | Ý nghĩa |
|---|---|
| `Io(String)` | Lỗi I/O cấp thấp (thiết bị, mạng, file) |
| `Provider(String)` | Lỗi từ provider cụ thể (cloud API trả lỗi, model local crash...) |
| `Cancelled` | Luồng xử lý bị dừng chủ động (user bấm Stop) |

## `AudioFrame`

Một khối mẫu âm thanh PCM.

- `samples: Vec<f32>` — dữ liệu mẫu, mono, đã chuẩn hoá về `[-1.0, 1.0]`.
- `sample_rate: u32` — tần số lấy mẫu (Hz).

## `StreamHandle`

Đại diện cho một phiên capture đang chạy, dùng để dừng (`stop()`).

- `fn stop(&self)` — dừng capture, giải phóng tài nguyên.

## `TranscriptChunk`

Kết quả từ `SpeechToText`.

- `text: String` — văn bản nhận dạng được.
- `is_final: bool` — `true` nếu là kết quả chốt, `false` nếu là kết quả tạm (có thể bị sửa sau).

## `Subtitle`

Phụ đề đã sẵn sàng, đầu ra của pipeline dịch, đầu vào của `SubtitleSink`.

- `source_text: String` — văn bản gốc.
- `translated_text: String` — văn bản đã dịch.
- `timestamp_ms: u64` — thời điểm (ms) tính từ lúc bắt đầu phiên, dùng cho lưu transcript (M7).

## Quan hệ & luồng dữ liệu

```text
AudioSource --AudioFrame--> (VAD, ngoài phạm vi PBI này)
      --segment audio--> SpeechToText --TranscriptChunk(is_final)--> Translator --(source+translated)--> Subtitle --> SubtitleSink
```

Không có trạng thái lưu trữ lâu dài (database) ở PBI này — các kiểu trên chỉ tồn tại trong bộ nhớ, truyền qua tham số hàm/trait.
