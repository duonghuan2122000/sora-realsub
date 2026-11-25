---
title: PBI 23 - Kéo-thả cửa sổ phụ đề
date: 2026-11-25
tags: [pbi, m5, ui, vue, tauri, capability]
sources: [.specify/specs/23/spec.md, .specify/specs/23/plan.md, .specify/specs/23/tasks.md, .specify/specs/23/evidence/]
---

# PBI 23 — Kéo-thả cửa sổ phụ đề (M5, mục 4/6)

Mục thứ tư của **M5 — Cửa sổ phụ đề (Subtitle Overlay UI)**, nối tiếp [[PBI-22-auto-scroll-fade-phu-de]]. Cửa sổ `decorations(false)` (PBI 20) không có title bar nên cần cách khác để người dùng di chuyển nó.

## Mục tiêu

Người dùng kéo-thả được cửa sổ phụ đề đến vị trí mong muốn; vị trí giữ nguyên khi phụ đề cập nhật/cuộn; chỉ click (không kéo) không làm cửa sổ di chuyển. Vị trí chỉ cần giữ trong phiên runtime hiện tại — lưu vị trí lâu dài qua các lần mở app thuộc M6, ngoài phạm vi PBI này.

## Cài đặt

- `app/src/subtitle/SubtitleApp.vue`: thêm thuộc tính HTML gốc của Tauri `data-tauri-drag-region` vào `.subtitle-overlay`. Tauri 2 tự phân biệt click vs drag ở tầng webview — không cần code JS/Rust tự viết để bắt mousedown/startDragging.
- Test `SubtitleApp.spec.ts`: thêm 1 test xác nhận `.subtitle-overlay` có attribute `data-tauri-drag-region`.
- **Gotcha phát hiện khi test tay** (xem [[Rui-ro-ky-thuat]]): `data-tauri-drag-region` gọi ngầm command `core:window:allow-start-dragging` qua ACL — `app/src-tauri/capabilities/default.json` chỉ gán `windows: ["main"]`, nên window `subtitle` (PBI 20) không có capability nào, lệnh kéo bị chặn âm thầm (không báo lỗi, chỉ đơn giản không kéo được). **Fix**: thêm file capability riêng `app/src-tauri/capabilities/subtitle.json` — `windows: ["subtitle"]`, `permissions: ["core:window:allow-start-dragging"]`.
- Không đụng `core/` Rust (ngoài capability JSON, không phải code) — tuân [[Kien-truc-core-ui]].

## Trạng thái xác nhận

- `bun run test` (vitest): **11/11 test pass** (6 test cũ SubtitleApp + 1 test mới + các file test khác) — log `.specify/specs/23/evidence/T003-result.log`.
- Test tay trên Windows thật (`bun run tauri dev`): lần 1 **FAIL** (cửa sổ trong suốt, không kéo được — do thiếu capability), sau khi thêm `subtitle.json` thì lần 2 **PASS** (người dùng xác nhận kéo được). Chi tiết `.specify/specs/23/evidence/T004-testcase.md`.
- **Hoàn tất mục 4/6 của M5.**

## Liên quan

- Thuộc [[Pipeline-xu-ly-audio]] ở công đoạn hiển thị, không đổi dữ liệu/pipeline — chỉ UX cửa sổ.
- Tuân [[Kien-truc-core-ui]]: toàn bộ thay đổi ở tầng UI + file khai báo capability Tauri (không phải logic nghiệp vụ).
- Bài học capability-per-window: xem [[Rui-ro-ky-thuat]] — mọi window Tauri 2 thêm sau `main` (subtitle, và các cửa sổ tương lai) cần capability riêng khai báo đúng `windows: [...]`, không tự động kế thừa `default.json`.
