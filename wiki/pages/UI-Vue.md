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
- **Cửa sổ phụ đề nổi**: trong suốt, always-on-top, kéo thả được (PBI 23), click-xuyên (tuỳ chọn, chưa làm).

## Mã nguồn hiện tại
`app/src/App.vue`, `main.ts` — skeleton Vite+Vue mặc định (chưa cài shadcn-vue/Pinia/vue-i18n/Lucide trong `package.json` hiện tại). **Mới (PBI 4, 2026-11-24)**: `app/src/components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic, gọi Tauri command qua `invoke`), gắn vào `App.vue`; kèm test `DeviceSelector.test.ts` (vitest, mock `invoke`). Xem [[PBI-4-liet-ke-thiet-bi-mic]], [[Trang-thai-ma-nguon]].

`app/src/subtitle/SubtitleApp.vue` — cửa sổ phụ đề nổi (PBI 20-23): hiển thị tối đa 3 dòng gần nhất từ event `subtitle-ready`, auto-scroll/fade (PBI 21-22), kéo-thả được qua `data-tauri-drag-region` + capability Tauri riêng (PBI 23, xem [[PBI-23-keo-tha-cua-so-phu-de]], [[Rui-ro-ky-thuat]]).

## Liên quan
[[Kien-truc-core-ui]], [[Style-guide-token]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-23-keo-tha-cua-so-phu-de]]
