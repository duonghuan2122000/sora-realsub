# Nghiên cứu: Capture mic thành PCM buffer (PBI 5)

- **Quyết định**: Dùng `cpal` để mở input stream mic, lấy mẫu qua callback.
  **Lý do**: Đã có trong `Cargo.toml` từ PBI 4 (liệt kê thiết bị), cross-platform, callback mẫu khớp tự nhiên với `AudioSource::start(on_frame)`.
  **Phương án khác đã xem xét**: Tự gọi WASAPI trực tiếp trên Windows — bỏ qua, phức tạp không cần thiết cho mic (khác với loopback, thứ `cpal` không hỗ trợ tốt và sẽ cần ở PBI riêng sau).

- **Quyết định**: Dùng `hound` (dev-dependency) để ghi `.wav` làm bằng chứng test, không đưa vào dependency chính.
  **Lý do**: Nhỏ, phổ biến, chỉ cần cho mục đích kiểm thử/nghe lại, không phải tính năng cho người dùng cuối.
  **Phương án khác đã xem xét**: Tự viết WAV header thủ công — tốn công, không mang thêm giá trị so với dùng crate có sẵn.

- **Quyết định**: Chuẩn hoá mẫu về `f32` mono trong `AudioFrame.samples`.
  **Lý do**: Khớp định nghĩa `AudioFrame` đã có từ PBI 3 (`core/types.rs`), các tầng sau (VAD, STT) dùng chung format.
  **Phương án khác đã xem xét**: Giữ định dạng gốc của thiết bị (có thể i16, nhiều kênh) rồi convert ở tầng gọi — bị loại vì phân tán logic convert ra nhiều nơi, khó test tại nguồn phát.

Không còn điểm `NEEDS CLARIFICATION`.
