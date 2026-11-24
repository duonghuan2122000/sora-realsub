# Test case: PBI 16 — T007 (tích hợp, sidecar + model dịch thật) — ĐÃ XÁC NHẬN

## Chuẩn bị môi trường thật
- Sidecar: crate độc lập `app/translate-sidecar/` (CLI Rust dùng `ct2rs`, feature `sentencepiece` + `ruy`). Build: `RUSTFLAGS="-C target-feature=+crt-static" cargo build --release` (cần build ngoài đường dẫn quá dài trên Windows — path CTranslate2/ruy build dễ vượt MAX_PATH 260, dùng `CARGO_TARGET_DIR` ngắn nếu gặp lỗi `MSB3501`).
- Model: **`gaudi/opus-mt-en-vi-ctranslate2`** (Hugging Face) — chuyển đổi CTranslate2 của `Helsinki-NLP/opus-mt-en-vi` (Marian-MT, dữ liệu OPUS). **License Apache-2.0, open-source, dùng được cho mục đích thương mại** (đổi từ model NLLB-200 ban đầu vì model đó license `cc-by-nc-4.0`, non-commercial — xem lịch sử ở `research.md`). ~148MB: `model.bin`, `source.spm`, `target.spm`, `vocab.json`, `shared_vocabulary.json`, `config.json`, `generation_config.json`, `tokenizer_config.json`.
- Model là 1 cặp ngôn ngữ cố định (en→vi), không cần mã ngôn ngữ FLORES-200 như NLLB — sidecar vẫn nhận `--source-lang`/`--target-lang` qua CLI (hợp đồng chung) nhưng không dùng tới giá trị khi gọi `translate_batch`.

## Input/Expected/Actual
- **Input**: `"Hello, how are you?"`, dịch 2 lần qua `SidecarTranslator` thật.
- **Expected**: lần 1 có bản dịch hợp nghĩa (SC-001); lần 2 cho đúng y kết quả lần 1 (FR-004).
- **Actual**: lần 1 = lần 2 = `"Xin chào, anh khỏe không?"` — hợp nghĩa câu chào hỏi, ổn định giữa 2 lần gọi.
- Thử thêm câu khác ngoài test tự động: `"The meeting will start at 9 AM tomorrow."` → `"Buổi họp bắt đầu lúc 9 giờ sáng ngày mai."` — dịch đúng nghĩa.
- **Kết quả**: **PASS**. `cargo test --lib core::translator -- --include-ignored`: 5/5 pass (bao gồm `dich_cau_co_dinh_dung_nghia_va_on_dinh`). Log đầy đủ: [T007-result.log](T007-result.log). Toàn project `cargo test --lib`: 52/52 pass.

## Giới hạn đã biết
- Giá trị tham chiếu `"Xin chào, anh khỏe không?"` chỉ đúng với đúng cặp sidecar+model này; đổi model/quantization có thể cho bản dịch khác (vẫn hợp nghĩa) — test sẽ fail và cần cập nhật lại giá trị tham chiếu, không phải lỗi logic.
- `ct2rs` log cảnh báo runtime "compute type float16 không hỗ trợ hiệu quả trên CPU này, tự chuyển sang float32" — vô hại, chỉ ảnh hưởng tốc độ không ảnh hưởng độ đúng.
- Marian-MT (opus-mt-en-vi) là model nhỏ hơn NLLB-200 (148MB vs 647MB), chưa đánh giá chất lượng dịch toàn diện (BLEU, câu dài, ngữ cảnh đa nghĩa, các cặp ngôn ngữ khác ngoài en→vi) — chỉ xác nhận pipeline kỹ thuật hoạt động đúng theo spec.md.

## Lịch sử đổi model (2026-11-24)
Lần xác nhận đầu dùng `JustFrederik/nllb-200-distilled-600M-ct2-int8` (NLLB-200, license `cc-by-nc-4.0` non-commercial). Theo yêu cầu người dùng đổi sang model open-source, đã thay bằng `gaudi/opus-mt-en-vi-ctranslate2` (Apache-2.0) — kiến trúc sidecar không đổi, chỉ đổi: feature `ct2rs` (`sentencepiece` thay `tokenizers`, vì model này dùng `source.spm`/`target.spm` thay `tokenizer.json`), cách gọi `translate_batch` (bỏ source/target prefix token vì Marian không cần như NLLB), và struct `NllbTranslator` đổi tên thành `SidecarTranslator` (tên cũ không còn đúng vì không còn là NLLB).
