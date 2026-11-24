# Nghiên cứu kỹ thuật: PBI 8 — Lệnh start_capture/stop_capture

**Quyết định**: Dùng `tauri::State` quản lý 1 `Mutex<Option<StreamHandle>>` làm trạng thái phiên thu âm toàn cục trong app Tauri.
**Lý do**: `StreamHandle` (đã có từ PBI 3/7) đóng gói sẵn `stop_fn` để giải phóng thread/handle khi gọi `.stop()`. Giữ `Option<StreamHandle>` cho phép biết có phiên đang chạy hay không (FR-003, FR-004) mà không cần thêm cấu trúc mới.
**Phương án khác đã xem xét**: Dùng `AtomicBool` riêng cho trạng thái "đang chạy" + `StreamHandle` riêng — bị loại vì tạo 2 nguồn sự thật (source of truth) có thể lệch nhau giữa 2 lần gọi liên tiếp (race).

**Quyết định**: Nguồn thu âm cho lệnh `start_capture` là `MixedAudioSource::new(Box::new(MicAudioSource::new(...)), Box::new(LoopbackAudioSource::new(...)))`, tái dùng nguyên trạng từ PBI 7.
**Lý do**: Spec PBI 8 chỉ yêu cầu thêm lớp điều khiển (start/stop) phía trên nguồn đã có, không đổi nguồn.
**Phương án khác**: Không có — PBI 7 đã chốt cách trộn.

**Quyết định**: Expose 2 lệnh Tauri: `start_capture() -> Result<(), String>` và `stop_capture() -> Result<(), String>`, lỗi trả về dạng chuỗi mô tả (theo convention `list_input_devices_cmd` đã có trong `lib.rs`).
**Lý do**: Giữ nhất quán với command hiện có trong `lib.rs`, UI (Vue) chỉ cần `invoke('start_capture')` / `invoke('stop_capture')`.
**Phương án khác**: Trả về `CoreError` tuần tự hoá (serde) — chưa cần vì `CoreError` chưa derive `Serialize`; để tối thiểu, dùng `String` như command cũ.

**Quyết định**: Kiểm thử "không leak thread/handle" bằng cách đếm số thread sống của process qua crate đã có sẵn trong workspace (nếu không có crate đo thread, dùng kỹ thuật đơn giản: `AtomicUsize` đếm số cài đặt `AudioSource` test đang "mở" tăng khi `start()` được gọi, giảm khi `stop_fn` chạy) ở mức unit test với mock `AudioSource`, không phụ thuộc thiết bị âm thanh thật.
**Lý do**: Test phải chạy được trên CI không có thiết bị âm thanh; đo qua mock tránh phụ thuộc phần cứng, vẫn xác minh đúng hợp đồng "start cấp 1 resource, stop giải phóng đúng 1 resource đó".
**Phương án khác**: Dùng crate ngoài để đếm thread OS thật — loại vì thêm dependency không cần thiết cho mục tiêu kiểm thử hợp đồng logic.
