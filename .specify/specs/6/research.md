# Nghiên cứu: Capture loopback âm thanh hệ thống (PBI 6)

- **Quyết định**: Dùng crate `wasapi` (bọc Windows Core Audio API) để mở loopback capture stream từ thiết bị output mặc định.
  **Lý do**: `cpal` (dùng cho mic ở PBI 5) không expose cờ `AUDCLNT_STREAMFLAGS_LOOPBACK`, nên không thể capture được âm thanh đang phát ra loa qua cpal; `wasapi` hỗ trợ loopback trực tiếp, là cách chuẩn trên Windows.
  **Phương án khác đã xem xét**: Tự gọi WASAPI qua `windows`/`windows-sys` crate thủ công — đúng cơ chế nhưng lặp lại công việc `wasapi` đã làm sẵn, không cần thiết ở bước này.

- **Quyết định**: Chỉ capture thiết bị output mặc định của hệ thống, không cho chọn thiết bị cụ thể.
  **Lý do**: Khớp phạm vi đã chốt trong spec.md; đơn giản cho bước đầu, việc chọn thiết bị cụ thể không có nhu cầu rõ ràng ngay.
  **Phương án khác đã xem xét**: Liệt kê + cho chọn thiết bị output như đã làm với mic (PBI 4) — để lại PBI sau nếu thực tế cần.

- **Quyết định**: Chuẩn hoá mẫu về `f32` mono trong `AudioFrame.samples`, dùng cùng cách convert như `MicAudioSource` (`to_mono`).
  **Lý do**: Khớp định nghĩa `AudioFrame` đã có từ PBI 3, tái dùng logic đã kiểm chứng ở PBI 5.
  **Phương án khác đã xem xét**: Giữ định dạng gốc rồi convert ở tầng sau — bị loại vì phân tán logic convert, khó test tại nguồn.

Không còn điểm `NEEDS CLARIFICATION`.
