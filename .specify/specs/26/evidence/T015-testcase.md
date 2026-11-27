# Test case bằng chứng: PBI 26 — Màn hình chính chọn thiết bị

## TC1 — Hiển thị và chọn mic + loopback hợp lệ (Vitest, `App.test.ts`)

- **Input**: mock `invoke` trả `list_input_devices_cmd` → 1 mic, `list_output_devices_cmd` → 1 loopback. Click item mic, click item loopback, bấm "Xác nhận".
- **Expected**: không có thông báo "Vui lòng chọn" sau khi xác nhận.
- **Actual**: PASS — xem `T011-T013-vitest-result.log`, test `xác nhận thành công khi đã chọn đủ mic + loopback`.

## TC2 — Thiếu cả 2 lựa chọn khi xác nhận

- **Input**: không chọn gì, bấm "Xác nhận".
- **Expected**: hiển thị "Vui lòng chọn microphone và nguồn loopback trước khi xác nhận."
- **Actual**: PASS — test `báo lỗi rõ ràng khi xác nhận lúc thiếu cả mic và loopback`.

## TC3 — Thiếu 1 lựa chọn (loopback)

- **Input**: chọn mic, không chọn loopback, bấm "Xác nhận".
- **Expected**: hiển thị "Vui lòng chọn nguồn loopback trước khi xác nhận."
- **Actual**: PASS — test `báo lỗi rõ ràng khi chỉ thiếu loopback`.

## TC4 — Thiết bị đã chọn không còn khả dụng sau khi làm mới (US2)

- **Input**: chọn mic "mic-a", sau đó mock `list_input_devices_cmd` trả danh sách rỗng, bấm "Làm mới" ở panel mic.
- **Expected**: lựa chọn mic bị xoá, hiển thị cảnh báo chứa "không còn khả dụng".
- **Actual**: PASS — test `xoá lựa chọn và báo cảnh báo khi thiết bị đã chọn không còn trong danh sách tải lại`.

## TC5 — `list_output_devices` phía Rust (`cargo test`)

- **Input**: `MockHost` trả 1 thiết bị / danh sách rỗng / lỗi.
- **Expected**: lần lượt trả đúng danh sách, danh sách rỗng, và `CoreError::Io`.
- **Actual**: PASS — 3 test mới (`list_output_devices_tra_dung_danh_sach`, `list_output_devices_rong_tra_danh_sach_rong`, `list_output_devices_loi_tra_ve_coreerror`) trong `T006-cargo-test-result.log`.

## TC6 — Xác minh UI thật qua `vite dev` (web-view only, chưa phải Tauri IPC thật)

- **Input**: chạy `bun run dev`, mở `http://localhost:1420` trong trình duyệt (không có runtime Tauri → `invoke` thất bại).
- **Expected**: layout 2 panel "Chọn microphone" / "Chọn nguồn loopback" hiển thị đúng; khi `invoke` lỗi (ở đây là do thiếu Tauri IPC), mỗi panel hiển thị thông báo lỗi rõ ràng thay vì màn hình trắng/crash.
- **Actual**: PASS — xem `T015-ui-result.png`. Đây là xác nhận bố cục UI + đường lỗi hiển thị đúng; **chưa xác nhận invoke thật qua Tauri runtime trên máy có thiết bị audio thật** (cần chạy `bun run tauri dev` trên máy Windows/Linux thật theo `quickstart.md`, bước 5 của quickstart vẫn cần xác nhận thủ công trên máy thật như các PBI trước).

## Tổng kết

- 72 test Rust: 70 pass, 2 ignored (có từ trước, không liên quan PBI 26).
- 22 test Vitest: 22 pass (bao gồm 5 test mới cho PBI 26: 2 ở `DeviceSelector.test.ts`, 4 ở `App.test.ts` — tổng 1 test cũ sửa tên không đổi + các test mới).
- Chưa xác nhận bằng tay trên máy thật với thiết bị audio thật + Tauri runtime đầy đủ (giống ghi chú "chưa kiểm thử tay" ở PBI 21/25) — cần người dùng xác nhận qua `bun run tauri dev` trên máy có mic/loopback thật.
