# Đặc tả tính năng: Event transcript-partial/transcript-final cho UI

**Mã PBI**: 14
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Pipeline VAD → STT (PBI 13) hiện chỉ trả văn bản trong nội bộ Core, UI chưa nhận được gì. Tính năng này phát 2 loại event Tauri từ Core sang UI khi có kết quả nhận diện giọng nói: `transcript-partial` (text tạm, có thể còn sửa đổi) và `transcript-final` (text đã chốt, không đổi nữa). UI nhận được các event này đúng thứ tự thời gian phát sinh, làm nền cho việc hiển thị phụ đề sau này (M5).

## Kịch bản & luồng người dùng

### Luồng chính

**Given** phiên capture đang chạy và pipeline VAD → STT đang xử lý audio
**When** một segment thoại được STT nhận diện ra văn bản
**Then** Core phát event `transcript-final` (hoặc `transcript-partial` nếu có giai đoạn tạm) cho UI, mang nội dung văn bản và thứ tự đúng với thời điểm segment phát sinh trong audio

### Kịch bản chấp nhận

1. **Given** audio mẫu có nhiều segment thoại liên tiếp, **When** chạy qua pipeline, **Then** UI (hoặc listener test) nhận được đúng số lượng event `transcript-final`, theo đúng thứ tự thời gian các segment, nội dung khớp với text STT trả ra.
2. **Given** một segment thoại đang được xử lý, **When** STT có kết quả tạm trước khi chốt, **Then** UI nhận được event `transcript-partial` trước, sau đó nhận `transcript-final` cho cùng segment đó.
3. **Given** audio mẫu toàn im lặng (không có segment thoại), **When** chạy qua pipeline, **Then** không có event `transcript-partial`/`transcript-final` nào được phát, không báo lỗi.

### Trường hợp biên

- STT không hỗ trợ kết quả tạm (chỉ trả 1 lần duy nhất khi xong) — hệ thống vẫn phát được `transcript-final`, có thể bỏ qua `transcript-partial` cho segment đó.
- Hai segment liên tiếp xử lý nhanh, gần như đồng thời — mỗi segment vẫn có event riêng, không gộp nhầm nội dung giữa 2 segment.
- Text rỗng từ STT (segment quá ngắn/không rõ) — vẫn phát event `transcript-final` với text rỗng, không bị bỏ qua lặng lẽ, không crash.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI phát event Tauri `transcript-final` cho UI mỗi khi một segment thoại được STT xử lý xong và có văn bản chốt.
- **FR-002**: Hệ thống PHẢI phát event Tauri `transcript-partial` cho UI khi có văn bản tạm (chưa chốt) cho một segment, trước khi event `transcript-final` tương ứng được phát.
- **FR-003**: Các event PHẢI được UI nhận đúng thứ tự thời gian phát sinh thực tế của các segment trong audio nguồn.
- **FR-004**: Mỗi event PHẢI mang đủ thông tin để UI xác định segment nào đang được cập nhật (ví dụ chỉ số/khoảng thời gian của segment) và nội dung văn bản tại thời điểm đó.
- **FR-005**: Với audio không có segment thoại nào, hệ thống PHẢI không phát event nào và không báo lỗi.

## Tiêu chí thành công

- **SC-001**: Với audio mẫu nhiều segment đã biết trước nội dung, số lượng và thứ tự event `transcript-final` nhận được khớp đúng số lượng và thứ tự segment thoại trong audio.
- **SC-002**: Với mọi segment có phát `transcript-partial`, event đó luôn đến trước `transcript-final` cùng segment, không bị đảo thứ tự.
- **SC-003**: Toàn bộ test (nhiều segment, một segment, toàn im lặng, text rỗng) chạy xong không crash, không event bị thiếu hoặc trùng.

## Thực thể chính

- **TranscriptEvent**: đại diện cho payload của `transcript-partial`/`transcript-final`, gồm chỉ số/khoảng thời gian segment và nội dung văn bản tại thời điểm phát event.

## Giả định

- Kế thừa luồng VAD → STT đã có từ PBI 13, không thay đổi logic nhận diện giọng nói.
- Nếu `SpeechToText` hiện tại (whisper.cpp local, PBI 12) không có khả năng trả kết quả tạm theo thời gian thực, việc phát `transcript-partial` chỉ cần thỏa khi có sẵn kết quả trung gian hợp lý (ví dụ: chưa đủ điều kiện in-place streaming thật, miễn thứ tự partial → final vẫn đúng); không bắt buộc đổi mô hình nhận diện.
- Kiểm thử bằng audio mẫu/file đã biết trước nội dung, chưa yêu cầu chạy trên luồng capture mic/loopback thời gian thực.

## Ngoài phạm vi

- Hiển thị phụ đề thật trên cửa sổ overlay (thuộc M5).
- Xử lý lỗi runtime model chi tiết (thiếu model file, lỗi load/infer) — PBI riêng kế tiếp trong M3.
- Dịch (Translation) nội dung — thuộc M4.
- Chạy pipeline trên luồng audio capture thời gian thực liên tục (chưa phải mục tiêu PBI này).
