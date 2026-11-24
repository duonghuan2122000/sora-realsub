# Test case T008 — capture_mixed_to_wav thật 6s khi loopback đang phát âm thanh (SC-001)

**Lệnh**: `cargo run --bin capture_mixed_to_wav -- 6`, đồng thời phát giọng nói qua TTS hệ thống (System.Speech) qua thiết bị output mặc định (loopback) trong lúc capture.

**Giới hạn môi trường**: phiên chạy tự động, không có người nói trực tiếp vào mic lúc test — nên kênh mic chỉ thu được tạp âm phòng/im lặng (không phải giọng nói chủ động). Phần khẳng định "trộn giữ được nội dung từ mic" đã được kiểm chứng bằng unit test [T006](T006-testcase.md) (`mot_nguon_im_lang_output_khop_nguon_con_lai`) với mic giả lập có tín hiệu thật.

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Capture 6s trộn mic+loopback trong lúc TTS phát câu tiếng Việt qua loa | `seconds=6`, loopback đang phát, mic ghi tạp âm phòng | In số khối mẫu, tạo `.wav`, không lỗi | In "Đã ghi 388 khối mẫu ra file: capture_mixed_to_wav_output.wav" | PASS |
| So sánh biên độ với baseline im lặng cả 2 nguồn ([T009](T009-testcase.md)) | Đọc raw PCM float32 | Biên độ trung bình (avg_abs) cao hơn rõ rệt baseline im lặng, có clamp khi vượt ngưỡng | `nsamples=279360`, `max=1.0` (có clamp), `avg_abs≈0.0733` — so với baseline im lặng `avg_abs≈0.0039` (≈19x thấp hơn) | PASS |

Bằng chứng:
- Log console: [T008-result.log](T008-result.log)
- Log phân tích biên độ: [T008-analysis.log](T008-analysis.log)
- File âm thanh thu được: [T008-mixed-sample.wav](T008-mixed-sample.wav) — mở bằng trình nghe hỗ trợ WAV float32 (VLC, Audacity) để nghe lại; nội dung loopback (giọng TTS) nghe rõ, kênh mic là tạp âm phòng do môi trường test tự động không có người nói trực tiếp.

Đáp ứng SC-001 (luồng trộn giữ được nội dung nguồn có âm thanh, không bị mất) và FR-005 (clamp chống méo nghiêm trọng — `max=1.0`, không vượt `[-1.0, 1.0]`).
