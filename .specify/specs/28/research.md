# Nghiên cứu: Nút Start/Stop phiên dịch phụ đề (PBI 28)

## Quyết định 1: Tái dùng lệnh Tauri có sẵn

- **Quyết định**: Dùng trực tiếp `start_capture` / `stop_capture` (đã có ở `app/src-tauri/src/lib.rs`, quản lý qua `CaptureSessionState`) — không tạo lệnh mới.
- **Lý do**: Core đã có đủ logic start/stop (mix mic+loopback, VAD, STT, dịch) và đã trả lỗi rõ ràng (`AlreadyRunning`, `NotRunning` → `String` qua `to_string()`). UI chỉ cần gọi và phản ánh kết quả.
- **Phương án khác đã xem xét**: Thêm lệnh `toggle_capture` gộp start/stop — bị bỏ vì làm UI khó biết chính xác đang ở trạng thái nào trước khi gọi, dễ gây nhầm lẫn hơn 2 lệnh tách biệt.

## Quyết định 2: Quản lý trạng thái nút bằng 4 giá trị, không chỉ boolean

- **Quyết định**: State máy ở frontend gồm `idle` (Start, sẵn sàng) → `starting` (Đang chạy..., khoá) → `running` (Stop, sẵn sàng) → `stopping` (Đang dừng..., khoá) → `idle`.
- **Lý do**: `invoke()` là async — giữa lúc bấm và lúc có phản hồi, nút phải khoá để không gửi lệnh trùng (FR-002, SC-003). Dùng boolean đơn (`isRunning`) không biểu diễn được trạng thái "đang chờ".
- **Phương án khác**: Disable nút bằng cờ `isLoading` riêng cộng `isRunning` — tương đương nhưng rối hơn so với 1 enum duy nhất; chọn enum cho dễ test & đọc.

## Quyết định 3: Lỗi từ Core chỉ xử lý qua kết quả `invoke()` (catch reject)

- **Quyết định**: Khi `invoke('start_capture')` hoặc `invoke('stop_capture')` reject, hiển thị message lỗi và đưa trạng thái về `idle` (nếu đang `starting`) hoặc giữ nguyên `running` (nếu `stop` lỗi do `NotRunning` — trường hợp hiếm, coi như đã dừng, về `idle`).
- **Lý do**: Core hiện tại trả lỗi start/stop đồng bộ qua `Result<(), String>`, không có event riêng báo "phiên đã dừng do lỗi runtime" (model-error chỉ báo lỗi STT/dịch, không dừng phiên — phiên vẫn `running`). Vì vậy case "Core tự dừng phiên do crash" ngoài phạm vi kỹ thuật hiện có của Core, không giả định event đó tồn tại.
- **Phương án khác**: Lắng nghe thêm 1 event mới `capture-stopped`/`capture-error` do Core phát khi crash — cần sửa Core (audio source) để phát event này, vượt phạm vi PBI 28 (chỉ thêm UI). Ghi nhận làm cải tiến sau nếu cần.

Không còn điểm `NEEDS CLARIFICATION`.
