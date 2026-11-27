# Quickstart: kiểm thử nhanh PBI 25 — Ẩn-hiện cửa sổ phụ đề

1. Chạy app dev: `bun run tauri dev` (trong `app/`).
2. Cửa sổ phụ đề hiện ra mặc định (always-on-top).
3. Click sang 1 app khác (để cửa sổ phụ đề mất focus) rồi nhấn `Ctrl+Alt+H` → cửa sổ phụ đề phải ẩn ngay (SC-002: < 0.5s).
4. Bấm Start capture (nếu đã nối UI) hoặc gọi `start_capture` qua devtools console — xác nhận phiên dịch vẫn chạy trong lúc ẩn (log console/log phụ đề mới vẫn phát sinh ở backend — FR-003).
5. Nhấn `Ctrl+Alt+H` lần nữa → cửa sổ hiện lại đúng vị trí/cỡ chữ trước đó (SC-003), hiển thị dòng phụ đề mới nhất đã phát sinh lúc ẩn.
6. Nhấn liên tục nhiều lần nhanh → trạng thái ẩn/hiện đổi đúng theo số lần nhấn (không kẹt).
