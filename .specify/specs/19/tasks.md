# Danh sách Task: Nối STT final → Dịch → subtitle-ready

**Mã PBI**: 19
**Nguồn**: plan.md, spec.md, data-model.md, research.md, quickstart.md

Spec.md không chia nhiều user story riêng — toàn bộ 5 kịch bản chấp nhận thuộc 1 luồng end-to-end duy nhất → gộp thành **User Story 1 (P1)**.

## Pha 1: Setup

- [X] T001 Đọc lại `app/src-tauri/src/lib.rs`, `core/translator.rs`, `core/transcript_pipeline.rs`, `core/mod.rs` (nơi định nghĩa `ModelErrorKind`) để xác định đúng vị trí sửa theo research.md/data-model.md trước khi code.

## Pha 2: Foundational

- [X] T002 Thêm biến thể `Translate` vào enum `ModelErrorKind` tại `app/src-tauri/src/core/mod.rs` (hoặc nơi enum được định nghĩa — xác nhận lại ở T001), cập nhật mọi `match` hiện có trên enum này để không thiếu nhánh khi build (data-model.md).
- [X] T003 Cập nhật `ModelErrorPayload::from` tại `app/src-tauri/src/lib.rs` để map `ModelErrorKind::Translate` → chuỗi `"translate"` (giống cách `Load`/`Infer` đã map `"load"`/`"infer"`).
- [X] T004 Thêm struct `SubtitlePayload` (camelCase, `#[derive(Serialize, Clone)]`) tại `app/src-tauri/src/lib.rs` với các trường `start_frame_index`, `end_frame_index`, `source_text`, `translated_text`, kèm hàm dựng từ `(&SegmentReady, source_text: &str, translated_text: &str)` — theo data-model.md.

## Pha 3: User Story 1 — Nối transcript final → dịch → subtitle-ready (Ưu tiên: P1)

**Mục tiêu**: Mỗi câu transcript final (không rỗng) trong 1 phiên capture được tự động dịch (có ngữ cảnh tối đa 2 câu gốc trước) và phát ra đúng 1 sự kiện `subtitle-ready` mang cả câu gốc + câu dịch, đúng thứ tự thời gian; partial/rỗng bị bỏ qua; lỗi dịch 1 câu không dừng phiên; thiếu translator thì bỏ qua bước dịch.

**Tiêu chí kiểm thử độc lập**: Chạy `cargo test -p app` với mock STT + mock Translator (không cần model/sidecar thật) xác nhận toàn bộ FR-001 đến FR-008; chạy test tích hợp `--ignored` (có model/sidecar thật) xác nhận SC-002.

- [X] T005 [US1] Trong `app/src-tauri/src/lib.rs::start_capture`, tạo `translator: Option<SidecarTranslator>` từ `SORA_TRANSLATE_SIDECAR_PATH` + `SORA_TRANSLATE_MODEL_PATH` (giống cách `stt` được tạo từ `WHISPER_MODEL_PATH`), và tạo `recent_ctx = RecentTranscriptContext::new()` mới cho phiên (research.md Quyết định 2, 5; FR-008).
- [X] T006 [US1] Trong closure `on_chunk` của `TranscriptPipeline::process_segments()` tại `app/src-tauri/src/lib.rs`, sau khi phát `transcript-final`/`transcript-partial` như hiện có: nếu `chunk.is_final` và `chunk.text` không rỗng/khoảng trắng và `translator.is_some()` — lấy `context = recent_ctx.recent()`, gọi `translator.translate(&chunk.text, &context)` (research.md Quyết định 1, 3; FR-001, FR-002, FR-005, FR-006).
- [X] T007 [US1] Trong cùng closure (`app/src-tauri/src/lib.rs`): nếu `translate()` trả `Ok(translated_text)`, phát event `"subtitle-ready"` với `SubtitlePayload::from(segment, &chunk.text, &translated_text)`, rồi `recent_ctx.push(&chunk.text)` (FR-003, FR-004; data-model.md luồng xử lý).
- [X] T008 [US1] Trong cùng closure (`app/src-tauri/src/lib.rs`): nếu `translate()` trả `Err(err)`, phát event `"model-error"` với `ModelErrorPayload::from(ModelErrorEvent { kind: ModelErrorKind::Translate, message: err.to_string(), segment_start_frame_index: Some(segment.start_frame_index) })`, không phát `subtitle-ready`, vẫn `recent_ctx.push(&chunk.text)`, không dừng vòng lặp segment (research.md Quyết định 4; FR-007).
- [X] T009 [US1] Viết test tại `app/src-tauri/src/lib.rs` (hoặc module test mới `#[cfg(test)]` cạnh `start_capture` nếu `start_capture` khó test trực tiếp do `AppHandle`/`State` — tách phần logic nối (T006-T008) thành 1 hàm thuần `build_subtitle_event(text, context, translate_fn) -> Result<...>` nếu cần để test không phụ thuộc Tauri runtime) xác nhận: nhiều segment final liên tiếp → đúng số lượng + đúng thứ tự `subtitle-ready` tương ứng (kịch bản chấp nhận 1, 3).
- [X] T010 [P] [US1] Viết test xác nhận: với mock Translator đếm số câu context nhận được, câu final thứ 2+ nhận đúng tối đa 2 câu context gần nhất theo thứ tự cũ→mới (kịch bản chấp nhận 2; FR-002) — đặt cạnh test T009.
- [X] T011 [P] [US1] Viết test xác nhận: chunk với `is_final = false` (partial) không gọi `translator.translate()` và không phát `subtitle-ready` (kịch bản chấp nhận 4; FR-005) — dùng mock Translator đếm số lần gọi.
- [X] T012 [P] [US1] Viết test xác nhận: chunk final với text rỗng/chỉ khoảng trắng không gọi `translate()`, không phát `subtitle-ready`, không được push vào context (kịch bản chấp nhận 5; FR-006).
- [X] T013 [P] [US1] Viết test xác nhận: mock Translator trả `Err` cho 1 câu giữa nhiều câu final liên tiếp → không panic, số `subtitle-ready` phát ra = số câu dịch thành công, có đúng 1 `model-error` kiểu `translate`, các câu sau vẫn tiếp tục xử lý (trường hợp biên; FR-007; SC-003).
- [X] T014 [US1] Viết test tích hợp `#[ignore]` tại `app/src-tauri/src/lib.rs` (theo mẫu PBI 16/17: đọc `WHISPER_MODEL_PATH`, `SORA_TRANSLATE_SIDECAR_PATH`, `SORA_TRANSLATE_MODEL_PATH` từ env, `expect` nếu thiếu) xác nhận SC-002: 2 segment liên tiếp (câu có đại từ phụ thuộc câu trước) qua luồng thật → `subtitle-ready` thứ 2 có `translatedText` khớp bản dịch có ngữ cảnh đã xác nhận ở PBI 17 T016.

## Pha cuối: Polish & Cross-cutting

- [X] T015 Chạy `cargo build -p app` và `cargo test -p app` (không `--ignored`) xác nhận toàn bộ build sạch (không thiếu nhánh `match` trên `ModelErrorKind`) và test mới pass.
- [X] T016 Chạy test tích hợp `--ignored` (T014) nếu đã có model/sidecar cục bộ; chụp ảnh/log output làm bằng chứng SC-002, lưu vào `.specify/specs/19/evidence/`.
- [X] T017 Cập nhật docstring/comment ngắn tại các vị trí sửa trong `lib.rs` nếu cần, trỏ về PBI 19 (theo quy ước comment ngắn đã dùng ở các PBI trước — chỉ ghi lý do/quyết định, không lặp lại mô tả code).

## Sơ đồ phụ thuộc

- Pha 1 (T001) → Pha 2 (T002-T004) → Pha 3 (T005-T014) → Pha cuối (T015-T017).
- Trong Pha 2: T002 phải xong trước T003 (map enum mới); T004 độc lập, có thể làm song song với T002/T003 nhưng cùng file `lib.rs` nên làm tuần tự để tránh conflict khi chỉnh tay.
- Trong Pha 3: T005 → T006 → T007/T008 phải tuần tự (cùng đoạn code trong `lib.rs`). T009-T013 là test, có thể viết song song với nhau (khác test case, đã đánh `[P]` cho T010-T013) nhưng đều phụ thuộc T005-T008 đã xong để có code chạy thật. T014 phụ thuộc toàn bộ T005-T008.
- Chỉ có 1 user story (P1) trong PBI này — không có phụ thuộc giữa nhiều story.

## Ví dụ chạy song song

Sau khi T005-T008 hoàn tất, có thể giao song song (khác test case, cùng file nhưng không đụng cùng đoạn code nếu thêm hàm test riêng):
```
T010, T011, T012, T013  (song song)
```

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, chỉ 1 story) — không chia nhỏ thêm.
- Thứ tự giao hàng: Setup → Foundational (enum + payload) → nối logic (T005-T008) → test logic (T009-T013, không cần model thật) → test tích hợp thật (T014, cần máy có sẵn model/sidecar) → polish.
