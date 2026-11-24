# Kế hoạch triển khai: Dịch local offline

**Mã PBI**: 16
**Liên kết spec**: .specify/specs/16/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core), edition 2024
- Framework / Thư viện chính: **(đã đổi, xem research.md mục "Cập nhật quyết định")** spawn tiến trình ngoài (sidecar executable chạy model NLLB/tương đương) qua `std::process::Command` — không dùng binding native in-process nữa vì xung đột CRT với `ort`
- Lưu trữ dữ liệu: không — chỉ nhận văn bản đầu vào, trả văn bản dịch, không lưu trạng thái
- Kiểm thử: `cargo test` trong `app/src-tauri`, dùng mock `Translator` (đã có ở `core/translator.rs`) cho test luồng, và test tích hợp riêng với model thật cho SC-001 (đánh dấu cần model cục bộ, không chạy trên CI nếu chưa có model)
- Nền tảng triển khai: desktop Windows (giống các PBI trước), không cần GPU
- Ràng buộc hiệu năng: không yêu cầu realtime trong PBI này (ghép pipeline thuộc mục roadmap sau); chỉ cần dịch 1 câu hoàn tất trong thời gian hợp lý cho test (vài giây)
- Ràng buộc khác: hoàn toàn offline (không gọi API dịch qua mạng) — FR-001

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án → bỏ qua bước này; áp dụng nguyên tắc chung đã nêu ở `CLAUDE.md`: Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào tầng UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | `NllbTranslator` nằm trong `core/translator.rs`, không đụng tới `app/src` (UI) |
| Giữ nhất quán kiến trúc STT trước đó | ✅ | Cùng mẫu `new(model_path, ...)` + implement trait như `WhisperSpeechToText` |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`: chọn `ct2rs` + model NLLB-200 (CTranslate2 format) + tokenizer SentencePiece, không còn điểm NEEDS CLARIFICATION.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không cần `data-model.md` riêng — chỉ có input/output là `String`, không có thực thể dữ liệu phức tạp.
- Hợp đồng giao diện: không cần `contracts/` — `Translator` là trait nội bộ Core, không phải API/CLI công khai cho PBI này.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/
  Cargo.toml                       (không thêm dependency native mới)
  src/core/
    translator.rs                  (+ struct NllbTranslator implement trait Translator, chạy sidecar qua std::process::Command)
    error.rs                       (tái dùng CoreError::Provider cho lỗi spawn/parse output, giống STT)
```

## Rủi ro & ngoại lệ có lý do

- **[ĐÃ XẢY RA — đã xử lý]** `ct2rs` (CTranslate2 binding) force-link CRT tĩnh, xung đột với `ort`/onnxruntime (CRT động) mà VAD đã dùng → không build được cùng binary. Đã revert, đổi sang kiến trúc sidecar process (xem research.md). Không còn dùng `ct2rs`.
- Model NLLB-200 (hoặc model tương đương chạy trong sidecar) khá lớn, không commit vào repo — test tích hợp dùng sidecar thật cần người chạy chuẩn bị sẵn executable + model, cấu hình qua biến môi trường. Test unit logic (input rỗng, sidecar không tồn tại) vẫn chạy được không cần sidecar thật.
- Việc build/đóng gói executable sidecar thật (convert model, biên dịch CLI) không thuộc phạm vi PBI 16 — PBI này chỉ cần `NllbTranslator` gọi đúng executable đã cấu hình qua đường dẫn, xử lý lỗi rõ ràng khi executable thiếu/lỗi.
