# Mô hình dữ liệu: PBI 4 — Liệt kê thiết bị âm thanh đầu vào

## `AudioDeviceInfo`

| Thuộc tính | Kiểu | Mô tả |
|---|---|---|
| `id` | `String` | Định danh duy nhất của thiết bị trong phiên hiện tại (tên thiết bị dùng làm id, vì `cpal` không cấp id ổn định riêng) |
| `name` | `String` | Tên hiển thị cho người dùng; thiết bị không tên → `"Thiết bị không tên"` |
| `is_default` | `bool` | Có phải thiết bị input mặc định của hệ điều hành hay không |

Không có trạng thái chuyển đổi (state transition) — đây là dữ liệu truy vấn tức thời (snapshot), không lưu trữ lâu dài.
