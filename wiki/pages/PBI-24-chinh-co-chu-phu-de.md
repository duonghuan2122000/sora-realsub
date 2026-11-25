---
title: PBI 24 — Chỉnh cỡ chữ phụ đề
date: 2026-11-25
tags: [pbi, M5, ui, subtitle-window]
sources: [.specify/specs/24/spec.md, .specify/specs/24/plan.md, .specify/specs/24/research.md, .specify/specs/24/tasks.md, .specify/specs/24/evidence/]
---

# PBI 24 — Chỉnh cỡ chữ phụ đề

Hoàn tất mục roadmap M5 "Cho phép resize / chỉnh cỡ chữ" — thu hẹp phạm vi còn **chỉnh cỡ chữ** (không làm resize khung cửa sổ bằng kéo viền), vì cửa sổ phụ đề không có border/title bar ([[PBI-20-cua-so-phu-de-noi]]) nên kéo-resize kiểu thông thường không tự nhiên. Resize khung cửa sổ được ghi lại là "Ngoài phạm vi" trong spec.md.

## Quyết định chính

- **Cơ chế điều khiển**: 2 nút `+`/`-` hiển thị đè góc trên-phải cửa sổ phụ đề, thay vì phím tắt toàn cục (global shortcut phức tạp hơn, không trực quan bằng).
- **Lưu trữ**: `localStorage` của webview `subtitle.html` — **không** thêm Tauri command/IPC hay file config Rust. Webview (WebView2/WebKitGTK) tự persist `localStorage` qua app data dir giữa các lần mở app, giống browser thật — đủ cho yêu cầu "lưu 1 giá trị cỡ chữ qua các lần mở app" mà không cần Core sở hữu state này. Cấu hình rộng hơn (thiết bị, ngôn ngữ, vị trí cửa sổ) vẫn để dành cho M6 như roadmap đã định.
- **Khoảng giá trị**: mặc định 1.5rem (khớp CSS cũ), min 1.0rem, max 3.0rem, bước 0.25rem, clamp 2 đầu không lỗi.
- Nút `+`/`-` đặt trong `data-tauri-drag-region="false"` để không bị vùng kéo-thả ([[PBI-23-keo-tha-cua-so-phu-de]]) nuốt sự kiện click — rút kinh nghiệm từ rủi ro đã ghi ở plan.md của PBI 23.

## Cài đặt

- `app/src/subtitle/SubtitleApp.vue`: thêm `fontSize` (ref, khởi tạo từ `readStoredFontSize()` đọc + validate `localStorage['subtitle-font-size']`, fallback mặc định nếu rỗng/NaN), `increaseFontSize`/`decreaseFontSize` (clamp + ghi lại `localStorage`), bind qua CSS variable `--subtitle-font-size` trên `.subtitle-overlay`, `.subtitle-text` dùng `font-size: var(--subtitle-font-size, 1.5rem)` thay giá trị cứng cũ.
- `app/src/subtitle/SubtitleApp.spec.ts`: thêm 5 test (tăng/giảm, clamp min/max, khôi phục từ localStorage hợp lệ, fallback khi localStorage hỏng, ghi lại localStorage khi tăng).

## Bằng chứng kiểm thử

- Unit test: 16/16 pass (`bun run test`) — xem `.specify/specs/24/evidence/T002-T008-testcase.md` + log kèm theo.
- Kiểm thử tay: container Linux (không có máy Windows thật, lặp lại hạn chế đã ghi ở [[Rui-ro-ky-thuat]]) — chạy `bun run dev` (vite thuần, không qua Tauri) và mở `subtitle.html` trực tiếp qua Claude Browser pane để xác nhận UI + localStorage persistence (bỏ qua phần `listen('subtitle-ready')` vì thiếu IPC Tauri, không thuộc phạm vi PBI này). Xác nhận: bấm "+" đổi 1.5rem→1.75rem + ghi localStorage; reload trang giữ đúng 1.75rem. Xem `.specify/specs/24/evidence/T009-manual-test-result.log`.
- **Chưa xác nhận trên webview WebView2 thật (Windows)** — cơ chế `localStorage` là API chuẩn nên rủi ro thấp, nhưng theo đúng thông lệ dự án (PBI 18, 20, 23), cần người có máy Windows xác nhận lại trước khi coi là chốt hoàn toàn.

## Liên quan

- [[PBI-20-cua-so-phu-de-noi]], [[PBI-21-hien-thi-phu-de]], [[PBI-22-auto-scroll-fade-phu-de]], [[PBI-23-keo-tha-cua-so-phu-de]] — cùng chuỗi M5 cửa sổ phụ đề.
- [[Trang-thai-ma-nguon]] — cập nhật M5 lên 5/6.
