---
title: "PBI 18: Loopback trên Linux + gate wasapi theo target"
date: 2026-11-25
tags: [pbi, m1, feature, linux]
sources: [.specify/specs/18/spec.md, .specify/specs/18/plan.md, .specify/specs/18/research.md, .specify/specs/18/quickstart.md, .specify/specs/18/tasks.md, .specify/specs/18/evidence, app/src-tauri/Cargo.toml, app/src-tauri/src/core/loopback_audio_source.rs, app/src-tauri/src/core/loopback_audio_source_linux.rs, app/src-tauri/src/core/mod.rs]
---

# PBI 18: Loopback trên Linux + gate `wasapi` theo target

Xử lý rủi ro đã ghi nhận từ PBI 17 ([[Rui-ro-ky-thuat]]): `wasapi` là
dependency vô điều kiện dù chỉ build được trên Windows, khiến
`cargo build`/`cargo test` **không chạy được trên Linux** bất kể PBI đang
làm có liên quan audio hay không. PBI này giải quyết đúng gốc vấn đề, đồng
thời thêm cài đặt `AudioSource` loopback thật cho Linux (mục roadmap M1 còn
lại cuối cùng), hoàn tất M1 cho cả hai OS.

## Yêu cầu chính (spec.md)

- FR-001/SC-003: `cargo build`/`cargo test` chạy được trên Linux không cần `wasapi`.
- FR-002/SC-002: hành vi loopback trên Windows không đổi (0 regression).
- FR-003/SC-001: loopback trên Linux cho ra nội dung đúng với âm thanh hệ thống đã phát.
- FR-004: báo lỗi rõ ràng khi không có PulseAudio/PipeWire, không crash/treo.
- FR-005: cùng một trait `AudioSource` cho cả hai OS (không đổi interface).

## Quyết định kỹ thuật (research.md)

- **Gate theo target trong Cargo.toml** (không phải feature flag, không phải
  chỉ `#[cfg]` trong code): chuyển `wasapi` vào
  `[target.'cfg(windows)'.dependencies]`; thêm `libpulse-binding` +
  `libpulse-simple-binding` vào `[target.'cfg(target_os = "linux")'.dependencies]`.
  Đây chính là "giải pháp đúng" mà [[Rui-ro-ky-thuat]] đã đề xuất từ PBI 17.
- **Loopback Linux**: dùng PulseAudio client API (`libpulse-simple-binding`,
  API `psimple::Simple`) mở recording stream từ pseudo-device
  **`@DEFAULT_MONITOR@`** (PulseAudio tự thay bằng monitor source của sink
  output mặc định hiện tại) — tương thích cả PulseAudio thuần và PipeWire
  (qua lớp `pipewire-pulse`). **Lưu ý khi implement**: ban đầu viết nhầm
  `"@DEFAULT_SINK@.monitor"` (chuỗi ghép thủ công, không phải pseudo-name
  hợp lệ) — không phải lỗi cú pháp nên không bị bắt lúc build, chỉ lộ ra khi
  chạy test thật với PulseAudio có sink thật (`auto_null`); đã sửa thành
  `@DEFAULT_MONITOR@` đúng chuẩn PulseAudio.
- **Tách file theo OS**: giữ `loopback_audio_source.rs` (bản WASAPI, PBI 6)
  gần như nguyên vẹn, chỉ thêm `#![cfg(windows)]`; tạo file mới
  `loopback_audio_source_linux.rs` cho bản Linux, gate
  `#![cfg(target_os = "linux")]`. `mod.rs` export `LoopbackAudioSource` theo
  đúng 1 trong 2 module tuỳ OS build — nhờ vậy `lib.rs` và các
  `src/bin/capture_loopback_to_wav.rs`/`capture_mixed_to_wav.rs` **không cần
  sửa gì** (chỉ gọi qua trait `AudioSource`, không biết đang chạy OS nào).
  Đơn giản hơn hướng cũ [[Rui-ro-ky-thuat]] đề xuất ở PBI 17 (gate riêng từng
  điểm gọi `LoopbackAudioSource`).

## Cài đặt (code)

- `app/src-tauri/Cargo.toml`: `wasapi` → `[target.'cfg(windows)'.dependencies]`;
  thêm `libpulse-binding = "2"` + `libpulse-simple-binding = "2"` vào
  `[target.'cfg(target_os = "linux")'.dependencies]`.
- `app/src-tauri/src/core/loopback_audio_source.rs`: thêm `#![cfg(windows)]`
  ở đầu file, logic WASAPI bên trong không đổi so với PBI 6.
- `app/src-tauri/src/core/loopback_audio_source_linux.rs` (mới):
  `LoopbackAudioSource` bản Linux — mở `psimple::Simple` từ
  `@DEFAULT_MONITOR@` (mono, s16le, 16kHz), chạy trên thread riêng (pattern
  `thread::spawn` + `mpsc` ready/stop giống bản WASAPI), decode PCM 16-bit
  sang `f32`, lỗi mở stream trả `CoreError::Io` rõ nghĩa (không panic).
- `app/src-tauri/src/core/mod.rs`: khai báo 2 module với
  `#[cfg(windows)]`/`#[cfg(target_os = "linux")]`, `pub use`
  `LoopbackAudioSource` tương ứng.
- Hệ thống cần cài thêm để build trên Linux (không phải Cargo dependency):
  `libpulse-dev`. Để build **toàn bộ** crate `app` (Tauri 2) trên Linux còn
  cần thêm các lib GTK/WebKit chuẩn của Tauri
  (`libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`,
  `librsvg2-dev`, `libsoup-3.0-dev`, `libjavascriptcoregtk-4.1-dev`) và ALSA
  (`libasound2-dev`, cho `cpal`) — các lib này **không liên quan PBI 18**
  (vốn đã cần từ trước, chỉ là lần đầu `cargo build` thật chạy hết được trên
  Linux nên mới lộ ra); ghi chú lại ở đây để lần sau build trên máy Linux
  mới không mất thời gian dò lại.

## Bằng chứng test (`.specify/specs/18/evidence/`)

- `cargo build` + `cargo test` trên Linux: build xong, 60 unit test + toàn
  bộ integration test pass (0 fail), bao gồm test mới
  `loopback_audio_source_linux::tests::stop_tren_handle_khong_panic_va_dung_nhan_mau`
  (tự bỏ qua vì container không có PulseAudio lúc đó — đúng thiết kế, giống
  pattern test Windows). Log: `T011-cargo-test-result.log`.
- **Capture loopback thật**: khởi động PulseAudio thật trong container
  (`pulseaudio -D`, sink `auto_null`), phát tone 440Hz test (`ffmpeg` sinh
  file, `paplay` phát), chạy `capture_loopback_to_wav 5` → ghi ra `.wav`;
  phân tích bằng `ffmpeg -af astats`: zero-crossing rate 0.055026 tại 16kHz
  → tần số ước tính ≈440.2Hz, khớp chính xác tone đã phát (không phải im
  lặng/nhiễu). File: `T012-capture_loopback_output.wav`, log:
  `T012-result.log`.
- **Trường hợp không có PulseAudio** (FR-004): `pulseaudio --kill` rồi chạy
  lại → in lỗi rõ ràng `"không mở được monitor source mặc định: Connection refused"`,
  không panic, không treo. Log: `T012-no-pulse-error-result.log`.
- `cargo clippy --all-targets`: không có warning mới từ code PBI 18 (1
  warning còn lại ở `tests/vad_real_speech.rs` là tiền tồn tại, ngoài phạm
  vi, không sửa).
- **Chưa xác nhận trên Windows thật** (SC-002): không có máy Windows trong
  môi trường implement (container Linux cloud session). File
  `loopback_audio_source.rs` chỉ thêm `#![cfg(windows)]` bọc ngoài, không
  sửa logic bên trong PBI 6 — về lý thuyết hành vi không đổi, nhưng cần xác
  nhận thật trên Windows trước khi coi SC-002 đã kiểm chứng đầy đủ.

## Liên quan

[[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]],
[[Rui-ro-ky-thuat]], [[PBI-6-capture-loopback-wasapi]], [[PBI-17-dich-co-ngu-canh]]
