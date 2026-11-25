# Roadmap Sora RealSub

Lộ trình tính năng, chia nhỏ để thi công và kiểm thử từng phần. Checkbox đánh dấu `[x]` khi hoàn thành + đã kiểm thử.

Quy ước: mỗi mục là 1 PBI/task nhỏ, nên tương ứng 1 commit hoặc 1 nhánh ngắn. Theo kiến trúc đã chốt: Core (Rust/Tauri) tách biệt UI (Vue), giao tiếp qua IPC commands/events.

## M0 — Quyết định nền tảng

- [x] Chốt OS mục tiêu (ban đầu: Windows trước, macOS sau — **cập nhật**: hỗ trợ **Windows + Linux song song**, macOS vẫn để sau; các mục khác nhau theo OS được tách riêng trong từng milestone, ví dụ mục "Capture loopback hệ thống — theo OS" ở M1)
- [x] Chốt cặp ngôn ngữ mặc định (ví dụ English → Vietnamese)
- [x] Chốt hướng xử lý: local (ảnh hưởng trait STT, Translator)
- [x] Định nghĩa trait Core: `AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` (chỉ interface, chưa implement)

## M1 — Bắt âm thanh (Audio Capture)

- [x] Liệt kê thiết bị audio input (mic) qua Tauri command, trả về UI
- [x] Capture mic thành PCM buffer (Rust), test bằng ghi ra file .wav
- [x] Trộn (mix) mic + loopback thành 1 stream, test so khớp cả 2 nguồn nghe được trong file output
- [x] Command Tauri `start_capture` / `stop_capture`, test start/stop không leak thread/handle

### Capture loopback hệ thống — theo OS

Dùng chung trait `AudioSource` (gate theo `cfg(windows)`/`cfg(target_os = "linux")`
trong `Cargo.toml`/`core/mod.rs`, xem `wiki/pages/Rui-ro-ky-thuat.md`), mỗi OS có
cài đặt loopback riêng:

- [x] **Windows**: capture loopback qua WASAPI loopback (crate `wasapi`), test ghi ra .wav (PBI 6)
- [ ] **Linux**: capture loopback qua PulseAudio/PipeWire monitor source (`libpulse-binding`, `@DEFAULT_MONITOR@`), test ghi ra .wav (PBI 18: đã làm xong + test pass — `cargo build`/`cargo test` chạy được trên Linux, capture loopback thật xác nhận bằng PulseAudio + tone test 440Hz ghi đúng ra .wav; xem `wiki/pages/PBI-18-loopback-linux.md`. **Chưa tick** vì chỉ xác nhận trên máy Linux dùng để implement — cần người có máy Linux khác (distro/cấu hình PulseAudio hoặc PipeWire khác) xác nhận lại trước khi tick)
- [ ] **Windows (regression sau PBI 18)**: xác nhận lại `cargo test`/`capture_loopback_to_wav` trên Windows thật không đổi hành vi so với trước PBI 18 (code chỉ thêm `#[cfg(windows)]` bọc ngoài, logic không đổi, nhưng chưa có máy Windows trong môi trường implement để chạy lại) — chỉ tick khi đã xác nhận 0 regression

## M2 — VAD (Voice Activity Detection)

- [x] Tích hợp model VAD (Silero VAD) đọc buffer audio, trả nhãn speech/silence cho từng frame
- [x] Cắt audio liên tục thành các đoạn (segment) theo khoảng lặng, test với file mẫu có nói + lặng xen kẽ
- [x] Expose event Tauri `vad-segment` phát mỗi khi có đoạn mới, test UI nhận được event (PBI 11; UI nhận được event xác nhận bằng test tích hợp audio thật `vad_pipeline_real_speech.rs`, chưa kiểm thử tay trên UI/mic thật)

## M3 — STT streaming (local)

- [x] Implement `SpeechToText` bằng model local (whisper.cpp hoặc faster-whisper), test gửi file audio mẫu → nhận text đúng (PBI 12: `whisper-rs`, xác nhận bằng model + giọng nói thật)
- [x] Ghép STT vào luồng: segment từ VAD → gửi STT → nhận text, test end-to-end với audio mẫu (PBI 13: `TranscriptPipeline`)
- [x] Event Tauri `transcript-partial` (text tạm, có thể sửa) và `transcript-final` (text chốt), test UI nhận đúng thứ tự (PBI 14: `TranscriptPipeline` nối thật vào `start_capture`, xác nhận bằng test mock STT; `transcript-partial` chỉ test được bằng mock vì whisper.cpp hiện chưa hỗ trợ partial thật)
- [x] Xử lý lỗi runtime model (thiếu model file, lỗi load/infer, thông báo), test giả lập các case lỗi trên (PBI 15: event Tauri `model-error` loại `load`/`infer`, lỗi 1 segment không dừng pipeline)

## M4 — Dịch (Translation)

- [x] Implement `Translator` bằng model local (NLLB hoặc tương đương, chạy offline), test dịch 1 câu cố định ra kết quả đúng (PBI 16: `SidecarTranslator` chạy qua sidecar process riêng `app/translate-sidecar/` (crate Cargo độc lập, dùng `ct2rs`) để tránh xung đột CRT với `ort`; model dịch `gaudi/opus-mt-en-vi-ctranslate2` (Marian-MT, **license Apache-2.0, open-source**), xác nhận SC-001 + FR-004 bằng dữ liệu thật, 5/5 test pass)
- [ ] Ghép ngữ cảnh: gửi kèm 1-2 câu trước khi dịch câu hiện tại, test câu có đại từ/thiếu ngữ cảnh dịch đúng hơn khi có context (PBI 17: code `SidecarTranslator::translate()` đã ghép/tách ngữ cảnh + fallback, 13/13 test logic pass trên crate scratch độc lập — **CHƯA tick** vì `cargo test -p app translator` chưa chạy được thật trong container Linux này, bị chặn bởi dependency `wasapi` không gate theo target [có từ PBI 6, không liên quan PBI 17] — xem `wiki/pages/Rui-ro-ky-thuat.md`; cần xác nhận lại trên máy Windows trước khi tick)
- [x] Nối STT final → Translator → event `subtitle-ready` (text gốc + text dịch), test end-to-end (PBI 19: hàm thuần `translate_final_chunk` mới trong `lib.rs`, tách khỏi `start_capture` để test bằng mock Translator; nối `RecentTranscriptContext` (PBI 17) + `SidecarTranslator` (PBI 16) vào callback `on_chunk` của `TranscriptPipeline`; phát event Tauri `subtitle-ready`, lỗi dịch báo qua `model-error` kiểu `translate` mới, không dừng phiên; 65/65 test pass (2 ignored cần sidecar/model thật — test tích hợp SC-002 `tests/subtitle_pipeline_real.rs` compile sạch nhưng chưa chạy thật trong container này); hoàn tất M4 (3/3))

## M5 — Cửa sổ phụ đề (Subtitle Overlay UI)

- [x] Tạo window Tauri riêng, always-on-top, không có border/title bar (PBI 20: `create_subtitle_window()` trong `lib.rs` + entry point `subtitle.html`; SC-001 + SC-002 đã xác nhận trực tiếp bởi người dùng trên máy Windows thật qua `bun run tauri dev`, xem `.specify/specs/20/evidence/T005-testcase.md`)
- [ ] Hiển thị text phụ đề nhận từ event `subtitle-ready`, test hiển thị đúng khi có event liên tục
- [ ] Auto-scroll/fade dòng cũ khi có dòng mới, test với nhiều dòng liên tiếp
- [ ] Cho phép kéo-thả di chuyển vị trí cửa sổ, test vị trí được giữ sau khi thả
- [ ] Cho phép resize / chỉnh cỡ chữ, test lưu lại cỡ chữ sau khi đóng-mở app
- [ ] Nút/phím tắt ẩn-hiện cửa sổ phụ đề, test toggle hoạt động đúng

## M6 — Điều khiển & cấu hình (Control UI)

- [ ] Màn hình chính: chọn thiết bị mic + nguồn loopback, test chọn sai thiết bị báo lỗi rõ ràng
- [ ] Nút Start/Stop phiên dịch phụ đề, test trạng thái UI đồng bộ với Core (đang chạy/đã dừng)
- [ ] Chọn cặp ngôn ngữ nguồn-đích từ UI, test đổi ngôn ngữ áp dụng ngay cho phiên mới
- [ ] Lưu cấu hình (thiết bị, ngôn ngữ, vị trí cửa sổ) vào file config, test khởi động lại app giữ cấu hình cũ

## M7 — Lưu transcript sau họp

- [ ] Tích lũy toàn bộ `transcript-final` (gốc + dịch) trong 1 phiên vào bộ nhớ Core
- [ ] Xuất transcript ra file (txt hoặc srt) khi dừng phiên, test file xuất ra đúng nội dung, đúng thứ tự thời gian
- [ ] Màn hình xem lại transcript các phiên cũ trong UI, test mở lại 1 phiên cũ hiển thị đúng nội dung đã lưu

## M8 — Hoàn thiện & đóng gói

- [ ] Xử lý lỗi toàn cục (crash audio device, mất mạng) hiển thị thông báo thân thiện cho user, test các case lỗi đã liệt kê

### Đóng gói installer — theo OS

- [ ] **Windows**: đóng gói installer qua Tauri bundler (MSI/NSIS), test cài trên máy sạch chạy được
- [ ] **Linux**: đóng gói installer qua Tauri bundler (AppImage/.deb), test cài trên máy sạch chạy được

- [ ] Viết tài liệu hướng dẫn sử dụng cơ bản (README) cho người dùng cuối

## Backlog (chưa ưu tiên, cần quyết định thêm)

- [ ] Hỗ trợ macOS (BlackHole/ScreenCaptureKit cho loopback)
- [ ] Đa ngôn ngữ UI (i18n)
- [ ] Theme/style tùy biến cửa sổ phụ đề theo style-guide token
