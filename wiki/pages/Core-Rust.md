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
| Bắt âm thanh | cpal (mic) + wasapi (loopback) | Windows trước; macOS ScreenCaptureKit sau. **Cập nhật PBI 6**: dự định ban đầu dùng `cpal` cho cả loopback đã sai — `cpal` không expose cờ loopback của WASAPI, phải dùng crate `wasapi` riêng cho loopback, xem [[PBI-6-capture-loopback-wasapi]] |
| Resample | rubato | 16kHz mono |
| VAD | Silero VAD v5 qua crate `voice_activity_detector` (wrapper `ort`/ONNX Runtime) | **Cập nhật PBI 9**: dùng crate có sẵn thay vì tự tích hợp `ort` thủ công, xem [[PBI-9-vad-silero]] |
| STT local | whisper-rs (whisper.cpp) | GPU accel nếu có |
| STT/dịch cloud | reqwest, tokio-tungstenite | Azure Speech Translation, Deepgram + LLM |
| Dịch local | LLM nhỏ hoặc NLLB (ONNX/ct2) | |
| Lưu trữ | SQLite (rusqlite/sqlx) | Transcript, cấu hình, lịch sử |
| Cấu hình/bí mật | serde + keyring | API key trong Windows Credential Manager |
| Log | tracing | |

## Mã nguồn hiện tại
`app/src-tauri/src/main.rs`, `lib.rs` — skeleton Tauri 2 mặc định. **PBI 3**: module `core/` đã có 4 trait (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu (`AudioFrame`, `TranscriptChunk`, `Subtitle`...) + `CoreError`, chỉ interface và mock test, chưa cài đặt logic thật. Dependency mới: `async-trait`, `thiserror`.
**(PBI 4, 2026-11-24)**: `core/devices.rs` — liệt kê thiết bị audio input dùng `cpal` thật (lần đầu dùng `cpal` thay vì chỉ ghi trong bảng lựa chọn), có Tauri command `list_input_devices_cmd`. Xem [[PBI-4-liet-ke-thiet-bi-mic]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]].
**(PBI 5, 2026-11-24)**: `core/mic_audio_source.rs` — `MicAudioSource` là cài đặt thật đầu tiên của trait `AudioSource` (dùng `cpal`, stream chạy trên thread riêng vì `cpal::Stream` không `Send`). Bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` bằng `hound` làm bằng chứng test. `lib.rs`: `mod core` → `pub mod core` để bin dùng được. Dependency mới: `hound` (ở `[dependencies]`, không phải dev — Cargo không cấp dev-deps cho bin thường). Xem [[PBI-5-capture-mic-pcm]].
**Mới (PBI 6, 2026-11-24)**: `core/loopback_audio_source.rs` — `LoopbackAudioSource` là cài đặt thứ hai của trait `AudioSource`, capture loopback từ thiết bị output mặc định. `cpal` không hỗ trợ loopback nên dùng crate riêng **`wasapi`**: mở `AudioClient` từ `Device` theo `Direction::Render` rồi `initialize_client(..., Direction::Capture, ...)`. Stream cũng chạy trên thread riêng (cùng lý do `Send`). Bin thủ công `src/bin/capture_loopback_to_wav.rs`. Dependency mới: `wasapi = "0.26"`. Xem [[PBI-6-capture-loopback-wasapi]].
**Mới (PBI 7, 2026-11-24)**: `core/mixed_audio_source.rs` — `MixedAudioSource`, nhận 2 `Box<dyn AudioSource>` (mic + loopback), trộn thành 1 stream: mỗi nguồn con tự chạy thread riêng của nó như cũ, đẩy mẫu vào buffer trung gian; 1 thread trộn riêng chu kỳ 15ms rút mẫu từ 2 buffer, cộng + `clamp(-1.0, 1.0)`, resample linear khi 2 nguồn có sample rate khác nhau. Không thêm dependency. Bin thủ công `src/bin/capture_mixed_to_wav.rs`. Xem [[PBI-7-tron-mic-loopback]].
**Mới (PBI 8, 2026-11-24)**: `core/capture_session.rs` — `CaptureSessionState` (`Mutex<Option<StreamHandle>>`) quản lý 1 phiên thu âm duy nhất, expose qua 2 Tauri command `start_capture`/`stop_capture` trong `lib.rs` (dùng `.manage()` + `State`), dựng `MixedAudioSource` từ PBI 7 bên trong. Thêm 2 variant lỗi `CoreError::AlreadyRunning`/`NotRunning`. Không thêm dependency. Hoàn tất M1 (Audio Capture). Xem [[PBI-8-start-stop-capture]].
**Mới (PBI 9, 2026-11-24)**: `core/vad.rs` — `detect()`/`detect_with_threshold()` chia buffer PCM thành frame 512 sample @16kHz (resample qua `resample_linear` tái dùng từ `mixed_audio_source.rs`, đổi `pub(super)`), gán nhãn speech/silence bằng crate `voice_activity_detector` (Silero VAD v5). Bắt đầu M2 (VAD). Dependency mới: `voice_activity_detector = "0.2"`. Chưa nối vào pipeline streaming, chưa có Tauri command/event. **Cập nhật PBI 10**: nhánh "speech" đã được xác nhận bằng giọng nói thật (fixture `vad_sample.wav`), không còn là giả định mở. Xem [[PBI-9-vad-silero]].
**Mới (PBI 10, 2026-11-24)**: `core/segment.rs` — `segment()`/`segment_with_min_silence()` dùng danh sách `FrameLabel` (từ `vad::detect`) để cắt thành `Segment { start_frame_index, end_frame_index }` theo thuật toán state machine silence-hangover (ngưỡng mặc định ~300ms/9 frame). Không thêm dependency. Tiếp tục M2 (VAD), chưa nối Tauri event `vad-segment`. Xem [[PBI-10-cat-segment-khoang-lang]].

## Liên quan
[[Kien-truc-core-ui]], [[Pipeline-xu-ly-audio]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]]
