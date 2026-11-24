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
`app/src-tauri/src/main.rs`, `lib.rs` — skeleton Tauri 2 mặc định, chưa có module audio/VAD/STT/dịch. Xem [[Trang-thai-ma-nguon]].

## Liên quan
[[Kien-truc-core-ui]], [[Pipeline-xu-ly-audio]]
