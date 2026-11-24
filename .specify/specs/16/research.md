# Research: PBI 16 — Dịch local offline

## Quyết định: thư viện binding chạy model dịch

**Quyết định**: Dùng crate `ct2rs` (binding Rust cho CTranslate2) để chạy model NLLB-200 đã convert sang format CTranslate2, nạp từ file model cục bộ — cùng mẫu với PBI 12 (`whisper-rs` + model ggml cục bộ cho STT).

**Lý do**:
- CTranslate2 là runtime inference CPU tối ưu, chạy hoàn toàn offline, không cần GPU, hỗ trợ sẵn model NLLB-200 (model đa ngôn ngữ roadmap M4 đã chọn).
- Giữ nhất quán kiến trúc: `Translator` (trait, core/translator.rs) nhận implementation cụ thể nạp model từ đường dẫn file, giống `WhisperSpeechToText::new(model_path)`.
- Không cần dịch vụ mạng → thoả FR-001/SC-002.

**Phương án khác đã xem xét**:
- `rust-bert` (tch/libtorch): chạy được M2M100/NLLB nhưng phụ thuộc libtorch native nặng, build phức tạp hơn trên Windows so với CTranslate2.
- Gọi API dịch online (Google/DeepL): vi phạm yêu cầu offline (FR-001), loại ngay.
- Tự implement inference ONNX cho NLLB: tốn công, không cần thiết khi đã có binding sẵn.

## Quyết định: tokenizer

**Quyết định**: Dùng tokenizer SentencePiece đi kèm model NLLB (file `.model`/`.spm`), nạp qua `ct2rs` hoặc crate `sentencepiece` nếu `ct2rs` không tự xử lý tokenize.

**Lý do**: NLLB-200 dùng SentencePiece, cần đúng tokenizer gốc để câu dịch đúng nghĩa (SC-001).

**Phương án khác đã xem xét**: Tự viết tokenizer — không khả thi, BPE/SentencePiece của NLLB có vocab lớn và quy tắc riêng.

## Điểm NEEDS CLARIFICATION

Không còn — đã chốt ở M0 cặp ngôn ngữ mặc định (English → Vietnamese) dùng làm cặp thử nghiệm cho PBI này.

## Cập nhật quyết định (2026-11-24) — đổi sang sidecar process

**Phát hiện khi implement**: `ct2rs` (CTranslate2) force-link CRT tĩnh (`/MT`) trên Windows, xung đột linker (`LNK2038`/`LNK1169`) với `ort`/onnxruntime (CRT động `/MD`) mà `voice_activity_detector` (VAD, M2) đã phụ thuộc — không build được trong cùng 1 binary. Thử `RUSTFLAGS=-C target-feature=+crt-static` cũng không giải quyết được (object của `ort_sys` dựng sẵn cố định `/MD`). Xem log chi tiết ở `tasks.md`.

**Quyết định mới**: Không chạy model dịch in-process qua binding native nữa. Thay vào đó, `NllbTranslator` (giữ tên, vẫn implement trait `Translator`) chạy model dịch bằng cách **spawn một tiến trình ngoài** (sidecar executable, ví dụ CLI dựng từ CTranslate2 hoặc một script Python đóng gói sẵn) qua `std::process::Command`, gửi câu cần dịch và nhận lại câu đã dịch qua stdin/stdout. Không thêm Cargo dependency native nào mới → không còn rủi ro xung đột CRT với `ort`.

**Lý do**:
- Cô lập hoàn toàn khỏi `app_lib` — tiến trình dịch có runtime/CRT riêng, không đụng tới `ort`/VAD dù dùng công nghệ gì bên trong (CTranslate2, ONNX, Python...).
- Chỉ dùng `std::process::Command` (stdlib), không thêm crate mới, tối thiểu rủi ro build.
- Vẫn thoả FR-001 (offline — tiến trình con chạy local, không gọi mạng).

**Phương án khác đã xem xét (sau khi phát hiện blocker)**:
- `candle` + `candle-transformers::models::marian` (pure Rust, không cần cmake): không có NLLB, chỉ có Marian-MT (`tương đương` theo spec, chấp nhận được) — nhưng models Marian/opus-mt tokenize bằng SentencePiece (`.spm`), và crate `sentencepiece` (binding) khi build cũng từng xuất hiện cùng kiểu lỗi static-CRT trong log thử ở trên (`libsentencepiece_sys` lẫn trong lỗi link) → rủi ro lặp lại đúng blocker vừa gặp, không chắc an toàn hơn `ct2rs`.
- Gỡ `ort`/đổi implementation VAD: ảnh hưởng ngược lại M2 đã hoàn thành và test qua, rủi ro cao nhất, loại.
- Sidecar process: rủi ro thấp nhất, không đổi gì ở M2 — **chọn phương án này**.

**Ràng buộc hệ quả**: Việc đóng gói app cuối (M8) sẽ cần kèm theo executable sidecar này (ngoài scope PBI 16). Trong PBI 16, implementation chỉ cần gọi đúng executable qua đường dẫn cấu hình (giống cách gọi model file ở STT) — việc build/đóng gói executable sidecar thật không thuộc phạm vi PBI này (xem "Ngoài phạm vi" ở spec.md, bổ sung ghi chú này).

## Cập nhật tiếp theo (2026-11-24) — đã dựng sidecar thật, rồi đổi sang model open-source

Thực tế đã dựng `app/translate-sidecar/` (crate riêng, dùng `ct2rs`) và xác nhận chạy được:
- Phát hiện thêm: `ct2rs` default feature `sentencepiece` (trong `all-tokenizers`) tự xung đột CRT NGAY TRONG NỘI BỘ nó (không cần `ort`) — `sentencepiece-sys` build `/MD`, `ct2rs-sys` build `/MT`. `RUSTFLAGS=-C target-feature=+crt-static` **giải quyết được** trường hợp này (khác với trường hợp `ort` — vì ở đây không có object nhị phân dựng sẵn cố định CRT nào, mọi thứ build lại từ source theo flag).
- Lần đầu dùng model `JustFrederik/nllb-200-distilled-600M-ct2-int8` (NLLB-200, feature `ct2rs` = `tokenizers`+`ruy`, không dùng `sentencepiece`) — chạy được, SC-001/FR-004 pass. Nhưng model này license `cc-by-nc-4.0` (non-commercial).
- **Theo yêu cầu người dùng đổi sang model open-source**: đổi sang `gaudi/opus-mt-en-vi-ctranslate2` (Marian-MT, chuyển đổi CTranslate2 của `Helsinki-NLP/opus-mt-en-vi`, **license Apache-2.0**). Model này dùng `source.spm`/`target.spm` (sentencepiece thô) → đổi feature `ct2rs` sang `sentencepiece`+`ruy`, xác nhận `RUSTFLAGS=-C target-feature=+crt-static` cũng fix được xung đột CRT của `sentencepiece-sys`. Marian-MT là model 1 cặp ngôn ngữ cố định, không cần source/target prefix token như NLLB — đổi cách gọi `translate_batch_with_source_and_target_prefix` → `translate_batch` thường.
- Đổi tên `NllbTranslator` → `SidecarTranslator` trong `core/translator.rs` vì tên cũ không còn phản ánh đúng model đang dùng.
- **Kết luận**: kiến trúc sidecar (crate riêng, build với `crt-static`) hoạt động tốt với cả 2 họ model (NLLB và Marian-MT), không phụ thuộc vào model cụ thể. Chọn model nào (NLLB chất lượng cao hơn nhưng non-commercial, hay Marian nhẹ hơn và Apache-2.0) là quyết định sản phẩm, không phải giới hạn kỹ thuật.
