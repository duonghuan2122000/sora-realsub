---
title: PBI 20 - Cửa sổ phụ đề nổi
date: 2026-11-25
tags: [pbi, m5, ui, tauri, multi-window]
sources: [.specify/specs/20/spec.md, .specify/specs/20/plan.md, .specify/specs/20/tasks.md, .specify/specs/20/evidence/]
---

# PBI 20 — Cửa sổ phụ đề nổi (M5, mục 1/6)

Mục đầu tiên của **M5 — Cửa sổ phụ đề (Subtitle Overlay UI)**, mở khoá sau khi M4 (Dịch) hoàn tất ([[PBI-19-noi-stt-dich-subtitle-ready]]).

## Mục tiêu

Tạo một cửa sổ Tauri thứ hai, riêng biệt với cửa sổ điều khiển chính:
- **always-on-top**: luôn nằm trên các ứng dụng khác kể cả khi mất focus.
- **không border/title bar** (`decorations(false)`).
- **trong suốt** (`transparent(true)`), **ẩn khỏi taskbar** (`skip_taskbar(true)`).

Chưa gắn nội dung phụ đề thật (việc đó để ở mục tiếp theo của M5, khi event `subtitle-ready` từ [[PBI-19-noi-stt-dich-subtitle-ready]] được nối vào UI).

## Cài đặt

- **Multi-window kiểu Tauri 2 khuyến nghị**: mỗi cửa sổ một entry HTML riêng, không dùng chung SPA route.
  - `app/subtitle.html` — entry point mới, trỏ tới `app/src/subtitle/main.ts`.
  - `app/src/subtitle/SubtitleApp.vue` — component Vue tối giản, hiện tại chỉ là khung trống (`.subtitle-overlay`, nền trong suốt, full viewport).
  - `app/vite.config.ts` — thêm `build.rollupOptions.input` liệt kê cả `index.html` (cửa sổ chính) và `subtitle.html` để Vite build đa trang.
- **Tạo cửa sổ bằng code, không khai báo tĩnh trong `tauri.conf.json`**: hàm `create_subtitle_window()` mới trong `app/src-tauri/src/lib.rs`, gọi `tauri::WebviewWindowBuilder::new(app, "subtitle", WebviewUrl::App("subtitle.html".into()))` với `.decorations(false)`, `.always_on_top(true)`, `.transparent(true)`, `.skip_taskbar(true)`, kích thước mặc định 800×150. Gọi trong `.setup()` của `tauri::Builder` ở `run()` — lý do chọn code thay vì JSON tĩnh: cần linh hoạt để mở rộng sau này (ẩn/hiện theo lệnh, resize lưu config ở M6), tránh cửa sổ phụ đề xuất hiện trước khi app sẵn sàng.
- **always-on-top cross-platform**: dùng API chung `always_on_top(true)` của Tauri 2 (tự wrap `HWND_TOPMOST` trên Windows / `gtk_window_set_keep_above` trên Linux qua wry/gtk) — không tự viết code theo `cfg(windows)`/`cfg(target_os = "linux")` như các PBI audio trước ([[PBI-18-loopback-linux]]), vì Tauri đã trừu tượng hoá đủ cho 2 OS mục tiêu hiện tại.
- Chưa thêm Tauri command/event mới — không cần hợp đồng IPC mới ở PBI này.

## Trạng thái xác nhận

- `cargo build` tại `app/src-tauri/` biên dịch sạch — log tại `.specify/specs/20/evidence/T004-cargo-build.log`.
- **SC-001 (always-on-top)** và **SC-002 (không viền/title bar)**: **ĐÃ XÁC NHẬN PASS** — chạy `bun run tauri dev` thật trên máy Windows của người dùng (không chạy được trong môi trường implement ban đầu do thiếu GUI, giống PBI 17/18), người dùng trực tiếp quan sát trên màn hình thật và xác nhận đạt cả 2 tiêu chí. Chi tiết: `.specify/specs/20/evidence/T005-testcase.md`. Roadmap đã tick dòng tương ứng.

## Liên quan

- Thuộc [[Pipeline-xu-ly-audio]] ở công đoạn hiển thị cuối (chưa nối, chỉ tạo khung cửa sổ).
- Tuân [[Kien-truc-core-ui]]: logic tạo cửa sổ nằm ở tầng Core (Rust `lib.rs`), UI (Vue) chỉ là khung trống.
- Mẫu "môi trường implement thiếu xác nhận thật" giống [[PBI-17-dich-co-ngu-canh]] và [[PBI-18-loopback-linux]] — xem thêm [[Rui-ro-ky-thuat]].
