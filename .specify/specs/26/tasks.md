# Danh sách Task: Màn hình chính chọn thiết bị

**Mã PBI**: 26
**Nguồn**: plan.md, spec.md, research.md, data-model.md, quickstart.md

## Pha 1: Setup

- [X] T001 Đọc lại `app/src-tauri/src/core/devices.rs` và `app/src/components/DeviceSelector.vue` hiện có để tái dùng đúng pattern (không tạo task mới, chỉ ghi chú — bỏ qua nếu đã nắm rõ từ plan.md)

## Pha 2: Foundational

- [X] T002 Thêm `output_devices()` vào trait `AudioHost` và cài đặt cho `CpalAudioHost` tại `app/src-tauri/src/core/devices.rs`
- [X] T003 [P] Thêm hàm `list_output_devices(host: &dyn AudioHost)` tại `app/src-tauri/src/core/devices.rs`
- [X] T004 Export `list_output_devices` trong `app/src-tauri/src/core/mod.rs`
- [X] T005 Thêm Tauri command `list_output_devices_cmd` và đăng ký vào `invoke_handler` tại `app/src-tauri/src/lib.rs`
- [X] T006 [P] Viết test Rust cho `output_devices`/`list_output_devices` (có thiết bị, danh sách rỗng, lỗi host) bằng mock `AudioHost`, bổ sung vào `mod tests` trong `app/src-tauri/src/core/devices.rs`

## Pha 3: User Story 1 - Chọn mic + loopback hợp lệ (Ưu tiên: P1)

**Mục tiêu**: Người dùng thấy danh sách mic + loopback, chọn được 1 mỗi loại, xác nhận thành công khi chọn đủ.
**Tiêu chí kiểm thử độc lập**: Mở màn hình chính, chọn 1 mic + 1 loopback, bấm Xác nhận → không có lỗi, lựa chọn được giữ trên UI.

- [X] T007 [US1] Sửa `app/src/components/DeviceSelector.vue`: thêm prop `title` (tên hiển thị, ví dụ "Chọn microphone"/"Chọn nguồn loopback") và prop `command` (tên Tauri command để invoke, dùng chung cho mic/loopback), thêm khả năng click chọn 1 item, emit event `update:selected` kèm `id` thiết bị đã chọn
- [X] T008 [US1] Sửa `app/src/App.vue`: thay thế nội dung skeleton mặc định bằng màn hình chính, ghép 2 `DeviceSelector` (1 cho mic dùng `list_input_devices_cmd`, 1 cho loopback dùng `list_output_devices_cmd`), giữ state `micDeviceId`/`loopbackDeviceId`
- [X] T009 [US1] Thêm nút "Xác nhận" tại `app/src/App.vue`, chỉ coi là xác nhận thành công khi cả `micDeviceId` và `loopbackDeviceId` đều đã chọn; nếu thiếu, hiển thị thông báo lỗi nêu rõ còn thiếu mic hay loopback (không gọi hành động nào khác ngoài validate + hiển thị)
- [X] T010 [P] [US1] Cập nhật/bổ sung test tại `app/src/components/DeviceSelector.test.ts`: chọn 1 item → emit đúng `id`; không có thiết bị → hiển thị thông báo rõ ràng "không có thiết bị khả dụng"
- [X] T011 [P] [US1] Viết test mới tại `app/src/App.test.ts`: chọn đủ mic + loopback rồi bấm Xác nhận → không có lỗi hiển thị; bấm Xác nhận khi thiếu 1 trong 2 → hiển thị lỗi nêu rõ phần thiếu

## Pha 4: User Story 2 - Phát hiện thiết bị không còn khả dụng (Ưu tiên: P2)

**Mục tiêu**: Khi thiết bị đã chọn trước đó không còn trong danh sách mới tải, hệ thống xoá lựa chọn và báo lỗi rõ ràng.
**Tiêu chí kiểm thử độc lập**: Giả lập danh sách thiết bị lần 2 không còn chứa `id` đã chọn ở lần 1 → sau khi làm mới, lựa chọn bị xoá và có thông báo lỗi tương ứng.

- [X] T012 [US2] Sửa `app/src/App.vue`: khi danh sách mic/loopback được tải lại (initial hoặc bấm "Làm mới"), so khớp `micDeviceId`/`loopbackDeviceId` hiện tại với danh sách mới; nếu không còn tồn tại → đặt lại `null` và set thông báo lỗi "Thiết bị đã chọn không còn khả dụng, vui lòng chọn lại"
- [X] T013 [P] [US2] Viết test tại `app/src/App.test.ts`: mock lần tải đầu có thiết bị A, lần tải lại (refresh) không còn A → lựa chọn bị xoá, thông báo lỗi xuất hiện

## Pha cuối: Polish & Cross-cutting

- [X] T014 Chạy `cargo test` trong `app/src-tauri/` và `bun run test` (hoặc tương đương Vitest) trong `app/` để xác nhận toàn bộ test pass, không có regression ở `DeviceSelector.test.ts`/`SubtitleApp.spec.ts` cũ
- [X] T015 Chụp ảnh/kết quả chạy `bun run tauri dev` theo `quickstart.md` làm bằng chứng test tay (theo yêu cầu của `/sora-implement`)

## Sơ đồ phụ thuộc

- Pha 1 → Pha 2 → Pha 3 (US1) → Pha 4 (US2) → Pha cuối.
- US2 phụ thuộc UI đã có ở US1 (T008/T009) vì cần state `micDeviceId`/`loopbackDeviceId` và cơ chế tải lại danh sách đã dựng ở US1.
- T002–T006 (Pha 2, phía Rust) độc lập hoàn toàn với T007–T011 (Pha 3, phía Vue) về file, nhưng T008 cần `list_output_devices_cmd` (T005) đã tồn tại để gọi `invoke` — chạy song song được, miễn T005 xong trước khi test tay US1 thật qua `tauri dev`.

## Chiến lược triển khai

- MVP đề xuất: User Story 1 (chọn + xác nhận đủ 2 thiết bị) — đáp ứng FR-001 đến FR-004.
- Thứ tự giao hàng tăng dần: Pha 1-2 (nền tảng Rust) → US1 (MVP dùng được) → US2 (FR-005/FR-006, xử lý lỗi thiết bị mất khả dụng/danh sách rỗng) → Polish.
