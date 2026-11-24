---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24 (cập nhật sau PBI 8)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **PBI 3**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test. Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **PBI 4**: `core/devices.rs` — liệt kê thiết bị audio input bằng `cpal` thật; Tauri command `list_input_devices_cmd`. Dependency: `cpal = "0.15"`. Chi tiết: [[PBI-4-liet-ke-thiet-bi-mic]].
- **PBI 5**: `core/mic_audio_source.rs` — `MicAudioSource`, cài đặt thật đầu tiên của trait `AudioSource` (capture mic qua `cpal`, stream giữ trên thread riêng vì `cpal::Stream` không `Send`); bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` (crate `hound`) làm bằng chứng nghe lại. `lib.rs`: `mod core` đổi thành `pub mod core`. Chi tiết: [[PBI-5-capture-mic-pcm]].
- **Mới (PBI 6)**: `core/loopback_audio_source.rs` — `LoopbackAudioSource`, cài đặt thứ hai của trait `AudioSource` (capture loopback từ thiết bị output mặc định qua crate **`wasapi`**, vì `cpal` không hỗ trợ loopback; stream giữ trên thread riêng cùng pattern với mic). Bin thủ công `src/bin/capture_loopback_to_wav.rs` ghi `.wav`. Dependency mới: `wasapi = "0.26"`. 16/16 test pass, clippy sạch. Chi tiết: [[PBI-6-capture-loopback-wasapi]].
- **Mới (PBI 7)**: `core/mixed_audio_source.rs` — `MixedAudioSource`, trộn 2 `AudioSource` con (mic + loopback) thành 1 stream bằng buffer trung gian + thread trộn chu kỳ 15ms (cộng mẫu + clamp), resample linear khi sample rate khác nhau. Không thêm dependency. Bin thủ công `src/bin/capture_mixed_to_wav.rs`. 21/21 test pass, clippy sạch. Chi tiết: [[PBI-7-tron-mic-loopback]].
- **Mới (PBI 8)**: `core/capture_session.rs` — `CaptureSessionState`, quản lý 1 phiên thu âm duy nhất (`Mutex<Option<StreamHandle>>`), expose qua Tauri command `start_capture`/`stop_capture` trong `lib.rs` (dùng `MixedAudioSource` từ PBI 7). Thêm 2 lỗi `CoreError::AlreadyRunning`/`NotRunning`. Không thêm dependency. 26/26 test pass; kiểm thử tay qua UI thật (quickstart.md) chưa chạy do môi trường không có GUI — xem [[PBI-8-start-stop-capture]]. **M1 (Audio Capture) hoàn tất.**
- **Vẫn chưa có**: chọn thiết bị output cụ thể cho loopback, VAD, STT, dịch, lưu trữ thật trong [[Pipeline-xu-ly-audio]]. Chưa thấy `rubato`, `ort`, `whisper-rs`, `rusqlite` trong Cargo deps.

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- **PBI 4**: `components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic từ danh sách thật, nút làm mới, thông báo rỗng/lỗi), gắn vào `App.vue`; kèm `DeviceSelector.test.ts` (vitest, 4 test pass).
- Chưa có cửa sổ chính/phụ đề nổi riêng, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].
- **PBI 5, PBI 6, PBI 7, PBI 8 không đụng UI** — chỉ thi công phần lõi Rust capture + điều khiển phiên (mic, loopback, trộn, start/stop), command Tauri đã có nhưng chưa có nút bấm trong Vue.

## Kết luận
M1 (Audio Capture) đã hoàn tất 5/5 mục: liệt kê thiết bị mic (PBI 4), capture mic thành PCM (PBI 5), capture loopback hệ thống thành PCM (PBI 6), trộn mic+loopback thành 1 stream (PBI 7), lệnh `start_capture`/`stop_capture` qua IPC (PBI 8) — cả năm chạy trên nền interface Core đã chốt ở PBI 3. Tiếp theo: M2 (VAD).

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]]
