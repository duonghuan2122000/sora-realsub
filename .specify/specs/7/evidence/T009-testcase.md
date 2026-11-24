# Test case T009 — output khớp đúng nguồn đang có âm thanh khi nguồn còn lại gần im lặng (SC-002)

**Lệnh**: `cargo run --bin capture_mixed_to_wav -- 5` khi cả mic và loopback đều không phát gì chủ động (baseline im lặng, chỉ có tạp âm nền rất nhỏ).

**Giới hạn môi trường**: giống [T008](T008-testcase.md) — phiên tự động không có người nói vào mic trực tiếp. Phép thử "chỉ 1 nguồn có tiếng" theo đúng nghĩa cách ly hoàn toàn (1 nguồn có tín hiệu rõ, nguồn kia tuyệt đối 0) đã được kiểm chứng đầy đủ bằng unit test [T006](T006-testcase.md) (`mot_nguon_im_lang_output_khop_nguon_con_lai`: nguồn A=0.3 cố định, nguồn B im lặng tuyệt đối → output khớp đúng A). Test thủ công ở đây dùng để đối chiếu baseline thực tế với trường hợp có âm thanh thật ở [T008](T008-testcase.md).

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Capture 5s khi cả 2 nguồn không chủ động phát gì | `seconds=5` | Chạy bình thường, không lỗi/crash, biên độ output thấp (gần im lặng) | In "Đã ghi 324 khối mẫu ra file: capture_mixed_to_wav_output.wav", không lỗi | PASS |
| Phân tích biên độ | Đọc raw PCM float32 | avg_abs thấp, max không bị clamp bất thường | `nsamples=233280`, `max≈0.0529`, `avg_abs≈0.00394` | PASS |

Bằng chứng:
- Log: [T009-result.log](T009-result.log)
- File âm thanh baseline: [T009-silence-sample.wav](T009-silence-sample.wav)

So với [T008](T008-testcase.md) (`avg_abs≈0.0733` khi loopback có TTS phát), baseline im lặng ở đây thấp hơn ~19 lần — xác nhận output phản ánh đúng nguồn đang có tín hiệu, không phải nhiễu/lỗi ngẫu nhiên không liên quan input.

Đáp ứng SC-002 kết hợp với bằng chứng unit test T006 cho trường hợp cách ly tuyệt đối 1 nguồn.
