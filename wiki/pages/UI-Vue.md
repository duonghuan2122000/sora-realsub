---
title: UI (Vue)
date: 2026-11-24
tags: [component]
sources: [docs/tech-stack.md, app/src]
---

# UI (Vue)

| Thành phần | Lựa chọn |
|---|---|
| Framework | Vue 3 + TypeScript |
| Build/package | Vite + Bun |
| Styling | SCSS + design tokens, BEM `sora-*` — xem [[Style-guide-token]] |
| Component | shadcn-vue (Reka UI) |
| Icon | Lucide |
| State | Pinia |
| i18n | vue-i18n (Việt/Anh) |

## Cửa sổ
- **Cửa sổ chính**: chọn nguồn âm thanh, ngôn ngữ, engine; xem transcript; lịch sử.
- **Cửa sổ phụ đề nổi**: trong suốt, always-on-top, kéo thả được, click-xuyên (tuỳ chọn).

## Mã nguồn hiện tại
`app/src/App.vue`, `main.ts` — skeleton Vite+Vue mặc định (chưa có cửa sổ chính/phụ đề, chưa cài shadcn-vue/Pinia/vue-i18n/Lucide trong `package.json` hiện tại). Xem [[Trang-thai-ma-nguon]].

## Liên quan
[[Kien-truc-core-ui]], [[Style-guide-token]]
