# Đặc tả tính năng: Hợp đồng giao diện Core

**Mã PBI**: 3
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Core (Rust/Tauri) cần một bộ hợp đồng (interface) rõ ràng để các thành phần xử lý âm thanh, nhận dạng giọng nói, dịch và hiển thị phụ đề có thể phát triển độc lập, thay thế lẫn nhau (cloud/local) mà không ảnh hưởng phần còn lại của hệ thống. PBI này chỉ định nghĩa hợp đồng — chưa cài đặt logic thật.

## Kịch bản & luồng người dùng

### Luồng chính

Given lập trình viên cần thêm một nguồn âm thanh, một provider nhận dạng giọng nói, một provider dịch hoặc một nơi xuất phụ đề mới,
When họ cài đặt (implement) đúng hợp đồng tương ứng,
Then thành phần mới hoạt động được trong pipeline hiện có mà không cần sửa code ở nơi khác.

### Kịch bản chấp nhận

1. **Given** hợp đồng `AudioSource` đã định nghĩa, **When** một cài đặt giả (stub/mock) implement hợp đồng này, **Then** nó biên dịch thành công và có thể được dùng thay cho cài đặt thật trong pipeline.
2. **Given** hợp đồng `SpeechToText` đã định nghĩa, **When** một cài đặt giả nhận vào đoạn âm thanh và trả ra văn bản giả, **Then** phần gọi hợp đồng này không cần biết cài đặt cụ thể là cloud hay local.
3. **Given** hợp đồng `Translator` đã định nghĩa, **When** một cài đặt giả nhận văn bản nguồn và trả văn bản dịch giả, **Then** phần gọi không cần biết provider dịch cụ thể.
4. **Given** hợp đồng `SubtitleSink` đã định nghĩa, **When** một cài đặt giả nhận phụ đề đã sẵn sàng, **Then** phần gọi không cần biết đích hiển thị cụ thể (overlay UI, file, log...).

### Trường hợp biên

- Một cài đặt hợp đồng xử lý bất đồng bộ (streaming) chậm hoặc lỗi giữa chừng — hợp đồng phải mô tả được cách báo lỗi/kết thúc luồng, không chỉ trường hợp thành công.
- Một hợp đồng có thể có nhiều cài đặt chạy đồng thời (ví dụ mic + loopback đều là `AudioSource`) — hợp đồng không được giả định chỉ có một thực thể duy nhất.

## Yêu cầu chức năng

- **FR-001**: Core PHẢI định nghĩa hợp đồng `AudioSource` mô tả khả năng cung cấp dữ liệu âm thanh liên tục (dạng streaming) cho pipeline.
- **FR-002**: Core PHẢI định nghĩa hợp đồng `SpeechToText` mô tả khả năng nhận một đoạn âm thanh và trả về văn bản nhận dạng được, hỗ trợ cả kết quả tạm (partial) và kết quả chốt (final).
- **FR-003**: Core PHẢI định nghĩa hợp đồng `Translator` mô tả khả năng nhận văn bản nguồn (kèm ngữ cảnh câu trước nếu có) và trả về văn bản đích.
- **FR-004**: Core PHẢI định nghĩa hợp đồng `SubtitleSink` mô tả khả năng nhận một phụ đề đã sẵn sàng (gốc + dịch) để đưa tới nơi hiển thị/lưu trữ.
- **FR-005**: Mỗi hợp đồng PHẢI độc lập với bất kỳ provider cụ thể (cloud hay local) — không rò rỉ chi tiết riêng của một provider vào chữ ký hợp đồng.
- **FR-006**: Mỗi hợp đồng PHẢI mô tả được đường lỗi (error path), không chỉ đường thành công.
- **FR-007**: PBI này KHÔNG yêu cầu cài đặt (implement) logic thật cho bất kỳ hợp đồng nào — chỉ định nghĩa interface.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Tồn tại 4 hợp đồng (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) biên dịch thành công trong Core, không có cài đặt logic thật.
- **SC-002**: Với mỗi hợp đồng, có ít nhất một cài đặt giả (stub) biên dịch và chạy được trong test, chứng minh hợp đồng dùng được thực tế.
- **SC-003**: Đổi một cài đặt giả bằng một cài đặt giả khác của cùng hợp đồng không yêu cầu sửa code ở nơi gọi hợp đồng đó.

## Giả định

- Các hợp đồng được định nghĩa dưới dạng `trait` Rust, tuân theo kiến trúc đã chốt trong CLAUDE.md (Core tách biệt UI).
- Giai đoạn này không cần quyết định chi tiết kiểu dữ liệu âm thanh/văn bản cuối cùng — chỉ cần đủ rõ để biên dịch và test bằng cài đặt giả; chi tiết kỹ thuật cụ thể (kiểu async, kiểu lỗi...) thuộc về `/sora-plan`.
- Hướng xử lý đã chốt là local (theo roadmap M0), nên hợp đồng cần tương thích để cài đặt bằng thư viện local (ví dụ whisper.cpp, NLLB) mà không cần sửa lại interface.

## Ngoài phạm vi

- Cài đặt thật cho `SpeechToText`/`Translator` (cloud hoặc local) — thuộc các PBI sau (M3, M4).
- Capture âm thanh thật (mic/loopback) — thuộc M1.
- Cửa sổ phụ đề UI thật — thuộc M5.
