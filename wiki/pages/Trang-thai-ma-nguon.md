---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24 (cập nhật sau PBI 3)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **Mới (PBI 3)**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test (8 test pass). Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **Vẫn chưa có**: cài đặt logic thật nào (audio capture, VAD, STT, dịch, lưu trữ thật) trong [[Pipeline-xu-ly-audio]].
- Chưa thấy crate `cpal`, `rubato`, `ort`, `whisper-rs`, `rusqlite` trong Cargo deps; đã thêm `async-trait`, `thiserror` (PBI 3).

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- Chưa có cửa sổ chính/phụ đề nổi, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].

## Kết luận
Repo đã bắt đầu triển khai: interface Core (trait, PBI 3) xong, nhưng **chưa có logic nghiệp vụ thật nào** (audio/VAD/STT/dịch) — vẫn ở giai đoạn nền tảng trước M1.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]]
