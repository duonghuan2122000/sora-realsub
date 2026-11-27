---
title: PBI 25 — Ẩn-hiện cửa sổ phụ đề bằng phím tắt
date: 2026-11-25
tags: [pbi, M5, core-rust, subtitle-window, global-shortcut]
sources: [.specify/specs/25/spec.md, .specify/specs/25/plan.md, .specify/specs/25/research.md, .specify/specs/25/tasks.md, .specify/specs/25/evidence/]
---

# PBI 25 — Ẩn-hiện cửa sổ phụ đề bằng phím tắt

Hoàn tất mục cuối cùng của roadmap M5 "Nút/phím tắt ẩn-hiện cửa sổ phụ đề". Khác các PBI M5 trước (20-24, toàn bộ ở UI Vue), PBI này thuần Core Rust vì cần bắt phím toàn cục ngay cả khi cửa sổ phụ đề không có focus (ví dụ đang focus Zoom/trình duyệt khi họp).

## Quyết định chính

- **Cơ chế bắt phím**: crate chính thức `tauri-plugin-global-shortcut` (v2) — hỗ trợ sẵn cả Windows/Linux, không cần code riêng theo OS như các PBI audio capture ([[PBI-6-capture-loopback-wasapi]], [[PBI-18-loopback-linux]]).
- **Tổ hợp phím**: `Ctrl+Alt+H` — chọn để tránh trùng shortcut phổ biến (Zoom `Alt+V`, Teams `Ctrl+Shift+M`, input method tiếng Việt thường dùng `Ctrl/Shift+phím chữ`).
- **Ẩn vs đóng cửa sổ**: dùng `WebviewWindow::hide()`/`show()`, không `close()`/tạo lại — giữ nguyên vị trí + cỡ chữ đã lưu qua `localStorage` ([[PBI-24-chinh-co-chu-phu-de]]) mà không cần thêm logic khôi phục state.
- **Nơi giữ trạng thái hiện/ẩn**: `AtomicBool` trong Tauri state (`SubtitleVisibilityState`), xử lý hoàn toàn ở Rust — Vue không cần biết (pipeline phụ đề chạy nền độc lập UI, đúng nguyên tắc tách Core/UI của [[Kien-truc-core-ui]]).

## Cài đặt

- `app/src-tauri/Cargo.toml`: thêm `tauri-plugin-global-shortcut = "2"`.
- `app/src-tauri/capabilities/default.json`: thêm permission `global-shortcut:default`.
- `app/src-tauri/src/lib.rs`:
  - Hàm thuần `toggle_subtitle_visibility(is_visible: bool) -> bool` (chỉ đảo bit) — tách khỏi Tauri runtime để `cargo test` trực tiếp, cùng pattern với `translate_final_chunk` ([[PBI-19-noi-stt-dich-subtitle-ready]]).
  - `SubtitleVisibilityState(AtomicBool)` quản lý qua `app.manage(...)`.
  - Đăng ký plugin `tauri_plugin_global_shortcut::Builder` với `with_handler` trong `run()`; đăng ký shortcut `Ctrl+Alt+H` trong `setup()`. Handler: đọc state → gọi `toggle_subtitle_visibility` → ghi lại state → `window("subtitle").show()`/`.hide()`.
  - Không đổi `create_subtitle_window` ([[PBI-20-cua-so-phu-de-noi]]), không thêm Tauri command/event mới, không đụng UI Vue.

## Bằng chứng kiểm thử

- `cargo build --lib`: PASS (`T001-cargo-build.log`), biên dịch plugin mới + toàn bộ code sạch.
- `cargo test --lib`: **67/67 pass, 0 fail, 2 ignored** (2 ignored từ trước, không liên quan PBI 25) — gồm 2 test mới cho `toggle_subtitle_visibility` (`T007-cargo-test.log`, `T007-testcase.md`): đảo đúng `true<->false`, và đảo liên tục 10 lần đúng chu kỳ 2 (trường hợp biên "nhấn liên tục nhiều lần nhanh" trong spec.md).
- **T008 — kiểm thử tay trên Windows thật**: PASS. Người dùng tự chạy `bun run tauri dev`, nhấn `Ctrl+Alt+H` khi cửa sổ phụ đề không focus, xác nhận ẩn/hiện đúng (2026-11-25). Xem `.specify/specs/25/evidence/T008-testcase.md`. Roadmap đã tick dòng tương ứng.

## Liên quan

- [[PBI-20-cua-so-phu-de-noi]], [[PBI-21-hien-thi-phu-de]], [[PBI-22-auto-scroll-fade-phu-de]], [[PBI-23-keo-tha-cua-so-phu-de]], [[PBI-24-chinh-co-chu-phu-de]] — cùng chuỗi M5, PBI này là mục cuối (6/6).
- [[Trang-thai-ma-nguon]] — cập nhật M5 lên 6/6 (hoàn tất), chờ xác nhận tay.
- [[Rui-ro-ky-thuat]] — mẫu rủi ro "chưa có máy Windows/GUI thật để xác nhận tay" lặp lại từ PBI 18.
