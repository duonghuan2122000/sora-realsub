---
title: PBI 21 - Hiển thị phụ đề
date: 2026-11-25
tags: [pbi, m5, ui, vue, tauri-event]
sources: [.specify/specs/21/spec.md, .specify/specs/21/plan.md, .specify/specs/21/tasks.md, .specify/specs/21/evidence/]
---

# PBI 21 — Hiển thị phụ đề (M5, mục 2/6)

Mục thứ hai của **M5 — Cửa sổ phụ đề (Subtitle Overlay UI)**, nối tiếp [[PBI-20-cua-so-phu-de-noi]] (cửa sổ always-on-top trống) và tiêu thụ event `subtitle-ready` đã có sẵn từ [[PBI-19-noi-stt-dich-subtitle-ready]].

## Mục tiêu

Làm cho `SubtitleApp.vue` (trước đây chỉ là khung trống) hiển thị nội dung text dịch (`translatedText`) mỗi khi nhận event Tauri `subtitle-ready`, đúng thứ tự, không rớt sự kiện, không vỡ layout với text dài.

## Cài đặt

- `app/src/subtitle/SubtitleApp.vue`: thêm `<script setup lang="ts">` — `ref` `currentSubtitle`, `onMounted` gọi `listen('subtitle-ready', ...)` từ `@tauri-apps/api/event` để subscribe, `onUnmounted` gọi `unlisten()`. Callback ghi đè `currentSubtitle.value = event.payload.translatedText` — chỉ giữ **1 dòng hiện tại**, không giữ lịch sử (auto-scroll/nhiều dòng để dành mục roadmap M5 kế tiếp).
- Payload dùng lại nguyên `SubtitlePayload` đã định nghĩa ở [[PBI-19-noi-stt-dich-subtitle-ready]] (`app/src-tauri/src/lib.rs`), không sửa gì phía Core/Rust — đúng tách biệt Core/UI theo [[Kien-truc-core-ui]].
- Template: ẩn hẳn phần tử `.subtitle-text` khi `translatedText` rỗng (`v-if`), tránh hiển thị dòng trống gây khó chịu (trường hợp biên khi dịch lỗi).
- CSS: `word-wrap`/`overflow-wrap`/`overflow: hidden` + `max-width: 90vw` để text dài (~200 ký tự) không vỡ layout cửa sổ.
- Test mới: `app/src/subtitle/SubtitleApp.spec.ts` (test Vue component đầu tiên của dự án, dùng `vitest` + `@vue/test-utils` đã có sẵn trong `devDependencies` — không cần thêm gì) — mock `@tauri-apps/api/event`, 3 case: hiển thị 1 event, 20 event liên tiếp đúng thứ tự (SC-002), ẩn khi rỗng.

## Trạng thái xác nhận

- `npm run test` (vitest): **8/8 test pass** (gồm 3 test case mới của `SubtitleApp.spec.ts`) — log tại `.specify/specs/21/evidence/T007-result.log`, chi tiết case tại `.specify/specs/21/evidence/T006-testcase.md`.
- `npm run build` (`vue-tsc --noEmit` + `vite build`): biên dịch/type-check sạch, không lỗi.
- Khác với [[PBI-20-cua-so-phu-de-noi]]/[[PBI-17-dich-co-ngu-canh]]/[[PBI-18-loopback-linux]]: PBI này **chạy và xác nhận hoàn toàn trong môi trường implement** (unit test jsdom, không cần GUI thật hay model thật) — không có mục nào còn chờ xác nhận tay trên máy thật.
- Roadmap M5 mục "Hiển thị text phụ đề..." đã tick.

## Liên quan

- Thuộc [[Pipeline-xu-ly-audio]] ở công đoạn hiển thị cuối cùng — lần đầu công đoạn này có nội dung động thật, nối tiếp khung trống của [[PBI-20-cua-so-phu-de-noi]].
- Tuân [[Kien-truc-core-ui]]: toàn bộ thay đổi chỉ ở tầng UI (Vue), tiêu thụ event Core đã phát sẵn, không có logic nghiệp vụ mới trong UI.
- Dự án lần đầu có test Vue/vitest cho component UI (trước đó test chủ yếu ở Rust/Core).
