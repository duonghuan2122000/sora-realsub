# Nghiên cứu: PBI 26 — Màn hình chính chọn thiết bị

## Quyết định 1: Nguồn danh sách "loopback"

- **Quyết định**: Dùng danh sách thiết bị **output** (phát âm thanh) do `cpal` liệt kê làm danh sách "nguồn loopback" để chọn, tương tự cách `list_input_devices` đã liệt kê mic. Capture loopback thật (PBI 6/18) hiện luôn bám theo thiết bị output **mặc định** của hệ điều hành — lựa chọn ở màn hình này ghi nhận ý định của người dùng (và dùng được ngay cho việc hiển thị/validate), còn việc truyền device đã chọn cụ thể vào `LoopbackAudioSource` (hiện đang hard-code thiết bị default) là phần mở rộng lõi capture, **ngoài phạm vi PBI này** — ghi chú lại trong Rủi ro & ngoại lệ ở plan.md.
- **Lý do**: Tái dùng đúng pattern/trait `AudioHost` đã có (`input_devices()` → thêm `output_devices()`), tránh phá vỡ `LoopbackAudioSource`/`LoopbackAudioSourceLinux` đang hoạt động ổn định, giữ diff nhỏ nhất theo đúng kiến trúc Core/UI đã tách biệt.
- **Phương án khác đã xem xét**: Viết enumerator loopback riêng theo OS (WASAPI render-device loopback list / PulseAudio sink-monitor list) ngay trong PBI này — bị loại vì tăng phạm vi đáng kể (phải sửa cả `LoopbackAudioSource` theo OS) so với yêu cầu thực tế của spec (chỉ cần UI chọn + validate, chưa yêu cầu capture theo thiết bị cụ thể).

## Quyết định 2: Lưu lựa chọn

- **Quyết định**: Lưu lựa chọn mic + loopback trong state Vue (reactive, trong phiên làm việc hiện tại), không persist ra file.
- **Lý do**: Spec đã ghi rõ "Ngoài phạm vi": lưu cấu hình lâu dài thuộc M6 mục khác.
- **Phương án khác đã xem xét**: Lưu vào `localStorage` ngay — bị loại vì không được yêu cầu, thêm trạng thái cần đồng bộ ngoài phạm vi spec.

## Quyết định 3: Phát hiện thiết bị không còn khả dụng

- **Quyết định**: Khi mở/làm mới màn hình, so khớp `id` thiết bị đã chọn trước đó với danh sách mới trả về từ `list_input_devices_cmd`/`list_output_devices_cmd`; nếu không còn trong danh sách → xoá lựa chọn đó và hiển thị cảnh báo.
- **Lý do**: Không cần thêm cơ chế lắng nghe sự kiện hot-plug ở OS (phức tạp, không được yêu cầu) — việc so khớp khi tải lại danh sách đủ để thoả FR-005 (báo lỗi rõ ràng khi thiết bị không còn khả dụng).
- **Phương án khác đã xem xét**: Lắng nghe sự kiện thiết bị hệ điều hành (device added/removed) theo thời gian thực — bị loại, không có trong `cpal`/hạ tầng hiện tại và vượt phạm vi cần thiết.
