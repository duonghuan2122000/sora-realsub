# Test case: Event transcript-partial/transcript-final (PBI 14)

**Lệnh chạy**: `cargo test` trong `app/src-tauri` (xem log đầy đủ `cargo-test-result.log` cùng thư mục)
**Kết quả**: PASS — 47 unit test (bao gồm 4 test mới trong `core::transcript_pipeline::tests`) + 8 test tích hợp, tổng 55 test, 0 fail.

## T004 — `phat_transcript_final_dung_thu_tu`

- **Input**: 3 `SegmentReady` giả (samples giống nhau), `MockSpeechToText` trả `is_final: true` tăng dần `doan-0`, `doan-1`, `doan-2`.
- **Expected**: callback nhận đúng 3 lần, theo thứ tự `(true, "doan-0")`, `(true, "doan-1")`, `(true, "doan-2")`.
- **Actual**: khớp expected. **PASS**.

## T005 — `danh_sach_rong_khong_goi_callback`

- **Input**: `segments = vec![]`.
- **Expected**: callback không được gọi lần nào (`call_count == 0`), không lỗi.
- **Actual**: khớp expected. **PASS**.

## T008 — `partial_den_truoc_final_cung_segment`

- **Input**: `ScriptedSpeechToText` trả lần lượt `TranscriptChunk { text: "xin ch", is_final: false }` rồi `TranscriptChunk { text: "xin chào", is_final: true }` cho 2 lần gọi (2 segment giả).
- **Expected**: callback nhận đúng thứ tự `(false, "xin ch")` trước, `(true, "xin chào")` sau.
- **Actual**: khớp expected. **PASS**.

## T003 (regression) — test PBI 13 cũ sau khi đổi signature `process_segments`

- `nhieu_segment_tra_ve_text_dung_thu_tu`, `danh_sach_segment_rong_tra_ve_rong_khong_loi` (unit), `pipeline_vad_stt_tra_ve_text_dung_so_luong_va_thu_tu`, `audio_toan_im_lang_khong_sinh_text_nao_khong_loi` (integration, `transcript_pipeline_real_speech.rs`, dùng model whisper thật qua `WHISPER_MODEL_PATH`).
- **Actual**: toàn bộ **PASS**, không bị ảnh hưởng bởi việc thêm tham số `on_chunk`.

## Ghi chú

- Không kiểm thử thủ công qua UI/Tauri dev (không bắt buộc cho PBI này theo quickstart.md); việc phát event `transcript-final`/`transcript-partial` qua `app_handle.emit` trong `lib.rs` (T006/T007) được xác nhận bằng code review + build thành công (`cargo build` không lỗi), logic phát event tái dùng đúng cơ chế callback đã test ở T004/T005/T008.
