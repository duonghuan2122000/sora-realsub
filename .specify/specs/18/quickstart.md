# Quickstart — kiểm thử nhanh PBI 18

## Trên Linux (PulseAudio/PipeWire)

1. Cài thư viện hệ thống cần cho build: `sudo apt install libpulse-dev` (hoặc
   tương đương distro khác).
2. Chạy test đơn vị không cần thiết bị thật:
   ```bash
   cd app/src-tauri && cargo test
   ```
   → phải build và chạy xong, không lỗi liên quan `wasapi`.
3. Phát một đoạn âm thanh test cố định (ví dụ `paplay test-tone.wav` hoặc mở
   video/nhạc bất kỳ).
4. Chạy binary ghi loopback ra file (tái dùng bin có sẵn, ví dụ
   `cargo run --bin capture_loopback_to_wav -- out_linux.wav`), ghi trong vài
   giây rồi dừng (Ctrl+C hoặc theo cơ chế dừng của bin).
5. Nghe lại `out_linux.wav` → phải nhận ra đúng nội dung đã phát ở bước 3.
6. Tắt PulseAudio/PipeWire (hoặc test trên máy không có service này) → chạy
   lại bước 4 → phải nhận lỗi rõ ràng, không crash/treo.

## Trên Windows (regression check)

1. Chạy `cargo test` trong `app/src-tauri` → toàn bộ test liên quan loopback
   hiện có phải pass như trước (không có thay đổi hành vi).
2. Chạy lại `capture_loopback_to_wav` như trước khi có PBI 18 → hành vi không
   đổi.
