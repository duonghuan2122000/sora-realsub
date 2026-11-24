---
title: Pipeline xử lý audio
date: 2026-11-24
tags: [concept, architecture]
sources: [docs/sora-realsub-overview.md, docs/tech-stack.md]
---

# Pipeline xử lý audio

1. **Lấy âm thanh**: loopback hệ thống (Windows: WASAPI loopback qua `cpal`; macOS roadmap: BlackHole/ScreenCaptureKit) + mic.
2. **Resample**: `rubato` chuẩn hoá về 16kHz mono cho STT.
3. **VAD**: Silero VAD qua `ort` (ONNX Runtime) — cắt đoạn có tiếng nói.
4. **STT streaming**: local (`whisper-rs`/whisper.cpp) hoặc cloud (HTTP/WebSocket qua `reqwest`, `tokio-tungstenite`: Azure Speech Translation, Deepgram + LLM).
5. **Dịch**: từng câu/cụm, kèm vài câu trước làm ngữ cảnh. Local: LLM nhỏ hoặc NLLB (ONNX/ct2). Cloud: LLM qua API.
6. **Hiển thị**: cửa sổ phụ đề nổi luôn-on-top; lưu transcript (SQLite) sau họp.

Toàn bộ chạy trên `tokio` async runtime bên Core Rust.

## Trait trừu tượng hoá
`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` — cloud/local là hai implementation khác nhau của cùng trait, xem [[Kien-truc-core-ui]].

## Trạng thái hiện tại
Chưa triển khai — mới ở mức thiết kế tài liệu. Xem [[Trang-thai-ma-nguon]].

## Liên quan
[[Tong-quan-du-an]], [[Core-Rust]], [[Rui-ro-ky-thuat]]
