---
title: "PBI 16: Dịch local offline (sidecar process)"
date: 2026-11-24
tags: [pbi, m4, feature, blocker]
sources: [.specify/specs/16/spec.md, .specify/specs/16/research.md, .specify/specs/16/plan.md, .specify/specs/16/tasks.md, .specify/specs/16/quickstart.md, .specify/specs/16/evidence, app/src-tauri/src/core/translator.rs, app/src-tauri/Cargo.toml]
---

# PBI 16: Dịch local offline (sidecar process)

Tính năng đầu của M4. Trước PBI này, trait `Translator` (từ PBI 3) chưa có cài đặt thật, chỉ có mock trong test. PBI này cài đặt dịch bằng model local (NLLB-200 hoặc tương đương, "offline"), nhưng **kiến trúc thực tế đổi giữa quá trình implement** do phát hiện một blocker nghiêm trọng — phần quan trọng nhất của page này.

## Yêu cầu chính (spec.md)
- Dịch 1 đoạn văn bản từ ngôn ngữ nguồn sang đích, hoàn toàn offline (FR-001).
- Lỗi rõ ràng, không crash khi "model" (hay thành phần dịch) chưa sẵn sàng (FR-002).
- Input rỗng/khoảng trắng không gây lỗi (FR-003).
- Cùng câu đầu vào → kết quả ổn định giữa các lần gọi (FR-004).
- Ngoài phạm vi: ghép ngữ cảnh nhiều câu, nối vào pipeline STT→subtitle, UI chọn ngôn ngữ (đều thuộc mục roadmap M4/M6 sau).

## ⚠️ Blocker kiến trúc phát hiện khi implement — rất quan trọng cho mọi PBI dịch sau này

**Quyết định ban đầu (research.md v1)**: dùng crate `ct2rs` (binding Rust cho CTranslate2) chạy model NLLB-200 in-process, giống pattern `whisper-rs` ở [[PBI-12-stt-local-whisper]].

**Thực tế khi build**: `cargo check` pass, nhưng `cargo test`/`cargo build` (link thật) fail với `LNK2038`/`LNK2005`/`LNK1169`. Nguyên nhân gốc: `ct2rs-sys`'s `build.rs` gọi `.static_crt(true)` — build CTranslate2 với CRT tĩnh (`/MT`) trên Windows MSVC. Project đã có `voice_activity_detector` (PBI 9, dùng cho VAD) phụ thuộc `ort` (onnxruntime), vốn phân phối object nhị phân cố định CRT động (`/MD`). Hai thư viện C++ CRT khác nhau **không thể cùng link vào 1 binary** (`app_lib`) trên MSVC — đây là xung đột gốc, không phải lỗi cấu hình tạm.

Đã thử `RUSTFLAGS="-C target-feature=+crt-static"` (gợi ý ngay trong `build.rs` của `ct2rs-sys`) — vẫn lỗi y hệt, vì object của `ort_sys` dựng sẵn, không đổi theo RUSTFLAGS.

**→ Bài học cho các PBI sau (M4 tiếp, hoặc bất kỳ ai định thêm binding C++ native khác)**: bất kỳ crate nào ép CRT tĩnh trên Windows MSVC (`static_crt(true)` trong `build.rs` của nó, thường gặp ở các binding ML C++ dùng `cmake` crate) sẽ **không thể cùng tồn tại** với `ort`/`voice_activity_detector` đã có trong `app_lib`. Kiểm tra điều này TRƯỚC khi chọn crate, không chỉ dựa vào `cargo check` (không link) — phải thử `cargo test`/`cargo build` thật.

**Quyết định cuối (research.md v2, đã áp dụng)**: Không chạy model dịch in-process nữa. `NllbTranslator` (giữ tên, implement trait `Translator`) spawn một **sidecar executable ngoài** qua `std::process::Command` (stdlib, không thêm Cargo dependency nào) — cô lập hoàn toàn khỏi `app_lib`, không còn rủi ro xung đột CRT dù sidecar bên trong dùng công nghệ gì (CTranslate2, ONNX, Python...).

Phương án khác đã xem xét và loại: `candle` + `candle-transformers::models::marian` (pure Rust, không NLLB nhưng có Marian-MT "tương đương") — loại vì models Marian/opus-mt cần tokenizer SentencePiece, và crate `sentencepiece` (binding) từng xuất hiện cùng kiểu lỗi static-CRT trong log thử nghiệm → rủi ro lặp lại blocker, không chắc an toàn hơn. Gỡ `ort`/đổi VAD — loại vì ảnh hưởng ngược M2 đã hoàn tất.

## Cài đặt (code)
- `core/translator.rs`: `NllbTranslator { executable_path: String, model_dir: String }`. `new()` chỉ lưu đường dẫn, không spawn ngay. `translate()`:
  - input rỗng/khoảng trắng → `Ok(String::new())` ngay, không spawn (FR-003).
  - spawn `Command::new(executable_path)` với args `--model-dir`, `--source-lang eng_Latn`, `--target-lang vie_Latn`, `--text <câu>` (mã FLORES-200, cặp mặc định đã chốt ở [[Quyet-dinh-ngon-ngu-mac-dinh]]).
  - spawn lỗi hoặc exit code ≠ 0 → `CoreError::Provider` (FR-002), không panic.
  - thành công → `stdout` trim làm kết quả.
- **Không thêm Cargo dependency nào** (đã thử `ct2rs` rồi revert sạch — xem lịch sử ở `.specify/specs/16/tasks.md` mục "Ghi chú chặn thi công").
- Việc build/đóng gói executable sidecar thật (convert model NLLB sang CTranslate2, biên dịch CLI) **không thuộc phạm vi PBI 16** — chỉ cần `NllbTranslator` gọi đúng executable đã cấu hình. Đây là nợ kỹ thuật cần giải quyết trước khi đóng gói app (M8).

## Phần 2 — Xác nhận SC-001 bằng sidecar + model thật (cùng ngày, sau khi người dùng yêu cầu "hoàn thiện tính năng")

Đã dựng thật kiến trúc sidecar đã quyết định ở trên, không chỉ để trống `#[ignore]`:

- **Sidecar thật**: crate Cargo **độc lập hoàn toàn** `app/translate-sidecar/` (không phải `[[bin]]` trong package `app` — Cargo.toml/Cargo.lock/target riêng, không liên quan gì tới `app_lib`/`ort`). CLI nhận `--model-dir --source-lang --target-lang --text`, dùng `ct2rs` thật để infer, in bản dịch ra stdout.
- **Phát hiện thêm 1 lớp blocker nữa**: `ct2rs` với feature `sentencepiece` (nằm trong default feature `all-tokenizers`) tự xung đột CRT NGAY TRONG NỘI BỘ nó — `sentencepiece-sys` build `/MD` còn `ct2rs-sys`/CTranslate2 build `/MT` — lỗi giống hệt lỗi với `ort` trước đó, nhưng lần này không có `ort` nào cả. Chứng minh: xung đột CRT ở Windows MSVC không chỉ là chuyện `ct2rs` vs `ort`, mà là `ct2rs` default build tự xung đột với bất kỳ crate C++ nào không cố ý build static CRT.
- **Cách giải quyết (chỉ khả thi vì sidecar là crate riêng)**:
  1. `ct2rs = { default-features = false, features = ["tokenizers", "ruy"] }` — bỏ feature `sentencepiece`, dùng tokenizer HF `tokenizers` (đọc `tokenizer.json`, pure Rust) thay binding sentencepiece native. `ruy` cần cho backend SGEMM CPU (thiếu sẽ lỗi runtime "No SGEMM backend on CPU", không phải lỗi build).
  2. Build với `RUSTFLAGS="-C target-feature=+crt-static"` — giải pháp này ĐÃ THỬ THẤT BẠI ở `app_lib` (vì `ort` prebuilt object cố định `/MD`), nhưng ở crate riêng không có `ort` nên hoạt động được. **Đây chính là lý do kiến trúc phải là sidecar (process/crate tách biệt), không thể chỉ "tách module" trong cùng crate.**
  3. Trên Windows, path build CTranslate2+ruy rất dài, dễ vượt MAX_PATH 260 ký tự → lỗi `MSB3501`; fix bằng `CARGO_TARGET_DIR` ngắn (vd `C:\ct2b`).
- **Model thật**: `JustFrederik/nllb-200-distilled-600M-ct2-int8` (Hugging Face, NLLB-200 600M đã convert CTranslate2 + quantize int8, ~647MB). **License `cc-by-nc-4.0` (non-commercial)** — chỉ dùng để dev/verify kỹ thuật ở đây; app phát hành thương mại cần đổi model.
- **Kết quả**: `cargo test --lib core::translator -- --include-ignored` (với `SORA_TRANSLATE_SIDECAR_PATH`/`SORA_NLLB_MODEL_PATH` trỏ sidecar/model thật) → **5/5 pass**, bao gồm `dich_cau_co_dinh_dung_nghia_va_on_dinh`: `"Hello, how are you?"` → `"Chào, anh thế nào?"`, ổn định giữa 2 lần gọi (FR-004). **SC-001 đã xác nhận bằng dữ liệu thật** — roadmap.md đã tick dòng dịch local.

## Bằng chứng test (`.specify/specs/16/evidence/`)
- `sidecar_khong_ton_tai_tra_loi` (FR-002) — **PASS**.
- `input_rong_tra_ve_rong_khong_spawn_tien_trinh` (FR-003) — **PASS**.
- `dich_kem_ngu_canh`, `goi_qua_trait_object` (mock cũ từ PBI 3) — **PASS**, không hồi quy.
- `dich_cau_co_dinh_dung_nghia_va_on_dinh` (SC-001, FR-004) — **PASS** (xác nhận bằng sidecar + model thật, xem phần 2 trên).
- 5/5 test pass; `cargo build` (binary chính `app`) vẫn thành công — sidecar không đụng tới `app_lib`.
- Chi tiết: `evidence/T005-T006-testcase.md`, `evidence/T007-testcase.md`, log: `evidence/T008-result.log`, `evidence/T007-result.log`.

## Liên quan
[[PBI-12-stt-local-whisper]], [[PBI-9-vad-silero]], [[Quyet-dinh-hop-dong-core]], [[Quyet-dinh-ngon-ngu-mac-dinh]], [[Rui-ro-ky-thuat]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]]
