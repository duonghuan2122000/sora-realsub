# Mô hình dữ liệu: PBI 26

## AudioDeviceInfo (đã có, tái dùng)

| Thuộc tính | Kiểu | Ý nghĩa |
|---|---|---|
| `id` | string | Định danh thiết bị (tên thiết bị từ `cpal`, dùng để so khớp còn khả dụng hay không) |
| `name` | string | Tên hiển thị cho người dùng |
| `is_default` | bool | Có là thiết bị mặc định của hệ điều hành không |

## DeviceSelection (mới, chỉ ở phía UI, không persist)

| Thuộc tính | Kiểu | Ý nghĩa |
|---|---|---|
| `micDeviceId` | string \| null | `id` của mic đã chọn |
| `loopbackDeviceId` | string \| null | `id` của nguồn loopback (thiết bị output) đã chọn |

### Luật hợp lệ

- Xác nhận lựa chọn chỉ hợp lệ khi cả `micDeviceId` và `loopbackDeviceId` đều không null và đều khớp với một thiết bị có trong danh sách hiện tại.
- Khi tải lại danh sách, nếu `micDeviceId`/`loopbackDeviceId` đang chọn không còn xuất hiện trong danh sách mới → đặt lại về `null` và phát sinh thông báo lỗi tương ứng.
