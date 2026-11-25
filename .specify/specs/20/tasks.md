# Danh sách Task: Cửa sổ phụ đề nổi

**Mã PBI**: 20
**Nguồn**: plan.md, spec.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm entry point mới `app/subtitle.html` (placeholder tối thiểu) và `app/src/subtitle/main.ts` (bootstrap Vue rỗng), theo cấu trúc dự kiến trong plan.md
- [X] T002 Thêm `subtitle.html` vào `rollupOptions.input` trong `app/vite.config.ts` để Vite build multi-page

## Pha 2: Foundational

- [X] T003 Trong `app/src-tauri/src/lib.rs`, thêm hàm tạo cửa sổ phụ đề bằng `tauri::WebviewWindowBuilder::new(app, "subtitle", WebviewUrl::App("subtitle.html".into()))` với `.decorations(false)`, `.always_on_top(true)`, `.transparent(true)`, `.skip_taskbar(true)`, kích thước/vị trí mặc định cố định, gọi trong `.setup()` của `tauri::Builder`

## Pha 3: User Story 1 - Cửa sổ phụ đề always-on-top, không viền (Ưu tiên: P1)

**Mục tiêu**: Cửa sổ phụ đề mở cùng app, luôn nằm trên các cửa sổ khác, không có border/title bar (FR-001, FR-002, FR-003, FR-004).
**Tiêu chí kiểm thử độc lập**: Chạy app dev, quan sát cửa sổ phụ đề xuất hiện không viền và luôn ở trên khi chuyển qua ≥3 ứng dụng khác (theo quickstart.md).

- [X] T004 [US1] Chạy `cargo build` tại `app/src-tauri/` xác nhận code biên dịch sạch sau khi thêm hàm tạo cửa sổ (T003) — PASS, xem `.specify/specs/20/evidence/T004-cargo-build.log`
- [X] T005 [US1] Kiểm thử thủ công theo `quickstart.md`: chạy `bun run tauri dev`, xác nhận cửa sổ phụ đề không viền/title bar (SC-002) và always-on-top khi chuyển qua ≥3 ứng dụng khác (SC-001) — PASS, xác nhận trực tiếp bởi người dùng trên máy Windows thật, xem `.specify/specs/20/evidence/T005-testcase.md`
- [X] T006 [US1] Xác nhận GUI thật đã hoàn tất (T005, PASS trên Windows thật) — không còn vướng mắc

## Pha cuối: Polish & Cross-cutting

- [X] T007 Cập nhật wiki dự án (qua skill `sora-wiki`) ghi lại cách tạo cửa sổ multi-window trong Tauri 2 cho PBI này

## Sơ đồ phụ thuộc

T001, T002 (Setup) → T003 (Foundational) → T004 → T005 → T006 (US1) → T007 (Polish). Chỉ có 1 user story (US1), không có story song song.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ US1 (feature nhỏ, không chia nhỏ thêm được).
- Thứ tự giao hàng: Setup → Foundational → US1 → Polish, tuần tự, không có task chạy song song (cùng sửa `lib.rs`/`vite.config.ts`, phụ thuộc lẫn nhau).
