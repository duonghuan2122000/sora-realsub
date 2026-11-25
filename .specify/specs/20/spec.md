# Đặc tả tính năng: Cửa sổ phụ đề nổi always-on-top

**Mã PBI**: 20
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Hiện tại hệ thống đã phát ra sự kiện `subtitle-ready` mang câu gốc + câu dịch, nhưng chưa có nơi nào hiển thị chúng cho người dùng trong lúc họp/stream. Tính năng này tạo một cửa sổ riêng, nổi luôn ở trên các ứng dụng khác (always-on-top), không có khung/viền hay thanh tiêu đề, dùng để hiển thị phụ đề thời gian thực mà không che khuất hoặc gây xao nhãng với ứng dụng họp/stream đang dùng.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng đã mở app và bắt đầu một phiên capture (hoặc chuẩn bị bắt đầu),
**When** cửa sổ phụ đề được tạo/hiển thị,
**Then** cửa sổ đó nổi luôn ở trên mọi cửa sổ khác trên màn hình (bao gồm cửa sổ ứng dụng họp/stream đang active), không có viền hay thanh tiêu đề, và không chiếm quyền focus bàn phím của ứng dụng người dùng đang dùng.

### Kịch bản chấp nhận

1. **Given** app đang chạy, **When** cửa sổ phụ đề được tạo, **Then** cửa sổ hiển thị không có border, không có title bar, và không có các nút minimize/maximize/close mặc định của hệ điều hành.
2. **Given** cửa sổ phụ đề đang hiển thị, **When** người dùng chuyển sang/click vào một ứng dụng khác (ví dụ ứng dụng họp), **Then** cửa sổ phụ đề vẫn hiển thị nổi ở trên ứng dụng đó, không bị ẩn xuống dưới.
3. **Given** cửa sổ phụ đề đang hiển thị nổi always-on-top, **When** người dùng click vào nội dung phía sau cửa sổ phụ đề (ví dụ nút trong ứng dụng họp nằm dưới vị trí cửa sổ phụ đề), **Then** cửa sổ phụ đề không chặn việc đó theo cách gây khó chịu quá mức — ở mức tối thiểu, cửa sổ phụ đề không tự động giành focus bàn phím khi xuất hiện.
4. **Given** app khởi động, **When** chưa có phiên capture nào chạy, **Then** cửa sổ phụ đề có thể đã tồn tại nhưng không hiển thị nội dung phụ đề nào (trống hoặc ẩn) — chưa yêu cầu nội dung hiển thị, vị trí/kéo-thả, hay resize (các phần đó thuộc các mục sau trong M5).

### Trường hợp biên

- Người dùng có nhiều màn hình (multi-monitor) → cửa sổ phụ đề vẫn always-on-top trên màn hình nó đang nằm, không bắt buộc hiển thị trên tất cả màn hình.
- Hệ điều hành/window manager không hỗ trợ always-on-top (hiếm, một số môi trường Linux) → cửa sổ vẫn được tạo và hiển thị bình thường (không crash), dù hành vi always-on-top có thể không được đảm bảo tuyệt đối do giới hạn của window manager.
- Đóng app hoặc dừng phiên capture → cửa sổ phụ đề đóng theo cùng vòng đời với cửa sổ chính, không để lại tiến trình/cửa sổ "ma" còn tồn tại.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI tạo một cửa sổ riêng biệt (khác cửa sổ điều khiển chính) dùng để hiển thị phụ đề.
- **FR-002**: Cửa sổ phụ đề PHẢI không có viền (border) và không có thanh tiêu đề (title bar) của hệ điều hành.
- **FR-003**: Cửa sổ phụ đề PHẢI luôn hiển thị nổi ở trên các cửa sổ ứng dụng khác (always-on-top) trong suốt thời gian nó đang mở.
- **FR-004**: Cửa sổ phụ đề PHẢI không tự động giành focus bàn phím của hệ thống khi được tạo hoặc khi nội dung của nó thay đổi.
- **FR-005**: Cửa sổ phụ đề PHẢI đóng cùng lúc khi ứng dụng chính đóng, không để lại handle/tiến trình bị leak.
- **FR-006**: Tính năng này KHÔNG bắt buộc phải hiển thị nội dung phụ đề thật, xử lý kéo-thả, resize, hay cỡ chữ — các yêu cầu đó thuộc các mục tiếp theo của milestone M5.

## Tiêu chí thành công

- **SC-001**: Khi app khởi động, cửa sổ phụ đề xuất hiện trong vòng 2 giây và không có bất kỳ viền/thanh tiêu đề nào quan sát được bằng mắt.
- **SC-002**: Trong một phiên thử nghiệm kéo dài ít nhất 5 phút với việc chuyển đổi qua lại giữa ứng dụng chính và ít nhất 2 ứng dụng khác, cửa sổ phụ đề không bị ẩn xuống dưới bất kỳ ứng dụng nào trong 100% số lần chuyển đổi được thử.
- **SC-003**: Sau khi đóng ứng dụng, không còn cửa sổ hoặc tiến trình liên quan đến cửa sổ phụ đề tồn tại trên hệ thống (kiểm tra bằng danh sách process/window của OS).

## Giả định

- Cửa sổ phụ đề áp dụng cho cả Windows và Linux (2 OS mục tiêu hiện tại của dự án), dùng cơ chế always-on-top sẵn có của Tauri 2 (window builder), không cần mã riêng theo OS ở mức tính năng này.
- "Không chiếm focus bàn phím" được hiểu ở mức: cửa sổ không tự ý đưa con trỏ/focus nhập liệu vào nó khi xuất hiện hoặc cập nhật nội dung; không yêu cầu cửa sổ phải "click-through" (xuyên qua click) hoàn toàn — hành vi click-through, nếu cần, sẽ là một mục riêng sau này nếu có nhu cầu thực tế.
- Vị trí và kích thước mặc định của cửa sổ khi mới tạo có thể là một vị trí/kích thước cố định hợp lý (ví dụ góc dưới màn hình); việc cho người dùng kéo-thả/resize và nhớ lại vị trí thuộc các mục tiếp theo trong M5, không thuộc phạm vi PBI này.

## Ngoài phạm vi

- Hiển thị nội dung phụ đề thật từ sự kiện `subtitle-ready` (mục tiếp theo trong M5).
- Auto-scroll/fade dòng cũ, kéo-thả di chuyển, resize/chỉnh cỡ chữ, phím tắt ẩn-hiện (các mục riêng tiếp theo trong M5).
- Hành vi click-through xuyên qua cửa sổ phụ đề để tương tác với ứng dụng phía dưới.
