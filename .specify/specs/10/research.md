# Research: Cắt audio thành segment theo khoảng lặng (PBI 10)

## Quyết định: Thuật toán cắt segment dựa trên hangover (silence hangover)

**Quyết định**: Dùng thuật toán state machine đơn giản chạy tuần tự qua `FrameLabel` (từ `vad::detect`, PBI 9): giữ 1 bộ đếm số frame "silence" liên tiếp kể từ frame speech cuối. Khi đang trong segment và bộ đếm silence liên tiếp vượt ngưỡng `min_silence_frames`, đóng segment tại frame speech cuối cùng (không tính các frame silence hangover vào segment). Khi đang ngoài segment và gặp frame speech, mở segment mới bắt đầu tại frame đó.

**Lý do**: Không cần thêm dependency nào — xử lý thuần trên `Vec<FrameLabel>` đã có. Thuật toán hangover là cách chuẩn, đơn giản để tránh cắt gãy câu do VAD nhiễu/ngắt hơi ngắn (tương đương "trường hợp biên" trong spec.md: khoảng lặng ngắn giữa câu không bị cắt).

**Phương án khác đã xem xét**:
- Cắt ngay khi gặp 1 frame silence đầu tiên: bị loại vì sẽ cắt gãy câu liên tục có ngắt hơi ngắn (vi phạm kịch bản chấp nhận #3 trong spec).
- Dùng crate chuyên dụng cho segmentation: không cần, vì bài toán chỉ là quét tuyến tính 1 lần qua danh sách nhãn đã có, không có logic phức tạp đáng để thêm dependency.

## Quyết định: Tham số cấu hình

**Quyết định**: `min_silence_frames: usize` (số frame VAD liên tiếp phải là "silence" để coi là điểm cắt), mặc định tương ứng khoảng 300ms silence (frame VAD 512 sample @ 16kHz ≈ 32ms/frame → ~9-10 frame). Hàm mặc định `segment(frames: &[FrameLabel]) -> Vec<Segment>` dùng default, có biến thể `segment_with_min_silence(frames, min_silence_frames)` để test/tùy biến tương tự cách `vad.rs` đã làm với `detect`/`detect_with_threshold`.

**Lý do**: Giữ convention đã có trong `vad.rs` (hàm mặc định + hàm `_with_*` cho test/tùy biến), không cần cấu hình phức tạp ở mức này (tầng UI/config thuộc M6).

**Phương án khác đã xem xét**: Tính ngưỡng theo milliseconds thay vì số frame — phức tạp hơn không cần thiết vì input đã là `FrameLabel` rời rạc theo frame cố định (512 sample), quy đổi ra ms chỉ cần nhân với `frame_duration` khi cần hiển thị, không ảnh hưởng logic cắt.

## Quyết định: Dữ liệu trả về của Segment

**Quyết định**: `Segment { start_frame_index: usize, end_frame_index: usize }` — chỉ lưu range chỉ số frame (không copy lại audio PCM ở PBI này), vì `FrameLabel` đã giữ `frame_index` tham chiếu đúng thứ tự.

**Lý do**: Spec chỉ yêu cầu xác định "thời điểm bắt đầu/kết thúc", chưa yêu cầu trích xuất PCM thực tế đi kèm — việc đó (ghép PCM theo segment để gửi STT) là chi tiết triển khai của bước ghép pipeline sau. Giữ tối giản, test vẫn kiểm được qua test tích hợp với file WAV mẫu bằng cách đối chiếu frame_index với nhãn gốc.
