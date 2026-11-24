# Quickstart: Capture loopback âm thanh hệ thống (PBI 6)

1. Mở một nguồn phát âm thanh/nhạc trên máy (ví dụ YouTube, trình nghe nhạc) qua thiết bị output mặc định.
2. Chạy bin test thủ công:
   ```
   cargo run --bin capture_loopback_to_wav -- 5
   ```
   (tham số là số giây capture, ví dụ 5 giây.)
3. Để âm thanh phát liên tục trong khoảng thời gian đó.
4. Mở file `.wav` xuất ra (đường dẫn in ra console), nghe lại:
   - Phải nghe đúng nội dung đã phát ra loa, không im lặng toàn phần, không chỉ toàn nhiễu.
5. Thử lại khi tắt hết âm thanh (im lặng hoàn toàn) → chương trình vẫn chạy bình thường, không lỗi, file .wav là im lặng.
6. Chạy `cargo test` trong `app/src-tauri` để xác nhận các unit test logic start/stop với `LoopbackAudioSource` vẫn pass.
