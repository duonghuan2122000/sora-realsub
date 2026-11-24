# Đặc tả tính năng: Xử lý lỗi runtime model STT

**Mã PBI**: 15
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Hiện tại nếu model STT (whisper.cpp) thiếu file hoặc lỗi khi nạp/khi infer, lỗi bị âm thầm bỏ qua (`.ok()`) hoặc chỉ in ra console (`eprintln!`) — người dùng vận hành app không hề biết STT đã dừng hoạt động, phụ đề đơn giản không xuất hiện mà không có lý do. Tính năng này làm cho các lỗi runtime của model STT được phát hiện, phân loại rõ, và báo cho UI/người dùng bằng thông báo dễ hiểu, thay vì thất bại âm thầm.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng bấm Start để bắt đầu phiên capture
**When** model STT không nạp được (thiếu file / file hỏng) hoặc lỗi xảy ra khi infer một đoạn audio
**Then** hệ thống phát một event lỗi runtime model cho UI, mang thông tin đủ để hiển thị thông báo rõ ràng cho người dùng (loại lỗi, mô tả ngắn), và phiên capture vẫn tiếp tục chạy (không crash ứng dụng)

### Kịch bản chấp nhận

1. **Given** biến môi trường/đường dẫn model STT trỏ tới file không tồn tại, **When** người dùng bấm Start, **Then** UI (hoặc listener test) nhận được event lỗi runtime model với loại lỗi "thiếu model file", phiên capture vẫn start thành công (VAD/audio vẫn chạy).
2. **Given** model nạp thành công nhưng quá trình infer một segment thoại gặp lỗi (ví dụ dữ liệu audio làm whisper.cpp lỗi nội bộ), **When** segment đó được xử lý, **Then** UI nhận event lỗi runtime model với loại lỗi "lỗi infer", kèm segment liên quan; các segment tiếp theo vẫn được xử lý bình thường (không dừng toàn bộ pipeline).
3. **Given** model nạp và infer đều thành công, **When** chạy pipeline bình thường, **Then** không có event lỗi runtime model nào được phát sinh.

### Trường hợp biên

- Model nạp lỗi ngay từ đầu phiên (trước khi có segment thoại nào) — vẫn phải phát được đúng 1 event lỗi loại "thiếu model file"/"lỗi nạp model", không chờ tới khi có audio.
- Nhiều segment liên tiếp đều lỗi infer — mỗi segment lỗi phát 1 event riêng, không gộp nhầm hoặc làm treo pipeline.
- Lỗi infer xảy ra giữa chừng rồi các segment sau lại thành công — hệ thống phải tiếp tục phát `transcript-final`/`transcript-partial` bình thường cho các segment sau đó.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI phát một event lỗi runtime model cho UI khi model STT không nạp được khi bắt đầu phiên capture (ví dụ thiếu file, file hỏng).
- **FR-002**: Hệ thống PHẢI phát một event lỗi runtime model cho UI khi quá trình infer một segment thoại cụ thể gặp lỗi, mà không làm dừng toàn bộ phiên capture.
- **FR-003**: Mỗi event lỗi PHẢI phân loại được nguyên nhân (ít nhất 2 loại: lỗi nạp model vs. lỗi infer) và mang một mô tả ngắn, dễ hiểu cho người dùng cuối (không phải thông báo lỗi kỹ thuật thô từ thư viện).
- **FR-004**: Khi model STT không khả dụng (do lỗi nạp), hệ thống PHẢI vẫn cho phép phiên capture/VAD chạy bình thường (không phát `transcript-partial`/`transcript-final`, chỉ báo lỗi một lần cho người dùng biết STT đang tắt).
- **FR-005**: Khi infer một segment lỗi, hệ thống PHẢI tiếp tục xử lý các segment thoại tiếp theo bình thường (lỗi cục bộ không lan ra toàn pipeline).

## Tiêu chí thành công

- **SC-001**: Với model path không tồn tại, người vận hành nhận được thông báo lỗi rõ ràng ngay khi bắt đầu phiên, thay vì phụ đề im lặng không rõ nguyên nhân.
- **SC-002**: Với một segment audio gây lỗi infer giả lập, hệ thống báo đúng 1 lỗi cho segment đó và vẫn xử lý đúng các segment hợp lệ còn lại trong cùng phiên.
- **SC-003**: Toàn bộ test giả lập các case lỗi (thiếu model, lỗi nạp, lỗi infer, xen giữa segment thành công) chạy xong không crash ứng dụng, không event bị thiếu hoặc trùng.

## Thực thể chính

- **ModelErrorEvent**: payload của event lỗi runtime model, gồm loại lỗi (nạp model / infer), mô tả ngắn cho người dùng, và thông tin segment liên quan nếu có (trường hợp lỗi infer).

## Giả định

- Kế thừa `WhisperSpeechToText`/`CoreError::Provider` đã có từ PBI 12–14, không đổi engine STT.
- "Thông báo thân thiện cho user" ở phạm vi PBI này là nội dung text rõ nghĩa trong event gửi cho UI; việc thiết kế UI hiển thị thông báo (toast/dialog) thuộc M6/M8, không bắt buộc trong PBI này.
- Kiểm thử bằng giả lập lỗi (đường dẫn sai, mock STT lỗi), không cần tạo ra lỗi thật từ thiết bị phần cứng.

## Ngoài phạm vi

- Xử lý lỗi toàn cục khác (crash audio device, mất mạng) — thuộc M8.
- Thiết kế UI hiển thị thông báo lỗi (toast/màn hình) — thuộc M6/M8.
- Tự động retry/tự phục hồi khi model lỗi — không yêu cầu trong PBI này.
