---
title: Core (Rust)
date: 2026-11-24
tags: [component]
sources: [docs/tech-stack.md, app/src-tauri]
---

# Core (Rust)

| Thành phần | Lựa chọn | Ghi chú |
|---|---|---|
| Shell desktop | Tauri 2 | |
| Async runtime | tokio | Pipeline streaming |
| Bắt âm thanh | cpal (WASAPI loopback + mic) | Windows trước; macOS ScreenCaptureKit sau |
| Resample | rubato | 16kHz mono |
| VAD | Silero VAD qua ort (ONNX Runtime) | |
| STT local | whisper-rs (whisper.cpp) | GPU accel nếu có |
| STT/dịch cloud | reqwest, tokio-tungstenite | Azure Speech Translation, Deepgram + LLM |
| Dịch local | LLM nhỏ hoặc NLLB (ONNX/ct2) | |
| Lưu trữ | SQLite (rusqlite/sqlx) | Transcript, cấu hình, lịch sử |
| Cấu hình/bí mật | serde + keyring | API key trong Windows Credential Manager |
| Log | tracing | |

## Mã nguồn hiện tại
`app/src-tauri/src/main.rs`, `lib.rs` — skeleton Tauri 2 mặc định. **PBI 3**: module `core/` đã có 4 trait (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu (`AudioFrame`, `TranscriptChunk`, `Subtitle`...) + `CoreError`, chỉ interface và mock test, chưa cài đặt logic thật. Dependency mới: `async-trait`, `thiserror`.
**(PBI 4, 2026-11-24)**: `core/devices.rs` — liệt kê thiết bị audio input dùng `cpal` thật (lần đầu dùng `cpal` thay vì chỉ ghi trong bảng lựa chọn), có Tauri command `list_input_devices_cmd`. Xem [[PBI-4-liet-ke-thiet-bi-mic]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]].
**Mới (PBI 5, 2026-11-24)**: `core/mic_audio_source.rs` — `MicAudioSource` là cài đặt thật đầu tiên của trait `AudioSource` (dùng `cpal`, stream chạy trên thread riêng vì `cpal::Stream` không `Send`). Bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` bằng `hound` làm bằng chứng test. `lib.rs`: `mod core` → `pub mod core` để bin dùng được. Dependency mới: `hound` (ở `[dependencies]`, không phải dev — Cargo không cấp dev-deps cho bin thường). Xem [[PBI-5-capture-mic-pcm]].

## Liên quan
[[Kien-truc-core-ui]], [[Pipeline-xu-ly-audio]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]]
