---
title: Pipeline xử lý audio
date: 2026-11-24
tags: [concept, architecture]
sources: [docs/sora-realsub-overview.md, docs/tech-stack.md]
---

# Pipeline xử lý audio

1. **Lấy âm thanh**: loopback hệ thống (Windows: WASAPI loopback qua crate `wasapi` — **không phải `cpal`**, xem [[PBI-6-capture-loopback-wasapi]]; macOS roadmap: BlackHole/ScreenCaptureKit) + mic (qua `cpal`).
2. **Resample**: `rubato` chuẩn hoá về 16kHz mono cho STT.
3. **VAD**: Silero VAD v5 qua crate `voice_activity_detector` (wrapper `ort`/ONNX Runtime, model nhúng sẵn) — gán nhãn speech/silence theo frame 512 sample @16kHz, xem [[PBI-9-vad-silero]]. Cắt đoạn (segment) theo khoảng lặng (state machine silence-hangover, không thêm dependency), xem [[PBI-10-cat-segment-khoang-lang]]. Streaming segmenter `VadPipeline` nối VAD + cắt segment vào luồng `start_capture` thật, phát event Tauri `vad-segment` ra UI mỗi khi 1 đoạn đóng — xem [[PBI-11-vad-segment-event]]. **M2 hoàn tất.**
4. **STT streaming**: local (`whisper-rs`/whisper.cpp) hoặc cloud (HTTP/WebSocket qua `reqwest`, `tokio-tungstenite`: Azure Speech Translation, Deepgram + LLM).
5. **Dịch**: từng câu/cụm, kèm vài câu trước làm ngữ cảnh. Local: LLM nhỏ hoặc NLLB (ONNX/ct2). Cloud: LLM qua API.
6. **Hiển thị**: cửa sổ phụ đề nổi luôn-on-top; lưu transcript (SQLite) sau họp.

Toàn bộ chạy trên `tokio` async runtime bên Core Rust.

## Trait trừu tượng hoá
`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` — cloud/local là hai implementation khác nhau của cùng trait, xem [[Kien-truc-core-ui]].

## Trạng thái hiện tại
M1 (lấy âm thanh) đã hoàn tất: capture mic qua `cpal` (PBI 5), capture loopback qua `wasapi` (PBI 6), trộn 2 nguồn qua `MixedAudioSource` (PBI 7), command Tauri `start_capture`/`stop_capture` (PBI 8). **M2 (VAD) đã hoàn tất**: gán nhãn speech/silence theo frame qua `voice_activity_detector`/Silero VAD v5 (PBI 9, SC-001/SC-003 đã xác nhận bằng giọng nói thật), cắt buffer thành segment theo khoảng lặng (PBI 10, xác nhận bằng audio thật ghép 2 lượt nói), và nối cả hai vào luồng capture thật + phát event Tauri `vad-segment` ra UI qua `VadPipeline` (PBI 11, xác nhận bằng test tích hợp audio thật; kiểm thử tay UI thật qua mic chưa chạy do môi trường). Các bước 4-6 (STT, dịch, hiển thị — M3 trở đi) vẫn chưa triển khai. Xem [[Trang-thai-ma-nguon]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]].

## Liên quan
[[Tong-quan-du-an]], [[Core-Rust]], [[Rui-ro-ky-thuat]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]]
