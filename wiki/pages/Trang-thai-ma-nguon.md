---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24 (cập nhật sau PBI 5)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **PBI 3**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test. Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **PBI 4**: `core/devices.rs` — liệt kê thiết bị audio input bằng `cpal` thật; Tauri command `list_input_devices_cmd`. Dependency: `cpal = "0.15"`. Chi tiết: [[PBI-4-liet-ke-thiet-bi-mic]].
- **Mới (PBI 5)**: `core/mic_audio_source.rs` — `MicAudioSource`, cài đặt thật đầu tiên của trait `AudioSource` (capture mic qua `cpal`, stream giữ trên thread riêng vì `cpal::Stream` không `Send`); bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` (crate `hound`) làm bằng chứng nghe lại. `lib.rs`: `mod core` đổi thành `pub mod core`. 15/15 test pass, clippy sạch. Chi tiết: [[PBI-5-capture-mic-pcm]].
- **Vẫn chưa có**: capture loopback hệ thống, mix mic+loopback, command `start_capture`/`stop_capture` qua Tauri IPC, VAD, STT, dịch, lưu trữ thật trong [[Pipeline-xu-ly-audio]]. Chưa thấy `rubato`, `ort`, `whisper-rs`, `rusqlite` trong Cargo deps.

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- **PBI 4**: `components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic từ danh sách thật, nút làm mới, thông báo rỗng/lỗi), gắn vào `App.vue`; kèm `DeviceSelector.test.ts` (vitest, 4 test pass).
- Chưa có cửa sổ chính/phụ đề nổi riêng, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].
- **PBI 5 không đụng UI** — chỉ thi công phần lõi Rust capture mic, chưa expose Tauri command start/stop ra UI.

## Kết luận
Repo đã có 2 lát cắt logic nghiệp vụ thật trong M1: liệt kê thiết bị mic (PBI 4) và capture mic thành PCM (PBI 5), cả hai chạy trên nền interface Core đã chốt ở PBI 3. Còn lại trong M1: capture loopback hệ thống, mix mic+loopback, command start/stop qua IPC — rồi mới sang M2 (VAD) và các milestone sau.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]]
