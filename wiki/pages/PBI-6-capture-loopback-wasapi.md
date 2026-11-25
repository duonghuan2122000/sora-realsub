---
title: "PBI 6: Capture loopback âm thanh hệ thống (WASAPI)"
date: 2026-11-24
tags: [pbi, m1, feature]
sources: [.specify/specs/6/spec.md, .specify/specs/6/plan.md, .specify/specs/6/research.md, .specify/specs/6/tasks.md, .specify/specs/6/evidence, app/src-tauri/src/core/loopback_audio_source.rs, app/src-tauri/src/bin/capture_loopback_to_wav.rs]
---

# PBI 6: Capture loopback âm thanh hệ thống (WASAPI)

Tính năng thứ ba của M1 (Audio Capture): cài đặt thứ hai của trait `AudioSource` (chốt interface ở PBI 3) — capture liên tục âm thanh đang phát ra từ thiết bị output **mặc định** của Windows (loopback), để sau này làm phụ đề cho cả nội dung mình nghe, không chỉ nội dung mình nói (mic, PBI 5).

## Yêu cầu chính (spec.md)
- Bắt đầu capture loopback khi thiết bị output mặc định đang phát âm thanh → nhận liên tục khối mẫu PCM qua callback cho tới khi dừng.
- Dừng capture → thu mẫu chấm dứt ngay (dưới 1s).
- Không có thiết bị output khả dụng/sự cố thiết bị → báo lỗi rõ, không treo/crash.
- Im lặng hoàn toàn (không phát gì) → vẫn chạy bình thường, không lỗi.
- Phạm vi: chỉ thiết bị output **mặc định**, chưa cho chọn thiết bị cụ thể; chưa trộn (mix) với mic (PBI riêng kế tiếp trong M1).

## Quyết định kỹ thuật (plan.md, research.md)
- `cpal` (dùng cho mic ở PBI 5) **không** expose cờ `AUDCLNT_STREAMFLAGS_LOOPBACK` của WASAPI → không dùng được cho loopback. Chuyển sang crate **`wasapi`** (bọc trực tiếp Windows Core Audio API, hỗ trợ loopback as first-class use case).
- **Cách mở loopback với crate `wasapi`**: lấy `Device` theo `Direction::Render` (thiết bị **output**, không phải Capture) qua `DeviceEnumerator::get_default_device(&Direction::Render)`, đọc format qua `device.get_device_format()`, rồi `audio_client.initialize_client(&format, &Direction::Capture, &mode)` — chính việc mở `AudioClient` từ 1 device **Render** nhưng khởi tạo stream với direction **Capture** là thứ tạo ra loopback (không có flag riêng lộ ra ngoài API mà crate đã ẩn đi khi thấy device là Render).
- Giữ nguyên pattern thread riêng + `mpsc` channel (ready/stop) từ `mic_audio_source.rs`, vì `AudioClient`/`AudioCaptureClient` của `wasapi` cũng không `Send`.
- **Phát hiện khi implement**: thiết bị output mặc định trên máy test thực tế trả format **`SampleType::Int`** (không phải `Float` như giả định ban đầu dựa theo ví dụ của crate) → phải decode cả 3 trường hợp `(Float,32)`, `(Int,32)`, `(Int,16)` thay vì chỉ giả định float. Hàm `decode_samples` tách riêng theo `(SampleType, bytes_per_sample)`.
- **Trường hợp biên quan sát được khi test im lặng hoàn toàn**: khi thiết bị output không phát gì, WASAPI event-driven shared mode không sinh event/packet nào trong suốt thời gian capture → thu được 0 khối mẫu (không phải lỗi, không treo, chỉ đơn giản không có dữ liệu). Khớp với kỳ vọng "chạy bình thường" ở spec.md, không cần xử lý đặc biệt.
- Chuẩn hoá mẫu về `f32` mono (khớp `AudioFrame.samples`), tái dùng logic `to_mono` giống `MicAudioSource`.

## Cài đặt (code)
- `app/src-tauri/src/core/loopback_audio_source.rs` (mới): `LoopbackAudioSource` (không cần tham số, chỉ dùng thiết bị output mặc định). `start()` spawn thread riêng mở `AudioClient` loopback qua `wasapi`, đọc raw bytes qua `AudioCaptureClient::read_from_device_to_deque`, decode theo format thật của thiết bị (`decode_samples`), convert mono, gọi `on_frame`, trả `StreamHandle`.
- `app/src-tauri/src/bin/capture_loopback_to_wav.rs` (mới): bin CLI `cargo run --bin capture_loopback_to_wav -- <giây>`, capture N giây loopback, ghi `.wav` (f32, hound) để nghe lại kiểm thử thủ công.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `LoopbackAudioSource`.
- `app/src-tauri/Cargo.toml`: thêm `wasapi = "0.26"` vào `[dependencies]`.

## Bằng chứng test (`.specify/specs/6/evidence/`)
- `cargo test --all`: 16/16 pass (thêm 1 test `loopback_audio_source::stop_tren_handle_khong_panic_va_dung_nhan_mau`). Log: `T009-result.log`.
- Capture loopback thật 6 giây trong lúc TTS (System.Speech) phát giọng nói qua loa: 600 khối mẫu, file `.wav` (`T007-capture-sample.wav`) có biên độ thật (rms ≈0.0996, max ≈0.985 — khớp âm thanh đã phát, không im lặng/nhiễu). Log: `T007-result.log`.
- Capture loopback 3 giây khi hoàn toàn im lặng: 0 khối mẫu, không lỗi/crash/treo, dừng đúng hạn. Log: `T008-result.log`.
- `cargo clippy --all-targets`: sạch, không warning.

## Cập nhật (PBI 18)

`wasapi = "0.26"` ban đầu nằm vô điều kiện trong `[dependencies]`, chặn
`cargo build`/`cargo test` trên Linux (phát hiện ở PBI 17, xem
[[Rui-ro-ky-thuat]]). PBI 18 đã gate nó vào
`[target.'cfg(windows)'.dependencies]` và thêm `#![cfg(windows)]` ở đầu file
`loopback_audio_source.rs` — logic bên trong không đổi. Linux có cài đặt
`LoopbackAudioSource` riêng (PulseAudio/PipeWire) tại
`loopback_audio_source_linux.rs`. Xem [[PBI-18-loopback-linux]].

## Liên quan
[[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[PBI-5-capture-mic-pcm]], [[PBI-18-loopback-linux]]
