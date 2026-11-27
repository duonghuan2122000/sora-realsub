---
title: PBI 26 — Màn hình chính chọn thiết bị mic + loopback
date: 2026-11-27
tags: [pbi, M6, core-rust, ui-vue, devices]
sources: [.specify/specs/26/spec.md, .specify/specs/26/plan.md, .specify/specs/26/research.md, .specify/specs/26/tasks.md, .specify/specs/26/evidence/]
---

# PBI 26 — Màn hình chính chọn thiết bị mic + loopback

Mở đầu **M6 (Điều khiển & cấu hình)**: thay `App.vue` skeleton mặc định của `create-tauri-app` bằng màn hình chính thật đầu tiên của app — chọn mic + nguồn loopback trước khi bắt đầu phiên. Chọn PBI này sau khi bỏ qua 2 mục roadmap M1 đầu (PBI 18 Linux, hồi quy Windows) vì cả hai chỉ cần xác nhận tay trên máy khác, không phải việc code được.

## Quyết định chính

- **"Nguồn loopback" = danh sách thiết bị output**: tái dùng đúng pattern `AudioHost`/`list_input_devices` đã có (PBI 4) thay vì viết enumerator loopback riêng theo OS. Lý do: `LoopbackAudioSource`/`LoopbackAudioSourceLinux` ([[PBI-6-capture-loopback-wasapi]], [[PBI-18-loopback-linux]]) hiện **chỉ capture thiết bị output mặc định**, chưa nhận tham số thiết bị cụ thể — nối lựa chọn UI vào capture thật là phạm vi **ngoài PBI này**, ghi rõ trong `plan.md` mục Rủi ro để không tự ý mở rộng.
- **Không persist lựa chọn**: chỉ giữ trong state Vue của phiên UI hiện tại; lưu file cấu hình thuộc mục M6 khác ("Lưu cấu hình...").
- **Phát hiện thiết bị mất khả dụng bằng so khớp khi tải lại danh sách** (không lắng nghe sự kiện hot-plug hệ điều hành): mỗi lần `DeviceSelector` tải lại danh sách (initial/"Làm mới"), `App.vue` so `id` đã chọn với danh sách mới, xoá + cảnh báo nếu không còn.

## Cài đặt

- `app/src-tauri/src/core/devices.rs`: thêm `output_devices()` vào trait `AudioHost`, cài đặt cho `CpalAudioHost` (dùng `cpal::Host::output_devices()` + `default_output_device()`), hàm `list_output_devices()` song song `list_input_devices()`. Tách helper `collect_devices()` dùng chung cho cả input/output để không lặp code.
- `app/src-tauri/src/lib.rs`: thêm Tauri command `list_output_devices_cmd`, đăng ký vào `invoke_handler`.
- `app/src/components/DeviceSelector.vue`: từ component cứng (chỉ hiển thị mic, PBI 4) thành component tái dùng được — thêm props `title`/`command`/`emptyMessage`/`selected`, click item để chọn, emit `update:selected` + `devices-loaded` (phục vụ việc `App.vue` so khớp thiết bị còn khả dụng hay không).
- `app/src/App.vue`: thay nội dung "Welcome to Tauri + Vue" mặc định bằng 2 `DeviceSelector` (mic dùng `list_input_devices_cmd`, loopback dùng `list_output_devices_cmd`), state `micDeviceId`/`loopbackDeviceId`, nút "Xác nhận" validate đủ 2 lựa chọn, xử lý cảnh báo khi thiết bị mất khả dụng.

## Bằng chứng kiểm thử

- `cargo test --lib`: **70/70 pass, 0 fail, 2 ignored** (2 ignored có từ trước, không liên quan) — gồm 3 test mới cho `list_output_devices` (thành công/rỗng/lỗi). Xem `T006-cargo-test-result.log`.
- Vitest: **22/22 pass** — 2 test mới ở `DeviceSelector.test.ts` (click chọn emit đúng `id`; dùng prop tuỳ biến title/command/emptyMessage), 4 test mới ở `App.test.ts` (xác nhận thành công khi đủ 2 lựa chọn; báo lỗi rõ khi thiếu cả 2/thiếu 1; xoá lựa chọn + cảnh báo khi thiết bị mất khả dụng sau "Làm mới"). Xem `T011-T013-vitest-result.log`.
- **Chưa xác nhận bằng tay trên máy thật có mic/loopback thật qua `bun run tauri dev`** — chỉ xác nhận layout + đường lỗi hiển thị đúng qua `vite dev` thuần trong Claude Browser pane (không có Tauri IPC thật, `invoke` lỗi có chủ đích để test đường lỗi). Cùng mẫu rủi ro "chưa có máy Windows/Linux thật trong môi trường implement" đã gặp ở [[PBI-18-loopback-linux]], PBI 24. Xem `T015-testcase.md`, `T015-ui-result.png`.

## Liên quan

- [[PBI-4-liet-ke-thiet-bi-mic]] — nguồn gốc `AudioHost`/`list_input_devices`/`DeviceSelector.vue`, PBI này mở rộng thêm nhánh output/loopback.
- [[PBI-6-capture-loopback-wasapi]], [[PBI-18-loopback-linux]] — lý do loopback hiện chỉ dùng được thiết bị mặc định, chưa nhận lựa chọn cụ thể từ PBI 26.
- [[Kien-truc-core-ui]] — enumeration thiết bị là Core (Rust), validate "đủ lựa chọn" + chọn thiết bị là UI (Vue), giữ đúng ranh giới tách biệt.
- [[Trang-thai-ma-nguon]] — cập nhật: M6 bắt đầu (1/4).
- [[Rui-ro-ky-thuat]] — bổ sung giới hạn mới: lựa chọn loopback ở UI chưa nối vào capture thật.
