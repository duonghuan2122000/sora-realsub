# Kế hoạch triển khai: Xử lý lỗi runtime model STT

**Mã PBI**: 15
**Liên kết spec**: .specify/specs/15/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core)
- Framework / Thư viện chính: `whisper-rs` (đã có từ PBI 12), `tauri::Emitter` cho event
- Lưu trữ dữ liệu: không có (event tức thời, không bền)
- Kiểm thử: `#[tokio::test]` trong `core/speech_to_text.rs` và `core/transcript_pipeline.rs`, dùng STT giả lập (mock) trả lỗi để giả lập case lỗi infer/nạp
- Nền tảng triển khai: desktop app Windows (không đổi so với PBI 12-14)
- Ràng buộc hiệu năng: không đáng kể — chỉ thêm nhánh xử lý lỗi, không đổi đường dẫn happy-path
- Ràng buộc khác: không phá vỡ hành vi hiện có của `transcript-partial`/`transcript-final` (PBI 14)

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong repo — bỏ qua bước này, bám theo nguyên tắc chung trong `CLAUDE.md`/`docs/style-guide.md` (tách Core/UI qua Tauri IPC, không đưa logic nghiệp vụ vào UI).

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt qua Tauri IPC | ✅ | Chỉ thêm event mới từ Core, UI chưa đổi (nằm ngoài phạm vi PBI) |
| Không phá vỡ hành vi/test đã có (PBI 12-14) | ✅ | Giữ nguyên `SpeechToText::transcribe`, `TranscriptPipeline::process_segments` signature hiện có càng nhiều càng tốt, chỉ thêm tham số callback lỗi |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md`
- Hợp đồng giao diện: không có hợp đồng bên ngoài mới (event Tauri nội bộ, theo đúng pattern `TranscriptEventPayload` đã có ở PBI 14) — bỏ qua `contracts/`
- Kịch bản khởi động nhanh: xem `quickstart.md`

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  error.rs              # thêm phân loại lỗi model (load/infer) nếu cần thiết hơn Provider(String)
  speech_to_text.rs      # WhisperSpeechToText::new giữ nguyên lỗi trả CoreError::Provider khi nạp lỗi
  transcript_pipeline.rs # process_segments: không dừng cả batch khi 1 segment lỗi infer, báo lỗi qua callback riêng rồi tiếp tục
  types.rs               # thêm struct/enum mô tả ModelErrorEvent (loại lỗi + mô tả + segment liên quan nếu có)
  mod.rs                 # export thêm kiểu mới nếu cần
lib.rs                   # khi WhisperSpeechToText::new lỗi -> emit "model-error" (loại "load") 1 lần trước khi tiếp tục không STT;
                          # khi process_segments báo lỗi qua callback -> emit "model-error" (loại "infer") kèm segment
```

## Rủi ro & ngoại lệ có lý do

- `process_segments` hiện dùng `?` nên 1 lỗi infer sẽ dừng toàn bộ `Vec<SegmentReady>` truyền vào. Vì `lib.rs` hiện gọi với `vec![segment]` (1 phần tử/lần) nên rủi ro thực tế thấp, nhưng để đúng FR-005 (lỗi 1 segment không lan toàn pipeline) sẽ đổi logic vòng lặp để bắt lỗi từng segment và tiếp tục, thay vì đổi hoàn toàn chữ ký hàm — giảm rủi ro phá vỡ test cũ ở PBI 13/14.
- Không có thiết kế UI thật để hiển thị thông báo (ngoài phạm vi, đã ghi trong spec) — chỉ đảm bảo Core phát đủ thông tin trong event.
