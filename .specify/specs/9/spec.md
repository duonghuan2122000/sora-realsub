# Đặc tả tính năng: Phát hiện hoạt động giọng nói (VAD)

**Mã PBI**: 9
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Hệ thống cần phân biệt được đoạn nào trong luồng âm thanh thu được (mic + loopback đã trộn từ PBI 7) là có người nói và đoạn nào là im lặng/nhiễu nền, làm nền tảng để sau này chỉ gửi các đoạn có giọng nói đi nhận dạng (STT). Tính năng này tích hợp một model phát hiện hoạt động giọng nói (VAD) để gán nhãn speech/silence cho từng frame audio.

## Kịch bản & luồng người dùng

### Luồng chính

1. Hệ thống nhận một buffer audio PCM (từ nguồn thu âm đã có).
2. Hệ thống chia buffer thành các frame có độ dài cố định.
3. Với mỗi frame, hệ thống chạy model VAD và gán nhãn "speech" hoặc "silence".
4. Nhãn của từng frame được trả về theo đúng thứ tự thời gian của frame trong buffer gốc.

### Kịch bản chấp nhận

1. **Given** một đoạn audio chỉ chứa giọng nói rõ ràng, **When** chạy qua VAD, **Then** phần lớn frame được gán nhãn "speech".
2. **Given** một đoạn audio chỉ chứa im lặng/nhiễu nền nhẹ, **When** chạy qua VAD, **Then** phần lớn frame được gán nhãn "silence".
3. **Given** một đoạn audio có nói + lặng xen kẽ, **When** chạy qua VAD, **Then** nhãn trả về phản ánh đúng thứ tự xen kẽ đó theo thời gian.
4. **Given** một buffer audio rỗng hoặc quá ngắn (ngắn hơn 1 frame), **When** chạy qua VAD, **Then** hệ thống trả về kết quả rỗng hoặc báo lỗi rõ ràng, không crash.

### Trường hợp biên

- Audio có độ ồn nền liên tục (quạt, điều hoà) nhưng không có giọng nói: không được gán nhãn "speech" tràn lan.
- Buffer có độ dài không chia hết cho kích thước frame: frame cuối thiếu dữ liệu phải được xử lý nhất quán (ví dụ pad hoặc bỏ qua), không gây lỗi.
- Gọi VAD liên tục nhiều buffer nối tiếp nhau không bị leak tài nguyên qua thời gian.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI nhận một buffer audio PCM và chia thành các frame có độ dài cố định.
- **FR-002**: Hệ thống PHẢI gán nhãn "speech" hoặc "silence" cho từng frame dựa trên model VAD.
- **FR-003**: Hệ thống PHẢI trả về nhãn của các frame theo đúng thứ tự thời gian xuất hiện trong buffer gốc.
- **FR-004**: Hệ thống PHẢI xử lý buffer rỗng hoặc ngắn hơn 1 frame mà không crash.

## Tiêu chí thành công

- **SC-001**: Với file audio mẫu có đoạn nói rõ ràng, ít nhất 90% frame trong đoạn đó được gán nhãn "speech".
- **SC-002**: Với file audio mẫu chỉ có im lặng/nhiễu nền nhẹ, ít nhất 90% frame được gán nhãn "silence".
- **SC-003**: Với file audio mẫu có nói + lặng xen kẽ, thứ tự nhãn trả về khớp đúng với thứ tự các đoạn nói/lặng đã biết trước trong file mẫu (kiểm tra bằng test tự động).

## Giả định

- Dùng lại nguồn audio PCM đã có từ `MixedAudioSource` (PBI 7), chưa cần nguồn mới.
- Model VAD cụ thể (ví dụ Silero VAD) được chọn ở bước lập kế hoạch kỹ thuật (`/sora-plan`), spec này chỉ yêu cầu khả năng gán nhãn speech/silence theo frame, không chốt công nghệ.
- Độ dài frame và ngưỡng phân loại speech/silence là tham số có thể cấu hình ở tầng kỹ thuật, không ảnh hưởng tới hành vi chức năng mô tả ở đây.

## Ngoài phạm vi

- Cắt buffer liên tục thành các segment theo khoảng lặng (thuộc PBI tiếp theo trong M2).
- Phát event Tauri `vad-segment` ra UI (thuộc PBI tiếp theo trong M2).
- STT, dịch (thuộc M3, M4).
