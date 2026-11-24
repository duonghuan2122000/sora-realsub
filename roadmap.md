# Roadmap Sora RealSub

Lộ trình tính năng, chia nhỏ để thi công và kiểm thử từng phần. Checkbox đánh dấu `[x]` khi hoàn thành + đã kiểm thử.

Quy ước: mỗi mục là 1 PBI/task nhỏ, nên tương ứng 1 commit hoặc 1 nhánh ngắn. Theo kiến trúc đã chốt: Core (Rust/Tauri) tách biệt UI (Vue), giao tiếp qua IPC commands/events.

## M0 — Quyết định nền tảng

- [x] Chốt OS mục tiêu (Windows trước, macOS sau hay song song)
- [x] Chốt cặp ngôn ngữ mặc định (ví dụ English → Vietnamese)
- [ ] Chốt hướng xử lý: cloud / local / cả hai (ảnh hưởng trait STT, Translator)
- [ ] Định nghĩa trait Core: `AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink` (chỉ interface, chưa implement)

## M1 — Bắt âm thanh (Audio Capture)

- [ ] Liệt kê thiết bị audio input (mic) qua Tauri command, trả về UI
- [ ] Capture mic thành PCM buffer (Rust), test bằng ghi ra file .wav
- [ ] Capture loopback âm thanh hệ thống trên Windows (WASAPI loopback), test ghi ra .wav
- [ ] Trộn (mix) mic + loopback thành 1 stream, test so khớp cả 2 nguồn nghe được trong file output
- [ ] Command Tauri `start_capture` / `stop_capture`, test start/stop không leak thread/handle

## M2 — VAD (Voice Activity Detection)

- [ ] Tích hợp model VAD (Silero VAD) đọc buffer audio, trả nhãn speech/silence cho từng frame
- [ ] Cắt audio liên tục thành các đoạn (segment) theo khoảng lặng, test với file mẫu có nói + lặng xen kẽ
- [ ] Expose event Tauri `vad-segment` phát mỗi khi có đoạn mới, test UI nhận được event

## M3 — STT streaming (khung cloud, dễ làm trước)

- [ ] Implement `SpeechToText` cho 1 provider cloud (Azure/Deepgram), test gửi file audio mẫu → nhận text đúng
- [ ] Ghép STT vào luồng: segment từ VAD → gửi STT → nhận text, test end-to-end với audio mẫu
- [ ] Event Tauri `transcript-partial` (text tạm, có thể sửa) và `transcript-final` (text chốt), test UI nhận đúng thứ tự
- [ ] Xử lý lỗi mạng/API (retry, thông báo), test giả lập mất kết nối giữa chừng

## M4 — Dịch (Translation)

- [ ] Implement `Translator` cho 1 provider (Azure Translator hoặc LLM API), test dịch 1 câu cố định ra kết quả đúng
- [ ] Ghép ngữ cảnh: gửi kèm 1-2 câu trước khi dịch câu hiện tại, test câu có đại từ/thiếu ngữ cảnh dịch đúng hơn khi có context
- [ ] Nối STT final → Translator → event `subtitle-ready` (text gốc + text dịch), test end-to-end

## M5 — Cửa sổ phụ đề (Subtitle Overlay UI)

- [ ] Tạo window Tauri riêng, always-on-top, không có border/title bar
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
- [ ] Đóng gói installer Windows (Tauri bundler), test cài trên máy sạch chạy được
- [ ] Viết tài liệu hướng dẫn sử dụng cơ bản (README) cho người dùng cuối

## Backlog (chưa ưu tiên, cần quyết định thêm)

- [ ] Hướng local/offline: whisper.cpp/faster-whisper + NLLB (thay thế M3/M4 khi cần riêng tư)
- [ ] Hỗ trợ macOS (BlackHole/ScreenCaptureKit cho loopback)
- [ ] Đa ngôn ngữ UI (i18n)
- [ ] Theme/style tùy biến cửa sổ phụ đề theo style-guide token
