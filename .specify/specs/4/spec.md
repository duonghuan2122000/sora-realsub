# Đặc tả tính năng: Liệt kê thiết bị âm thanh đầu vào

**Mã PBI**: 4
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Người dùng cần chọn thiết bị microphone để app bắt âm thanh khi bắt đầu phiên phụ đề. Tính năng này cho phép UI lấy danh sách thiết bị audio input (mic) hiện có trên máy, hiển thị để người dùng chọn trước khi bắt đầu capture.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng mở màn hình cấu hình thiết bị
**When** màn hình được tải
**Then** hệ thống hiển thị danh sách tên các thiết bị mic đang có trên máy, kèm đánh dấu thiết bị mặc định (nếu có)

### Kịch bản chấp nhận

1. **Given** máy có ít nhất 1 mic được hệ điều hành nhận diện, **When** người dùng mở màn hình chọn thiết bị, **Then** danh sách hiển thị đầy đủ tên các mic đó.
2. **Given** máy không có mic nào, **When** người dùng mở màn hình chọn thiết bị, **Then** hệ thống hiển thị thông báo rõ ràng rằng không tìm thấy thiết bị mic nào.
3. **Given** danh sách thiết bị đã hiển thị, **When** người dùng cắm thêm hoặc rút mic rồi yêu cầu làm mới danh sách, **Then** danh sách được cập nhật phản ánh đúng trạng thái thiết bị hiện tại.

### Trường hợp biên

- Hệ điều hành trả về thiết bị không có tên (rỗng) → hiển thị tên thay thế dễ hiểu (ví dụ "Thiết bị không tên").
- Việc lấy danh sách thiết bị bị lỗi (hệ điều hành từ chối quyền truy cập) → hiển thị thông báo lỗi rõ ràng, không làm crash UI.
- Có nhiều thiết bị trùng tên → vẫn hiển thị riêng biệt từng thiết bị, người dùng vẫn chọn được đúng thiết bị mong muốn.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI cung cấp một cách để UI yêu cầu danh sách thiết bị audio input hiện có trên máy.
- **FR-002**: Hệ thống PHẢI trả về cho mỗi thiết bị ít nhất: tên hiển thị và trạng thái có phải thiết bị mặc định hay không.
- **FR-003**: UI PHẢI hiển thị danh sách thiết bị nhận được dưới dạng người dùng chọn được 1 thiết bị.
- **FR-004**: Hệ thống PHẢI cho phép người dùng yêu cầu làm mới danh sách thiết bị bất kỳ lúc nào trước khi bắt đầu phiên.
- **FR-005**: Khi không có thiết bị nào hoặc khi lấy danh sách thất bại, hệ thống PHẢI thông báo tình trạng đó cho người dùng thay vì để trống không rõ lý do.

## Tiêu chí thành công

- **SC-001**: Người dùng thấy danh sách mic đang có trên máy trong vòng 1 giây sau khi mở màn hình chọn thiết bị.
- **SC-002**: 100% thiết bị mic được hệ điều hành nhận diện đều xuất hiện trong danh sách.
- **SC-003**: Khi không có mic, người dùng nhận được thông báo rõ nghĩa, không thấy màn hình trống không giải thích.

## Giả định

- Chỉ liệt kê thiết bị audio **input** (mic); thiết bị output không thuộc phạm vi tính năng này.
- Danh sách lấy theo trạng thái hệ điều hành tại thời điểm yêu cầu, không cần tự động theo dõi real-time khi cắm/rút thiết bị (người dùng bấm làm mới).
- Chạy trên Windows trước (theo quyết định M0), chưa cần hỗ trợ macOS.

## Ngoài phạm vi

- Chọn/cấu hình thiết bị loopback (âm thanh hệ thống) — thuộc PBI khác trong M1.
- Capture thực tế âm thanh từ thiết bị đã chọn — thuộc PBI khác trong M1.
- Tự động phát hiện thiết bị mới cắm vào theo thời gian thực (hot-plug event).
