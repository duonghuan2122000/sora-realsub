---
title: "PBI 4: Liệt kê thiết bị âm thanh đầu vào"
date: 2026-11-24
tags: [pbi, m1, feature]
sources: [.specify/specs/4/spec.md, .specify/specs/4/plan.md, .specify/specs/4/tasks.md, app/src-tauri/src/core/devices.rs, app/src/components/DeviceSelector.vue]
---

# PBI 4: Liệt kê thiết bị âm thanh đầu vào

Tính năng đầu tiên của M1 (Audio Capture) đã thi công: UI lấy danh sách thiết bị mic trên máy để người dùng chọn trước khi bắt đầu capture.

## Yêu cầu chính (spec.md)
- Lấy danh sách thiết bị audio **input** (mic), mỗi thiết bị có tên + đánh dấu có phải default hay không.
- Có nút làm mới danh sách (không theo dõi real-time hot-plug).
- Khi rỗng hoặc lỗi truy vấn → thông báo rõ nghĩa, không để UI trống không giải thích.
- Phạm vi: chỉ mic input, chưa gồm loopback hệ thống (PBI khác trong M1); chưa capture audio thật.

## Quyết định kỹ thuật (plan.md, research.md)
- Dùng crate **`cpal`** để enumerate device (đã được dự tính trước trong [[Core-Rust]], giờ dùng thật lần đầu) — cùng thư viện sẽ dùng cho capture mic/loopback ở PBI kế tiếp.
- **Không** mở rộng trait `AudioSource` (đã chốt ở PBI 3, xem [[Quyet-dinh-hop-dong-core]]) — enumerate là truy vấn một lần, khác ngữ nghĩa stream liên tục của trait đó.
- Tách trait nội bộ `AudioHost` để mock test không cần phần cứng thật.
- IPC: 1 Tauri command request/response (`list_input_devices_cmd`), không dùng event — vì chưa cần theo dõi real-time.

## Cài đặt (code)
- `app/src-tauri/src/core/devices.rs` (mới): struct `AudioDeviceInfo { id, name, is_default }`, trait `AudioHost`, cài đặt `CpalAudioHost` (dùng `cpal::default_host()`), hàm `list_input_devices(host: &dyn AudioHost)`. Tên thiết bị rỗng → `"Thiết bị không tên"`.
- `app/src-tauri/src/lib.rs`: thêm Tauri command `list_input_devices_cmd`, đăng ký vào `invoke_handler!`.
- `app/src-tauri/Cargo.toml`: thêm dependency `cpal = "0.15"` — dependency thật đầu tiên ngoài bộ skeleton + `async-trait`/`thiserror` của PBI 3.
- `app/src/components/DeviceSelector.vue` (mới): gọi `invoke('list_input_devices_cmd')`, hiển thị danh sách/đánh dấu mặc định/nút làm mới/thông báo rỗng-lỗi. Gắn vào `App.vue` — component UI thực sự đầu tiên ngoài skeleton Tauri+Vue mặc định.

## Bằng chứng test (`.specify/specs/4/evidence/`)
- Core: `cargo test devices` — 4/4 pass (có thiết bị + default, rỗng, tên rỗng, lỗi host). Log: `T007-result.log`.
- UI: `npm test` (vitest) — 4/4 case mới pass (hiển thị danh sách+default, rỗng, lỗi, bấm làm mới gọi lại invoke). Log: `T009-vitest-result.log`.
- `cargo clippy` sạch, không warning.
- Chưa có ảnh chụp UI thật (môi trường agent không có mic/hiển thị) — cần người dùng xác nhận thủ công theo `quickstart.md` khi có máy thật.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[Pipeline-xu-ly-audio]]
