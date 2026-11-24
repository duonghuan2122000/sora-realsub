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
