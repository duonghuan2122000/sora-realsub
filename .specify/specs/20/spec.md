# Đặc tả tính năng: Cửa sổ phụ đề nổi

**Mã PBI**: 20
**Ngày tạo**: 2026-11-25
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Tạo một cửa sổ riêng biệt, luôn hiển thị trên các ứng dụng khác (always-on-top), không có viền/thanh tiêu đề, dùng để sau này hiển thị phụ đề thời gian thực trong lúc họp/stream mà không che khuất hay làm phân tán giao diện chính.

## Kịch bản & luồng người dùng

### Luồng chính

Khi người dùng bắt đầu một phiên phụ đề, cửa sổ phụ đề nổi xuất hiện, nằm trên tất cả cửa sổ khác kể cả khi người dùng chuyển sang ứng dụng khác (trình duyệt, Zoom, ...), và không có thanh tiêu đề/viền gây vướng mắt.

### Kịch bản chấp nhận

1. **Given** app đã khởi động, **When** cửa sổ phụ đề được mở, **Then** cửa sổ hiển thị không có border/title bar của hệ điều hành.
2. **Given** cửa sổ phụ đề đang mở, **When** người dùng chuyển focus sang một ứng dụng khác, **Then** cửa sổ phụ đề vẫn hiển thị phía trên ứng dụng đó.
3. **Given** cửa sổ phụ đề đang mở, **When** người dùng đóng cửa sổ chính (cửa sổ điều khiển), **Then** hành vi đóng cửa sổ phụ đề được xác định rõ ràng (xem Giả định).

### Trường hợp biên

- Cửa sổ phụ đề mở trên nhiều màn hình (multi-monitor): cửa sổ xuất hiện trên màn hình hiện tại, không bị kẹt ở màn hình đã ngắt kết nối.
- Hệ điều hành không hỗ trợ always-on-top (hiếm): ghi log cảnh báo, cửa sổ vẫn hiển thị bình thường (không always-on-top).

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI tạo được một cửa sổ riêng biệt với cửa sổ điều khiển chính.
- **FR-002**: Cửa sổ phụ đề PHẢI luôn hiển thị trên các cửa sổ ứng dụng khác (always-on-top) trên Windows và Linux.
- **FR-003**: Cửa sổ phụ đề PHẢI không có viền và thanh tiêu đề mặc định của hệ điều hành (decorations tắt).
- **FR-004**: Cửa sổ phụ đề PHẢI có thể được tạo/hiển thị độc lập, chưa cần gắn nội dung phụ đề thật (nội dung hiển thị sẽ làm ở PBI sau).

## Tiêu chí thành công

- **SC-001**: Cửa sổ phụ đề vẫn hiển thị phía trên khi người dùng chuyển qua ít nhất 3 ứng dụng khác nhau liên tiếp, xác nhận bằng kiểm thử thủ công trên Windows và Linux.
- **SC-002**: 100% các lần mở cửa sổ phụ đề không hiển thị viền/thanh tiêu đề hệ điều hành.

## Giả định

- Cửa sổ phụ đề đóng cùng lúc với toàn bộ ứng dụng (khi app thoát) ở giai đoạn này; hành vi đóng/ẩn riêng lẻ do người dùng điều khiển sẽ làm ở mục tiếp theo của M5 ("Nút/phím tắt ẩn-hiện cửa sổ phụ đề").
- Vị trí và kích thước mặc định ban đầu là cố định (góc dưới màn hình chính); kéo-thả và resize sẽ làm ở các mục sau của M5.
- macOS nằm ngoài phạm vi hiện tại (theo roadmap, để sau).

## Ngoài phạm vi

- Hiển thị nội dung phụ đề thật từ event `subtitle-ready` (mục tiếp theo của M5).
- Kéo-thả di chuyển vị trí, resize/đổi cỡ chữ, nút ẩn-hiện (các mục sau của M5).
- Auto-scroll/fade dòng cũ (mục sau của M5).
