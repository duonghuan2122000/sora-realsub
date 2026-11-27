# Đặc tả tính năng: Ẩn-hiện cửa sổ phụ đề

**Mã PBI**: 25
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Người dùng cần nhanh chóng ẩn cửa sổ phụ đề nổi (ví dụ khi chia sẻ màn hình, chụp ảnh, hoặc tạm thời không muốn phụ đề che nội dung) và hiện lại khi cần, không phải dừng hẳn phiên dịch phụ đề đang chạy.

## Kịch bản & luồng người dùng

### Luồng chính

Given cửa sổ phụ đề đang hiển thị và phiên dịch đang chạy,
When người dùng nhấn phím tắt ẩn-hiện,
Then cửa sổ phụ đề biến mất khỏi màn hình nhưng phiên dịch (capture, VAD, STT, dịch) vẫn tiếp tục chạy ở nền.

Given cửa sổ phụ đề đang ẩn,
When người dùng nhấn lại phím tắt,
Then cửa sổ phụ đề hiện lại đúng vị trí và cỡ chữ đã lưu trước đó, tiếp tục hiển thị phụ đề mới nhất.

### Kịch bản chấp nhận

1. **Given** phiên dịch đang chạy và cửa sổ phụ đề đang hiện, **When** nhấn phím tắt, **Then** cửa sổ ẩn ngay (không có hiệu ứng trễ đáng kể), phụ đề mới vẫn được xử lý ở nền.
2. **Given** cửa sổ phụ đề đang ẩn, **When** nhấn lại phím tắt, **Then** cửa sổ hiện lại tại đúng vị trí/kích thước cỡ chữ trước khi ẩn, và hiển thị ngay dòng phụ đề mới nhất đã phát sinh trong lúc ẩn.
3. **Given** app vừa khởi động (chưa nhấn phím tắt lần nào), **When** phiên dịch bắt đầu, **Then** cửa sổ phụ đề ở trạng thái hiện (mặc định).

### Trường hợp biên

- Nhấn phím tắt liên tục nhiều lần nhanh: trạng thái ẩn/hiện phải đổi đúng theo số lần nhấn, không bị kẹt hoặc đảo ngược.
- Nhấn phím tắt khi chưa có phiên dịch nào chạy: cửa sổ phụ đề (nếu đã mở) vẫn ẩn/hiện bình thường, không gây lỗi.
- Đóng hẳn cửa sổ phụ đề (không phải ẩn) rồi nhấn phím tắt: hệ thống không được crash; hành vi mong đợi là mở lại cửa sổ ở trạng thái hiện.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI cung cấp một phím tắt toàn cục để ẩn hoặc hiện cửa sổ phụ đề, hoạt động được ngay cả khi cửa sổ phụ đề không ở trạng thái focus.
- **FR-002**: Mỗi lần nhấn phím tắt PHẢI đảo trạng thái hiện tại (hiện → ẩn, ẩn → hiện).
- **FR-003**: Khi cửa sổ phụ đề bị ẩn, hệ thống PHẢI tiếp tục chạy pipeline phụ đề ở nền (không dừng phiên dịch).
- **FR-004**: Khi hiện lại, cửa sổ PHẢI giữ nguyên vị trí và cỡ chữ đã thiết lập trước khi ẩn.

## Tiêu chí thành công

- **SC-001**: Người dùng ẩn/hiện được cửa sổ phụ đề bằng một phím tắt, không cần thao tác chuột.
- **SC-002**: Thời gian từ lúc nhấn phím tắt đến khi cửa sổ đổi trạng thái hiển thị dưới 0.5 giây.
- **SC-003**: Sau khi ẩn rồi hiện lại, vị trí và cỡ chữ cửa sổ không thay đổi so với trước khi ẩn.
- **SC-004**: Phiên dịch phụ đề không bị gián đoạn hoặc mất dữ liệu trong lúc cửa sổ đang ẩn.

## Giả định

- Phím tắt là tổ hợp bàn phím toàn cục (global shortcut) do hệ thống bắt, không phải nút bấm trong UI, vì màn hình điều khiển chính (M6) chưa được thi công.
- "Ẩn" nghĩa là cửa sổ không hiển thị trên màn hình (ví dụ `hide()`), không phải đóng/destroy cửa sổ.
- Tổ hợp phím tắt cụ thể sẽ do bước lập kế hoạch kỹ thuật (`/sora-plan`) chọn, tránh trùng với phím tắt hệ điều hành thông dụng.

## Ngoài phạm vi

- Nút bấm ẩn-hiện trong màn hình điều khiển chính (thuộc M6, chưa thi công).
- Cho người dùng tự cấu hình/đổi tổ hợp phím tắt.
