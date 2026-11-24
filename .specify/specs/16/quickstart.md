# Quickstart: kiểm thử PBI 16 — Dịch local offline

**Kiến trúc** (đã đổi sau khi phát hiện xung đột CRT, xem research.md): `NllbTranslator` gọi một **sidecar executable** qua `std::process::Command`, không chạy model in-process. Sidecar thật đã dựng tại `app/translate-sidecar/` (crate Cargo độc lập, không phải package `app`).

1. Chạy test unit (không cần sidecar thật): `cargo test -p app translator` trong `app/src-tauri` — test input rỗng và test sidecar không tồn tại đều pass ngay.

2. **Build sidecar thật** (`app/translate-sidecar/`):
   ```
   cd app/translate-sidecar
   RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-msvc
   ```
   - Nếu gặp lỗi `MSB3501`/path quá dài (MAX_PATH 260 trên Windows, hay gặp khi build CTranslate2+ruy): build với `CARGO_TARGET_DIR` ngắn, ví dụ `CARGO_TARGET_DIR=C:\ct2b`.
   - Executable ra ở `target/<target-triple>/release/translate-sidecar.exe` (hoặc dưới `CARGO_TARGET_DIR` nếu dùng).

3. **Tải model thật**: `JustFrederik/nllb-200-distilled-600M-ct2-int8` trên Hugging Face (NLLB-200 600M, convert CTranslate2 + quantize int8, ~647MB). **License `cc-by-nc-4.0` (non-commercial)** — chỉ dùng để dev/test, đổi model khác trước khi phát hành thương mại. Tải các file sau vào 1 thư mục:
   ```
   config.json, model.bin, sentencepiece.bpe.model, shared_vocabulary.txt,
   special_tokens_map.json, tokenizer.json, tokenizer_config.json
   ```
   từ `https://huggingface.co/JustFrederik/nllb-200-distilled-600M-ct2-int8/resolve/main/<filename>` (ví dụ bằng `curl -fL -o <filename> <url>` cho từng file — `tokenizer.json` là file lớn nhất, ~17MB, `model.bin` ~623MB).

4. Đối chiếu bằng mắt trước khi tin test: gọi trực tiếp sidecar —
   ```
   ./translate-sidecar --model-dir <thư mục model> --source-lang eng_Latn --target-lang vie_Latn --text "Hello, how are you?"
   ```
   Kết quả đã xác nhận: `"Chào, anh thế nào?"` (xem `evidence/T007-testcase.md`).

5. Chạy test tích hợp SC-001/FR-004 thật:
   ```
   SORA_TRANSLATE_SIDECAR_PATH=<đường dẫn translate-sidecar.exe> \
   SORA_NLLB_MODEL_PATH=<thư mục model> \
   cargo test -p app translator -- --include-ignored
   ```
   Kỳ vọng: 5/5 test pass, bao gồm `dich_cau_co_dinh_dung_nghia_va_on_dinh`.
