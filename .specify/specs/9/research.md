# Nghiên cứu kỹ thuật: PBI 9 — VAD

## Quyết định: Thư viện VAD

- **Quyết định**: dùng crate Rust `voice_activity_detector` (wrapper Silero VAD v5, chạy ONNX runtime nội bộ qua `ort`, model đã nhúng sẵn trong crate).
- **Lý do**: khỏi phải tự tải/ quản lý file model ONNX, tự viết binding ONNX runtime; crate expose API đơn giản nhận `&[f32]`/`&[i16]` theo chunk cố định (ví dụ 512 sample @16kHz) trả về xác suất speech. Phù hợp hướng "chưa cần build custom pipeline ONNX" ở giai đoạn này.
- **Phương án khác đã xem xét**:
  - Tự tích hợp `ort` + tải model Silero VAD `.onnx` thủ công: nhiều việc hơn, phải tự quản lý resample 16kHz, tự viết sliding window — không cần thiết khi đã có crate làm sẵn.
  - WebRTC VAD (qua crate `webrtc-vad`): nhẹ hơn nhưng độ chính xác thấp hơn Silero theo cộng đồng, và yêu cầu audio 8/16kHz mono cố định, ít linh hoạt hơn cho mở rộng sau.

## Quyết định: Sample rate & kích thước frame

- **Quyết định**: resample input về 16kHz mono trước khi đưa vào VAD (nếu nguồn chưa đúng rate), frame cố định 512 sample (~32ms @16kHz) theo yêu cầu của Silero VAD v5.
- **Lý do**: đây là rate/kích thước frame mà model Silero VAD được huấn luyện và `voice_activity_detector` yêu cầu; dùng đúng ràng buộc của model để đạt độ chính xác như tài liệu.
- **Phương án khác đã xem xét**: để nguyên sample rate gốc (ví dụ 48kHz từ WASAPI) — bị crate từ chối/không đúng ngưỡng huấn luyện, độ chính xác giảm.

## Quyết định: Ngưỡng phân loại speech/silence

- **Quyết định**: dùng ngưỡng xác suất mặc định 0.5 (có thể cấu hình qua tham số, không hardcode cứng) để map xác suất model trả về thành nhãn speech/silence.
- **Lý do**: giá trị mặc định được khuyến nghị bởi tài liệu Silero VAD, đơn giản, đủ dùng cho M2; việc tinh chỉnh ngưỡng theo môi trường thực tế có thể làm sau khi có dữ liệu thật.
- **Phương án khác đã xem xét**: tự tính ngưỡng động theo mức nhiễu nền — phức tạp hơn mức cần cho PBI này, để lại cho cải tiến sau nếu cần.
