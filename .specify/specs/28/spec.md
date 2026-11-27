# Đặc tả tính năng: Nút Start/Stop phiên dịch phụ đề

**Mã PBI**: 28
**Ngày tạo**: 2026-11-27
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Màn hình chính (PBI 26) đã cho người dùng chọn mic + loopback nhưng chưa có cách bắt đầu/kết thúc phiên dịch phụ đề. Tính năng này thêm 1 nút Start/Stop: bấm Start để khởi động phiên (capture + VAD + STT + dịch), bấm Stop để kết thúc; trạng thái hiển thị trên nút luôn khớp với trạng thái thật của Core (đang chạy/đã dừng), kể cả khi Core báo lỗi.

## Kịch bản & luồng người dùng

### Luồng chính

1. Người dùng đã chọn mic + loopback ở màn hình chính, nút ở trạng thái "Start", có thể bấm.
2. Người dùng bấm "Start" → UI gọi lệnh bắt đầu phiên ở Core → nút chuyển "Đang chạy..." (vô hiệu hoá, tránh bấm đè) → khi Core xác nhận đã chạy, nút chuyển thành "Stop" (bấm được).
3. Người dùng bấm "Stop" → UI gọi lệnh dừng phiên ở Core → nút chuyển "Đang dừng..." (vô hiệu hoá) → khi Core xác nhận đã dừng, nút chuyển lại "Start" (bấm được).

### Kịch bản chấp nhận

1. **Given** chưa chọn đủ mic + loopback, **When** xem màn hình chính, **Then** nút Start bị vô hiệu hoá (không bấm được).
2. **Given** đã chọn đủ mic + loopback và chưa chạy phiên nào, **When** bấm Start, **Then** nút hiển thị "Đang chạy..." trong lúc chờ, sau đó chuyển thành "Stop" khi Core xác nhận.
3. **Given** phiên đang chạy, **When** bấm Stop, **Then** nút chuyển "Đang dừng..." rồi về "Start" khi Core xác nhận đã dừng.
4. **Given** đang bấm Start, **When** Core trả lỗi (ví dụ thiết bị audio lỗi), **Then** nút trở lại "Start" (bấm được) và hiển thị thông báo lỗi rõ ràng cho người dùng.
5. **Given** phiên đang chạy, **When** đổi lựa chọn mic/loopback, **Then** lựa chọn thiết bị bị khoá (không đổi được) cho tới khi Stop.

### Trường hợp biên

- Core báo lỗi runtime (vd. mất thiết bị audio, crash) trong khi đang chạy → nút tự chuyển về "Start" và hiển thị lỗi, không yêu cầu người dùng bấm Stop thủ công.
- Người dùng bấm Start/Stop liên tiếp nhiều lần trước khi có phản hồi → chỉ 1 lệnh được gửi đi, các lần bấm thêm trong lúc đang chờ bị bỏ qua (nút đã vô hiệu hoá ở bước transition).
- Đóng app khi phiên đang chạy → phiên phải được dừng sạch (không leak thread/handle), theo hành vi `stop_capture` đã có (PBI 4).

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI hiển thị 1 nút duy nhất thể hiện đúng 1 trong các trạng thái: Start (sẵn sàng), Đang chạy..., Stop (sẵn sàng), Đang dừng....
- **FR-002**: Người dùng PHẢI bấm được nút để chuyển giữa trạng thái Start và Stop, chỉ khi nút đang ở trạng thái "sẵn sàng" (Start hoặc Stop).
- **FR-003**: Hệ thống PHẢI vô hiệu hoá nút Start khi chưa chọn đủ mic + loopback.
- **FR-004**: Hệ thống PHẢI vô hiệu hoá việc đổi mic/loopback khi phiên đang chạy hoặc đang trong trạng thái chuyển tiếp.
- **FR-005**: Hệ thống PHẢI đồng bộ trạng thái nút với trạng thái thật của Core — nút chỉ hiển thị "Stop" sau khi Core xác nhận đã bắt đầu, và chỉ hiển thị "Start" sau khi Core xác nhận đã dừng (hoặc báo lỗi).
- **FR-006**: Khi Core báo lỗi trong lúc bắt đầu, đang chạy, hoặc dừng phiên, hệ thống PHẢI hiển thị thông báo lỗi cho người dùng và đưa nút về trạng thái "sẵn sàng" phù hợp (Start nếu phiên không chạy nữa).

## Tiêu chí thành công

- **SC-001**: Trong mọi trường hợp (thành công hoặc lỗi), nút luôn phản ánh đúng trạng thái thật của phiên dịch (không có trường hợp nút hiển thị "Stop" khi Core đã dừng, hoặc ngược lại).
- **SC-002**: Người dùng bắt đầu được 1 phiên dịch phụ đề trong tối đa 2 bước (chọn thiết bị, bấm Start).
- **SC-003**: 100% các lần bấm Start/Stop khi nút đang ở trạng thái chuyển tiếp (Đang chạy.../Đang dừng...) không tạo thêm lệnh gửi tới Core.

## Giả định

- Việc truyền ID thiết bị mic/loopback đã chọn (PBI 26) vào lệnh bắt đầu phiên thực tế ở Core là phần việc kỹ thuật của tasks/plan, không phải quyết định phạm vi của spec này.
- "Core" ở đây là lớp Rust/Tauri đã có sẵn lệnh bắt đầu/kết thúc phiên (hiện là `start_capture`/`stop_capture`) — tên lệnh cụ thể không ảnh hưởng đến đặc tả này.

## Ngoài phạm vi

- Chọn cặp ngôn ngữ nguồn-đích (PBI riêng ở M6).
- Lưu cấu hình phiên vào file (PBI riêng ở M6).
- Hiển thị tiến trình/log chi tiết của phiên ngoài trạng thái Start/Stop.
