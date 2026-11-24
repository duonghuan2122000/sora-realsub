# Đặc tả tính năng: Phát sự kiện đoạn VAD cho UI

**Mã PBI**: 11
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Khi Core cắt được một đoạn âm thanh (segment) mới từ luồng VAD, hệ thống phát một sự kiện Tauri `vad-segment` kèm thông tin đoạn đó để tầng UI nhận và xử lý tiếp (hiển thị debug, hoặc làm đầu vào cho bước STT sau này). Đây là cầu nối giữa Core (đã cắt segment ở PBI 9-10) và UI, hoàn thiện milestone M2.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** phiên capture đang chạy (`start_capture` đã gọi) và VAD đang cắt segment theo khoảng lặng
**When** Core xác định một đoạn speech vừa kết thúc (ranh giới segment)
**Then** Core phát sự kiện `vad-segment` tới UI kèm dữ liệu đoạn đó, và UI nhận được sự kiện theo đúng thứ tự thời gian phát sinh

### Kịch bản chấp nhận

1. **Given** phiên capture đang chạy với audio có nói rồi lặng, **When** VAD cắt xong 1 segment, **Then** UI nhận được 1 sự kiện `vad-segment` tương ứng, đúng 1 lần (không lặp, không thiếu).
2. **Given** audio có nhiều đoạn nói xen lặng liên tiếp, **When** VAD cắt nhiều segment, **Then** UI nhận các sự kiện `vad-segment` theo đúng thứ tự thời gian phát sinh của từng segment.
3. **Given** phiên capture đã dừng (`stop_capture`), **When** không còn audio mới, **Then** không còn sự kiện `vad-segment` nào được phát thêm.

### Trường hợp biên

- Segment rất ngắn (gần ngưỡng tối thiểu) vẫn phải phát sự kiện đầy đủ, không bị bỏ qua.
- UI chưa kịp lắng nghe sự kiện (ví dụ window UI mở sau khi phiên đã bắt đầu) — không yêu cầu replay lại các sự kiện đã phát trước đó (ngoài phạm vi).
- Nhiều segment phát sinh liên tiếp rất nhanh — UI vẫn phải nhận đủ, không bị dồn/mất sự kiện.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI phát một sự kiện Tauri tên `vad-segment` mỗi khi VAD cắt xong một đoạn audio mới.
- **FR-002**: Sự kiện PHẢI mang theo dữ liệu nhận diện được đoạn audio tương ứng (ví dụ thời điểm bắt đầu/kết thúc và dữ liệu/âm thanh của đoạn) để UI dùng được ngay không cần hỏi lại Core.
- **FR-003**: Thứ tự các sự kiện `vad-segment` nhận ở UI PHẢI khớp với thứ tự thời gian các segment phát sinh thực tế.
- **FR-004**: Hệ thống KHÔNG được phát sự kiện `vad-segment` trùng lặp cho cùng một segment, và KHÔNG được bỏ sót segment nào đã được VAD cắt xong trong lúc phiên đang chạy.
- **FR-005**: Sau khi phiên capture dừng, hệ thống PHẢI ngừng phát sự kiện `vad-segment`.

## Tiêu chí thành công

- **SC-001**: Với file audio mẫu có N đoạn nói xen lặng, UI nhận đúng N sự kiện `vad-segment`, đúng thứ tự thời gian.
- **SC-002**: Không có sự kiện `vad-segment` nào bị trễ quá mức gây cảm giác giật/lag khi quan sát trực tiếp trong lúc phiên đang chạy.
- **SC-003**: 0 sự kiện `vad-segment` phát sinh sau khi phiên capture đã dừng.

## Giả định

- Việc cắt segment từ khoảng lặng (PBI 9-10) đã hoạt động đúng; PBI này chỉ thêm bước phát sự kiện ra UI, không thay đổi logic VAD/cắt segment.
- UI lắng nghe sự kiện qua cơ chế event của Tauri đã dùng cho các sự kiện khác trong dự án (nhất quán với kiến trúc IPC command/event đã chốt).
- Trong PBI này, UI chỉ cần nhận và log/hiển thị debug sự kiện; việc dùng segment làm đầu vào STT thuộc M3, ngoài phạm vi.

## Ngoài phạm vi

- Xử lý/STT nội dung của segment (thuộc M3).
- Lưu lại lịch sử segment để replay cho UI mở muộn.
- Giao diện hiển thị chính thức cho segment trong Control UI (thuộc M6).
