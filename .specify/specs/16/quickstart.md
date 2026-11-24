# Quickstart: kiểm thử PBI 16 — Dịch local offline

**Kiến trúc** (đã đổi sau khi phát hiện xung đột CRT, xem research.md): `SidecarTranslator` gọi một **sidecar executable** qua `std::process::Command`, không chạy model in-process. Sidecar thật đã dựng tại `app/translate-sidecar/` (crate Cargo độc lập, không phải package `app`).

1. Chạy test unit (không cần sidecar thật): `cargo test -p app translator` trong `app/src-tauri` — test input rỗng và test sidecar không tồn tại đều pass ngay.

2. **Build sidecar thật** (`app/translate-sidecar/`), dùng feature `sentencepiece` (model Marian-MT dùng `source.spm`/`target.spm`):
   ```
   cd app/translate-sidecar
   RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-msvc
   ```
   - Nếu gặp lỗi `MSB3501`/path quá dài (MAX_PATH 260 trên Windows, hay gặp khi build CTranslate2+ruy): build với `CARGO_TARGET_DIR` ngắn, ví dụ `CARGO_TARGET_DIR=C:\ct2b`.
   - Executable ra ở `target/<target-triple>/release/translate-sidecar.exe` (hoặc dưới `CARGO_TARGET_DIR` nếu dùng).

3. **Tải model thật**: `gaudi/opus-mt-en-vi-ctranslate2` trên Hugging Face (Marian-MT, chuyển đổi CTranslate2 của `Helsinki-NLP/opus-mt-en-vi`, ~148MB). **License Apache-2.0 — open-source, dùng được cho mục đích thương mại.** Tải các file sau vào 1 thư mục:
   ```
   config.json, generation_config.json, tokenizer_config.json, vocab.json,
   shared_vocabulary.json, source.spm, target.spm, model.bin
   ```
   từ `https://huggingface.co/gaudi/opus-mt-en-vi-ctranslate2/resolve/main/<filename>` (ví dụ bằng `curl -fL -o <filename> <url>` cho từng file — `model.bin` là file lớn nhất, ~144MB).

4. Đối chiếu bằng mắt trước khi tin test: gọi trực tiếp sidecar —
   ```
   ./translate-sidecar --model-dir <thư mục model> --source-lang en --target-lang vi --text "Hello, how are you?"
   ```
   Kết quả đã xác nhận: `"Xin chào, anh khỏe không?"` (xem `evidence/T007-testcase.md`). Model này là 1 cặp ngôn ngữ cố định (en→vi) — `--source-lang`/`--target-lang` nhận qua CLI cho đồng bộ hợp đồng sidecar nhưng không ảnh hưởng kết quả.

5. Chạy test tích hợp SC-001/FR-004 thật:
   ```
   SORA_TRANSLATE_SIDECAR_PATH=<đường dẫn translate-sidecar.exe> \
   SORA_TRANSLATE_MODEL_PATH=<thư mục model> \
   cargo test -p app translator -- --include-ignored
   ```
   Kỳ vọng: 5/5 test pass, bao gồm `dich_cau_co_dinh_dung_nghia_va_on_dinh`.

## Model khác đã thử (không dùng nữa)

Lần xác nhận đầu dùng `JustFrederik/nllb-200-distilled-600M-ct2-int8` (NLLB-200, feature `ct2rs` = `tokenizers`+`ruy`, cần mã ngôn ngữ FLORES-200 `eng_Latn`/`vie_Latn`). Đã đổi sang model trên vì NLLB-200 gốc license `cc-by-nc-4.0` (non-commercial) — không phù hợp nếu app phát hành thương mại. Nếu cần chất lượng dịch cao hơn Marian và chấp nhận ràng buộc non-commercial, có thể quay lại NLLB (xem lịch sử git của file này).
