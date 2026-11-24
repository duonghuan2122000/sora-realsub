---
title: "PBI 13: Ghép STT vào luồng VAD → Transcript"
date: 2026-11-24
tags: [pbi, m3, feature]
sources: [.specify/specs/13/spec.md, .specify/specs/13/plan.md, .specify/specs/13/research.md, .specify/specs/13/data-model.md, .specify/specs/13/quickstart.md, .specify/specs/13/tasks.md, .specify/specs/13/evidence, app/src-tauri/src/core/transcript_pipeline.rs, app/src-tauri/tests/transcript_pipeline_real_speech.rs]
---

# PBI 13: Ghép STT vào luồng VAD → Transcript

Tính năng thứ hai của M3. Nối hai khả năng đã có độc lập — `VadPipeline` (cắt segment, [[PBI-11-vad-segment-event]]) và `SpeechToText`/`WhisperSpeechToText` (nhận diện giọng nói local, [[PBI-12-stt-local-whisper]]) — thành một luồng xử lý end-to-end: segment thoại VAD cắt ra → đưa qua STT → nhận văn bản, đúng thứ tự thời gian.

## Yêu cầu chính (spec.md)
- Mỗi segment thoại VAD cắt ra PHẢI được đưa qua STT để nhận diện (FR-001).
- Thứ tự thời gian giữa các đoạn văn bản trả về PHẢI khớp thứ tự segment gốc (FR-002).
- Audio không có segment thoại nào (toàn im lặng) → không sinh văn bản nào, không lỗi (FR-003).
- Luồng xử lý end-to-end từ 1 file/nguồn audio mẫu đến danh sách văn bản, không cần can thiệp tay ở bước giữa (FR-004).
- Ngoài phạm vi: event Tauri `transcript-partial`/`transcript-final`, xử lý lỗi runtime model chi tiết, chạy trên luồng capture thời gian thực liên tục, dịch (Translation) — các PBI/milestone sau.

## Quyết định kỹ thuật (plan.md, research.md)
- **Bộ điều phối mới `TranscriptPipeline`** (cùng thư mục `core/`) — lớp mỏng nối `VadPipeline` và `SpeechToText` đã có, không đổi gì ở 2 khả năng đó (giữ tách biệt trách nhiệm: VAD không cần biết về STT).
- **Xử lý tuần tự, không song song** (`await` lần lượt từng segment, không `join_all`) — đơn giản, đảm bảo đúng thứ tự thời gian (FR-002) mà không cần logic sắp xếp lại; quy mô vài segment/lần `push_frame` chưa cần tối ưu song song.
- **Test end-to-end vẫn dùng file audio mẫu** (chưa dùng audio capture mic/loopback thật) — nhất quán với cách PBI 9-12 đã test, vì chưa tới lúc ghép vào luồng capture thời gian thực (đó là phạm vi PBI sau).

## Cài đặt (code)
- `app/src-tauri/src/core/transcript_pipeline.rs` (mới): `struct TranscriptPipeline<'a>` giữ `&'a dyn SpeechToText`; `async fn process_segments(&self, segments: Vec<SegmentReady>) -> Result<Vec<TranscriptChunk>, CoreError>` — chuyển từng `SegmentReady` thành `AudioFrame`, gọi `transcribe` tuần tự, gom `Vec<TranscriptChunk>` theo thứ tự; `Vec` rỗng trả `Vec` rỗng.
- `app/src-tauri/src/core/mod.rs`: khai báo module `transcript_pipeline`, export `TranscriptPipeline`.
- `app/src-tauri/tests/transcript_pipeline_real_speech.rs` (mới): test tích hợp nạp fixture `.wav` thật (`vad_sample.wav`, tái dùng từ PBI 9-12) → `VadPipeline` → `TranscriptPipeline` + `WhisperSpeechToText`, đọc model từ `WHISPER_MODEL_PATH` (tự skip nếu thiếu, giống `stt_whisper.rs` ở PBI 12).

## Bằng chứng test (`.specify/specs/13/evidence/`)
- Unit test (mock `SpeechToText`, trong `transcript_pipeline.rs`): nhiều segment trả đúng thứ tự text — **PASS**; danh sách segment rỗng trả `Vec` rỗng không lỗi — **PASS**.
- Integration test (`transcript_pipeline_real_speech.rs`): máy chạy không có `WHISPER_MODEL_PATH` nên 2 test tích hợp **tự skip** (không fail) qua `model_or_skip()`, giống hành vi đã chấp nhận ở PBI 12 — **khuyến nghị chạy lại trên máy có model thật để xác nhận đầy đủ nhánh không-skip**.
- Toàn bộ `cargo test` (44 unit test + 8 integration test các file) pass, không crash.
- Chi tiết: `evidence/T003-T004-testcase.md`, log: `evidence/T005-cargo-test-result.log`.

## Liên quan
[[PBI-12-stt-local-whisper]], [[PBI-11-vad-segment-event]], [[Pipeline-xu-ly-audio]], [[Kien-truc-core-ui]], [[Trang-thai-ma-nguon]]
