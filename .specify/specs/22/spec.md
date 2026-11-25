# Đặc tả tính năng: Auto-scroll & fade dòng phụ đề cũ

**Mã PBI**: 22
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Cửa sổ phụ đề (PBI 20, PBI 21) hiện chỉ hiển thị 1 dòng — mỗi event `subtitle-ready` mới thay thế hoàn toàn dòng cũ. Tính năng này giữ lại một số dòng phụ đề gần nhất, tự động cuộn lên khi có dòng mới, và làm mờ dần (fade) các dòng cũ hơn, giúp người xem theo kịp mạch nói mà không bị mất ngữ cảnh câu trước.

## Kịch bản & luồng người dùng

### Luồng chính

Given cửa sổ phụ đề đang mở và đã hiển thị một số dòng, When Core phát event `subtitle-ready` mới, Then dòng mới xuất hiện ở vị trí dưới cùng, các dòng cũ tự động cuộn lên trên và mờ dần theo độ cũ, dòng vượt quá giới hạn hiển thị bị loại bỏ khỏi khung nhìn.

### Kịch bản chấp nhận

1. **Given** cửa sổ phụ đề đang trống, **When** nhận event `subtitle-ready` đầu tiên, **Then** dòng đó hiển thị rõ nét (không mờ) ở vị trí mới nhất.
2. **Given** cửa sổ đang hiển thị N dòng (N nhỏ hơn giới hạn tối đa), **When** nhận thêm 1 event mới, **Then** dòng mới thêm vào cuối, các dòng cũ cuộn lên và dòng càng cũ càng mờ hơn.
3. **Given** cửa sổ đã hiển thị đủ số dòng tối đa cho phép, **When** nhận thêm 1 event mới, **Then** dòng cũ nhất bị loại khỏi khung nhìn, các dòng còn lại cuộn lên một bậc, tổng số dòng hiển thị không vượt quá giới hạn.
4. **Given** nhiều event `subtitle-ready` liên tục dồn dập, **When** cửa sổ nhận lần lượt, **Then** thứ tự các dòng hiển thị đúng theo thứ tự event phát ra (dòng phát sau luôn nằm dưới/mới hơn dòng phát trước).

### Trường hợp biên

- Event có text dịch rỗng → không thêm dòng trống vào danh sách cuộn.
- Text 1 dòng quá dài tràn chiều rộng cửa sổ → dòng tự xuống hàng (wrap) trong khung, không phá vỡ layout các dòng khác.
- Cửa sổ vừa mở, chưa có dòng nào → không có hiệu ứng cuộn/mờ bất thường khi dòng đầu tiên xuất hiện.

## Yêu cầu chức năng

- **FR-001**: Cửa sổ phụ đề PHẢI giữ lại tối đa N dòng phụ đề gần nhất (N là một số cố định hợp lý cho khung nhìn, ví dụ 3), thay vì chỉ 1 dòng như hiện tại.
- **FR-002**: Khi có event `subtitle-ready` mới, hệ thống PHẢI thêm dòng mới vào vị trí mới nhất và tự động cuộn các dòng hiện có lên trên.
- **FR-003**: Hệ thống PHẢI làm mờ dần (giảm độ rõ) các dòng cũ hơn theo thứ tự thời gian — dòng càng cũ độ mờ càng cao.
- **FR-004**: Khi số dòng vượt quá giới hạn N, hệ thống PHẢI loại bỏ dòng cũ nhất khỏi khung nhìn.
- **FR-005**: Hệ thống PHẢI giữ đúng thứ tự thời gian của các dòng hiển thị theo đúng thứ tự event được phát ra.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Với chuỗi liên tiếp nhiều hơn N event `subtitle-ready`, cửa sổ luôn hiển thị đúng N dòng gần nhất theo đúng thứ tự, không đảo thứ tự, không sót dòng mới.
- **SC-002**: Dòng mới nhất luôn hiển thị rõ nét (độ mờ thấp nhất); các dòng cũ hơn có độ mờ tăng dần quan sát được bằng mắt.
- **SC-003**: Khi số dòng vượt giới hạn N, dòng cũ nhất biến mất khỏi khung nhìn ngay trong lần cập nhật kế tiếp.

## Giả định

- Giới hạn số dòng hiển thị (N) là một hằng số cố định trong phiên bản này, chưa cần cấu hình từ UI (việc cho người dùng chỉnh số dòng để lại cho M6 — Điều khiển & cấu hình).
- Chỉ áp dụng cho text dịch đang hiển thị hiện tại (kế thừa phạm vi PBI 21), không mở rộng sang hiển thị song song text gốc.

## Ngoài phạm vi

- Cho phép người dùng tuỳ chỉnh số dòng hiển thị hoặc tốc độ fade (để dành M6 — cấu hình).
- Kéo-thả di chuyển cửa sổ, resize/chỉnh cỡ chữ, ẩn-hiện bằng phím tắt (các mục roadmap M5 riêng khác).
