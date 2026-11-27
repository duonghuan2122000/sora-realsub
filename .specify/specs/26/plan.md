# Kế hoạch triển khai: Màn hình chính chọn thiết bị

**Mã PBI**: 26
**Liên kết spec**: .specify/specs/26/spec.md
**Ngày tạo**: 2026-11-27

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core) + TypeScript (Vue 3, UI)
- Framework / Thư viện chính: `cpal` (enumeration thiết bị audio, đã dùng cho mic), Vue 3 `<script setup>`
- Lưu trữ dữ liệu: không — state chỉ trong phiên UI (xem `research.md` quyết định 2)
- Kiểm thử: `cargo test` (Rust, mock `AudioHost`) + Vitest (Vue component, theo pattern `SubtitleApp.vue` đã có test)
- Nền tảng triển khai: Windows + Linux (theo M0)
- Ràng buộc hiệu năng: không có yêu cầu đặc biệt, chỉ cần phản hồi tức thời khi chọn/làm mới
- Ràng buộc khác: Core (Rust) và UI (Vue) tách biệt, giao tiếp qua Tauri IPC — không đưa logic chọn/validate thiết bị vào Core, chỉ enumeration là Core; việc chọn + validate "đủ 2 lựa chọn" thuộc UI

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — áp dụng nguyên tắc nêu ở `CLAUDE.md` (Core/UI tách biệt).

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) và UI (Vue) tách biệt, logic nghiệp vụ không nằm ở UI | ✅ | Enumeration thiết bị (Core) qua `AudioHost` mở rộng; validate "đủ lựa chọn" là ràng buộc hiển thị UI, không phải nghiệp vụ Core |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: tái dùng `AudioHost` (thêm `output_devices()`) làm nguồn danh sách loopback, không persist lựa chọn, phát hiện thiết bị mất khả dụng bằng so khớp khi tải lại danh sách.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` (`AudioDeviceInfo` tái dùng, `DeviceSelection` mới ở UI)
- Hợp đồng giao diện: không có API bên ngoài; giao tiếp Core↔UI qua Tauri command `list_output_devices_cmd` (mới, cùng dạng `list_input_devices_cmd` đã có) — không cần tài liệu `contracts/` riêng vì chỉ là 1 command nội bộ, mô tả đủ ở đây
- Kịch bản khởi động nhanh: xem `quickstart.md`

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/devices.rs   # thêm output_devices() vào trait AudioHost + impl CpalAudioHost, hàm list_output_devices()
app/src-tauri/src/core/mod.rs       # export thêm list_output_devices
app/src-tauri/src/lib.rs            # thêm Tauri command list_output_devices_cmd, đăng ký vào invoke_handler
app/src/components/DeviceSelector.vue  # đổi thành có thể chọn (click vào item), nhận prop "loại thiết bị" dùng lại cho cả mic và loopback, emit lựa chọn ra ngoài
app/src/App.vue                     # ghép 2 DeviceSelector (mic + loopback) + nút Xác nhận + validate đủ 2 lựa chọn + xử lý thiết bị mất khả dụng
```

## Rủi ro & ngoại lệ có lý do

- `LoopbackAudioSource`/`LoopbackAudioSourceLinux` hiện chỉ capture thiết bị output **mặc định**, chưa nhận tham số thiết bị cụ thể từ lựa chọn người dùng. PBI này chỉ làm phần UI chọn + validate (đúng theo spec, không có FR nào yêu cầu capture theo thiết bị đã chọn). Việc truyền lựa chọn vào pipeline capture thật là công việc tiếp theo, chưa có trong roadmap hiện tại dưới dạng mục riêng — cần bổ sung mục mới nếu muốn làm, không tự ý mở rộng phạm vi PBI 26.
