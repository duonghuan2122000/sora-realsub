---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24 (cập nhật sau PBI 11)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **PBI 3**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test. Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **PBI 4**: `core/devices.rs` — liệt kê thiết bị audio input bằng `cpal` thật; Tauri command `list_input_devices_cmd`. Dependency: `cpal = "0.15"`. Chi tiết: [[PBI-4-liet-ke-thiet-bi-mic]].
- **PBI 5**: `core/mic_audio_source.rs` — `MicAudioSource`, cài đặt thật đầu tiên của trait `AudioSource` (capture mic qua `cpal`, stream giữ trên thread riêng vì `cpal::Stream` không `Send`); bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` (crate `hound`) làm bằng chứng nghe lại. `lib.rs`: `mod core` đổi thành `pub mod core`. Chi tiết: [[PBI-5-capture-mic-pcm]].
- **Mới (PBI 6)**: `core/loopback_audio_source.rs` — `LoopbackAudioSource`, cài đặt thứ hai của trait `AudioSource` (capture loopback từ thiết bị output mặc định qua crate **`wasapi`**, vì `cpal` không hỗ trợ loopback; stream giữ trên thread riêng cùng pattern với mic). Bin thủ công `src/bin/capture_loopback_to_wav.rs` ghi `.wav`. Dependency mới: `wasapi = "0.26"`. 16/16 test pass, clippy sạch. Chi tiết: [[PBI-6-capture-loopback-wasapi]].
- **Mới (PBI 7)**: `core/mixed_audio_source.rs` — `MixedAudioSource`, trộn 2 `AudioSource` con (mic + loopback) thành 1 stream bằng buffer trung gian + thread trộn chu kỳ 15ms (cộng mẫu + clamp), resample linear khi sample rate khác nhau. Không thêm dependency. Bin thủ công `src/bin/capture_mixed_to_wav.rs`. 21/21 test pass, clippy sạch. Chi tiết: [[PBI-7-tron-mic-loopback]].
- **Mới (PBI 8)**: `core/capture_session.rs` — `CaptureSessionState`, quản lý 1 phiên thu âm duy nhất (`Mutex<Option<StreamHandle>>`), expose qua Tauri command `start_capture`/`stop_capture` trong `lib.rs` (dùng `MixedAudioSource` từ PBI 7). Thêm 2 lỗi `CoreError::AlreadyRunning`/`NotRunning`. Không thêm dependency. 26/26 test pass; kiểm thử tay qua UI thật (quickstart.md) chưa chạy do môi trường không có GUI — xem [[PBI-8-start-stop-capture]]. **M1 (Audio Capture) hoàn tất.**
- **Mới (PBI 9)**: `core/vad.rs` — `detect()`/`detect_with_threshold()` chia buffer frame 512 sample @16kHz, gán nhãn speech/silence qua crate `voice_activity_detector` (Silero VAD v5). Tái dùng `resample_linear` từ `mixed_audio_source.rs` (đổi `pub(super)`). Dependency mới: `voice_activity_detector = "0.2"`. Test tích hợp `tests/vad_real_speech.rs` dùng fixture giọng nói thật `tests/fixtures/vad_sample.wav` (copy từ `capture_mic_to_wav_output.wav`, 8s im lặng-nói-im lặng) xác nhận SC-001/003 bằng đối chiếu RMS năng lượng. 31/31 test pass (30 unit + 1 fixture thật), clippy sạch. Chưa nối vào pipeline streaming, chưa có Tauri command/event. Xem [[PBI-9-vad-silero]].
- **Mới (PBI 10)**: `core/segment.rs` — `segment()`/`segment_with_min_silence()`, state machine silence-hangover cắt `Vec<FrameLabel>` (từ `vad::detect`, PBI 9) thành `Vec<Segment>` theo lượt nói, ngưỡng mặc định `DEFAULT_MIN_SILENCE_FRAMES = 9` (~300ms). Không thêm dependency. Test tích hợp `tests/segment_real_speech.rs` ghép 2 lần audio thật `vad_sample.wav` với khoảng lặng 2s chèn giữa, xác nhận đúng 2 segment. 7/7 test pass (6 unit + 1 fixture thật), clippy sạch. Chưa nối vào pipeline streaming, chưa có Tauri event. Xem [[PBI-10-cat-segment-khoang-lang]].
- **Mới (PBI 11)**: `core/vad_pipeline.rs` — `VadPipeline` nối `vad::detect` (PBI 9) + `segment::segment` (PBI 10) thành streaming segmenter: `push_frame()` tích lũy `AudioFrame`, trả về `SegmentReady` cho segment MỚI chắc chắn đóng. `lib.rs::start_capture` nối closure `on_frame` (trước đó bỏ qua mọi frame, PBI 8) vào `VadPipeline`, phát sự kiện Tauri `vad-segment` (payload camelCase) cho mỗi segment đóng. Không thêm dependency; đổi vài const/fn ở `vad.rs`/`segment.rs` sang `pub(crate)`. 42 unit test + 4 test tích hợp (gồm `tests/vad_pipeline_real_speech.rs` mới, dùng fixture `vad_sample.wav`) — tất cả pass. Kiểm thử tay UI thật qua `pnpm tauri dev` **chưa chạy** (môi trường không có mic/GUI thật, giống PBI 8). **M2 (VAD) hoàn tất.** Xem [[PBI-11-vad-segment-event]].
- **Vẫn chưa có**: chọn thiết bị output cụ thể cho loopback, STT, dịch, lưu trữ thật trong [[Pipeline-xu-ly-audio]] (M3 trở đi). Chưa thấy `rubato`, `whisper-rs`, `rusqlite` trong Cargo deps.

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- **PBI 4**: `components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic từ danh sách thật, nút làm mới, thông báo rỗng/lỗi), gắn vào `App.vue`; kèm `DeviceSelector.test.ts` (vitest, 4 test pass).
- Chưa có cửa sổ chính/phụ đề nổi riêng, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].
- **PBI 5, PBI 6, PBI 7, PBI 8 không đụng UI** — chỉ thi công phần lõi Rust capture + điều khiển phiên (mic, loopback, trộn, start/stop), command Tauri đã có nhưng chưa có nút bấm trong Vue.

## Kết luận
M1 (Audio Capture) đã hoàn tất 5/5 mục (PBI 4-8). **M2 (VAD) đã hoàn tất 3/3 mục**: gán nhãn speech/silence theo frame qua Silero VAD (PBI 9), cắt segment theo khoảng lặng (PBI 10), và phát sự kiện Tauri `vad-segment` ra UI (PBI 11) — cả ba đã xác minh bằng dữ liệu giọng nói thật qua test tích hợp. Kiểm thử tay trên UI thật (có mic/GUI) vẫn là khoảng trống chung từ PBI 8. M3 (STT streaming) là milestone kế tiếp.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]]
