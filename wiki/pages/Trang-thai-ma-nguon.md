---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- Chỉ có `main.rs`, `lib.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate), **chưa có** module audio/VAD/STT/dịch/lưu trữ nào trong [[Pipeline-xu-ly-audio]].
- Chưa thấy crate `cpal`, `rubato`, `ort`, `whisper-rs`, `rusqlite` trong Cargo deps tại thời điểm ingest.

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- Chưa có cửa sổ chính/phụ đề nổi, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].

## Kết luận
Repo hiện ở giai đoạn **tài liệu thiết kế xong, code chưa bắt đầu triển khai tính năng** — chỉ có skeleton app khởi tạo bằng `create-tauri-app`.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]]
