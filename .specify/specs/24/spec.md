# Đặc tả tính năng: Chỉnh cỡ chữ phụ đề

**Mã PBI**: 24
**Ngày tạo**: 2026-11-25
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Người dùng cần tăng/giảm cỡ chữ hiển thị trong cửa sổ phụ đề nổi (PBI 20-23) để phù hợp với khoảng cách xem, độ phân giải màn hình, hoặc thị lực cá nhân. Cỡ chữ đã chọn phải được giữ lại giữa các lần mở app, không phải chọn lại mỗi lần.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** cửa sổ phụ đề đang hiển thị với cỡ chữ mặc định, **When** người dùng tăng hoặc giảm cỡ chữ qua điều khiển trên cửa sổ (ví dụ nút +/- hoặc phím tắt), **Then** chữ trong cửa sổ đổi cỡ ngay lập tức và giữ nguyên cho các dòng phụ đề tiếp theo.

### Kịch bản chấp nhận

1. **Given** cửa sổ phụ đề đang mở với cỡ chữ mặc định, **When** người dùng tăng cỡ chữ, **Then** toàn bộ dòng phụ đề đang hiển thị (cũ + mới) đổi sang cỡ chữ lớn hơn.
2. **Given** người dùng đã chỉnh cỡ chữ về một giá trị khác mặc định, **When** người dùng đóng app rồi mở lại, **Then** cửa sổ phụ đề hiển thị đúng cỡ chữ đã chỉnh lần trước.
3. **Given** cỡ chữ đang ở mức nhỏ nhất (hoặc lớn nhất) cho phép, **When** người dùng tiếp tục bấm giảm (hoặc tăng), **Then** cỡ chữ không đổi nữa và không có lỗi/crash.

### Trường hợp biên

- Lần đầu mở app (chưa có cấu hình cỡ chữ lưu trước): dùng cỡ chữ mặc định.
- File cấu hình cỡ chữ bị hỏng hoặc chứa giá trị không hợp lệ: hệ thống dùng cỡ chữ mặc định, không crash.
- Đổi cỡ chữ trong lúc phụ đề đang cập nhật liên tục (auto-scroll/fade PBI 22): việc đổi cỡ không làm mất hoặc xô lệch thứ tự dòng đang hiển thị.

## Yêu cầu chức năng

- **FR-001**: Người dùng PHẢI có thể tăng cỡ chữ phụ đề qua một điều khiển trên cửa sổ phụ đề.
- **FR-002**: Người dùng PHẢI có thể giảm cỡ chữ phụ đề qua một điều khiển trên cửa sổ phụ đề.
- **FR-003**: Hệ thống PHẢI giới hạn cỡ chữ trong khoảng tối thiểu/tối đa hợp lý, không cho chữ nhỏ tới mức không đọc được hoặc lớn tới mức tràn khỏi cửa sổ.
- **FR-004**: Hệ thống PHẢI lưu lại cỡ chữ hiện tại khi người dùng đổi, và áp dụng lại đúng cỡ chữ đó ở lần mở app tiếp theo.
- **FR-005**: Hệ thống PHẢI dùng cỡ chữ mặc định khi chưa có cỡ chữ nào được lưu trước đó, hoặc dữ liệu lưu bị hỏng/không hợp lệ.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Người dùng đổi cỡ chữ và thấy phụ đề hiển thị đúng cỡ mới ngay lập tức, trong một lần bấm.
- **SC-002**: Sau khi đóng và mở lại app, cỡ chữ hiển thị đúng với lần chỉnh gần nhất, không cần chỉnh lại.

## Giả định

- Điều khiển tăng/giảm cỡ chữ là cơ chế chính cho tính năng này (không phải kéo-resize khung cửa sổ bằng chuột ở viền) — vì cửa sổ phụ đề không có border/title bar (PBI 20) nên resize kiểu kéo viền không tự nhiên; chỉnh cỡ chữ đáp ứng đúng nhu cầu "đọc dễ hơn/khó hơn tùy khoảng cách".
- Việc lưu cỡ chữ ra file cấu hình trong PBI này chỉ giới hạn ở giá trị cỡ chữ; các cấu hình khác (thiết bị, ngôn ngữ, vị trí cửa sổ) vẫn thuộc M6 như roadmap đã định, không gộp vào đây.
- Cỡ chữ áp dụng đồng nhất cho mọi dòng phụ đề đang hiển thị (không chỉnh riêng từng dòng).

## Ngoài phạm vi

- Resize khung cửa sổ phụ đề bằng kéo chuột ở viền.
- Lưu các cấu hình khác ngoài cỡ chữ (thiết bị mic/loopback, cặp ngôn ngữ, vị trí cửa sổ — thuộc M6).
- Tùy biến font chữ, màu chữ, hoặc theme (thuộc Backlog: "Theme/style tùy biến cửa sổ phụ đề").
