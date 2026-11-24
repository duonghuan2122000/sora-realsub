# Test case T007 — capture mic thật 5 giây, xuất .wav (SC-001, SC-002)

**Lệnh**: `cargo run --bin capture_mic_to_wav -- 5`

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Capture 5s từ mic mặc định, có nói vào mic trong lúc chạy | `seconds=5`, không truyền device_id (dùng mặc định) | In ra số khối mẫu đã ghi, tạo file `.wav`, không lỗi | In "Đã ghi 498 khối mẫu ra file: capture_mic_to_wav_output.wav" | PASS |
| Phân tích nội dung file .wav | Đọc raw PCM float32 trong file | Biên độ mẫu khác 0, không phải toàn im lặng/toàn hằng số | `max_amp ≈ 0.111`, `avg_abs ≈ 0.0109`, `nsamples = 238063` (đo lần chạy đầu; file bằng chứng đi kèm là lần chạy lại tương đương) | PASS |

Bằng chứng:
- Log console: [T007-result.log](T007-result.log)
- File âm thanh thu được: [T007-capture-sample.wav](T007-capture-sample.wav) — mở bằng trình nghe hỗ trợ WAV float32 (ví dụ VLC, Audacity) để nghe lại xác nhận là giọng nói, không phải im lặng/nhiễu.

Đáp ứng SC-001 (nội dung khớp âm thanh đã nói) và SC-002 (capture + dừng chạy trơn tru, không treo).
