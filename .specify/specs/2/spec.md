# Đặc tả tính năng: Chốt cặp ngôn ngữ mặc định

**Mã PBI**: 2
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Sora RealSub cần dịch phụ đề thời gian thực từ ngôn ngữ nói sang ngôn ngữ đích. Trước khi thiết kế trait `Translator` và luồng STT→Dịch (M3, M4), nhóm phát triển cần chốt cặp ngôn ngữ mặc định (nguồn → đích) mà app dùng khi người dùng chưa tự chọn, làm nền cho các PBI sau.

## Kịch bản & luồng người dùng

### Luồng chính

Given người dùng mở app lần đầu và chưa cấu hình cặp ngôn ngữ, When người dùng bắt đầu một phiên phụ đề, Then app dùng cặp ngôn ngữ mặc định đã chốt (nguồn → đích) mà không yêu cầu chọn trước.

### Kịch bản chấp nhận

1. **Given** chưa có cấu hình ngôn ngữ nào được lưu, **When** người dùng bắt đầu phiên đầu tiên, **Then** app nhận diện giọng nói bằng ngôn ngữ nguồn mặc định và hiển thị phụ đề dịch bằng ngôn ngữ đích mặc định.
2. **Given** cặp ngôn ngữ mặc định đã chốt trong tài liệu dự án, **When** một PBI M3/M4 (STT, Translator) được lập kế hoạch, **Then** kế hoạch dùng đúng mã ngôn ngữ (theo chuẩn BCP-47) đã chốt làm giá trị mặc định/fallback.

### Trường hợp biên

- Nếu người dùng nói ngôn ngữ khác ngôn ngữ nguồn mặc định, hệ thống vẫn cố nhận diện theo cấu hình hiện tại (không tự động phát hiện ngôn ngữ) — khả năng tự nhận diện đa ngôn ngữ thuộc phạm vi khác.
- Việc người dùng tự đổi cặp ngôn ngữ qua UI thuộc M6, không thuộc PBI này.

## Yêu cầu chức năng

- **FR-001**: Tài liệu dự án PHẢI ghi rõ ngôn ngữ nguồn mặc định và ngôn ngữ đích mặc định, theo mã chuẩn BCP-47 (ví dụ `en-US`, `vi-VN`).
- **FR-002**: Cặp ngôn ngữ mặc định PHẢI được dùng làm giá trị khởi tạo (fallback) khi chưa có cấu hình người dùng nào được lưu.
- **FR-003**: Quyết định PHẢI nêu lý do chọn (ví dụ: đối tượng người dùng mục tiêu, mức độ sẵn có của provider STT/Dịch cho cặp ngôn ngữ đó).

## Tiêu chí thành công

- **SC-001**: Người đọc tài liệu dự án xác định được cặp ngôn ngữ mặc định (nguồn, đích) trong vòng 1 phút đọc.
- **SC-002**: Các PBI thuộc M3–M4 có thể bắt đầu lập kế hoạch kỹ thuật mà không cần hỏi lại "ngôn ngữ mặc định là gì".

## Giả định

- Đối tượng người dùng ban đầu của Sora RealSub nghe nội dung tiếng Anh (họp/stream quốc tế) và cần phụ đề tiếng Việt, nên chọn cặp mặc định: nguồn `en-US` (English), đích `vi-VN` (Vietnamese) — đúng như gợi ý trong roadmap.md.
- Cặp ngôn ngữ mặc định chỉ là giá trị khởi tạo; người dùng đổi được qua UI (M6), không bị khoá cứng.

## Ngoài phạm vi

- Giao diện chọn/đổi cặp ngôn ngữ (thuộc M6).
- Tự động phát hiện ngôn ngữ nói (không nằm trong roadmap hiện tại).
- Danh sách đầy đủ ngôn ngữ được hỗ trợ (phụ thuộc provider STT/Dịch chọn ở M3/M4).
