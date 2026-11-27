# Test case T008 — Xác nhận tay trên Windows thật (quickstart.md)

**Loại**: Kiểm thử tay, cần tương tác GUI thật (global shortcut, focus đổi app, quan sát cửa sổ phụ đề) — không thể tự động hóa trong phiên làm việc này (không có công cụ điều khiển desktop/chụp màn hình tương tác được cấp cho phiên này, và cổng dev server 1420 đang bị một tiến trình `node.exe` khác chiếm — có thể là phiên `tauri dev` khác của người dùng đang mở, không nên tắt).

## Các bước cần người dùng tự xác nhận (theo `quickstart.md`)

1. `bun run tauri dev` trong `app/`.
2. Click sang app khác để cửa sổ phụ đề mất focus, nhấn `Ctrl+Alt+H` → cửa sổ phụ đề ẩn trong < 0.5s.
3. Gọi `start_capture` → xác nhận phiên dịch vẫn chạy khi cửa sổ đang ẩn.
4. Nhấn `Ctrl+Alt+H` lần nữa → cửa sổ hiện lại đúng vị trí/cỡ chữ cũ, hiển thị dòng phụ đề mới nhất.
5. Nhấn liên tục nhiều lần nhanh → trạng thái đổi đúng theo số lần nhấn.

**Trạng thái**: PASS — người dùng đã tự chạy `bun run tauri dev` trên Windows thật, nhấn `Ctrl+Alt+H` khi cửa sổ phụ đề không focus, xác nhận ẩn/hiện đúng (2026-11-25). `cargo build`/`cargo test` đã PASS từ trước (xem T001, T007).
