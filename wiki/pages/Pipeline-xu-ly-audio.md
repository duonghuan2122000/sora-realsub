---
title: Pipeline xử lý audio
date: 2026-11-24
tags: [concept, architecture]
sources: [docs/sora-realsub-overview.md, docs/tech-stack.md]
---

# Pipeline xử lý audio

1. **Lấy âm thanh**: loopback hệ thống (Windows: WASAPI loopback qua crate `wasapi` — **không phải `cpal`**, xem [[PBI-6-capture-loopback-wasapi]]; macOS roadmap: BlackHole/ScreenCaptureKit) + mic (qua `cpal`).
2. **Resample**: `rubato` chuẩn hoá về 16kHz mono cho STT.
3. **VAD**: Silero VAD qua `ort` (ONNX Runtime) — cắt đoạn có tiếng nói.
4. **STT streaming**: local (`whisper-rs`/whisper.cpp) hoặc cloud (HTTP/WebSocket qua `reqwest`, `tokio-tungstenite`: Azure Speech Translation, Deepgram + LLM).
5. **Dịch**: từng câu/cụm, kèm vài câu trước làm ngữ cảnh. Local: LLM nhỏ hoặc NLLB (ONNX/ct2). Cloud: LLM qua API.
6. **Hiển thị**: cửa sổ phụ đề nổi luôn-on-top; lưu transcript (SQLite) sau họp.

Toàn bộ chạy trên `tokio` async runtime bên Core Rust.

## Trait trừu tượng hoá
`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` — cloud/local là hai implementation khác nhau của cùng trait, xem [[Kien-truc-core-ui]].

## Trạng thái hiện tại
Bước 1 (lấy âm thanh) đang hoàn thiện dần: capture mic thật qua `cpal` (PBI 5) và capture loopback hệ thống thật qua `wasapi` (PBI 6) đã xong, nhưng **chưa mix 2 nguồn lại thành 1 stream** — đó là PBI tiếp theo trong M1. Các bước 2-6 (resample, VAD, STT, dịch, hiển thị) vẫn chưa triển khai. Xem [[Trang-thai-ma-nguon]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]].

## Liên quan
[[Tong-quan-du-an]], [[Core-Rust]], [[Rui-ro-ky-thuat]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]]
