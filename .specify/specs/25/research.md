# Nghiên cứu: PBI 25 — Ẩn-hiện cửa sổ phụ đề

## Quyết định 1: Cơ chế bắt phím tắt toàn cục

- **Quyết định**: Dùng crate chính thức `tauri-plugin-global-shortcut` (v2), đăng ký 1 shortcut cố định (ví dụ `Ctrl+Alt+H`) lúc `setup()`.
- **Lý do**: Phải bắt được phím khi cửa sổ phụ đề không focus (FR-001) — window-level `keydown` trong Vue không đủ vì cửa sổ phụ đề `skip_taskbar`/không có focus khi người dùng đang ở app khác (Zoom, trình duyệt...). Plugin chính thức của Tauri, không cần code OS-specific.
- **Phương án khác đã xem xét**: Tự bắt raw input qua `winapi`/`x11` riêng từng OS — vi phạm nguyên tắc tái dùng, tăng rủi ro kỹ thuật không cần thiết khi đã có plugin chính thức hỗ trợ cả Windows/Linux.

## Quyết định 2: Tổ hợp phím tắt mặc định

- **Quyết định**: `Ctrl+Alt+H` (Hide/show).
- **Lý do**: Ít trùng với shortcut hệ thống/app phổ biến (Zoom dùng `Alt+V`, Teams dùng `Ctrl+Shift+M`...), dễ nhớ (H = hide).
- **Phương án khác**: `Ctrl+Shift+Space` — dễ trùng input method tiếng Việt (Telex/VNI thường dùng tổ hợp có Shift+phím chữ), loại.

## Quyết định 3: Ẩn vs đóng cửa sổ

- **Quyết định**: Dùng `WebviewWindow::hide()`/`show()`, không `close()`/`destroy()`.
- **Lý do**: Giữ nguyên state Vue (vị trí cuộn, N dòng phụ đề đang hiển thị) và tránh phải tạo lại window (giữ nguyên vị trí/cỡ chữ đã lưu — FR-004, SC-003) — tái dùng window hiện có từ PBI 20, không tạo thêm luồng quản lý state.

## Quyết định 4: Nơi lưu trạng thái ẩn/hiện

- **Quyết định**: Lưu 1 biến `AtomicBool` (hoặc `Mutex<bool>`) trong Tauri state (`app.manage`), không lưu ở phía Vue.
- **Lý do**: Phím tắt được xử lý hoàn toàn ở Rust (global-shortcut handler chạy ngoài context của webview); Vue không cần biết trạng thái ẩn/hiện để hoạt động đúng (FR-003: pipeline chạy nền không phụ thuộc UI).
