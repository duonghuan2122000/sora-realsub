# Test case T007 — capture loopback thật 6 giây khi có âm thanh phát, xuất .wav (SC-001, SC-002)

**Lệnh**: `cargo run --bin capture_loopback_to_wav -- 6`, đồng thời phát giọng nói qua TTS hệ thống (System.Speech) qua thiết bị output mặc định trong lúc capture.

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Capture 6s loopback trong lúc TTS đang phát câu tiếng Việt qua loa | `seconds=6`, thiết bị output mặc định đang phát âm thanh | In ra số khối mẫu đã ghi, tạo file `.wav`, không lỗi | In "Đã ghi 600 khối mẫu ra file: capture_loopback_to_wav_output.wav" | PASS |
| Phân tích nội dung file .wav | Đọc raw PCM float32 trong chunk `data` | Biên độ mẫu khác 0 rõ rệt, không phải toàn im lặng | `samples = 288000`, `rms ≈ 0.0996`, `max ≈ 0.985`, `min ≈ -0.897` | PASS |

Bằng chứng:
- Log console: [T007-result.log](T007-result.log)
- File âm thanh thu được: [T007-capture-sample.wav](T007-capture-sample.wav) — mở bằng trình nghe hỗ trợ WAV float32 (ví dụ VLC, Audacity) để nghe lại xác nhận khớp nội dung đã phát ra loa.

Đáp ứng SC-001 (nội dung khớp âm thanh đã phát) và SC-002 (capture + dừng chạy trơn tru, không treo, không panic).
