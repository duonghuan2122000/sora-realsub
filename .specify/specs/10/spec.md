# Đặc tả tính năng: Cắt audio thành segment theo khoảng lặng

**Mã PBI**: 10
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Hệ thống đã gán nhãn speech/silence cho từng frame audio (PBI 9). Bước tiếp theo là dùng các nhãn đó để cắt luồng audio liên tục thành các đoạn (segment) rời rạc, mỗi segment tương ứng một lượt nói, dựa trên khoảng lặng giữa các lượt nói. Đây là nền tảng để sau này mỗi segment được gửi riêng đi nhận dạng giọng nói (STT).

## Kịch bản & luồng người dùng

### Luồng chính

1. Hệ thống nhận luồng nhãn speech/silence theo frame (từ VAD, PBI 9) cùng dữ liệu audio PCM tương ứng.
2. Hệ thống theo dõi các frame liên tiếp: khi gặp frame "speech" sau một khoảng "silence" đủ dài, bắt đầu một segment mới.
3. Khi gặp khoảng "silence" đủ dài sau các frame "speech", hệ thống kết thúc segment hiện tại.
4. Mỗi segment hoàn chỉnh gồm: dữ liệu audio PCM của đoạn đó, thời điểm bắt đầu, thời điểm kết thúc.
5. Các segment được trả về/emit theo đúng thứ tự thời gian xuất hiện trong audio gốc.

### Kịch bản chấp nhận

1. **Given** audio có 1 lượt nói liên tục rồi im lặng kéo dài, **When** xử lý qua bộ cắt segment, **Then** hệ thống trả về đúng 1 segment chứa trọn lượt nói đó.
2. **Given** audio có nói - lặng - nói - lặng xen kẽ (nhiều lượt nói tách biệt), **When** xử lý, **Then** hệ thống trả về đúng số segment tương ứng số lượt nói, theo đúng thứ tự thời gian, không ghép 2 lượt nói khác nhau vào 1 segment.
3. **Given** audio chỉ có khoảng lặng ngắn xen giữa 2 đoạn nói (ví dụ ngắt hơi giữa câu), **When** khoảng lặng đó ngắn hơn ngưỡng cấu hình, **Then** hệ thống coi là 1 segment liên tục, không bị cắt gãy giữa câu.
4. **Given** audio chỉ toàn im lặng, không có frame "speech" nào, **When** xử lý, **Then** hệ thống trả về danh sách segment rỗng, không lỗi.
5. **Given** audio kết thúc khi vẫn đang trong đoạn nói (chưa gặp khoảng lặng kết thúc), **When** luồng audio dừng, **Then** hệ thống vẫn trả về segment cuối cùng đó (không bị mất dữ liệu đang nói).

### Trường hợp biên

- Khoảng lặng rất ngắn (vài chục ms) do nhiễu VAD không được coi là điểm cắt, tránh tạo quá nhiều segment vụn.
- Lượt nói rất ngắn (ví dụ 1 tiếng "ừ") vẫn phải được trả về như 1 segment hợp lệ, không bị loại bỏ.
- Audio rỗng hoặc không có dữ liệu: hệ thống trả về danh sách segment rỗng, không crash.
- Xử lý liên tục nhiều buffer nối tiếp (streaming) không làm mất hoặc trùng lặp segment ở ranh giới giữa các buffer.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI dùng nhãn speech/silence theo frame để xác định điểm bắt đầu và kết thúc của mỗi segment.
- **FR-002**: Hệ thống PHẢI coi khoảng lặng ngắn hơn một ngưỡng cấu hình là không đủ để cắt segment (gộp vào cùng 1 lượt nói).
- **FR-003**: Hệ thống PHẢI trả về các segment theo đúng thứ tự thời gian xuất hiện trong audio gốc, kèm thời điểm bắt đầu/kết thúc của từng segment.
- **FR-004**: Hệ thống PHẢI trả về segment đang mở (chưa gặp khoảng lặng kết thúc) khi luồng audio đầu vào kết thúc, không làm mất dữ liệu.
- **FR-005**: Hệ thống PHẢI trả về danh sách segment rỗng khi audio không chứa frame "speech" nào, không báo lỗi.

## Tiêu chí thành công

- **SC-001**: Với file audio mẫu có nói + lặng xen kẽ đã biết trước số lượt nói, số segment trả về khớp đúng 100% với số lượt nói thực tế trong file mẫu.
- **SC-002**: Với file audio mẫu có khoảng lặng ngắn giữa câu (dưới ngưỡng), hệ thống không cắt gãy câu đó thành 2 segment.
- **SC-003**: Thời điểm bắt đầu/kết thúc mỗi segment trả về sai lệch không quá 1 frame so với ranh giới nói/lặng thực tế trong file mẫu.

## Thực thể chính

- **Segment**: một đoạn audio liên tục được xác định là có giọng nói, gồm dữ liệu PCM, thời điểm bắt đầu, thời điểm kết thúc.

## Giả định

- Dùng lại nhãn speech/silence theo frame từ VAD (PBI 9), không cần thay đổi model VAD.
- Ngưỡng độ dài khoảng lặng tối thiểu để cắt segment là tham số có thể cấu hình ở tầng kỹ thuật, không ảnh hưởng hành vi chức năng mô tả ở đây.
- Xử lý theo kiểu buffer nối tiếp (streaming), chưa cần giữ toàn bộ audio phiên họp trong bộ nhớ.

## Ngoài phạm vi

- Phát event Tauri `vad-segment` ra UI (thuộc PBI tiếp theo trong M2).
- STT, dịch segment (thuộc M3, M4).
