# Đặc tả tính năng: Màn hình chính chọn thiết bị

**Mã PBI**: 26
**Ngày tạo**: 2026-11-27
**Trạng thái**: Nháp

## Mô tả tổng quan

Người dùng cần một màn hình chính để chọn thiết bị microphone và nguồn loopback (âm thanh hệ thống) trước khi bắt đầu phiên phụ đề. Nếu chọn thiết bị không hợp lệ hoặc không khả dụng, hệ thống phải báo lỗi rõ ràng để người dùng chọn lại.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng mở app lần đầu hoặc chưa chọn thiết bị, **When** màn hình chính hiển thị, **Then** danh sách thiết bị mic và danh sách nguồn loopback khả dụng được hiển thị để chọn.

### Kịch bản chấp nhận

1. **Given** có ít nhất 1 thiết bị mic và 1 nguồn loopback khả dụng, **When** người dùng mở màn hình chính, **Then** cả hai danh sách hiển thị tên thiết bị, có thể chọn 1 mục trong mỗi danh sách.
2. **Given** người dùng đã chọn 1 mic và 1 nguồn loopback hợp lệ, **When** người dùng xác nhận lựa chọn, **Then** lựa chọn được ghi nhận làm thiết bị đang dùng cho phiên tiếp theo.
3. **Given** thiết bị đã chọn trước đó bị rút/tắt trước khi mở lại màn hình, **When** màn hình chính tải danh sách thiết bị, **Then** thiết bị không còn khả dụng không xuất hiện trong danh sách chọn được, kèm thông báo rõ ràng rằng lựa chọn cũ không còn khả dụng.
4. **Given** không có thiết bị mic nào hoặc không có nguồn loopback nào trên máy, **When** màn hình chính tải danh sách, **Then** hiển thị thông báo rõ ràng rằng không có thiết bị khả dụng, thay vì danh sách trống không giải thích.

### Trường hợp biên

- Thiết bị bị rút ra (unplug) ngay sau khi danh sách đã tải nhưng trước khi xác nhận chọn.
- Hệ thống không lấy được danh sách thiết bị (lỗi truy vấn hệ điều hành).
- Người dùng chưa chọn đủ cả mic và loopback mà đã bấm xác nhận.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI hiển thị danh sách thiết bị mic khả dụng trên máy khi mở màn hình chính.
- **FR-002**: Hệ thống PHẢI hiển thị danh sách nguồn loopback (âm thanh hệ thống) khả dụng trên máy khi mở màn hình chính.
- **FR-003**: Người dùng PHẢI có thể chọn đúng 1 thiết bị mic và đúng 1 nguồn loopback từ danh sách.
- **FR-004**: Hệ thống PHẢI ngăn xác nhận lựa chọn khi chưa chọn đủ cả mic và loopback, kèm thông báo rõ thiếu phần nào.
- **FR-005**: Hệ thống PHẢI hiển thị thông báo lỗi rõ ràng, dễ hiểu khi thiết bị đã chọn không còn khả dụng (bị rút, bị tắt) hoặc khi không lấy được danh sách thiết bị.
- **FR-006**: Hệ thống PHẢI hiển thị thông báo rõ ràng khi không có thiết bị mic hoặc không có nguồn loopback nào khả dụng trên máy.

## Tiêu chí thành công

- **SC-001**: Người dùng xác định và chọn được đúng thiết bị mic + loopback mong muốn trong vòng 1 phút kể từ khi mở màn hình chính.
- **SC-002**: 100% trường hợp chọn thiết bị không hợp lệ (không còn khả dụng, thiếu lựa chọn) hiển thị thông báo lỗi mô tả rõ nguyên nhân, không để màn hình im lặng hoặc crash.

## Giả định

- Danh sách thiết bị mic đã có sẵn qua tính năng liệt kê thiết bị input (M1). Màn hình chính tái sử dụng dữ liệu này, bổ sung phần chọn nguồn loopback.
- "Nguồn loopback" là danh sách do hệ thống cấp (ví dụ thiết bị loopback mặc định theo OS), không yêu cầu người dùng tự định cấu hình driver ảo.
- Lựa chọn thiết bị chỉ cần lưu trong phạm vi phiên làm việc hiện tại của app; lưu persist lâu dài giữa các lần mở app thuộc PBI khác (M6 "Lưu cấu hình").

## Ngoài phạm vi

- Lưu cấu hình thiết bị đã chọn ra file để dùng lại giữa các lần mở app (thuộc mục riêng trong M6).
- Nút Start/Stop phiên dịch phụ đề và chọn cặp ngôn ngữ (thuộc các mục riêng trong M6).
- Hỗ trợ chọn nhiều mic hoặc nhiều nguồn loopback đồng thời.
