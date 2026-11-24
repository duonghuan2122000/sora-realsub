# Đặc tả tính năng: Lệnh bắt đầu/kết thúc phiên bắt âm thanh

**Mã PBI**: 8
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Người dùng cần một cách để bắt đầu và kết thúc việc thu âm (mic + loopback đã trộn) từ giao diện, thay vì chỉ chạy được qua binary thử nghiệm ghi file .wav. Tính năng này đưa ra 2 lệnh điều khiển phiên thu âm: bắt đầu và dừng, đảm bảo dừng đúng cách không để sót tiến trình/tài nguyên.

## Kịch bản & luồng người dùng

### Luồng chính

1. Người dùng (qua UI) gọi lệnh bắt đầu phiên thu âm.
2. Hệ thống bắt đầu thu âm từ mic + loopback đã trộn và xác nhận phiên đang chạy.
3. Người dùng gọi lệnh kết thúc phiên thu âm.
4. Hệ thống dừng thu âm, giải phóng toàn bộ tài nguyên liên quan (thread, thiết bị âm thanh), xác nhận phiên đã dừng.

### Kịch bản chấp nhận

1. **Given** chưa có phiên thu âm nào đang chạy, **When** người dùng gọi lệnh bắt đầu, **Then** phiên thu âm khởi động thành công và hệ thống báo trạng thái "đang chạy".
2. **Given** phiên thu âm đang chạy, **When** người dùng gọi lệnh dừng, **Then** phiên thu âm kết thúc, hệ thống báo trạng thái "đã dừng", và không còn luồng/thiết bị âm thanh nào bị giữ lại.
3. **Given** phiên thu âm đang chạy, **When** người dùng gọi lại lệnh bắt đầu lần nữa, **Then** hệ thống báo lỗi rõ ràng rằng đã có phiên đang chạy, không khởi động phiên thứ hai song song.
4. **Given** chưa có phiên thu âm nào đang chạy, **When** người dùng gọi lệnh dừng, **Then** hệ thống báo lỗi rõ ràng rằng không có phiên nào để dừng, không gây crash.

### Trường hợp biên

- Gọi dừng ngay sau khi gọi bắt đầu (race điều kiện khởi động chưa xong) phải xử lý nhất quán, không để treo hoặc leak.
- Thiết bị mic hoặc loopback bị rút/lỗi giữa phiên: lệnh dừng vẫn phải giải phóng được tài nguyên đã cấp.
- Gọi bắt đầu/dừng liên tiếp nhiều lần (start → stop → start → stop) nhiều lượt phải không tích lũy leak theo thời gian.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI cung cấp một lệnh để bắt đầu phiên thu âm (mic + loopback trộn).
- **FR-002**: Hệ thống PHẢI cung cấp một lệnh để dừng phiên thu âm đang chạy.
- **FR-003**: Hệ thống PHẢI từ chối bắt đầu phiên mới khi đã có phiên đang chạy, kèm thông báo lỗi rõ ràng.
- **FR-004**: Hệ thống PHẢI từ chối lệnh dừng khi không có phiên nào đang chạy, kèm thông báo lỗi rõ ràng.
- **FR-005**: Khi dừng phiên, hệ thống PHẢI giải phóng toàn bộ thread và handle thiết bị âm thanh đã cấp cho phiên đó.

## Tiêu chí thành công

- **SC-001**: Sau khi gọi lệnh dừng, không còn thread hoặc handle âm thanh nào liên quan đến phiên đó còn tồn tại (kiểm tra được qua test tự động).
- **SC-002**: Lặp lại 10 lần chu trình bắt đầu → dừng không làm tăng số lượng thread/handle còn sống so với trước khi lặp.
- **SC-003**: Gọi bắt đầu khi đã chạy, hoặc dừng khi chưa chạy, luôn trả về lỗi rõ ràng, không làm crash ứng dụng.

## Giả định

- Nguồn thu âm sử dụng lại `MixedAudioSource` (mic + loopback) đã có từ PBI 7, không cần nguồn mới.
- Mỗi thời điểm chỉ hỗ trợ 1 phiên thu âm duy nhất (chưa cần nhiều phiên song song).
- "Leak thread/handle" được xác minh ở mức unit/integration test trong Rust (đếm số handle còn sống sau khi dừng), không yêu cầu công cụ profiling ngoài.

## Ngoài phạm vi

- Xử lý VAD, STT, dịch (thuộc các milestone M2-M4).
- Giao diện UI để bấm nút Start/Stop (thuộc M6); PBI này chỉ cung cấp lệnh ở tầng Core/IPC.
