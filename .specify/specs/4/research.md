# Nghiên cứu kỹ thuật: PBI 4 — Liệt kê thiết bị âm thanh đầu vào

## Quyết định 1: Thư viện liệt kê thiết bị audio

- **Quyết định**: dùng crate `cpal` (host API trừu tượng hoá qua WASAPI trên Windows) để enumerate input device.
- **Lý do**: `cpal` là lựa chọn chuẩn trong ecosystem Rust cho capture audio cross-platform, sẽ tiếp tục dùng cho PBI capture mic/loopback kế tiếp trong M1 — tránh phải đổi thư viện giữa 2 PBI liên quan. Có API `host.input_devices()`, `device.name()`, `host.default_input_device()`.
- **Phương án khác đã xem xét**: gọi trực tiếp WASAPI qua `windows-rs` — nhiều boilerplate hơn, không cần thiết khi `cpal` đã bọc sẵn và đa nền tảng (hỗ trợ mở rộng macOS về sau theo roadmap Backlog).

## Quyết định 2: Vị trí đặt code enumerate thiết bị

- **Quyết định**: thêm hàm liệt kê thiết bị trong `core` (module mới `core/devices.rs` hoặc hàm độc lập), KHÔNG đưa vào trait `AudioSource` hiện có — `AudioSource` chỉ lo việc start/stream dữ liệu của 1 thiết bị cụ thể đã chọn.
- **Lý do**: giữ tách biệt trách nhiệm: liệt kê thiết bị là tác vụ truy vấn một lần, khác với trait `AudioSource` (đang dùng để capture liên tục). Tránh phá vỡ interface đã chốt ở PBI 3.
- **Phương án khác đã xem xét**: thêm method `list_devices()` vào trait `AudioSource` — bị loại vì trait này định nghĩa theo instance đã gắn với 1 thiết bị, không phù hợp ngữ nghĩa liệt kê toàn hệ thống.

## Quyết định 3: Hợp đồng IPC với UI

- **Quyết định**: 1 Tauri command `list_input_devices() -> Result<Vec<AudioDeviceInfo>, String>`, trả về JSON qua `serde`.
- **Lý do**: đồng bộ với cách `greet` command hiện có trong skeleton, không cần event — vì đây là truy vấn theo yêu cầu (request/response), không phải luồng liên tục.
- **Phương án khác đã xem xét**: phát event `devices-updated` — không cần vì chưa có yêu cầu theo dõi real-time (đã ghi trong Giả định của spec.md).
