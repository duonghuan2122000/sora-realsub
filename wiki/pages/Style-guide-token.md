---
title: Style guide & design tokens
date: 2026-11-24
tags: [component, style]
sources: [docs/style-guide.md, docs/tech-stack.md]
---

# Style guide & design tokens

Quy ước UI dùng chung toàn app — mọi component chỉ dùng token/class, không hardcode.

## Quy tắc
- SCSS thay CSS thuần (không viết `.css` tay).
- Class theo **BEM**, tiền tố bắt buộc `sora-`: block `sora-block`, element `sora-block__element`, modifier `sora-block--modifier` / `sora-block__element--modifier`.
  - Vd: `sora-subtitle__text--active`, `sora-button--danger`.
- Trạng thái UI: tiền tố `sora-is-` (`sora-is-loading`, `sora-is-error`, `sora-is-empty`, `sora-is-disconnected`).

## Cấu trúc file (dự kiến)
```
ui/src/styles/
  tokens.scss     # màu, spacing, typography, radius, shadow, z-index
  base.scss       # reset, base element
  components/     # 1 file scss / component theo tên block BEM
```
Lưu ý: cấu trúc thực tế hiện ở `app/src/` (không phải `ui/src/` như doc ghi) — thư mục styles/tokens.scss **chưa được tạo** trong skeleton hiện tại. Xem [[Trang-thai-ma-nguon]].

## Token
- **Màu** (semantic, light/dark): `--color-bg`, `--color-surface`, `--color-text`, `--color-text-muted`, `--color-primary`, `--color-danger`, `--color-border`...
- **Typography**: 1 font UI + 1 font mono; thang cỡ `xs, sm, base, lg, xl`.
- **Spacing**: thang 4px (`4, 8, 12, 16, 24, 32`).
- Bo góc / đổ bóng / z-index: theo token, không hardcode.

## Phụ đề (subtitle overlay)
Cỡ chữ, độ đậm, màu nền/viền, độ trong suốt chỉnh được qua settings, map vào token. Mặc định độ tương phản cao để đọc khi chia sẻ màn hình.

## Trợ năng & hiệu năng
- Tương phản tối thiểu WCAG AA; mọi control hỗ trợ bàn phím, `:focus-visible` riêng.
- Không block UI thread khi khởi động — UI hiện ngay, model/engine lazy-load sau.

## Liên quan
[[UI-Vue]], [[Kien-truc-core-ui]]
