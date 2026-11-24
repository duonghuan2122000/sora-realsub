# Test case T008 — capture loopback khi thiết bị output im lặng (trường hợp biên, SC-003)

**Lệnh**: `cargo run --bin capture_loopback_to_wav -- 3`, không phát bất kỳ âm thanh nào qua thiết bị output mặc định trong lúc capture.

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Capture 3s khi thiết bị output mặc định im lặng hoàn toàn | `seconds=3`, không có âm thanh phát ra | Chạy bình thường, không lỗi/crash/treo; dừng đúng hạn | In "Đã ghi 0 khối mẫu ra file: capture_loopback_to_wav_output.wav", chương trình kết thúc bình thường sau ~3s | PASS |
| Kiểm tra file .wav xuất ra | File tạo khi im lặng | File .wav hợp lệ (rỗng hoặc gần rỗng vẫn chấp nhận được) | File tạo thành công, 0 mẫu dữ liệu (không có packet nào từ WASAPI khi hoàn toàn im lặng — engine không phát sự kiện) | PASS |

Bằng chứng:
- Log console: [T008-result.log](T008-result.log)
- File .wav xuất ra (0 mẫu): [T008-silence-sample.wav](T008-silence-sample.wav)

Đáp ứng trường hợp biên "thiết bị output mặc định không phát gì" trong spec.md và SC-003 (không treo/crash khi gặp trạng thái bất thường của thiết bị output).
