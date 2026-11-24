# Đặc tả tính năng: Capture mic thành PCM buffer

**Mã PBI**: 5
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Người dùng chọn một thiết bị mic (đã liệt kê ở PBI 4), bắt đầu ghi âm, và hệ thống thu liên tục dữ liệu âm thanh thô từ mic thành các khối PCM. Đây là nền tảng để các bước sau (VAD, STT) xử lý.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng đã chọn một thiết bị mic hợp lệ, **When** người dùng bắt đầu capture, **Then** hệ thống liên tục nhận được các khối mẫu âm thanh (PCM) từ mic cho đến khi người dùng dừng lại.

### Kịch bản chấp nhận

1. **Given** thiết bị mic hợp lệ đang cắm, **When** bắt đầu capture và nói vào mic trong vài giây rồi dừng, **Then** dữ liệu thu được khi ghi ra file .wav nghe lại đúng là âm thanh đã nói (không im lặng, không nhiễu trắng toàn phần).
2. **Given** capture đang chạy, **When** người dùng dừng capture, **Then** việc thu mẫu dừng ngay, không còn khối mẫu mới nào được tạo ra sau đó.
3. **Given** không chọn thiết bị hoặc thiết bị không tồn tại, **When** người dùng bắt đầu capture, **Then** hệ thống báo lỗi rõ ràng, không treo ứng dụng.

### Trường hợp biên

- Thiết bị mic bị rút ra giữa lúc đang capture → hệ thống phải dừng capture và báo lỗi, không crash.
- Capture được bắt đầu rồi dừng ngay lập tức (không có mẫu nào) → không lỗi, file .wav rỗng hoặc gần như rỗng vẫn hợp lệ.
- Gọi dừng capture khi chưa từng bắt đầu → không crash, trả lỗi hoặc no-op rõ ràng.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI thu liên tục dữ liệu âm thanh thô từ thiết bị mic được chỉ định sau khi bắt đầu capture.
- **FR-002**: Hệ thống PHẢI cho phép dừng capture theo yêu cầu, và việc thu mẫu phải chấm dứt ngay sau đó.
- **FR-003**: Hệ thống PHẢI báo lỗi rõ ràng khi thiết bị mic không hợp lệ hoặc không khả dụng, không treo hoặc crash ứng dụng.
- **FR-004**: Âm thanh thu được PHẢI có thể xuất ra file .wav để nghe/kiểm tra lại, và nội dung nghe được phải khớp với âm thanh đã phát vào mic trong lúc capture.

## Tiêu chí thành công

- **SC-001**: Ghi thử một đoạn nói 5-10 giây vào mic, nghe lại file .wav xuất ra khớp đúng nội dung đã nói (không rè, không câm, không bị cắt đầu/cuối quá 0.5s).
- **SC-002**: Dừng capture luôn dừng việc thu mẫu trong vòng dưới 1 giây, không để lại tiến trình/luồng chạy ngầm.
- **SC-003**: 100% các lần thử với thiết bị mic không hợp lệ đều nhận được thông báo lỗi thay vì ứng dụng treo/crash.

## Giả định

- Thiết bị mic dùng để capture được chọn từ danh sách đã có ở PBI 4 (liệt kê thiết bị audio input).
- Tạm thu một stream mic duy nhất; chưa cần trộn với loopback hệ thống (việc này thuộc PBI kế tiếp trong M1).
- Định dạng số hoá trung gian là PCM (số thực hoặc số nguyên tuỳ cài đặt), việc ghi ra .wav chỉ dùng để kiểm thử/làm bằng chứng, không phải tính năng cuối cho người dùng.

## Ngoài phạm vi

- Trộn (mix) mic với loopback hệ thống — thuộc PBI tiếp theo trong M1.
- Hiển thị waveform/mức âm lượng trên UI.
- Xử lý VAD, STT, hoặc bất kỳ xử lý ngữ nghĩa nào trên audio.
