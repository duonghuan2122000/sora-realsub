# Danh sách Task: Dịch local offline

**Mã PBI**: 16
**Nguồn**: plan.md, spec.md, research.md

## Pha 1: Setup

- [x] T001 ~~Thêm dependency `ct2rs`~~ — **ĐỔI HƯỚNG** (xem "Ghi chú chặn thi công"): không thêm dependency native nào, dùng `std::process::Command` (đã có sẵn trong stdlib) để gọi sidecar dịch

## Pha 2: Foundational

*(Không cần — trait `Translator` đã có sẵn tại `app/src-tauri/src/core/translator.rs`, không có task nền tảng mới)*

## Pha 3: User Story 1 - Dịch một câu offline đúng nghĩa (Ưu tiên: P1)

**Mục tiêu**: Người dùng/hệ thống gọi `Translator::translate` với một câu cố định, nhận lại bản dịch đúng, hoàn toàn offline, không crash với input bất thường.

**Tiêu chí kiểm thử độc lập**: Chạy `cargo test -p app translator` trong `app/src-tauri` — test dịch câu cố định khớp bản dịch tham chiếu (cần model thật, đánh dấu `#[ignore]` nếu chưa có model trên máy CI), cộng các test input rỗng / model lỗi chạy được không cần model.

- [x] T002 [US1] Cài đặt `struct NllbTranslator` + `impl Translator` tại `app/src-tauri/src/core/translator.rs`: `new(executable_path: &str, model_dir: &str) -> Result<Self, CoreError>` chỉ lưu đường dẫn (không spawn ngay); `translate()` gọi `std::process::Command::new(executable_path)` kèm args ngôn ngữ nguồn/đích (`eng_Latn`/`vie_Latn`) + model_dir, gửi câu qua arg, đọc stdout làm kết quả
- [x] T003 [US1] Trong `translate()` của `NllbTranslator`: xử lý input rỗng/khoảng trắng → trả `Ok(String::new())` ngay, không spawn tiến trình (FR-003)
- [x] T004 [US1] Trong `translate()` của `NllbTranslator`: khi spawn lỗi (executable không tồn tại) hoặc tiến trình trả exit code khác 0 → trả `CoreError::Provider` rõ ràng, không panic (FR-002)
- [x] T005 [US1] Viết test tại `app/src-tauri/src/core/translator.rs`: gọi `translate` với `executable_path` không tồn tại phải trả `Err(CoreError::Provider(_))`, không panic (đối chiếu T004)
- [x] T006 [US1] Viết test tại `app/src-tauri/src/core/translator.rs`: gọi `translate("", &[])` và `translate("   ", &[])` phải trả `Ok("".to_string())`, không spawn tiến trình (đối chiếu T003)
- [x] T007 [US1] Viết test tích hợp (đánh dấu `#[ignore]`, chạy thủ công khi có sidecar thật) tại `app/src-tauri/src/core/translator.rs`: dịch câu tiếng Anh cố định (ví dụ "Hello, how are you?") sang tiếng Việt bằng `NllbTranslator` qua sidecar thật, so khớp bản dịch tham chiếu đã biết trước (SC-001); gọi lại lần 2 với câu giống nhau, xác nhận kết quả giống lần 1 (FR-004)
- [x] T008 [US1] Chạy `cargo test -p app translator` (không ignored) lấy log làm bằng chứng test, lưu vào `.specify/specs/16/evidence/` — xem [T008-result.log](evidence/T008-result.log)

## Sơ đồ phụ thuộc

T001 → T002 → (T003, T004 song song vì khác đoạn logic trong cùng file nhưng không đánh `[P]` vì cùng file) → T005, T006 (phụ thuộc T003/T004) → T007 (phụ thuộc T002-T004) → T008 (phụ thuộc T007 chạy được).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (chỉ có 1 story trong PBI này) — T001 đến T006 chạy được không cần model thật; T007-T008 cần model NLLB cục bộ để xác nhận SC-001 cuối cùng.
- Không có cơ hội chạy song song thực sự `[P]` vì toàn bộ task đều sửa cùng 2 file (`translator.rs`, `Cargo.toml`) theo thứ tự phụ thuộc nhau.

## Ghi chú chặn thi công (2026-11-24)

**T001 BLOCKED — không retry được, cần quyết định của người dùng.**

Đã thử thêm `ct2rs` (binding CTranslate2, quyết định ở `research.md`) và cài `NllbTranslator`. `cargo check` pass, nhưng `cargo test`/`cargo build` thất bại ở bước **link**:

- Lần 1: build mặc định → `LNK2038`/`LNK2005`/`LNK1169` — `ct2rs-sys` build CTranslate2 với `static_crt(true)` (CRT tĩnh `/MT`), xung đột trực tiếp với `ort`/onnxruntime (CRT động `/MD`) mà crate `voice_activity_detector` (đã dùng cho VAD, PBI 10-11) phụ thuộc — cả hai cùng nằm trong `app_lib`.
- Lần 2: thử `RUSTFLAGS="-C target-feature=+crt-static"` (gợi ý ngay trong `build.rs` của `ct2rs-sys`) → vẫn lỗi y hệt, vì `ort_sys` dùng object `onnxruntime_c_api.obj` dựng sẵn (`MD_DynamicRelease`) cố định, không đổi theo RUSTFLAGS.
- Đây là xung đột kiến trúc giữa 2 native dependency (không phải lỗi tạm/flaky) → không retry lần 3 với cùng hướng, đã dừng và **revert hoàn toàn** `ct2rs` khỏi `Cargo.toml` + revert `translator.rs` về bản chỉ có trait/mock để không phá build toàn project. Đã xác nhận `cargo test --lib core::translator` pass lại sau revert.

**Cần người dùng quyết định hướng tiếp theo** (không tự chọn thay vì ảnh hưởng kiến trúc toàn project):

1. Đổi sang runtime pure-Rust không cần cmake/C++ (ví dụ `candle` + `candle-transformers`) — tránh xung đột CRT, nhưng cần research lại xem có hỗ trợ NLLB/M2M100 sẵn không.
2. Chạy model dịch ở tiến trình phụ (sidecar process, ví dụ gọi CLI `ctranslate2`/Python qua `std::process::Command`) để cô lập hoàn toàn khỏi `app_lib`, đổi lại mất lợi ích gọi hàm trực tiếp trong-process.
3. Gỡ `ort`/`voice_activity_detector` (đổi VAD sang implementation khác) để nhường chỗ CRT tĩnh cho `ct2rs` — ảnh hưởng ngược lại M2, rủi ro cao hơn.

Chưa quay lại `/sora-task 16`/`/sora-implement 16` cho đến khi chọn hướng ở trên (có thể cần viết lại `research.md`/`plan.md` PBI 16 theo hướng mới).

## Cập nhật — ĐÃ GIẢI QUYẾT (2026-11-24, phiên sau)

Đã chọn hướng sidecar process, implement xong `NllbTranslator` (T002-T006). Sau đó thực sự build sidecar thật + tải model NLLB thật để xác nhận T007/SC-001 (theo yêu cầu người dùng "hoàn thiện tính năng"):

- Tạo crate RIÊNG `app/translate-sidecar/` (không phải `[[bin]]` trong package `app` — một package Cargo độc lập hoàn toàn, Cargo.toml/Cargo.lock/target riêng) dùng `ct2rs` để làm CLI dịch thật.
- **Phát hiện thêm**: `ct2rs` với feature `sentencepiece` (mặc định, trong `all-tokenizers`) tự xung đột CRT NGAY TRONG NỘI BỘ nó (`sentencepiece-sys` build `/MD`, `ct2rs-sys`/CTranslate2 build `/MT`) — không cần `ort` cũng vẫn lỗi `LNK2038`. Khi đã ở crate riêng (không có `ort`), fix được bằng:
  1. `default-features = false, features = ["tokenizers", "ruy"]` (bỏ `sentencepiece`, dùng tokenizer HF `tokenizers` đọc `tokenizer.json` thay vì sentencepiece binding; `ruy` cần cho backend SGEMM CPU, không có sẽ lỗi "No SGEMM backend on CPU").
  2. Build với `RUSTFLAGS="-C target-feature=+crt-static"` — **chỉ khả thi vì giờ không có `ort` trong cùng crate** để xung đột; đây là lý do kiến trúc sidecar (crate riêng) bắt buộc, không thể gộp vào `app`/`app_lib`.
  3. Trên Windows, đường dẫn build CTranslate2+ruy rất dài, dễ vượt `MAX_PATH` (260 ký tự) gây lỗi `MSB3501` — build ở `CARGO_TARGET_DIR` ngắn (ví dụ `C:\ct2b`) nếu gặp lỗi này.
- Model thật: `JustFrederik/nllb-200-distilled-600M-ct2-int8` (Hugging Face, ~647MB, license `cc-by-nc-4.0` — non-commercial, cần đổi model trước khi phát hành thương mại).
- Chạy `cargo test --lib core::translator -- --include-ignored` với `SORA_TRANSLATE_SIDECAR_PATH`/`SORA_NLLB_MODEL_PATH` trỏ tới sidecar/model thật: **5/5 test pass**, bao gồm T007 (dịch đúng + ổn định, SC-001 + FR-004 xác nhận bằng dữ liệu thật).
- `app/translate-sidecar/` không commit `target/`, model không commit (quá lớn, không thuộc repo) — xem `quickstart.md` để build lại từ đầu.
