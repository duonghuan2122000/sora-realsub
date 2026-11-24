# Test case: PBI 16 — T007 (tích hợp, cần sidecar + model thật)

- **Input**: câu cố định `"Hello, how are you?"`, dịch 2 lần qua `NllbTranslator` thật (executable + model đọc từ `SORA_TRANSLATE_SIDECAR_PATH` / `SORA_NLLB_MODEL_PATH`)
- **Expected**: lần 1 khớp bản dịch tham chiếu `"Xin chào, bạn khỏe không?"` (SC-001); lần 2 cho kết quả giống lần 1 (FR-004)
- **Actual**: **CHƯA CHẠY** — test đánh dấu `#[ignore]`, môi trường hiện tại không có sidecar dịch + model NLLB cục bộ để xác minh
- **Kết quả**: KHÔNG XÁC ĐỊNH (not run) — cần người có sidecar/model cục bộ chạy `cargo test -p app translator -- --ignored` theo hướng dẫn `quickstart.md` để xác nhận SC-001 cuối cùng. Task T007 không được đánh dấu `[x]` cho tới khi có bằng chứng chạy thật.
