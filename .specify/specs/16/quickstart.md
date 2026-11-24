# Quickstart: kiểm thử PBI 16 — Dịch local offline

**Kiến trúc** (đã đổi sau khi phát hiện xung đột CRT, xem research.md): `NllbTranslator` gọi một **sidecar executable** qua `std::process::Command`, không chạy model in-process.

1. Chạy test unit (không cần sidecar thật): `cargo test -p app translator` trong `app/src-tauri` — test input rỗng và test sidecar không tồn tại đều pass ngay.
2. Để xác nhận SC-001 (dịch đúng câu cố định) cần chuẩn bị sidecar thật:
   - Chuẩn bị một executable CLI chạy model NLLB-200 (hoặc tương đương) nhận args `--model-dir <dir> --source-lang eng_Latn --target-lang vie_Latn --text "<câu>"` và in bản dịch ra stdout.
   - Đặt `SORA_TRANSLATE_SIDECAR_PATH` = đường dẫn executable đó, `SORA_NLLB_MODEL_PATH` = thư mục model.
   - Chạy: `cargo test -p app translator -- --ignored` để chạy test tích hợp `dich_cau_co_dinh_dung_nghia_va_on_dinh`.
3. (Thủ công, không bắt buộc) Gọi trực tiếp sidecar từ dòng lệnh với cùng args để đối chiếu output bằng mắt trước khi tin tưởng kết quả test.
