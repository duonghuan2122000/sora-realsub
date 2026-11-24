# Danh sách Task: Lệnh bắt đầu/kết thúc phiên bắt âm thanh

**Mã PBI**: 8
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Đọc lại `app/src-tauri/src/core/mixed_audio_source.rs`, `audio_source.rs`, `types.rs`, `lib.rs` để xác nhận chữ ký `AudioSource::start`, `StreamHandle` chưa đổi so với plan.md

## Pha 2: Foundational

- [X] T002 Tạo `app/src-tauri/src/core/capture_session.rs` với struct `CaptureSessionState { handle: Mutex<Option<StreamHandle>> }`, có hàm `new()`, `start(source: Box<dyn AudioSource>, on_frame) -> Result<(), CoreError>` (lỗi nếu đã có phiên chạy) và `stop() -> Result<(), CoreError>` (lỗi nếu chưa có phiên)
- [X] T003 Thêm biến thể lỗi mới (ví dụ `CoreError::AlreadyRunning`, `CoreError::NotRunning`) vào `app/src-tauri/src/core/error.rs` nếu `CoreError` hiện tại chưa đủ biểu diễn 2 lỗi này
- [X] T004 Export `CaptureSessionState` trong `app/src-tauri/src/core/mod.rs`

## Pha 3: User Story 1 - Bắt đầu/dừng phiên thu âm qua lệnh IPC (Ưu tiên: P1)

**Mục tiêu**: Người dùng (qua UI/DevTools) gọi `start_capture`/`stop_capture` để điều khiển phiên thu âm mic+loopback, không leak tài nguyên.
**Tiêu chí kiểm thử độc lập**: Gọi `start_capture` → `stop_capture` qua `invoke` trong webview không lỗi; gọi sai thứ tự trả lỗi rõ ràng; `cargo test` xác nhận không leak.

- [X] T005 [US1] Viết unit test trong `app/src-tauri/src/core/capture_session.rs` dùng mock `AudioSource` (đếm start/stop qua `AtomicUsize`) xác nhận: start thành công khi chưa chạy, start thất bại khi đã chạy, stop thành công khi đang chạy (và resource được giải phóng — đếm giảm về 0), stop thất bại khi chưa chạy
- [X] T006 [US1] Viết test lặp chu trình start→stop 10 lần trong `app/src-tauri/src/core/capture_session.rs` xác nhận số resource còn sống luôn về 0 sau mỗi lần stop (SC-002)
- [X] T007 [US1] Thêm `tauri::command` `start_capture` và `stop_capture` trong `app/src-tauri/src/lib.rs`, dùng `tauri::State<CaptureSessionState>`, dựng `MixedAudioSource::new(Box::new(MicAudioSource::...), Box::new(LoopbackAudioSource::...))` khi start, trả `Result<(), String>`
- [X] T008 [US1] Đăng ký `CaptureSessionState` qua `.manage(CaptureSessionState::new())` và thêm `start_capture, stop_capture` vào `tauri::generate_handler!` trong `app/src-tauri/src/lib.rs`
- [ ] T009 [US1] Kiểm thử tay theo `quickstart.md`: `npm run tauri dev` trong `app/`, gọi `invoke('start_capture')`/`invoke('stop_capture')` qua DevTools console, xác nhận đúng 4 kịch bản chấp nhận trong spec.md; chụp ảnh/log kết quả làm bằng chứng — **CHƯA CHẠY**: môi trường thi công hiện tại (CLI, không có GUI session) không mở được cửa sổ Tauri dev để test tay; đã bù bằng unit test T005/T006 phủ đủ 4 kịch bản qua `CaptureSessionState` trực tiếp (không qua IPC thật). Cần người dùng tự chạy `npm run tauri dev` và xác nhận tay trên máy có GUI trước khi coi PBI này đã kiểm thử tay đầy đủ.

## Pha cuối: Polish & Cross-cutting

- [X] T010 Chạy `cargo test` toàn bộ trong `app/src-tauri/` xác nhận không có test nào vỡ, review lại code mới theo style-guide (Core/UI tách biệt, không logic nghiệp vụ lọt vào `lib.rs` ngoài phần khởi tạo command)

## Sơ đồ phụ thuộc

Setup (T001) → Foundational (T002-T004) → User Story 1 (T005-T009, tuần tự vì cùng sửa `capture_session.rs`/`lib.rs`) → Polish (T010).
Không có User Story 2 — PBI này chỉ có 1 lát cắt (P1).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (T005-T009) — đây là PBI nhỏ, chỉ có 1 story.
- Thứ tự giao hàng: Setup → Foundational → US1 → Polish, không cắt nhỏ hơn.
