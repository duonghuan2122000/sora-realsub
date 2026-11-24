# Đặc tả tính năng: Ghép STT vào luồng VAD → Transcript

**Mã PBI**: 13
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Nối khả năng `SpeechToText` (PBI 12) vào luồng VAD đang chạy: mỗi segment thoại VAD cắt ra được đưa qua STT, kết quả văn bản được thu thập lại theo đúng thứ tự thời gian. Đây là bước ghép pipeline end-to-end đầu tiên của M3, biến hai khả năng độc lập (VAD cắt segment, STT nhận diện) thành một luồng xử lý liên tục.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** một file audio mẫu chứa nhiều đoạn nói xen lặng
**When** audio được đưa qua pipeline VAD → STT
**Then** hệ thống trả về danh sách văn bản tương ứng với từng segment thoại, đúng thứ tự xuất hiện trong audio

### Kịch bản chấp nhận

1. **Given** file audio mẫu có 2-3 đoạn nói rõ, xen các khoảng lặng, **When** chạy qua pipeline VAD → STT, **Then** nhận được đúng số lượng đoạn text tương ứng, đúng thứ tự, nội dung khớp đoạn đã nói.
2. **Given** file audio mẫu chỉ có 1 đoạn nói duy nhất, **When** chạy qua pipeline, **Then** nhận được đúng 1 đoạn text tương ứng.
3. **Given** file audio mẫu toàn im lặng (không có đoạn thoại nào), **When** chạy qua pipeline, **Then** không có đoạn text nào được sinh ra, không báo lỗi.

### Trường hợp biên

- Segment thoại rất ngắn (gần ngưỡng tối thiểu VAD) — pipeline vẫn chạy qua STT, không crash, có thể trả text rỗng.
- Hai segment thoại liền nhau gần như không có khoảng lặng giữa — mỗi segment vẫn được gửi STT riêng biệt, không bị gộp sai hoặc rớt.
- Audio mẫu dài, nhiều segment liên tiếp — thứ tự văn bản trả về phải khớp đúng thứ tự thời gian các segment, không bị đảo lẫn.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI nhận mỗi segment thoại do VAD cắt ra và đưa qua `SpeechToText` để nhận diện thành văn bản.
- **FR-002**: Hệ thống PHẢI giữ đúng thứ tự thời gian giữa các đoạn văn bản trả về, tương ứng với thứ tự các segment thoại trong audio nguồn.
- **FR-003**: Với audio không có segment thoại nào (toàn im lặng), hệ thống PHẢI không sinh ra văn bản nào và không báo lỗi.
- **FR-004**: Luồng xử lý PHẢI hoạt động end-to-end từ một file/nguồn audio mẫu đầu vào đến danh sách văn bản đầu ra, không cần can thiệp tay ở bước giữa.

## Tiêu chí thành công

- **SC-001**: Với file audio mẫu đã biết trước nội dung từng đoạn nói (ground truth), danh sách văn bản trả về khớp đúng số lượng đoạn và đúng thứ tự.
- **SC-002**: Nội dung từng đoạn văn bản khớp đúng nghĩa với đoạn thoại tương ứng, ở mức đọc hiểu được.
- **SC-003**: Toàn bộ test end-to-end (nhiều đoạn, một đoạn, toàn im lặng) đều chạy xong không crash, không lỗi không xử lý.

## Giả định

- Kế thừa khả năng `SpeechToText` đã có từ PBI 12 (whisper.cpp local), không đổi model/thư viện ở PBI này.
- Đầu vào vẫn là file audio mẫu để test end-to-end, CHƯA yêu cầu chạy trên luồng capture mic/loopback thời gian thực.
- Việc thiết kế cơ chế truyền segment (hàng đợi, callback, hay gọi trực tiếp) là quyết định kỹ thuật, thuộc phạm vi `/sora-plan`.

## Ngoài phạm vi

- Phát event Tauri `transcript-partial`/`transcript-final` cho UI (PBI kế tiếp trong M3).
- Xử lý lỗi runtime model chi tiết (thiếu model file, lỗi load/infer) — PBI riêng trong M3.
- Chạy pipeline trên luồng audio capture thời gian thực liên tục (chưa phải mục tiêu PBI này).
- Dịch (Translation) nội dung — thuộc M4.
