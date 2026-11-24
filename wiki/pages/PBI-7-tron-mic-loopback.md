---
title: "PBI 7: Trộn mic + loopback thành 1 stream"
date: 2026-11-24
tags: [pbi, m1, feature]
sources: [.specify/specs/7/spec.md, .specify/specs/7/plan.md, .specify/specs/7/research.md, .specify/specs/7/tasks.md, .specify/specs/7/evidence, app/src-tauri/src/core/mixed_audio_source.rs, app/src-tauri/src/bin/capture_mixed_to_wav.rs]
---

# PBI 7: Trộn mic + loopback thành 1 stream

Tính năng thứ tư của M1 (Audio Capture): trộn (mix) 2 cài đặt `AudioSource` đã có — mic (PBI 5) và loopback hệ thống (PBI 6) — thành 1 luồng PCM duy nhất, để các bước xử lý sau (VAD, STT) chỉ cần làm việc với 1 nguồn audio tổng hợp.

## Yêu cầu chính (spec.md)
- Trộn liên tục luồng PCM mic + loopback khi cả 2 đang capture; luồng kết quả phải giữ được nội dung nghe được từ cả 2 nguồn.
- Chỉ 1 nguồn có âm thanh → output phải khớp đúng nguồn đó, không bị suy giảm.
- Xử lý được trường hợp mic/loopback có sample rate hoặc số kênh khác nhau, không crash.
- Dừng capture → việc trộn dừng ngay (dưới 1s).
- Tránh clipping nghiêm trọng khi cả 2 nguồn cùng lớn.
- Phạm vi: chưa cho tuỳ chỉnh tỉ lệ gain giữa 2 nguồn (mặc định 1:1); chưa có command Tauri `start_capture`/`stop_capture` thống nhất (PBI tiếp theo trong M1).

## Quyết định kỹ thuật (plan.md, research.md)
- **Không thêm dependency mới** — chỉ dùng `std` (thread, Mutex, VecDeque), tái dùng trait `AudioSource`/`AudioFrame`/`StreamHandle` đã có từ PBI 3.
- **Kiến trúc trộn**: `MixedAudioSource` nhận 2 `Box<dyn AudioSource>` (bất kỳ 2 nguồn nào, không chỉ mic+loopback) trong constructor. Mỗi nguồn con tự chạy thread riêng của chính nó (như hiện có, không sửa `MicAudioSource`/`LoopbackAudioSource`), đẩy `AudioFrame` vào 1 buffer trung gian (`VecDeque<f32>` có `Mutex`) riêng cho từng nguồn. Một **thread trộn riêng** chạy theo chu kỳ cố định 15ms, rút N mẫu từ mỗi buffer (thiếu thì coi là `0.0`), cộng mẫu + `clamp(-1.0, 1.0)`, rồi gọi `on_frame` ra ngoài.
- **Lý do dùng buffer trung gian + chu kỳ cố định** (thay vì gọi trực tiếp nhau): 2 `AudioSource` con tự gọi callback bất đồng bộ, không có cách bắt chúng đồng bộ hoá thời điểm sinh mẫu mà không sửa code đã test pass của chúng.
- **Resample khi sample rate khác nhau**: `target_rate` được chốt bằng sample rate của frame **đầu tiên** nhận được (từ nguồn nào đến trước), lưu trong `Arc<Mutex<Option<u32>>>` chia sẻ giữa 2 buffer. Mọi frame sau đó (từ cả 2 nguồn) được resample bằng **linear interpolation** (`resample_linear`) về `target_rate` trước khi đẩy vào buffer của nó — đủ để tránh lỗi/méo nặng, không cần resampler chất lượng cao (sinc) ở bước này.
- **Chống clipping**: cộng trực tiếp 2 mẫu rồi `clamp(-1.0, 1.0)` — đơn giản, đủ đáp ứng "không méo tới mức không nghe được", đánh đổi nhỏ về âm lượng khi cả 2 nguồn cùng lớn (chấp nhận theo giả định trong spec.md).
- **Dừng**: `StreamHandle` trả về đóng cả 2 `StreamHandle` con + báo dừng thread trộn qua `stop_flag` rồi `join`.

## Cài đặt (code)
- `app/src-tauri/src/core/mixed_audio_source.rs` (mới): `MixedAudioSource` (constructor nhận 2 `Box<dyn AudioSource>`), hàm `resample_linear`, struct `SourceBuffer` nội bộ quản lý buffer + target_rate dùng chung.
- `app/src-tauri/src/bin/capture_mixed_to_wav.rs` (mới): bin CLI `cargo run --bin capture_mixed_to_wav -- <giây>`, chạy `MixedAudioSource::new(MicAudioSource::new(None), LoopbackAudioSource::new())` thật, ghi `.wav` (f32, hound) để nghe lại kiểm thử.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `MixedAudioSource`.
- Không sửa `Cargo.toml` — không cần dependency mới.

## Bằng chứng test (`.specify/specs/7/evidence/`)
- `cargo test`: 21/21 pass (thêm 5 test mới cho `mixed_audio_source`: trộn 2 nguồn có tiếng → tổng + clamp đúng; 1 nguồn im lặng → output khớp nguồn còn lại; `stop()` không panic; resample giữ nguyên khi cùng rate / đổi độ dài khi khác rate). Log: `T006-result.log`.
- Capture trộn thật 6s trong lúc TTS (System.Speech) phát giọng nói qua loa (loopback có tín hiệu): `avg_abs≈0.0733`, `max=1.0` (có clamp, không vượt ngưỡng) — so với baseline im lặng cả 2 nguồn `avg_abs≈0.0039` (~19x thấp hơn), xác nhận output phản ánh đúng nguồn có tín hiệu. File: `T008-mixed-sample.wav`.
- **Giới hạn môi trường ghi nhận rõ trong evidence**: phiên test tự động không có người nói trực tiếp vào mic, nên phần "mic có tiếng thật" chỉ được kiểm chứng đầy đủ bằng unit test (mock nguồn có tín hiệu cố định), không phải bằng giọng nói thật như mic ở PBI 5. Test thủ công thật tập trung vào nhánh loopback + đối chiếu baseline.
- `cargo clippy --all-targets`: sạch, không warning.

## Liên quan
[[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]]
