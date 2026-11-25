# Đặc tả tính năng: Kéo-thả cửa sổ phụ đề

**Mã PBI**: 23
**Ngày tạo**: 2026-11-25
**Trạng thái**: Sẵn sàng lập kế hoạch

## Mô tả tổng quan

Người dùng cần di chuyển cửa sổ phụ đề nổi (always-on-top, không viền, không title bar — PBI 20) đến vị trí thuận tiện trên màn hình bằng cách kéo-thả, vì cửa sổ không có title bar để kéo theo cách thông thường. Vị trí mới phải được giữ nguyên trong suốt phiên làm việc hiện tại.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** cửa sổ phụ đề đang hiển thị ở vị trí A, **When** người dùng nhấn giữ chuột trái lên vùng cửa sổ và kéo đến vị trí B rồi thả, **Then** cửa sổ di chuyển theo con trỏ trong lúc kéo và dừng lại đúng tại vị trí B khi thả.

### Kịch bản chấp nhận

1. **Given** cửa sổ phụ đề đang mở, **When** người dùng kéo-thả cửa sổ sang vị trí khác trên màn hình, **Then** cửa sổ nằm đúng vị trí mới sau khi thả chuột.
2. **Given** cửa sổ đã được kéo đến vị trí mới, **When** có phụ đề mới xuất hiện (event `subtitle-ready`) hoặc danh sách dòng phụ đề cuộn/fade (PBI 21, 22), **Then** cửa sổ vẫn giữ đúng vị trí đã kéo, không tự nhảy về vị trí cũ.
3. **Given** người dùng chỉ click chuột (không di chuyển) lên cửa sổ, **When** thả chuột tại cùng vị trí, **Then** cửa sổ không bị dịch chuyển.

### Trường hợp biên

- Kéo cửa sổ ra ngoài biên màn hình: cửa sổ vẫn di chuyển theo con trỏ, không bị chặn cứng (hệ điều hành tự xử lý giới hạn nếu có).
- Kéo rất nhanh (nhiều sự kiện chuột liên tiếp): cửa sổ không bị giật/lag thấy rõ, vị trí cuối cùng vẫn đúng với điểm thả.
- Thả chuột ngoài vùng cửa sổ trong lúc kéo: hành động kéo vẫn kết thúc đúng, cửa sổ dừng ở vị trí tương ứng con trỏ lúc thả.

## Yêu cầu chức năng

- **FR-001**: Người dùng PHẢI có thể bắt đầu kéo cửa sổ phụ đề bằng cách nhấn giữ chuột trái tại bất kỳ điểm nào trên vùng hiển thị của cửa sổ (trừ các control khác nếu có trong tương lai).
- **FR-002**: Hệ thống PHẢI di chuyển cửa sổ theo vị trí con trỏ chuột trong suốt quá trình kéo.
- **FR-003**: Hệ thống PHẢI giữ cửa sổ tại vị trí vừa thả ngay khi người dùng nhả chuột.
- **FR-004**: Hệ thống PHẢI giữ vị trí cửa sổ không đổi khi nội dung phụ đề bên trong cập nhật (dòng mới, cuộn, fade).
- **FR-005**: Hệ thống KHÔNG được di chuyển cửa sổ khi người dùng chỉ click (nhấn rồi thả tại cùng điểm) mà không kéo.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Người dùng kéo-thả cửa sổ phụ đề đến vị trí mới và thấy cửa sổ nằm đúng vị trí đó ngay khi thả chuột, trong một lần thử.
- **SC-002**: Sau khi kéo đến vị trí mới, cửa sổ giữ nguyên vị trí đó trong suốt phần còn lại của phiên làm việc (không tự dịch chuyển khi phụ đề cập nhật liên tục).

## Giả định

- Vị trí cửa sổ chỉ cần giữ trong phiên làm việc hiện tại (runtime); lưu vị trí lâu dài qua các lần mở app thuộc M6 ("Lưu cấu hình... vị trí cửa sổ"), không thuộc phạm vi PBI này.
- Cửa sổ phụ đề hiện chưa có control/nút nào khác bên trong (theo PBI 20-22), nên toàn bộ vùng cửa sổ dùng để kéo.

## Ngoài phạm vi

- Lưu/khôi phục vị trí cửa sổ giữa các lần khởi động app (M6).
- Giới hạn vị trí cửa sổ trong biên màn hình (snap, không cho kéo ra ngoài).
- Resize cửa sổ hoặc chỉnh cỡ chữ (mục riêng trong M5).
