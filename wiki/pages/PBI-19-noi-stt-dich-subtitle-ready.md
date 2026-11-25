---
title: "PBI 19: Nối STT final → Dịch → subtitle-ready"
date: 2026-11-25
tags: [pbi, m4, feature]
sources: [.specify/specs/19/spec.md, .specify/specs/19/research.md, .specify/specs/19/plan.md, .specify/specs/19/tasks.md, .specify/specs/19/evidence, app/src-tauri/src/lib.rs, app/src-tauri/src/core/types.rs, app/src-tauri/src/core/mod.rs, app/src-tauri/tests/subtitle_pipeline_real.rs]
---

# PBI 19: Nối STT final → Dịch → subtitle-ready

Mục roadmap M4 thứ 3 và cuối ("Nối STT final → Translator → event `subtitle-ready`"). Trước PBI
này, [[PBI-16-dich-local-sidecar]] và [[PBI-17-dich-co-ngu-canh]] đã có `SidecarTranslator` dịch
đúng 1 câu (kèm ngữ cảnh), nhưng hoàn toàn chưa gắn vào luồng capture thật — `start_capture`
(`lib.rs`) chỉ phát `transcript-final`/`transcript-partial`, không tự động dịch. PBI này nối 2
phần lại, hoàn tất M4 (3/3 mục).

## Yêu cầu chính (spec.md)
- Mỗi câu transcript final (không rỗng) tự động được dịch, kèm tối đa 2 câu final gốc gần nhất
  làm ngữ cảnh (FR-001, FR-002).
- Phát 1 event duy nhất mang cả câu gốc + câu dịch (FR-003), đúng thứ tự thời gian (FR-004).
- Partial hoặc câu rỗng/khoảng trắng: không dịch, không phát event, không tính vào ngữ cảnh
  (FR-005, FR-006).
- Lỗi dịch 1 câu: báo lỗi, không dừng phiên, câu sau vẫn tiếp tục (FR-007).
- Thiếu cấu hình dịch (sidecar/model): bỏ qua bước dịch, `transcript-final` vẫn phát như cũ
  (FR-008) — pattern "model optional" giống STT từ PBI 14/15.

## Quyết định kỹ thuật (research.md)

1. **Nối ngay trong `lib.rs::start_capture`**, không sửa `TranscriptPipeline` — giữ
   `TranscriptPipeline` chỉ biết VAD→STT, không biết `Translator` (tách mối quan tâm).
2. **Tái dùng `RecentTranscriptContext`** (PBI 17) — tạo 1 instance mới mỗi lần `start_capture`
   (reset theo phiên, đúng giả định ở spec.md).
3. **Push context SAU khi dịch xong**, dùng câu gốc (không phải câu dịch) — câu hiện tại không tự
   làm ngữ cảnh cho chính nó; câu lỗi dịch vẫn được push (bản thân câu gốc vẫn hợp lệ).
4. **Lỗi dịch → `model-error` kiểu `translate` mới** (mở rộng `ModelErrorKind`, thêm biến thể
   song song `Load`/`Infer` đã có từ PBI 15) — tái dùng đúng cơ chế báo lỗi đã quen, không thêm
   cơ chế lỗi khác cho UI phải học.
5. **`translator: Option<SidecarTranslator>`** dựng từ `SORA_TRANSLATE_SIDECAR_PATH` +
   `SORA_TRANSLATE_MODEL_PATH` — giống cách `stt: Option<WhisperSpeechToText>` đã có, giữ khả
   năng test capture/VAD/STT độc lập không cần sidecar dịch.

## Cài đặt (code)

- `core/types.rs`: `ModelErrorKind` thêm biến thể `Translate` (cạnh `Load`, `Infer`).
- `core/mod.rs`: export thêm `SidecarTranslator`, `RecentTranscriptContext` (trước đó chỉ export
  trait `Translator`, không export 2 cài đặt/struct cụ thể này ra ngoài `core`).
- `lib.rs`:
  - `SubtitlePayload` (mới, camelCase): `startFrameIndex`, `endFrameIndex`, `sourceText`,
    `translatedText` — payload event Tauri `subtitle-ready`.
  - `ModelErrorPayload::from` map thêm `ModelErrorKind::Translate → "translate"`.
  - Hàm thuần mới `translate_final_chunk(translator, recent_ctx, chunk, on_subtitle, on_error)`
    (`async fn`, tách khỏi `start_capture` để **test được bằng mock `Translator`, không cần
    `AppHandle`/Tauri runtime**) — chứa toàn bộ logic quyết định 1-4 ở trên: bỏ qua
    partial/rỗng, lấy context, gọi `translate()`, gọi đúng 1 trong 2 callback, rồi `push` context.
  - `start_capture`: sau khi `TranscriptPipeline::process_segments` trả `chunks`, với mỗi
    `chunk` gọi `rt.block_on(translate_final_chunk(...))` — callback `on_subtitle` emit
    `"subtitle-ready"`, callback `on_error` emit `"model-error"` kiểu `translate`. **Không** gọi
    `translator.translate()` lồng trong `block_on` của `process_segments` (tránh nested
    `block_on` panic) — tách thành 2 lần `block_on` tuần tự, không nested.
- Test mock mới trong `lib.rs::tests` (T009-T013): `RecordingTranslator` (ghi lại text+context
  nhận được), `FlakyTranslator` (lỗi đúng tại lần gọi định trước) — xác nhận thứ tự, ngữ cảnh tối
  đa 2 câu, bỏ qua partial, bỏ qua rỗng, lỗi 1 câu không lan.
- Test tích hợp mới `tests/subtitle_pipeline_real.rs` (`#[ignore]`, cần model STT + sidecar dịch
  thật — theo mẫu PBI 16/17): xác nhận SC-002 (ngữ cảnh lấy từ `RecentTranscriptContext` khớp
  đúng ngữ cảnh truyền tay như PBI 17 T016).

## Bằng chứng test

- `cargo build -p app` + `cargo test -p app` (không `--ignored`): **65 passed, 0 failed, 2
  ignored** (2 ignored là test PBI 16/17 cần sidecar thật, không liên quan code PBI 19) — log:
  `.specify/specs/19/evidence/T015-build-test-result.log`,
  `T009-T013-lib-tests-result.log`.
- Test case bằng chứng chi tiết từng kịch bản: `T009-T013-testcase.md` (5 test mới, PASS cả 5).
- `tests/subtitle_pipeline_real.rs`: compile sạch, `1 ignored` đúng như kỳ vọng (môi trường này
  không có model/sidecar cục bộ) — `T014-compile-result.log`, `T014-testcase.md`. **SC-002 chưa
  được xác nhận bằng chạy thật** trong container này — cần người có sẵn `WHISPER_MODEL_PATH` +
  sidecar/model dịch cục bộ chạy `-- --ignored` theo `quickstart.md`.

## ✅ Môi trường: container Linux lần này build/test thành công, không gặp lại chặn PBI 17

Khác với [[PBI-17-dich-co-ngu-canh]] (bị chặn hoàn toàn bởi `wasapi` không gate theo target —
đã xử lý ở [[PBI-18-loopback-linux]]), lần thi công PBI 19 này **`cargo build -p app` thành công
ngay** sau khi cài thêm vài hệ thống lib còn thiếu trên container (không liên quan code):
`libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libjavascriptcoregtk-4.1-dev`, `libsoup-3.0-dev`,
`librsvg2-dev`, `libasound2-dev`, `libpulse-dev`. Đây là lần đầu các lib Tauri/GTK này được cần
tới thật sự trong build (trước đó PBI 18 chỉ cần `libpulse-dev`/`libasound2-dev` để build
`core/`, chưa chắc đã kéo toàn bộ `tauri`/`wry`/`webkit2gtk` — xem lại
[[PBI-18-loopback-linux]] để đối chiếu danh sách lib đã cài). Xác nhận môi trường container hiện
tại **có thể build/test `cargo test -p app` đầy đủ, kể cả phần Tauri/GTK**, không chỉ phần
`core/` thuần Rust như trước.

## Liên quan
[[PBI-16-dich-local-sidecar]], [[PBI-17-dich-co-ngu-canh]], [[PBI-18-loopback-linux]],
[[PBI-14-transcript-partial-final-event]], [[PBI-15-xu-ly-loi-runtime-model-stt]],
[[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]]
