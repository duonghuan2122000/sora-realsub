# Quickstart: Capture mic thành PCM buffer (PBI 5)

1. Cắm/chọn một thiết bị mic hợp lệ trên máy.
2. Chạy bin test thủ công:
   ```
   cargo run --bin capture_mic_to_wav -- 5
   ```
   (tham số là số giây capture, ví dụ 5 giây.)
3. Nói vào mic trong khoảng thời gian đó.
4. Mở file `.wav` xuất ra (đường dẫn in ra console), nghe lại:
   - Phải nghe đúng nội dung đã nói, không im lặng toàn phần, không chỉ toàn nhiễu.
5. Thử lại với mic không tồn tại (ví dụ sửa device id sai) → chương trình phải in lỗi rõ ràng, không treo/crash.
6. Chạy `cargo test` trong `app/src-tauri` để xác nhận các unit test logic start/stop với mock `AudioSource` vẫn pass.
