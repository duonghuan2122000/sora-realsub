# Danh sách Task: Hợp đồng giao diện Core

**Mã PBI**: 3
**Nguồn**: plan.md, spec.md, data-model.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm dependency `async-trait` và `thiserror` vào `app/src-tauri/Cargo.toml`
- [X] T002 Tạo module `app/src-tauri/src/core/mod.rs`, khai báo submodule và thêm `mod core;` vào `app/src-tauri/src/lib.rs`

## Pha 2: Foundational

- [X] T003 Định nghĩa `CoreError` (biến thể `Io`, `Provider`, `Cancelled`, dùng `thiserror`) tại `app/src-tauri/src/core/error.rs`
- [X] T004 Định nghĩa kiểu dữ liệu `AudioFrame`, `StreamHandle`, `TranscriptChunk`, `Subtitle` tại `app/src-tauri/src/core/types.rs`

## Pha 3: User Story 1 - Hợp đồng AudioSource (Ưu tiên: P1)

**Mục tiêu**: Có trait `AudioSource` biên dịch được, cài đặt giả chứng minh dùng được.
**Tiêu chí kiểm thử độc lập**: `cargo test core::audio_source::` pass với mock, đổi mock khác không cần sửa code gọi.

- [X] T005 [US1] Định nghĩa trait `AudioSource` (phương thức `start`/`stop` theo callback, trả `Result<StreamHandle, CoreError>`) tại `app/src-tauri/src/core/audio_source.rs`
- [X] T006 [US1] Viết 2 cài đặt giả (`MockAudioSourceSilent`, `MockAudioSourceNoise`) và test chứng minh thay thế lẫn nhau không cần sửa code gọi, tại `app/src-tauri/src/core/audio_source.rs`

## Pha 4: User Story 2 - Hợp đồng SpeechToText (Ưu tiên: P1)

**Mục tiêu**: Có trait `SpeechToText` biên dịch được, hỗ trợ kết quả partial/final, cài đặt giả chứng minh dùng được.
**Tiêu chí kiểm thử độc lập**: `cargo test core::speech_to_text::` pass với mock.

- [X] T007 [US2] Định nghĩa trait `SpeechToText` (`async fn transcribe(&self, frame: AudioFrame) -> Result<TranscriptChunk, CoreError>`, qua `async_trait`) tại `app/src-tauri/src/core/speech_to_text.rs`
- [X] T008 [US2] Viết cài đặt giả `MockSpeechToText` trả lần lượt 1 `TranscriptChunk { is_final: false }` rồi 1 `is_final: true`, kèm test, tại `app/src-tauri/src/core/speech_to_text.rs`

## Pha 5: User Story 3 - Hợp đồng Translator (Ưu tiên: P2)

**Mục tiêu**: Có trait `Translator` biên dịch được, nhận ngữ cảnh câu trước, cài đặt giả chứng minh dùng được.
**Tiêu chí kiểm thử độc lập**: `cargo test core::translator::` pass với mock.

- [X] T009 [US3] Định nghĩa trait `Translator` (`async fn translate(&self, text: &str, context: &[String]) -> Result<String, CoreError>`, qua `async_trait`) tại `app/src-tauri/src/core/translator.rs`
- [X] T010 [US3] Viết cài đặt giả `MockTranslator` (ví dụ thêm tiền tố `[dịch]`) kèm test xác nhận gọi đúng với context, tại `app/src-tauri/src/core/translator.rs`

## Pha 6: User Story 4 - Hợp đồng SubtitleSink (Ưu tiên: P2)

**Mục tiêu**: Có trait `SubtitleSink` biên dịch được, cài đặt giả chứng minh dùng được.
**Tiêu chí kiểm thử độc lập**: `cargo test core::subtitle_sink::` pass với mock.

- [X] T011 [US4] Định nghĩa trait `SubtitleSink` (`fn push(&self, subtitle: Subtitle) -> Result<(), CoreError>`) tại `app/src-tauri/src/core/subtitle_sink.rs`
- [X] T012 [US4] Viết cài đặt giả `MockSubtitleSink` (lưu vào `Vec<Subtitle>` nội bộ) kèm test xác nhận nhận đúng dữ liệu, tại `app/src-tauri/src/core/subtitle_sink.rs`

## Pha cuối: Polish & Cross-cutting

- [X] T013 [P] Re-export 4 trait + kiểu dữ liệu qua `app/src-tauri/src/core/mod.rs` để dùng tiện từ `lib.rs`
- [X] T014 [P] Chạy `cargo build` + `cargo test core::` toàn bộ (SC-001, SC-002), chụp ảnh/log kết quả làm bằng chứng
- [X] T015 [P] Chạy `cargo clippy` (lint đã khai trong `Cargo.toml`), sửa warning nếu có

## Sơ đồ phụ thuộc

```text
T001, T002 (Setup)
   -> T003, T004 (Foundational)
        -> US1 (T005, T006)  ─┐
        -> US2 (T007, T008)  ─┼─ độc lập với nhau, có thể làm song song sau Foundational
        -> US3 (T009, T010)  ─┤
        -> US4 (T011, T012)  ─┘
             -> T013, T014, T015 (Polish)
```

Ví dụ chạy song song: sau khi xong T001–T004, có thể giao đồng thời T005–T006 (US1), T007–T008 (US2), T009–T010 (US3), T011–T012 (US4) cho các lần implement khác nhau vì chạm 4 file riêng biệt, không phụ thuộc chéo.

## Chiến lược triển khai

- **MVP đề xuất**: US1 (`AudioSource`) + US2 (`SpeechToText`) — đủ để chứng minh pattern hợp đồng streaming/async hoạt động, các M1–M3 cần trước.
- **Thứ tự giao hàng tăng dần**: Setup → Foundational → US1 → US2 → US3 → US4 → Polish (khớp thứ tự M1→M2→M3→M4→M5 dùng các trait này trong roadmap).
