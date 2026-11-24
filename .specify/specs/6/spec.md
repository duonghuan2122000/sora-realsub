# Đặc tả tính năng: Capture loopback âm thanh hệ thống (Windows)

**Mã PBI**: 6
**Ngày tạo**: 2026-11-24
**Trạng thái**: Đã implement, test pass

## Mô tả tổng quan

Bên cạnh mic (PBI 4-5), hệ thống cần bắt được âm thanh đang phát ra từ loa/thiết bị output của máy (ví dụ tiếng người khác nói trong cuộc họp online, âm thanh video) để làm phụ đề cho cả nội dung mình nghe, không chỉ nội dung mình nói.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** thiết bị output mặc định của Windows đang phát âm thanh, **When** người dùng bắt đầu capture loopback, **Then** hệ thống liên tục nhận được các khối mẫu âm thanh (PCM) là bản sao của chính âm thanh đang phát ra loa, cho đến khi người dùng dừng lại.

### Kịch bản chấp nhận

1. **Given** thiết bị output mặc định đang phát một đoạn âm thanh/nhạc, **When** bắt đầu capture loopback trong vài giây rồi dừng, **Then** dữ liệu ghi ra file .wav nghe lại đúng là âm thanh đã phát ra loa trong lúc đó (không im lặng, không chỉ toàn nhiễu).
2. **Given** capture loopback đang chạy, **When** người dùng dừng capture, **Then** việc thu mẫu dừng ngay, không còn khối mẫu mới nào được tạo sau đó.
3. **Given** thiết bị output mặc định không phát gì (im lặng), **When** đang capture loopback, **Then** hệ thống vẫn chạy bình thường, không lỗi, output là im lặng (không crash, không treo).

### Trường hợp biên

- Không có thiết bị output nào khả dụng (ví dụ không có loa/driver audio) → báo lỗi rõ ràng, không treo ứng dụng.
- Thiết bị output mặc định bị đổi (người dùng chuyển loa) hoặc rút ra giữa lúc đang capture → hệ thống phải dừng capture và báo lỗi, không crash.
- Capture được bắt đầu rồi dừng ngay lập tức (không có mẫu nào) → không lỗi, file .wav rỗng hoặc gần như rỗng vẫn hợp lệ.
- Gọi dừng capture khi chưa từng bắt đầu → không crash, trả lỗi hoặc no-op rõ ràng.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI thu liên tục dữ liệu âm thanh thô là bản sao (loopback) của âm thanh đang phát ra từ thiết bị output mặc định của hệ thống, sau khi bắt đầu capture.
- **FR-002**: Hệ thống PHẢI cho phép dừng capture loopback theo yêu cầu, và việc thu mẫu phải chấm dứt ngay sau đó.
- **FR-003**: Hệ thống PHẢI báo lỗi rõ ràng khi không có thiết bị output khả dụng hoặc thiết bị gặp sự cố, không treo hoặc crash ứng dụng.
- **FR-004**: Âm thanh loopback thu được PHẢI có thể xuất ra file .wav để nghe/kiểm tra lại, và nội dung nghe được phải khớp với âm thanh đã phát ra loa trong lúc capture.

## Tiêu chí thành công

- **SC-001**: Phát thử một đoạn âm thanh/nhạc 5-10 giây qua loa, nghe lại file .wav xuất ra khớp đúng nội dung đã phát (không rè, không câm, không bị cắt đầu/cuối quá 0.5s).
- **SC-002**: Dừng capture luôn dừng việc thu mẫu trong vòng dưới 1 giây, không để lại tiến trình/luồng chạy ngầm.
- **SC-003**: 100% các lần thử khi không có thiết bị output khả dụng đều nhận được thông báo lỗi thay vì ứng dụng treo/crash.

## Giả định

- Chỉ bắt loopback của thiết bị output **mặc định** của hệ điều hành (không cho chọn thiết bị output cụ thể ở bước này) — việc chọn thiết bị output cụ thể, nếu cần, để lại cho PBI sau.
- Dùng cơ chế loopback native của Windows (WASAPI loopback), chỉ áp dụng cho Windows theo đúng quyết định nền tảng ở M0.
- Tạm thu một stream loopback độc lập, chưa trộn (mix) với mic — việc trộn thuộc PBI tiếp theo trong M1.
- Định dạng số hoá trung gian là PCM; việc ghi ra .wav chỉ dùng để kiểm thử/làm bằng chứng, không phải tính năng cuối cho người dùng.

## Ngoài phạm vi

- Trộn (mix) loopback với mic — thuộc PBI tiếp theo trong M1.
- Cho phép chọn thiết bị output cụ thể (khác thiết bị mặc định) để capture.
- Hỗ trợ macOS/Linux cho loopback.
- Xử lý VAD, STT, hoặc bất kỳ xử lý ngữ nghĩa nào trên audio.
