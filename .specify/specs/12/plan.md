# Kế hoạch triển khai: Nhận diện giọng nói local (STT)

**Mã PBI**: 12
**Liên kết spec**: .specify/specs/12/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 Core), edition 2024
- Framework / Thư viện chính: `whisper-rs` (binding whisper.cpp) cho STT; trait `SpeechToText` đã có sẵn tại `app/src-tauri/src/core/speech_to_text.rs`
- Lưu trữ dữ liệu: không áp dụng ở PBI này (chưa lưu transcript, thuộc M7)
- Kiểm thử: `cargo test`, test tích hợp đọc file audio mẫu → gọi `transcribe` → so khớp text kỳ vọng
- Nền tảng triển khai: Windows (theo M0), model ggml/gguf nạp từ file cục bộ
- Ràng buộc hiệu năng: chạy xong trong thời gian hợp lý để dùng tiếp theo thời gian thực ở M3 sau; không đo số cụ thể ở PBI này
- Ràng buộc khác: không gửi audio ra ngoài qua mạng (yêu cầu local từ spec FR-001)

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này; đối chiếu trực tiếp với CLAUDE.md gốc: Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | STT nằm hoàn toàn trong `core/`, không đụng UI |
| Qua IPC commands/events | ✅ | PBI này chưa cần expose IPC (chưa ghép luồng/UI), thuộc PBI sau |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: chọn `whisper-rs`, input 16kHz mono f32, model nạp từ file cục bộ.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không có entity dữ liệu mới ngoài `AudioFrame`/`TranscriptChunk` đã có ở `core/types.rs` — bỏ qua `data-model.md`.
- Hợp đồng giao diện: dự án hoàn toàn nội bộ ở PBI này (chưa expose IPC/API ra ngoài) — bỏ qua `contracts/`.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/
  Cargo.toml                         # thêm dependency whisper-rs
  src/core/
    speech_to_text.rs                # (đã có trait) thêm implementation WhisperSpeechToText
    mod.rs                           # export implementation mới
  tests/
    stt_whisper.rs                   # test tích hợp đọc file mẫu → transcribe → so khớp text
  tests/fixtures/                    # (nếu cần) file audio mẫu cho test
```

## Rủi ro & ngoại lệ có lý do

- Model ggml/gguf không commit vào repo (quá nặng) → test tích hợp cần model tại đường dẫn cấu hình được; nếu máy chạy test không có model, test tương ứng có thể bị `#[ignore]` kèm hướng dẫn trong comment, không phải vi phạm constitution mà là ràng buộc hạ tầng CI/local.
- Độ chính xác STT phụ thuộc chất lượng model (`tiny`/`base`...) — PBI này không cam kết độ chính xác tuyệt đối, chỉ cam kết chạy đúng luồng và khớp nghĩa ở mức đọc hiểu được (SC-001).
