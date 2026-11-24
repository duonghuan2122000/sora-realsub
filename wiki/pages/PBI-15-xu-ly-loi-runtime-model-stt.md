---
title: "PBI 15: Xử lý lỗi runtime model STT"
date: 2026-11-24
tags: [pbi, m3, feature]
sources: [.specify/specs/15/spec.md, .specify/specs/15/plan.md, .specify/specs/15/research.md, .specify/specs/15/data-model.md, .specify/specs/15/quickstart.md, .specify/specs/15/tasks.md, .specify/specs/15/evidence, app/src-tauri/src/core/types.rs, app/src-tauri/src/core/speech_to_text.rs, app/src-tauri/src/core/transcript_pipeline.rs, app/src-tauri/src/lib.rs]
---

# PBI 15: Xử lý lỗi runtime model STT

Tính năng cuối của M3. Trước PBI này, lỗi model STT (whisper.cpp) bị âm thầm bỏ qua: `WhisperSpeechToText::new(&path).ok()` nuốt lỗi nạp model, `eprintln!` chỉ in console khi infer lỗi — người vận hành app không hề biết STT đã dừng hoạt động. PBI này làm 2 loại lỗi runtime (nạp model, infer 1 segment) được phát hiện, phân loại và báo ra UI qua event Tauri `model-error`, thay vì thất bại âm thầm.

## Yêu cầu chính (spec.md)
- Model STT không nạp được (thiếu file/file hỏng) khi bắt đầu phiên → phát `model-error` loại `load`, phiên capture/VAD vẫn chạy (FR-001, FR-004).
- 1 segment lỗi infer → phát `model-error` loại `infer` kèm segment liên quan, KHÔNG dừng toàn pipeline, các segment sau vẫn xử lý bình thường (FR-002, FR-005).
- Mỗi event phải phân loại được nguyên nhân + mô tả ngắn dễ hiểu (FR-003).
- Không có lỗi → không event `model-error` nào phát sinh.
- Ngoài phạm vi: thiết kế UI hiển thị thông báo (toast/dialog, thuộc M6/M8), tự động retry, lỗi toàn cục khác (crash device, mất mạng — M8).

## Quyết định kỹ thuật (plan.md, research.md)
- **Enum `ModelErrorKind { Load, Infer }` + struct `ModelErrorEvent`** mới trong `core/types.rs`, KHÔNG tái dùng/đổi `CoreError::Provider(String)` đang có — tránh phải parse chuỗi lỗi kỹ thuật thô từ `whisper-rs` để đoán loại lỗi (dễ vỡ khi đổi câu lỗi thư viện).
- **`TranscriptPipeline::process_segments` đổi cách xử lý lỗi infer**: trước đây dùng `?` nên 1 lỗi làm dừng toàn bộ `Vec<SegmentReady>` truyền vào; nay match `Ok`/`Err` trong vòng lặp, lỗi gọi `on_error(&segment, &err)` rồi `continue` sang segment kế tiếp — không đổi kiểu trả về (`Result<Vec<TranscriptChunk>, CoreError>`), chỉ thêm tham số `on_error: impl FnMut(&SegmentReady, &CoreError)`. Segment lỗi không có chunk tương ứng trong `Vec` trả về.
- **Lỗi nạp model phát đúng 1 lần**: `lib.rs::start_capture` emit `model-error` (loại `load`) ngay khi `WhisperSpeechToText::new` trả `Err`, trước khi tiếp tục phiên với `stt = None` — không lặp lại mỗi khi có `vad-segment` mới trong khi STT vẫn `None`.
- **Không cấu hình `WHISPER_MODEL_PATH` ≠ lỗi nạp model**: `std::env::var(...).ok()` trả `None` vẫn giữ nguyên hành vi im lặng của PBI 14 (chưa cấu hình là trạng thái hợp lệ, không phải lỗi runtime).

## Cài đặt (code)
- `core/types.rs`: thêm `ModelErrorKind` (enum `Load`/`Infer`) và `ModelErrorEvent { kind, message, segment_start_frame_index: Option<usize> }`.
- `core/mod.rs`: export thêm `ModelErrorEvent`, `ModelErrorKind`.
- `core/transcript_pipeline.rs`: `process_segments` thêm tham số `on_error`, đổi `?` thành `match` + `continue` khi lỗi. Cập nhật toàn bộ call site cũ (test nội bộ + `tests/transcript_pipeline_real_speech.rs`) truyền thêm closure thứ 3.
- `core/speech_to_text.rs`: thêm test `nap_model_duong_dan_khong_ton_tai_tra_loi` xác nhận `WhisperSpeechToText::new` với path sai trả `Err(CoreError::Provider(..))`.
- `lib.rs`: thêm `ModelErrorPayload` (camelCase: `kind`/`message`/`segmentStartFrameIndex`) + `From<core::ModelErrorEvent>`; `start_capture` emit `model-error` loại `load` khi nạp lỗi, emit loại `infer` kèm `segment.start_frame_index` trong closure `on_error` truyền cho `process_segments`.

## Bằng chứng test (`.specify/specs/15/evidence/`)
- `nap_model_duong_dan_khong_ton_tai_tra_loi` (load lỗi) — **PASS**.
- `loi_infer_1_segment_khong_lam_dung_cac_segment_khac` (3 segment, lỗi ở segment 2, 2 segment còn lại vẫn thành công, `on_error` gọi đúng 1 lần) — **PASS**.
- `nhieu_segment_lien_tiep_loi_infer_khong_panic` (3 segment đều lỗi, `on_error` gọi đúng 3 lần, không panic, `chunks` rỗng) — **PASS**.
- Hồi quy PBI 13/14 (`nhieu_segment_tra_ve_text_dung_thu_tu`, `phat_transcript_final_dung_thu_tu`, `partial_den_truoc_final_cung_segment`, ...) — **PASS**, không đổi hành vi happy-path dù đổi chữ ký `process_segments`.
- Toàn bộ `cargo test`: 50 unit test + 8 file test tích hợp, tổng 58 test, 0 fail. **Hoàn tất M3 (STT streaming), 4/4.**
- Không kiểm thử tay qua UI Tauri dev thật (ngoài phạm vi, chưa có UI hiển thị `model-error`) — chỉ xác nhận Core phát đúng event qua test + review logic.
- Chi tiết: `evidence/pbi15-testcase.md`, log: `evidence/cargo-test-result.log`.

## Liên quan
[[PBI-14-transcript-partial-final-event]], [[PBI-13-ghep-stt-vao-luong]], [[PBI-12-stt-local-whisper]], [[Pipeline-xu-ly-audio]], [[Kien-truc-core-ui]], [[Trang-thai-ma-nguon]]
