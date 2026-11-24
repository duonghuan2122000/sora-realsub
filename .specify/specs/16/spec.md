# Đặc tả tính năng: Dịch local offline

**Mã PBI**: 16
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Người dùng cần phụ đề được dịch sang ngôn ngữ đích ngay khi họp/stream, không phụ thuộc kết nối mạng. Tính năng này bổ sung khả năng dịch văn bản (từ transcript gốc) sang ngôn ngữ đích bằng model chạy hoàn toàn offline trên máy người dùng.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** app đã có một đoạn văn bản transcript gốc (ví dụ câu tiếng Anh) và cặp ngôn ngữ nguồn-đích đã chọn,
**When** đoạn văn bản đó được đưa vào bộ dịch,
**Then** hệ thống trả về văn bản đã dịch sang ngôn ngữ đích, không cần kết nối Internet.

### Kịch bản chấp nhận

1. **Given** câu đầu vào cố định ở ngôn ngữ nguồn, **When** gọi dịch, **Then** kết quả trả về là câu đã dịch đúng nghĩa sang ngôn ngữ đích, lặp lại nhiều lần cho cùng câu đầu vào cho kết quả giống nhau.
2. **Given** máy không có kết nối Internet, **When** gọi dịch, **Then** việc dịch vẫn hoàn tất bình thường.
3. **Given** văn bản đầu vào rỗng hoặc chỉ chứa khoảng trắng, **When** gọi dịch, **Then** hệ thống không dịch và trả về kết quả rỗng/tương đương, không phát sinh lỗi crash.

### Trường hợp biên

- Model dịch chưa được tải/cài sẵn trên máy → hệ thống báo lỗi rõ ràng, không crash toàn bộ pipeline.
- Văn bản đầu vào chứa ký tự đặc biệt, số, tên riêng → không crash, trả về bản dịch tốt nhất có thể.
- Văn bản đầu vào rất dài (nhiều câu dồn lại) → hệ thống vẫn trả về kết quả, không treo quá lâu.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI dịch một đoạn văn bản từ ngôn ngữ nguồn sang ngôn ngữ đích đã cấu hình, hoàn toàn offline (không gọi dịch vụ dịch qua mạng).
- **FR-002**: Hệ thống PHẢI trả về lỗi rõ ràng (không crash) khi model dịch chưa sẵn sàng (thiếu file model, lỗi tải model).
- **FR-003**: Hệ thống PHẢI xử lý văn bản đầu vào rỗng/khoảng trắng mà không gây lỗi.
- **FR-004**: Với cùng một câu đầu vào và cùng cặp ngôn ngữ, hệ thống PHẢI cho kết quả dịch ổn định (không ngẫu nhiên khác nhau giữa các lần gọi).

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Dịch đúng nghĩa một câu cố định đã biết trước (so khớp với bản dịch tham chiếu), kiểm tra bằng test tự động.
- **SC-002**: Toàn bộ quá trình dịch hoàn tất mà không cần kết nối Internet.
- **SC-003**: Không có trường hợp dịch làm crash ứng dụng, kể cả khi model lỗi hoặc đầu vào bất thường (rỗng, dài, ký tự đặc biệt).

## Giả định

- Cặp ngôn ngữ mặc định đã chốt ở M0 (ví dụ English → Vietnamese) được dùng làm cặp ngôn ngữ thử nghiệm chính cho PBI này.
- PBI này chỉ cung cấp khả năng dịch độc lập (nhận văn bản, trả văn bản dịch); việc ghép vào luồng STT final → `subtitle-ready` là phạm vi của mục roadmap kế tiếp (M4, mục 3), không thuộc PBI này.
- Việc ghép ngữ cảnh (gửi kèm câu trước để dịch chính xác hơn) cũng là mục roadmap kế tiếp (M4, mục 2), không thuộc PBI này.

## Ngoài phạm vi

- Ghép Translator vào pipeline STT → subtitle (thuộc PBI/mục roadmap sau).
- Dịch có ngữ cảnh nhiều câu liên tiếp (thuộc PBI/mục roadmap sau).
- Giao diện người dùng chọn/đổi cặp ngôn ngữ (thuộc M6).
