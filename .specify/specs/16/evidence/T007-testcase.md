# Test case: PBI 16 — T007 (tích hợp, sidecar + model NLLB thật) — ĐÃ XÁC NHẬN

## Chuẩn bị môi trường thật
- Sidecar: crate độc lập mới `app/translate-sidecar/` (CLI Rust dùng `ct2rs`, feature `tokenizers` + `ruy`, KHÔNG dùng feature `sentencepiece` — binding đó cũng force CRT tĩnh, xung đột nội bộ ngay trong `ct2rs`, không liên quan `ort`). Build: `RUSTFLAGS="-C target-feature=+crt-static" cargo build --release` (cần build ngoài đường dẫn quá dài trên Windows — path CTranslate2/ruy build dễ vượt MAX_PATH 260, dùng `CARGO_TARGET_DIR` ngắn nếu gặp lỗi `MSB3501`).
- Model: `JustFrederik/nllb-200-distilled-600M-ct2-int8` từ Hugging Face (NLLB-200 600M, đã convert CTranslate2 + quantize int8, ~647MB: `model.bin`, `sentencepiece.bpe.model`, `tokenizer.json`, `shared_vocabulary.txt`, `special_tokens_map.json`, `tokenizer_config.json`, `config.json`). **License `cc-by-nc-4.0` (non-commercial)** — chỉ dùng để xác minh kỹ thuật ở đây; nếu app phát hành thương mại cần đổi sang model có license phù hợp hoặc tự convert từ `facebook/nllb-200-distilled-600M` (license CC-BY-NC gốc của Meta cũng cần xem lại tuỳ mục đích).

## Input/Expected/Actual
- **Input**: `"Hello, how are you?"`, dịch 2 lần qua `NllbTranslator` thật.
- **Expected**: lần 1 có bản dịch non-trivial, hợp nghĩa (SC-001, không yêu cầu khớp 1 câu "chuẩn" tuyệt đối — NMT có thể cho nhiều cách dịch đúng); lần 2 cho **đúng y** kết quả lần 1 (FR-004, model inference deterministic với greedy/beam search cố định seed/không sampling).
- **Actual**: lần 1 = lần 2 = `"Chào, anh thế nào?"` — hợp nghĩa câu chào hỏi, ổn định giữa 2 lần gọi.
- Thử thêm câu khác ngoài test tự động để kiểm tra bằng mắt: `"The meeting will start at 9 AM tomorrow."` → `"Cuộc họp sẽ bắt đầu lúc 9 giờ sáng ngày mai."` — dịch đúng nghĩa.
- **Kết quả**: **PASS**. `cargo test --lib core::translator -- --include-ignored`: 5/5 pass (bao gồm `dich_cau_co_dinh_dung_nghia_va_on_dinh`). Log đầy đủ: [T007-result.log](T007-result.log).

## Giới hạn đã biết
- Giá trị tham chiếu `"Chào, anh thế nào?"` trong test chỉ đúng với đúng cặp sidecar+model này; đổi model/quantization có thể cho bản dịch khác (vẫn hợp nghĩa) — test sẽ fail và cần cập nhật lại giá trị tham chiếu, không phải lỗi logic.
- Model int8 — chưa đánh giá chất lượng dịch toàn diện (BLEU, câu dài, ngữ cảnh đa nghĩa), chỉ xác nhận pipeline kỹ thuật hoạt động đúng theo spec.md.
