---
title: "PBI 5: Capture mic thành PCM buffer"
date: 2026-11-24
tags: [pbi, m1, feature]
sources: [.specify/specs/5/spec.md, .specify/specs/5/plan.md, .specify/specs/5/tasks.md, .specify/specs/5/evidence, app/src-tauri/src/core/mic_audio_source.rs, app/src-tauri/src/bin/capture_mic_to_wav.rs]
---

# PBI 5: Capture mic thành PCM buffer

Tính năng thứ hai của M1 (Audio Capture): cài đặt thật đầu tiên của trait `AudioSource` (chốt interface ở PBI 3) — capture liên tục từ thiết bị mic đã chọn (PBI 4) thành PCM, có thể dừng, báo lỗi rõ khi thiết bị không hợp lệ.

## Yêu cầu chính (spec.md)
- Bắt đầu capture từ mic hợp lệ → nhận liên tục khối mẫu PCM qua callback cho tới khi dừng.
- Dừng capture → thu mẫu chấm dứt ngay (dưới 1s), không leak thread/handle.
- Thiết bị không hợp lệ/không khả dụng → báo lỗi rõ, không treo/crash.
- Phạm vi: chỉ 1 nguồn mic; **chưa** trộn với loopback hệ thống (PBI riêng kế tiếp trong M1).

## Quyết định kỹ thuật (plan.md, research.md)
- `struct MicAudioSource` implement trait `AudioSource` (đã chốt PBI 3, xem [[Quyet-dinh-hop-dong-core]]) dùng **`cpal`** (đã có từ PBI 4, xem [[PBI-4-liet-ke-thiet-bi-mic]]) để mở input stream.
- **Vấn đề kỹ thuật phát hiện khi code**: `cpal::Stream` **không** implement `Send` (marker `NotSendSyncAcrossAllPlatforms`), nhưng `StreamHandle::stop` của trait `AudioSource` yêu cầu closure `FnOnce() + Send`. → Giải pháp: tạo và giữ sống `Stream` trên **1 thread riêng** do `start()` spawn; giao tiếp với thread chính qua 2 `mpsc::channel` — một báo "ready" (hoặc lỗi setup), một báo "stop". `StreamHandle::stop()` gửi tín hiệu stop rồi `join()` thread đó.
- Chuẩn hoá mẫu về `f32` mono (khớp `AudioFrame.samples`) bất kể thiết bị trả `F32` hay `I16`, nhiều kênh hay mono — convert tại nguồn (`to_mono`), không đẩy việc convert ra tầng gọi.
- Thêm `hound` để ghi `.wav` làm bằng chứng test, dùng cho bin thủ công `capture_mic_to_wav`.
  - **Phát hiện khi implement**: Cargo **không** cấp dev-dependencies cho `[[bin]]` thường (chỉ test/example/bench) → `hound` phải khai ở `[dependencies]` chính thức dù chỉ phục vụ mục đích kiểm thử/bằng chứng, không phải tính năng cho người dùng cuối. (plan.md ban đầu định dùng dev-dependency, phải sửa lại khi build fail.)
- Đổi `mod core;` → `pub mod core;` trong `lib.rs` để bin thủ công (`src/bin/`) có thể `use app_lib::core::{...}` (trước đó `core` module private, chỉ dùng nội bộ crate).

## Cài đặt (code)
- `app/src-tauri/src/core/mic_audio_source.rs` (mới): `MicAudioSource { device_id: Option<String> }` — `None` = thiết bị mặc định. `find_device()` tìm theo tên (khớp cách `devices.rs` dùng tên làm `id`). `start()` spawn thread giữ `cpal::Stream`, convert sample → `AudioFrame`, trả `StreamHandle`.
- `app/src-tauri/src/bin/capture_mic_to_wav.rs` (mới): bin CLI `cargo run --bin capture_mic_to_wav -- <giây> [device_id]`, capture N giây, ghi `.wav` (f32, hound) để nghe lại kiểm thử thủ công.
- `app/src-tauri/src/lib.rs`: `mod core` → `pub mod core`.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `MicAudioSource`.
- `app/src-tauri/Cargo.toml`: thêm `hound = "3"` vào `[dependencies]` (không phải dev-dependencies).

## Bằng chứng test (`.specify/specs/5/evidence/`)
- `cargo test`: 15/15 pass (gồm 2 test mới của `mic_audio_source` — thiết bị không tồn tại trả `CoreError::Io`, `stop()` trên handle thật không panic). Log: `T005-result.log`.
- Capture mic thật 5 giây: 498 khối mẫu, file `.wav` (`T007-capture-sample.wav`) có biên độ thật (max ≈0.11, avg ≈0.011 — không im lặng toàn phần, không toàn hằng số). Log: `T007-result.log`.
- Thử device id không tồn tại: in lỗi `không tìm thấy thiết bị mic: ...`, exit code 1, không treo. Log: `T008-result.log`.
- `cargo clippy --all-targets`: sạch sau khi sửa 1 warning `type_complexity` (thêm type alias `CollectedFrames` trong bin). Log: `T009-result.log`.
- Trường hợp biên "dừng khi chưa từng start" không cần test runtime: API thiết kế sao cho không thể gọi `stop()` mà chưa có `StreamHandle` (chỉ `start()` trả ra handle) — đảm bảo bởi kiểu dữ liệu.

## Liên quan
[[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]]
