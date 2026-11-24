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
4. **STT streaming**: local (`whisper-rs`/whisper.cpp) hoặc cloud (HTTP/WebSocket qua `reqwest`, `tokio-tungstenite`: Azure Speech Translation, Deepgram + LLM). **Cập nhật PBI 12**: đã cài đặt thật `WhisperSpeechToText` (local, whisper.cpp), xác nhận bằng model + giọng nói thật — xem [[PBI-12-stt-local-whisper]]. **Cập nhật PBI 13**: `TranscriptPipeline` nối `VadPipeline` (segment VAD, PBI 11) → `WhisperSpeechToText` (PBI 12) thành luồng end-to-end trên file audio mẫu, giữ đúng thứ tự thời gian — xem [[PBI-13-ghep-stt-vao-luong]]. **Cập nhật PBI 14**: `TranscriptPipeline` nối thật vào `start_capture`, phát event Tauri `transcript-partial`/`transcript-final` ra UI mỗi khi 1 segment có kết quả STT (hiện tại chỉ `transcript-final` vì whisper.cpp chưa hỗ trợ partial thật) — xem [[PBI-14-transcript-partial-final-event]]. **Cập nhật PBI 15**: xử lý lỗi runtime model (nạp/infer), phát event Tauri `model-error`, lỗi 1 segment không dừng pipeline — xem [[PBI-15-xu-ly-loi-runtime-model-stt]]. **M3 hoàn tất 4/4.**
5. **Dịch**: từng câu/cụm, kèm vài câu trước làm ngữ cảnh. **Cập nhật PBI 16**: `NllbTranslator` cài đặt thật đầu tiên của trait `Translator`, nhưng chạy model qua **sidecar executable ngoài** (`std::process::Command`) chứ không in-process — đổi hướng giữa implement do xung đột CRT native giữa binding dịch (`ct2rs`) và `ort`/VAD, xem [[Rui-ro-ky-thuat]], [[PBI-16-dich-local-sidecar]]. Dịch đúng câu cố định (SC-001) **chưa xác nhận bằng model thật**. Chưa ghép ngữ cảnh, chưa nối vào bước 4→6. **M4 bắt đầu 1/3.**
6. **Hiển thị**: cửa sổ phụ đề nổi luôn-on-top; lưu transcript (SQLite) sau họp.

Toàn bộ chạy trên `tokio` async runtime bên Core Rust.

## Trait trừu tượng hoá
`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` — cloud/local là hai implementation khác nhau của cùng trait, xem [[Kien-truc-core-ui]].

## Trạng thái hiện tại
M1 (lấy âm thanh) đã hoàn tất: capture mic qua `cpal` (PBI 5), capture loopback qua `wasapi` (PBI 6), trộn 2 nguồn qua `MixedAudioSource` (PBI 7), command Tauri `start_capture`/`stop_capture` (PBI 8). M2 (VAD) đã hoàn tất: gán nhãn speech/silence theo frame qua `voice_activity_detector`/Silero VAD v5 (PBI 9), cắt buffer thành segment theo khoảng lặng (PBI 10), nối cả hai vào luồng capture thật + phát event Tauri `vad-segment` ra UI qua `VadPipeline` (PBI 11). M3 (STT streaming) đã hoàn tất 4/4: cài đặt thật `WhisperSpeechToText` bằng whisper.cpp local (PBI 12); `TranscriptPipeline` ghép segment VAD → STT end-to-end (PBI 13); nối thật vào `start_capture`, phát event Tauri `transcript-partial`/`transcript-final` (PBI 14); xử lý lỗi runtime model, phát event `model-error` (PBI 15). **M4 (Dịch) bắt đầu 1/3**: `NllbTranslator` cài đặt thật trait `Translator`, chạy qua sidecar process (PBI 16) — chưa xác nhận dịch đúng bằng model thật, chưa ghép ngữ cảnh, chưa nối vào bước 6 (hiển thị). Xem [[Trang-thai-ma-nguon]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]], [[PBI-12-stt-local-whisper]], [[PBI-13-ghep-stt-vao-luong]], [[PBI-14-transcript-partial-final-event]], [[PBI-15-xu-ly-loi-runtime-model-stt]], [[PBI-16-dich-local-sidecar]].

## Liên quan
[[Tong-quan-du-an]], [[Core-Rust]], [[Rui-ro-ky-thuat]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]], [[PBI-12-stt-local-whisper]], [[PBI-13-ghep-stt-vao-luong]], [[PBI-14-transcript-partial-final-event]], [[PBI-15-xu-ly-loi-runtime-model-stt]], [[PBI-16-dich-local-sidecar]]
