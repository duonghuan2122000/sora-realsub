# Danh sách Task: Liệt kê thiết bị âm thanh đầu vào

**Mã PBI**: 4
**Nguồn**: plan.md, spec.md, data-model.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm dependency `cpal` vào `app/src-tauri/Cargo.toml`

## Pha 2: Foundational

- [X] T002 Tạo struct `AudioDeviceInfo` (`id`, `name`, `is_default`) với `#[derive(Serialize)]` tại `app/src-tauri/src/core/devices.rs`
- [X] T003 Thêm `pub mod devices;` vào `app/src-tauri/src/core/mod.rs`

## Pha 3: User Story 1 - Xem danh sách mic để chọn (Ưu tiên: P1)

**Mục tiêu**: Người dùng mở màn hình chọn thiết bị và thấy danh sách mic thật trên máy, kèm đánh dấu mặc định; có cách làm mới danh sách; có thông báo rõ khi rỗng hoặc lỗi.

**Tiêu chí kiểm thử độc lập**: Chạy `cargo test` trong `app/src-tauri` pass toàn bộ case mock (có thiết bị, rỗng, tên trống, lỗi host); mở `npm run tauri dev`, gọi command qua UI/DevTools thấy đúng danh sách mic thật.

- [X] T004 [US1] Định nghĩa trait nội bộ `AudioHost` (ví dụ `fn input_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError>`) tại `app/src-tauri/src/core/devices.rs` để tách phần gọi `cpal` thật ra khỏi logic test
- [X] T005 [US1] Cài đặt `CpalAudioHost` (dùng `cpal::default_host()`, `host.input_devices()`, `host.default_input_device()`) implement trait `AudioHost`, map tên rỗng → `"Thiết bị không tên"`, map lỗi truy vấn → `CoreError::Io` tại `app/src-tauri/src/core/devices.rs`
- [X] T006 [US1] Viết hàm `list_input_devices(host: &dyn AudioHost) -> Result<Vec<AudioDeviceInfo>, CoreError>` dùng để cả command và test gọi chung, tại `app/src-tauri/src/core/devices.rs`
- [X] T007 [P] [US1] Viết unit test mock `AudioHost` cho 4 case: có thiết bị (kèm đánh dấu default đúng thiết bị), danh sách rỗng, thiết bị tên rỗng → hiển thị "Thiết bị không tên", host trả lỗi → trả `CoreError` rõ nghĩa, tại `app/src-tauri/src/core/devices.rs` (mod `tests`)
- [X] T008 [US1] Thêm Tauri command `list_input_devices_cmd() -> Result<Vec<AudioDeviceInfo>, String>` (gọi `devices::list_input_devices(&CpalAudioHost)`, map lỗi qua `to_string()`) và đăng ký vào `invoke_handler![...]` tại `app/src-tauri/src/lib.rs` (đổi tên so với plan để tránh đụng tên với hàm `list_input_devices` import từ `core`)
- [X] T009 [US1] Thêm `app/src/components/DeviceSelector.vue` gọi `invoke('list_input_devices_cmd')`, hiển thị danh sách + đánh dấu mặc định, nút làm mới, thông báo khi rỗng/lỗi; gắn vào `app/src/App.vue`; kèm test `DeviceSelector.test.ts` (4 case, mock `invoke`)

## Pha cuối: Polish & Cross-cutting

- [X] T010 Chạy `cargo fmt` + `cargo clippy` cho `app/src-tauri` — sạch, không warning (log: `evidence/T010-clippy.log`)
- [X] T011 Lưu bằng chứng test: log `cargo test` (`evidence/T007-result.log`), log `vitest` (`evidence/T009-vitest-result.log`), file test case (`evidence/devices-testcase.md`). Không chụp ảnh UI thật vì môi trường agent không có mic/hiển thị UI thật — đã ghi chú trong test case, cần người dùng xác nhận thủ công khi có điều kiện.

## Sơ đồ phụ thuộc

T001 → T002 → T003 → T004 → T005 → T006 → (T007 song song với T008) → T008 → T009 → T010 → T011.
Không có User Story 2/3 — toàn bộ phạm vi PBI 4 chỉ có 1 user story (P1).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, không chia MVP nhỏ hơn được nữa).
- Thứ tự giao hàng: Setup → Foundational → US1 (test trước, implement sau theo thứ tự T004→T009) → Polish.
