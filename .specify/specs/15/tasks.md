# Danh sách Task: Xử lý lỗi runtime model STT

**Mã PBI**: 15
**Nguồn**: plan.md, spec.md, research.md, data-model.md, quickstart.md

## Pha 1: Setup

- [X] T001 Đọc lại `app/src-tauri/src/core/error.rs`, `speech_to_text.rs`, `transcript_pipeline.rs`, `types.rs`, `lib.rs` để xác định chính xác điểm chèn thay đổi (không tạo file mới ngoài phạm vi đã nêu trong `plan.md`)

## Pha 2: Foundational

- [X] T002 Thêm enum `ModelErrorKind { Load, Infer }` và struct `ModelErrorEvent { kind: ModelErrorKind, message: String, segment_start_frame_index: Option<u64> }` vào `app/src-tauri/src/core/types.rs` (theo `data-model.md`)
- [X] T003 Export `ModelErrorKind`, `ModelErrorEvent` từ `app/src-tauri/src/core/mod.rs` nếu `mod.rs` đang re-export các type tương tự (theo pattern `TranscriptChunk`/`SegmentReady` hiện có)

## Pha 3: User Story 1 - Báo lỗi khi model không nạp được (Ưu tiên: P1)

**Mục tiêu**: Khi `WHISPER_MODEL_PATH` trỏ tới file không tồn tại/hỏng, người vận hành nhận được thông báo lỗi rõ ràng, phiên capture vẫn chạy.
**Tiêu chí kiểm thử độc lập**: Test gọi `WhisperSpeechToText::new` với đường dẫn sai → `Err`; test ở tầng `lib.rs`/tích hợp xác nhận `start_capture` vẫn `Ok` và có đúng 1 event `model-error` loại `load`.

- [X] T004 [US1] Viết test trong `app/src-tauri/src/core/speech_to_text.rs` xác nhận `WhisperSpeechToText::new("đường-dẫn-không-tồn-tại")` trả `Err(CoreError::Provider(..))` với message dễ đọc (case thiếu file)
- [X] T005 [US1] Trong `app/src-tauri/src/lib.rs`, sau dòng `WhisperSpeechToText::new(&path).ok()`, khi `Err`, chuyển message kỹ thuật thành `ModelErrorEvent { kind: Load, message: "...", segment_start_frame_index: None }` và `emit("model-error", ...)` đúng 1 lần trước khi tiếp tục `start_capture` với `stt = None`
- [X] T006 [US1] Xử lý case `std::env::var("WHISPER_MODEL_PATH")` không được set (hiện `.ok()` trả `None` âm thầm): giữ hành vi không coi là lỗi runtime model (không cấu hình ≠ lỗi nạp) — chỉ phát `model-error` khi có đường dẫn nhưng `WhisperSpeechToText::new` trả `Err`, test lại bằng kịch bản thủ công/README ghi chú trong `quickstart.md` (không cần test tự động nếu không có cách giả lập `env var` trong unit test hiện có)

## Pha 4: User Story 2 - Lỗi infer 1 segment không làm dừng pipeline (Ưu tiên: P1)

**Mục tiêu**: Khi 1 segment lỗi infer, Core báo lỗi cho đúng segment đó và tiếp tục xử lý các segment còn lại trong cùng lần gọi.
**Tiêu chí kiểm thử độc lập**: Test `TranscriptPipeline::process_segments` với STT giả lập lỗi ở 1 segment giữa 2 segment thành công → callback lỗi gọi đúng 1 lần cho segment lỗi, `on_chunk` vẫn gọi đúng cho 2 segment còn lại, hàm vẫn trả `Ok`.

- [X] T007 [US2] Sửa chữ ký `TranscriptPipeline::process_segments` trong `app/src-tauri/src/core/transcript_pipeline.rs`: thêm tham số `mut on_error: impl FnMut(&SegmentReady, &CoreError)`, đổi vòng lặp để khi `self.stt.transcribe(frame).await` trả `Err(e)` thì gọi `on_error(&segment, &e)` rồi `continue` (không `?`/`return Err` sớm), giữ nguyên hành vi `on_chunk` cho các segment thành công
- [X] T008 [US2] Cập nhật toàn bộ lệnh gọi `process_segments(...)` hiện có trong test (`transcript_pipeline.rs`) và trong `app/src-tauri/src/lib.rs` để truyền thêm closure `on_error` (ở test cũ dùng `|_, _| {}` nếu không cần kiểm tra lỗi; ở `lib.rs` emit `ModelErrorEvent { kind: Infer, message: ..., segment_start_frame_index: Some(segment.start_frame_index) }` qua `app_handle.emit("model-error", ...)`)
- [X] T009 [US2] Viết test mới trong `app/src-tauri/src/core/transcript_pipeline.rs`: STT giả lập (`FlakySpeechToText` kiểu `ScriptedSpeechToText` nhưng trả `Err` ở lần gọi thứ 2 trong 3 lần) → xác nhận `on_error` được gọi đúng 1 lần cho segment thứ 2, `on_chunk` được gọi cho segment 1 và 3 với kết quả đúng, hàm trả `Ok(chunks)` với `chunks.len() == 2` (chỉ 2 chunk thành công, không có chunk rỗng/placeholder cho segment lỗi)
- [X] T010 [US2] Viết test xác nhận nhiều segment liên tiếp đều lỗi infer → `on_error` được gọi đúng số lần bằng số segment lỗi, không trùng/thiếu, không panic (trường hợp biên trong spec.md)

## Pha cuối: Polish & Cross-cutting

- [X] T011 Chạy `cargo test --manifest-path app/src-tauri/Cargo.toml` toàn bộ, xác nhận không có test cũ (PBI 11-14) bị hỏng do đổi chữ ký `process_segments`
- [X] T012 Chụp ảnh/log kết quả `cargo test` lưu vào `.specify/specs/15/evidence/` kèm ghi chú case đã test (load lỗi, infer lỗi 1 segment, infer lỗi nhiều segment, không lỗi)

## Sơ đồ phụ thuộc

- Pha 1 (Setup) → Pha 2 (Foundational: `ModelErrorKind`/`ModelErrorEvent`) → cả US1 và US2 đều cần Pha 2 xong trước.
- US1 (T004-T006) và US2 (T007-T010) độc lập về file chính (US1 sửa `lib.rs` phần load; US2 sửa `transcript_pipeline.rs` + phần gọi trong `lib.rs`) nhưng cả hai cùng sửa `lib.rs` nên **không** đánh dấu `[P]` giữa US1 và US2 để tránh conflict khi thi công song song.
- Pha cuối (T011-T012) phải chạy sau khi cả US1 và US2 hoàn tất.

## Chiến lược triển khai

- MVP đề xuất: US1 (báo lỗi khi model không nạp được) — giá trị cao nhất, sửa tối thiểu, không đổi chữ ký hàm nào.
- Thứ tự giao hàng tăng dần: Setup → Foundational → US1 → US2 → Polish. US2 có thể làm độc lập sau US1 nếu cần chia nhỏ commit, vì chỉ đụng `transcript_pipeline.rs` + phần gọi liên quan trong `lib.rs`.
