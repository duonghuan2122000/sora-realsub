---
title: "PBI 17: Dịch có ngữ cảnh câu trước"
date: 2026-11-25
tags: [pbi, m4, feature, env-limitation]
sources: [.specify/specs/17/spec.md, .specify/specs/17/research.md, .specify/specs/17/plan.md, .specify/specs/17/tasks.md, .specify/specs/17/evidence, app/src-tauri/src/core/translator.rs]
---

# PBI 17: Dịch có ngữ cảnh câu trước

Mục roadmap M4 thứ 2 ("Ghép ngữ cảnh: gửi kèm 1-2 câu trước khi dịch câu hiện tại"). Trước PBI
này, `Translator::translate(text, context)` (trait có từ PBI 3) đã nhận tham số `context` nhưng
`SidecarTranslator` ([[PBI-16-dich-local-sidecar]]) bỏ qua hoàn toàn (`_context`). PBI này làm
cho tham số đó thực sự được dùng.

## Yêu cầu chính (spec.md)
- Dịch câu hiện tại kèm tối đa 2 câu ngữ cảnh ngay trước nó → kết quả chính xác hơn với đại
  từ/chủ ngữ ẩn (FR-001).
- Kết quả trả về CHỈ là bản dịch câu hiện tại, không lẫn bản dịch câu ngữ cảnh (FR-002).
- Context > 2 câu → chỉ dùng 2 câu gần nhất (FR-003); context rỗng → hành vi y PBI 16 (FR-004);
  câu ngữ cảnh rỗng/khoảng trắng bị loại (FR-005); ổn định giữa các lần gọi (FR-006).
- Ngoài phạm vi: nối vào pipeline STT→`subtitle-ready` thật (mục roadmap M4 kế tiếp), đổi
  model/cặp ngôn ngữ.

## Quyết định kỹ thuật (research.md)

1. **Không đổi CLI sidecar** (`app/translate-sidecar/src/main.rs` giữ nguyên, không build lại).
   Marian-MT (`opus-mt-en-vi`) là model dịch câu đơn, không có cơ chế "ngữ cảnh" riêng như
   model document-level. Giải pháp thực dụng: ghép (nối chuỗi) tối đa 2 câu ngữ cảnh + câu hiện
   tại thành **1 input duy nhất** (cũ→mới), gửi qua `--text` như một lệnh dịch bình thường —
   model vẫn "thấy" ngữ cảnh qua attention vì toàn bộ nằm trong 1 lần inference.
2. **Tách bản dịch câu hiện tại khỏi bản dịch ghép** bằng heuristic dấu kết câu (`.`/`!`/`?`):
   tách bản dịch trả về thành N đoạn, nếu N khớp đúng số câu đầu vào (ngữ cảnh+hiện tại) thì lấy
   đoạn **cuối cùng**; nếu không khớp (model gộp câu, không giữ đúng dấu câu) → **fallback**: gọi
   lại dịch riêng câu hiện tại, không ngữ cảnh (như PBI 16) — đảm bảo không bao giờ trả về bản
   dịch lẫn ngữ cảnh hoặc rác, đổi lại đôi khi mất lợi ích ngữ cảnh cho câu khó tách. Đây là
   **hạn chế biết trước, chấp nhận được** — không có alignment chính xác từ Marian-MT CLI hiện
   có.
3. **`RecentTranscriptContext`**: struct nhỏ (bounded FIFO tối đa 2 câu, `push()`/`recent()`) giữ
   câu transcript final gần nhất. Chỉ ở mức **đơn vị, có test riêng** — KHÔNG nối vào
   `TranscriptPipeline`/`lib.rs` thật (việc đó là mục roadmap M4 kế tiếp, "nối STT final →
   Translator → `subtitle-ready`").

## Cài đặt (code — `core/translator.rs`)
- `normalize_context(context)`: lọc câu rỗng/khoảng trắng, giữ tối đa 2 câu cuối (FR-003, FR-005).
- `split_sentences(text)`: tách theo `.`/`!`/`?`, giữ dấu câu.
- `extract_current_translation(combined_translation, expected_count) -> Option<String>`: hàm
  **thuần** (không IO) cho quyết định tách/fallback — tách riêng ra để test được không cần spawn
  process.
- `SidecarTranslator::translate_raw(input)`: hàm nội bộ gọi sidecar cho 1 input (được tái dùng
  cho cả câu đơn và chuỗi ghép).
- `SidecarTranslator::translate()`: text rỗng → `""` ngay; context rỗng (sau normalize) → gọi
  `translate_raw(text)` y PBI 16; có context → ghép input, gọi `translate_raw`, rồi
  `extract_current_translation` quyết định trả kết quả hay fallback gọi lại không ngữ cảnh.
- **Không thêm Cargo dependency nào**, không đổi `app/translate-sidecar/`.

## ⚠️ Phát hiện môi trường quan trọng — container Linux không build được crate `app`

Khi thi công PBI này, `cargo test -p app translator` **không chạy được** trong container Linux
(môi trường thi công của session này), độc lập hoàn toàn với thay đổi của PBI 17:

- `cargo tree -i windows-future` xác nhận chuỗi phụ thuộc duy nhất:
  `windows-future → windows → wasapi → app`. Crate `wasapi` (dùng cho
  [[PBI-6-capture-loopback-wasapi]], `LoopbackAudioSource`) **không gate theo
  `cfg(windows)`** trong `Cargo.toml` (`wasapi = "0.26"` vô điều kiện) — tồn tại từ PBI 6 (M1),
  không phải lỗi PBI 17.
- Build lỗi `E0425` khi compile `windows-future` (phiên bản crate `windows`/`windows-future` hiện
  tại trên crates.io không tương thích nhau khi build cho target Linux dù chỉ resolve, không
  thực sự chạy trên Windows).
- Đã thử tạm gate `mod loopback_audio_source` bằng `#[cfg(windows)]` + `wasapi` optional để kiểm
  tra giả thuyết — xác nhận đúng, build tiến xa hơn nhưng dừng đúng lỗi trên. **Đã revert hoàn
  toàn**, không có trong commit (chỉ để chẩn đoán).
- Không có máy Windows gắn vào session để build/test trực tiếp trên nền tảng mục tiêu thật.

**Hệ quả**: `cargo test -p app translator` (toàn bộ test mock PBI 16/17) và test tích hợp sidecar
thật (SC-001 PBI 17) **chưa chạy được** trong container này. Để vẫn có bằng chứng thật, các hàm
thuần (`normalize_context`, `split_sentences`, `extract_current_translation`,
`RecentTranscriptContext`, toàn bộ nhánh quyết định của `translate()`) được **sao y nguyên logic**
vào 1 crate Rust độc lập ở scratchpad (không commit), chạy `cargo test` thật tại đó:
**13/13 test pass**. Chi tiết: `.specify/specs/17/evidence/README-moi-truong.md`,
`T017-logic-tests.log`, `T005-T015-testcase.md`.

**Khuyến nghị cho dự án**: PBI riêng để gate `wasapi` (và các dependency Windows-only tương tự)
theo `cfg(target_os = "windows")` trong `Cargo.toml` + `core/mod.rs`/`lib.rs`, nếu muốn dev/test
một phần trên Linux/CI đa nền tảng. Ngoài phạm vi PBI 17 — xem thêm [[Rui-ro-ky-thuat]].

## Liên quan
[[PBI-16-dich-local-sidecar]], [[PBI-6-capture-loopback-wasapi]], [[Rui-ro-ky-thuat]],
[[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]]
