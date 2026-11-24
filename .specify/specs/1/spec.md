# Đặc tả tính năng: Chốt OS mục tiêu

**Mã PBI**: 1
**Ngày tạo**: 2026-11-24
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Dự án Sora RealSub cần quyết định hệ điều hành mục tiêu (Windows trước, macOS sau, hay cả hai song song) trước khi thiết kế các trait Core (`AudioSource`, v.v.) vì cơ chế bắt âm thanh hệ thống (loopback) khác nhau giữa Windows (WASAPI) và macOS (BlackHole/ScreenCaptureKit). Quyết định này là nền tảng cho toàn bộ roadmap M1 trở đi.

## Kịch bản & luồng người dùng

### Luồng chính

Given nhóm phát triển chưa chốt OS mục tiêu, When xem xét ràng buộc kỹ thuật (API loopback audio, công cụ build, đối tượng người dùng), Then nhóm phát triển ghi lại quyết định OS mục tiêu và thứ tự triển khai vào tài liệu dự án để các PBI sau (M1 Audio Capture) dựa vào.

### Kịch bản chấp nhận

1. **Given** chưa có quyết định OS trong tài liệu dự án, **When** hoàn tất PBI này, **Then** `docs/` hoặc wiki có ghi rõ: OS mục tiêu đầu tiên, có làm OS thứ hai không và khi nào.
2. **Given** quyết định đã chốt là "Windows trước", **When** một PBI M1 (audio capture) được lập kế hoạch, **Then** kế hoạch chỉ định rõ dùng WASAPI loopback, không cần trừu tượng hoá đa-OS ngay.

### Trường hợp biên

- Nếu sau này cần hỗ trợ OS thứ hai, quyết định phải nêu rõ đây là mở rộng sau (không block M1–M8), tránh over-engineering trait Core ngay từ đầu.

## Yêu cầu chức năng

- **FR-001**: Tài liệu dự án PHẢI ghi rõ OS mục tiêu được triển khai đầu tiên.
- **FR-002**: Tài liệu dự án PHẢI nêu rõ có dự định hỗ trợ OS thứ hai hay không, và nếu có, ở giai đoạn nào (M-nào hoặc Backlog).
- **FR-003**: Quyết định PHẢI dựa trên ít nhất một lý do kỹ thuật hoặc người dùng cụ thể (ví dụ: đối tượng dùng Windows chiếm đa số, hoặc API loopback Windows sẵn có hơn).

## Tiêu chí thành công

- **SC-001**: Người đọc tài liệu dự án (ví dụ thành viên mới) xác định được OS mục tiêu đầu tiên trong vòng 1 phút đọc.
- **SC-002**: Các PBI thuộc M1 trở đi có thể bắt đầu lập kế hoạch kỹ thuật mà không cần hỏi lại "làm OS nào trước".

## Giả định

- Đối tượng người dùng ban đầu của Sora RealSub (họp/stream) chủ yếu dùng Windows, nên Windows được chọn làm OS mục tiêu đầu tiên; macOS triển khai sau, thuộc Backlog (`roadmap.md` mục "Hỗ trợ macOS").
- Tauri 2 hỗ trợ build đa nền tảng nên quyết định chỉ ảnh hưởng thứ tự ưu tiên, không loại trừ macOS vĩnh viễn.

## Ngoài phạm vi

- Thiết kế chi tiết cơ chế loopback audio cho từng OS (thuộc PBI M1).
- Hỗ trợ Linux (không nằm trong roadmap hiện tại).
