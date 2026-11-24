# Test case: T003 (unit, mock STT) & T004 (integration, audio thật)

## T003 — `core::transcript_pipeline::tests` (unit, mock `SpeechToText`)

| # | Input | Expected | Actual | Kết quả |
|---|-------|----------|--------|---------|
| 1 | `nhieu_segment_tra_ve_text_dung_thu_tu`: 3 `SegmentReady` (mock STT đánh số lần gọi: "doan-0","doan-1","doan-2") | `Vec<TranscriptChunk>` dài 3, đúng thứ tự "doan-0","doan-1","doan-2" | `[doan-0, doan-1, doan-2]` | PASS |
| 2 | `danh_sach_segment_rong_tra_ve_rong_khong_loi`: `Vec::new()` segment | `Ok(Vec::new())`, không lỗi | `Ok([])` | PASS |

## T004 — `tests/transcript_pipeline_real_speech.rs` (integration, cần `WHISPER_MODEL_PATH`)

Môi trường chạy test hiện tại **không có** biến `WHISPER_MODEL_PATH` (không có model whisper cài sẵn) — test tự nhận biết qua `model_or_skip()` và bỏ qua phần kiểm thử thật, vẫn trả `ok` (không fail), giống hành vi đã chấp nhận ở PBI 12 (`stt_whisper.rs`).

| # | Input | Expected | Actual | Kết quả |
|---|-------|----------|--------|---------|
| 1 | `pipeline_vad_stt_tra_ve_text_dung_so_luong_va_thu_tu`: file `tests/fixtures/vad_sample.wav` qua `VadPipeline` → `TranscriptPipeline` + `WhisperSpeechToText` | Số đoạn text khớp số segment VAD, ≥1 đoạn text không rỗng | SKIP (thiếu `WHISPER_MODEL_PATH`), test không fail | PASS (self-skip, xem log) |
| 2 | `audio_toan_im_lang_khong_sinh_text_nao_khong_loi`: audio im lặng 3s qua `VadPipeline` → `TranscriptPipeline` | 0 segment, 0 text, không lỗi | SKIP (thiếu `WHISPER_MODEL_PATH`), test không fail | PASS (self-skip, xem log) |

Log chạy đầy đủ (toàn bộ `cargo test`, gồm cả 2 test trên và các test unit T003): xem [T005-cargo-test-result.log](T005-cargo-test-result.log).

**Khuyến nghị**: chạy lại trên máy có `WHISPER_MODEL_PATH` trỏ tới model ggml/gguf thật để xác nhận đầy đủ nhánh không-skip (giống rủi ro đã ghi ở PBI 12, chưa phát sinh thêm ở PBI 13).
