# Đặc tả tính năng: Bắt âm thanh loopback đa hệ điều hành (Windows + Linux)

**Mã PBI**: 18
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Hiện tại tính năng bắt âm thanh hệ thống (loopback) chỉ hoạt động trên Windows, khiến việc build/test trên Linux bị chặn hoàn toàn. Tính năng này cho phép người dùng và lập trình viên chạy được ứng dụng — bao gồm khả năng bắt âm thanh loopback — trên cả Windows và Linux, để Sora RealSub thực sự hỗ trợ song song hai hệ điều hành như đã chốt ở M0.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** người dùng chạy Sora RealSub trên máy Linux (có PulseAudio hoặc PipeWire), **when** người dùng bắt đầu một phiên phụ đề có bật nguồn âm thanh hệ thống, **then** ứng dụng bắt được âm thanh phát ra từ hệ thống (loopback) trên Linux giống như trên Windows, không cần cấu hình thủ công ngoài những gì hệ thống Linux đã hỗ trợ sẵn.

**Given** lập trình viên clone source code trên máy Linux, **when** họ chạy `cargo build` hoặc `cargo test`, **then** quá trình build/test chạy được bình thường, không bị lỗi vì phụ thuộc chỉ dành riêng cho Windows.

### Kịch bản chấp nhận

1. **Given** máy Linux có PipeWire hoặc PulseAudio đang chạy, **when** người dùng khởi động capture loopback, **then** file .wav ghi ra chứa đúng âm thanh hệ thống đã phát trong lúc ghi (nghe lại nhận ra nội dung).
2. **Given** máy Windows, **when** người dùng khởi động capture loopback như trước đây, **then** hành vi không thay đổi so với hiện tại (không bị ảnh hưởng bởi thay đổi cho Linux).
3. **Given** mã nguồn được build trên máy Linux không cài wasapi (vì đây là thư viện Windows-only), **when** chạy `cargo build`/`cargo test`, **then** build/test thành công, không có lỗi biên dịch liên quan đến thư viện chỉ dành cho Windows.

### Trường hợp biên

- Trên Linux không có PulseAudio/PipeWire chạy, hoặc không có monitor source nào khả dụng: ứng dụng phải báo lỗi rõ ràng (không crash, không treo) cho người dùng biết nguồn loopback không khả dụng.
- Trên Linux, thiết bị loopback (monitor source) bị rút/tắt giữa phiên ghi: ứng dụng dừng ghi nguồn đó một cách an toàn, không crash toàn bộ tiến trình.
- Build trên macOS hoặc OS khác chưa được hỗ trợ: không bắt buộc phải build thành công (ngoài phạm vi M0 hiện tại), nhưng không được vô tình phá build Windows/Linux.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI cho phép build và chạy test thành công trên Linux mà không cần thư viện chỉ hỗ trợ Windows.
- **FR-002**: Hệ thống PHẢI giữ nguyên hành vi bắt âm thanh loopback hiện có trên Windows sau khi thay đổi.
- **FR-003**: Hệ thống PHẢI cung cấp khả năng bắt âm thanh loopback trên Linux, cho ra kết quả âm thanh tương đương về nội dung với nguồn phát hệ thống trong lúc ghi.
- **FR-004**: Hệ thống PHẢI báo lỗi rõ ràng cho người dùng khi không tìm được nguồn loopback khả dụng trên Linux, thay vì im lặng thất bại hoặc crash.
- **FR-005**: Hệ thống PHẢI dùng cùng một giao diện (trait) bắt âm thanh cho cả hai hệ điều hành, để phần còn lại của pipeline (VAD, STT, dịch) không cần biết đang chạy trên OS nào.

## Tiêu chí thành công

- **SC-001**: Trên máy Linux có PipeWire/PulseAudio, 100% số lần chạy thử capture loopback với âm thanh test cố định cho ra file ghi âm có thể nghe lại nhận đúng nội dung đã phát.
- **SC-002**: Trên máy Windows, toàn bộ test hiện có liên quan đến capture loopback vẫn pass không có thay đổi kết quả (0 regression).
- **SC-003**: Trên máy Linux không có thư viện Windows-only, lệnh build/test chạy xong trong một lần, không có lỗi biên dịch liên quan đến hệ điều hành.

## Giả định

- Máy Linux dùng để test có PulseAudio hoặc PipeWire (PipeWire thường đi kèm lớp tương thích Pulse) đang chạy — đây là cấu hình phổ biến trên các distro desktop hiện đại.
- Phạm vi M0 chỉ yêu cầu Windows + Linux; macOS tiếp tục nằm ở Backlog, không thuộc phạm vi tính năng này.
- "Loopback" trên Linux được hiểu là ghi lại âm thanh đang được phát ra loa/thiết bị output của hệ thống (tương đương khái niệm loopback trên Windows), không phải ghi từ một thiết bị input vật lý riêng.

## Ngoài phạm vi

- Hỗ trợ macOS (BlackHole/ScreenCaptureKit) — đã có mục riêng trong Backlog.
- Giao diện cấu hình cho người dùng chọn thủ công monitor source cụ thể trên Linux (nếu có nhiều output) — thuộc phạm vi M6 (Control UI), không xử lý ở đây.
- Đóng gói installer Linux (AppImage/.deb) — thuộc M8.
