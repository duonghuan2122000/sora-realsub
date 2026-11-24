---
title: "PBI 14: Event transcript-partial/transcript-final cho UI"
date: 2026-11-24
tags: [pbi, m3, feature]
sources: [.specify/specs/14/spec.md, .specify/specs/14/plan.md, .specify/specs/14/research.md, .specify/specs/14/data-model.md, .specify/specs/14/contracts/transcript-events.md, .specify/specs/14/quickstart.md, .specify/specs/14/tasks.md, .specify/specs/14/evidence, app/src-tauri/src/core/transcript_pipeline.rs, app/src-tauri/src/lib.rs, app/src-tauri/Cargo.toml]
---

# PBI 14: Event transcript-partial/transcript-final cho UI

Tính năng thứ ba của M3. Phát 2 loại event Tauri từ Core sang UI khi `TranscriptPipeline` ([[PBI-13-ghep-stt-vao-luong]]) nhận được kết quả từ `SpeechToText`: `transcript-partial` (text tạm, có thể sửa) và `transcript-final` (text chốt). Khác PBI 13 (chỉ trả `Vec<TranscriptChunk>` nội bộ), PBI này nối pipeline vào `start_capture` thật trong `lib.rs` và emit event ra ngoài, làm nền cho hiển thị phụ đề ở M5.

## Yêu cầu chính (spec.md)
- Phát `transcript-final` mỗi khi 1 segment có văn bản chốt (FR-001).
- Phát `transcript-partial` khi có văn bản tạm, PHẢI đến trước `transcript-final` của cùng segment (FR-002).
- Event PHẢI đến UI đúng thứ tự thời gian phát sinh thực tế của segment (FR-003).
- Mỗi event PHẢI mang đủ thông tin định danh segment (`startFrameIndex`/`endFrameIndex`) + nội dung text (FR-004).
- Audio không có segment thoại nào → không event nào phát ra, không lỗi (FR-005).
- Ngoài phạm vi: hiển thị phụ đề thật (M5), xử lý lỗi runtime model chi tiết (PBI kế tiếp M3), dịch (M4), chạy trên luồng capture thời gian thực liên tục kiểu production-grade.

## Quyết định kỹ thuật (plan.md, research.md)
- **Thêm callback vào `TranscriptPipeline::process_segments`**: `process_segments(segments, on_chunk: impl FnMut(&SegmentReady, &TranscriptChunk))` — gọi `on_chunk` ngay sau mỗi segment có kết quả, thay vì gom hết rồi trả `Vec` một lần. Giữ nguyên việc trả `Vec<TranscriptChunk>` để không phá API cũ của PBI 13 (test PBI 13 vẫn pass, chỉ thêm tham số).
- **`WhisperSpeechToText` hiện tại không đổi** — vẫn luôn trả `TranscriptChunk { is_final: true }` (không hỗ trợ partial thật). Theo đó mỗi segment chỉ phát đúng 1 event `transcript-final`, không có `transcript-partial` đi kèm. Field `is_final` đã có sẵn từ PBI 12/13 nên khi sau này STT hỗ trợ streaming partial thật, chỉ cần set `is_final = false` ở lần gọi trung gian — không cần sửa `TranscriptPipeline`/`lib.rs`.
- **Payload event dùng `startFrameIndex`/`endFrameIndex`** của `SegmentReady` gốc để UI định danh segment, nhất quán với contract `vad-segment` đã có ở [[PBI-11-vad-segment-event]] — không sinh ID mới.
- **`tokio` chuyển từ `[dev-dependencies]` sang `[dependencies]`** trong `Cargo.toml` — cần runtime để `block_on` gọi `TranscriptPipeline::process_segments` (async) từ closure đồng bộ (`FnMut`) của `CaptureSessionState::start`.
- **STT trong `start_capture` lấy từ `WHISPER_MODEL_PATH`** (biến môi trường, tái dùng đúng convention đã có ở test PBI 12/13) — không bắt buộc để capture/VAD chạy được; nếu thiếu, bỏ qua bước phát transcript-partial/final, `vad-segment` vẫn phát như cũ. Đây là quyết định tạm để thỏa phạm vi PBI (nối được pipeline thật), KHÔNG phải cấu hình người dùng chính thức — màn hình cấu hình chọn model thuộc M6/M8.

## Cài đặt (code)
- `app/src-tauri/src/core/transcript_pipeline.rs`: `process_segments` thêm tham số `on_chunk`, gọi ngay sau mỗi `stt.transcribe(...)` thành công.
- `app/src-tauri/src/lib.rs`: thêm `TranscriptEventPayload` (camelCase, `startFrameIndex`/`endFrameIndex`/`text`/`isFinal`); `start_capture` tạo `tokio::runtime::Builder::new_current_thread()`, nạp `WhisperSpeechToText` qua `WHISPER_MODEL_PATH` (tuỳ chọn), trong callback VAD mỗi `SegmentReady` được đưa qua `TranscriptPipeline::process_segments` (`rt.block_on`), emit `transcript-final` hoặc `transcript-partial` tuỳ `chunk.is_final`.
- `app/src-tauri/Cargo.toml`: `tokio` chuyển sang `[dependencies]`.
- `app/src-tauri/tests/transcript_pipeline_real_speech.rs`: cập nhật 2 lời gọi `process_segments` cho khớp signature mới (`|_, _| {}`).

## Bằng chứng test (`.specify/specs/14/evidence/`)
- Unit test mới trong `core::transcript_pipeline::tests`: `phat_transcript_final_dung_thu_tu` (3 segment → 3 event final đúng thứ tự) — **PASS**; `danh_sach_rong_khong_goi_callback` (segment rỗng → callback 0 lần) — **PASS**; `partial_den_truoc_final_cung_segment` (mock STT trả partial rồi final cho cùng segment, qua `ScriptedSpeechToText`) — **PASS**.
- Toàn bộ `cargo test`: 47 unit test + 8 integration test (bao gồm test thật với model whisper qua `WHISPER_MODEL_PATH`), tổng 55 test, 0 fail — không có test PBI 9-13 nào bị hỏng do đổi signature `process_segments`.
- Không kiểm thử tay qua UI Tauri dev thật (không bắt buộc cho PBI này) — việc emit event trong `lib.rs` xác nhận bằng `cargo build` thành công + review logic tái dùng đúng callback đã test.
- Chi tiết: `evidence/transcript-events-testcase.md`, log: `evidence/cargo-test-result.log`.

## Liên quan
[[PBI-13-ghep-stt-vao-luong]], [[PBI-12-stt-local-whisper]], [[PBI-11-vad-segment-event]], [[Pipeline-xu-ly-audio]], [[Kien-truc-core-ui]], [[Trang-thai-ma-nguon]]
