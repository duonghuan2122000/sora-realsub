# Test case — T012: capture loopback thật trên Linux (PulseAudio)

**Mục tiêu kiểm thử**: FR-003, FR-004, SC-001 — capture loopback trên Linux
cho ra nội dung đúng với âm thanh đã phát; báo lỗi rõ ràng khi không có
PulseAudio/PipeWire.

## Kịch bản 1 — Có PulseAudio, phát test tone 440Hz

### Input

- Khởi động `pulseaudio -D --exit-idle-time=-1` (sink mặc định: `auto_null`,
  monitor: `auto_null.monitor`).
- Sinh file test tone 440Hz, mono, 44.1kHz, 6 giây bằng
  `ffmpeg -f lavfi -i "sine=frequency=440:duration=6" /tmp/test_tone.wav`.
- Phát `paplay /tmp/test_tone.wav` đồng thời chạy
  `cargo run --bin capture_loopback_to_wav -- 5`.

### Expected

- Chương trình ghi được file `.wav`, không lỗi, không treo.
- Nội dung ghi lại phải chứa tín hiệu 440Hz (không phải im lặng/nhiễu ngẫu
  nhiên) — xác nhận gián tiếp bằng tần số zero-crossing của file ghi được.

### Actual

- Chạy xong: `Đã ghi 24 khối mẫu ra file: capture_loopback_to_wav_output.wav`.
- Phân tích bằng `ffmpeg -af astats`: `Zero crossings rate: 0.055026` tại
  sample rate 16000 Hz → tần số ước tính = 0.055026 × 16000 / 2 ≈ **440.2 Hz**
  — khớp chính xác với tần số 440Hz đã phát. `RMS level dB: -21.07` (có tín
  hiệu thật, không phải im lặng tuyệt đối).
- File ghi lại lưu tại `T012-capture_loopback_output.wav`, log phân tích đầy
  đủ tại `T012-result.log`.
- Lưu ý: trong lúc viết code (T007) phát hiện và sửa 1 lỗi — tên nguồn ban
  đầu dùng chuỗi literal `"@DEFAULT_SINK@.monitor"` (không phải pseudo-name
  hợp lệ của PulseAudio), đã sửa thành `"@DEFAULT_MONITOR@"` (pseudo-name
  chuẩn, PulseAudio tự thay bằng monitor source của sink mặc định hiện tại).
  Kết quả test tone ở trên dùng bản đã sửa.

### Kết quả: **PASS**

## Kịch bản 2 — Không có PulseAudio/PipeWire chạy (trường hợp biên FR-004)

### Input

- `pulseaudio --kill` để dừng server, xác nhận `pactl info` báo
  "Connection refused".
- Chạy lại `cargo run --bin capture_loopback_to_wav -- 2`.

### Expected

- Không panic, không treo quá thời gian hợp lý; in ra lỗi rõ ràng cho người
  dùng.

### Actual

- Output: `Lỗi capture loopback: lỗi I/O: không mở được monitor source mặc định: Connection refused`
  — chương trình thoát ngay, không treo, không panic. Log đầy đủ tại
  `T012-no-pulse-error-result.log`.

### Kết quả: **PASS**
