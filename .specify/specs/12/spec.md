# Đặc tả tính năng: Nhận diện giọng nói local (STT)

**Mã PBI**: 12
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Hệ thống chuyển một đoạn audio (segment đã cắt từ VAD) thành văn bản, chạy hoàn toàn local trên máy người dùng bằng model nhận diện giọng nói (không gọi dịch vụ cloud). Đây là bước khởi đầu milestone M3, cung cấp khả năng `SpeechToText` làm nền cho việc ghép vào luồng VAD → STT → dịch sau này.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** có một file/đoạn audio mẫu chứa giọng nói
**When** hệ thống đưa đoạn audio đó vào bộ nhận diện giọng nói local
**Then** hệ thống trả về văn bản (text) khớp đúng nội dung đã nói trong đoạn audio

### Kịch bản chấp nhận

1. **Given** một file audio mẫu tiếng nói rõ, ngắn (vài giây), **When** chạy qua STT, **Then** nhận được text đúng nội dung đã nói (cho phép sai khác nhỏ về dấu câu/hoa-thường).
2. **Given** một file audio mẫu dài hơn (vài chục giây, nhiều câu), **When** chạy qua STT, **Then** nhận được text đầy đủ, đúng thứ tự nội dung đã nói.
3. **Given** một file audio mẫu gần như im lặng/không có giọng nói, **When** chạy qua STT, **Then** hệ thống trả về text rỗng hoặc gần rỗng, không báo lỗi runtime.

### Trường hợp biên

- Audio mẫu có nhiễu nền nhẹ — STT vẫn phải chạy xong và trả về kết quả (không yêu cầu chính xác tuyệt đối).
- Audio mẫu rất ngắn (dưới 1 giây) — hệ thống không crash, trả về text rỗng hoặc kết quả hợp lý.
- Định dạng/sample rate audio đầu vào không khớp yêu cầu model — hệ thống cần xử lý hoặc báo lỗi rõ ràng, không crash âm thầm.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI cung cấp khả năng nhận diện giọng nói (`SpeechToText`) chạy bằng model local trên máy người dùng, không gửi audio ra ngoài qua mạng.
- **FR-002**: Hệ thống PHẢI nhận đầu vào là một đoạn audio (buffer hoặc file mẫu) và trả về văn bản tương ứng với nội dung giọng nói trong đoạn đó.
- **FR-003**: Với đoạn audio không có giọng nói (im lặng), hệ thống PHẢI trả về kết quả rỗng/gần rỗng, không được báo lỗi hay crash.
- **FR-004**: Kết quả văn bản trả về PHẢI giữ đúng thứ tự nội dung đã nói trong đoạn audio.

## Tiêu chí thành công

- **SC-001**: Với file audio mẫu đã biết trước nội dung (ground truth), text nhận được khớp đúng nội dung ở mức đọc hiểu được, không sai lệch nghĩa.
- **SC-002**: Toàn bộ các file audio mẫu dùng để test (gồm cả giọng rõ, giọng dài, gần im lặng) đều cho ra kết quả mà không gây crash hoặc lỗi không xử lý.
- **SC-003**: Quá trình nhận diện 1 đoạn audio mẫu (vài giây đến vài chục giây) hoàn tất trong thời gian hợp lý để có thể dùng tiếp theo thời gian thực sau này (không yêu cầu đo số cụ thể ở PBI này, ghi nhận là giả định cho M3 sau).

## Giả định

- PBI này chỉ xây dựng và kiểm thử khả năng `SpeechToText` độc lập bằng file audio mẫu, CHƯA ghép vào luồng capture/VAD thời gian thực (thuộc PBI sau trong M3: "Ghép STT vào luồng").
- Ngôn ngữ đầu vào audio mẫu test theo cặp ngôn ngữ mặc định đã chốt ở M0.
- Việc lựa chọn model/thư viện STT cụ thể (whisper.cpp, faster-whisper, hay tương đương) là quyết định kỹ thuật, thuộc phạm vi `/sora-plan`, không mô tả ở đây.

## Ngoài phạm vi

- Ghép STT vào luồng end-to-end từ VAD segment thời gian thực (PBI kế tiếp trong M3).
- Phát event Tauri `transcript-partial`/`transcript-final` cho UI (PBI kế tiếp trong M3).
- Xử lý lỗi runtime model chi tiết (thiếu model file, lỗi load/infer) — thuộc PBI riêng trong M3.
- Dịch (Translation) nội dung — thuộc M4.
