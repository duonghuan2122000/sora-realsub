---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-24
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-24 (cập nhật sau PBI 4)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **PBI 3**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test. Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **Mới (PBI 4)**: `core/devices.rs` — liệt kê thiết bị audio input bằng `cpal` thật (lần đầu có logic nghiệp vụ thật, không còn chỉ interface); Tauri command `list_input_devices_cmd`; 4 test pass. Dependency mới: `cpal = "0.15"`. Chi tiết: [[PBI-4-liet-ke-thiet-bi-mic]].
- **Vẫn chưa có**: capture audio thật (mic/loopback), VAD, STT, dịch, lưu trữ thật trong [[Pipeline-xu-ly-audio]]. Chưa thấy `rubato`, `ort`, `whisper-rs`, `rusqlite` trong Cargo deps.

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định.
- **Mới (PBI 4)**: `components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic từ danh sách thật, nút làm mới, thông báo rỗng/lỗi), gắn vào `App.vue`; kèm `DeviceSelector.test.ts` (vitest, 4 test pass).
- Chưa có cửa sổ chính/phụ đề nổi riêng, chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].

## Kết luận
Repo đã có logic nghiệp vụ thật đầu tiên: liệt kê thiết bị mic (PBI 4), chạy trên nền interface Core đã chốt ở PBI 3. Vẫn còn hầu hết M1 (capture mic/loopback/mix/start-stop) và các milestone sau chưa làm.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]]
