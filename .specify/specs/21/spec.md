# Đặc tả tính năng: Hiển thị phụ đề

**Mã PBI**: 21
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Cửa sổ phụ đề nổi (PBI 20) hiện đang trống, chưa hiển thị nội dung gì. Tính năng này làm cho cửa sổ hiển thị text gốc + text dịch mỗi khi có event `subtitle-ready` phát ra từ Core, để người dùng xem được phụ đề thời gian thực trong lúc họp/stream.

## Kịch bản & luồng người dùng

### Luồng chính

Given cửa sổ phụ đề đang mở và phiên dịch đang chạy, When Core phát event `subtitle-ready` chứa text gốc và text dịch, Then cửa sổ phụ đề hiển thị ngay nội dung đó cho người dùng đọc.

### Kịch bản chấp nhận

1. **Given** cửa sổ phụ đề đang mở, **When** nhận 1 event `subtitle-ready` đầu tiên, **Then** text dịch (và text gốc nếu cấu hình hiển thị cả hai) xuất hiện trên cửa sổ.
2. **Given** cửa sổ phụ đề đang hiển thị 1 dòng, **When** nhận thêm event `subtitle-ready` kế tiếp, **Then** nội dung được cập nhật sang dòng mới, không hiển thị chồng lẫn dòng cũ/mới.
3. **Given** nhiều event `subtitle-ready` liên tục dồn dập, **When** cửa sổ nhận lần lượt, **Then** mỗi event đều được hiển thị đúng thứ tự phát ra (không bị đảo thứ tự, không bị mất event).

### Trường hợp biên

- Event `subtitle-ready` có text dịch rỗng (ví dụ lỗi dịch) → cửa sổ không hiển thị dòng trống gây khó chịu, hoặc hiển thị rõ trạng thái không có nội dung.
- Cửa sổ phụ đề mở sau khi phiên đã chạy một lúc → không cần hiển thị lại lịch sử cũ, chỉ hiển thị từ event tiếp theo trở đi.
- Text phụ đề quá dài tràn khỏi khung nhìn cửa sổ → không vỡ layout, không tràn ra ngoài màn hình.

## Yêu cầu chức năng

- **FR-001**: Cửa sổ phụ đề PHẢI lắng nghe event `subtitle-ready` phát từ Core trong suốt thời gian cửa sổ còn mở.
- **FR-002**: Cửa sổ phụ đề PHẢI hiển thị nội dung text dịch nhận được từ event `subtitle-ready` mới nhất.
- **FR-003**: Cửa sổ phụ đề PHẢI cập nhật nội dung hiển thị mỗi khi có event `subtitle-ready` mới, thay thế nội dung cũ.
- **FR-004**: Hệ thống PHẢI giữ đúng thứ tự hiển thị các event theo đúng thứ tự event được phát ra.
- **FR-005**: Cửa sổ phụ đề PHẢI không bị vỡ layout khi text dài hơn chiều rộng cửa sổ.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Khi có 1 event `subtitle-ready` phát ra, nội dung tương ứng xuất hiện trên cửa sổ phụ đề trong vòng dưới 1 giây.
- **SC-002**: Với chuỗi 20 event `subtitle-ready` liên tục phát cách nhau dưới 1 giây, cửa sổ hiển thị đúng 100% nội dung theo đúng thứ tự, không bỏ sót, không đảo thứ tự.
- **SC-003**: Text phụ đề dài (ví dụ 200 ký tự) hiển thị không bị tràn ra ngoài khung cửa sổ.

## Giả định

- Cửa sổ phụ đề đã tồn tại (PBI 20) và chỉ cần bổ sung phần hiển thị nội dung động.
- Phiên bản này chỉ hiển thị text dịch (không yêu cầu đồng thời hiển thị text gốc) — việc hiển thị song song gốc/dịch (nếu cần) để lại cho tính năng cấu hình sau (M6).
- Chỉ hiển thị 1 dòng phụ đề hiện tại tại một thời điểm; auto-scroll/fade nhiều dòng cũ là tính năng riêng (mục kế tiếp trong roadmap M5).

## Ngoài phạm vi

- Auto-scroll/fade dòng cũ khi có dòng mới (mục roadmap riêng, làm sau).
- Kéo-thả di chuyển cửa sổ, resize/chỉnh cỡ chữ, ẩn-hiện bằng phím tắt (các mục roadmap M5 riêng, làm sau).
- Cấu hình hiển thị song song text gốc + text dịch (để dành M6).
