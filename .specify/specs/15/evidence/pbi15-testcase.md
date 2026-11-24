# Test case bằng chứng: PBI 15 — Xử lý lỗi runtime model STT

**Log đầy đủ**: [cargo-test-result.log](./cargo-test-result.log) (`cargo test -- --nocapture` chạy tại `app/src-tauri`)

## T004 — Lỗi nạp model (thiếu file)

- **Test**: `core::speech_to_text::tests::nap_model_duong_dan_khong_ton_tai_tra_loi`
- **Input**: `WhisperSpeechToText::new("duong-dan-khong-ton-tai.bin")`
- **Expected**: `Err(CoreError::Provider(msg))` với `msg` không rỗng
- **Actual**: `ok` — log whisper.cpp in ra `failed to open 'duong-dan-khong-ton-tai.bin'`, hàm trả đúng `Err(CoreError::Provider(..))`
- **Kết quả**: PASS

## T009 — Lỗi infer 1 segment giữa 2 segment thành công

- **Test**: `core::transcript_pipeline::tests::loi_infer_1_segment_khong_lam_dung_cac_segment_khac`
- **Input**: 3 segment, `FlakySpeechToText` lỗi đúng ở lần gọi thứ 2 (index 1)
- **Expected**: `on_error` gọi đúng 1 lần cho segment lỗi; `chunks.len() == 2` (segment 1, 3); hàm trả `Ok`
- **Actual**: `ok` — `chunks[0].text == "doan-0"`, `chunks[1].text == "doan-2"`, `errors.len() == 1`
- **Kết quả**: PASS

## T010 — Nhiều segment liên tiếp lỗi infer

- **Test**: `core::transcript_pipeline::tests::nhieu_segment_lien_tiep_loi_infer_khong_panic`
- **Input**: 3 segment, tất cả đều lỗi infer
- **Expected**: `on_error` gọi đúng 3 lần, không panic, `chunks` rỗng, hàm trả `Ok`
- **Actual**: `ok` — `error_count == 3`, `chunks.is_empty() == true`
- **Kết quả**: PASS

## Hồi quy (không lỗi)

- **Test**: `nhieu_segment_tra_ve_text_dung_thu_tu`, `danh_sach_segment_rong_tra_ve_rong_khong_loi`, `phat_transcript_final_dung_thu_tu`, `danh_sach_rong_khong_goi_callback`, `partial_den_truoc_final_cung_segment` (PBI 13/14 cũ, đã cập nhật chữ ký `process_segments` thêm tham số `on_error`)
- **Expected**: Hành vi happy-path không đổi
- **Actual**: `ok` toàn bộ 5 test
- **Kết quả**: PASS

## Tổng kết `cargo test`

`test result: ok. 50 passed; 0 failed; 0 ignored` (unit tests) + 8 integration test khác (`segment_real_speech`, `stt_whisper`, `transcript_pipeline_real_speech`, `vad_pipeline_real_speech`, `vad_real_speech`) đều `ok`, không có test nào fail hoặc bị bỏ qua ngoài các test cần `WHISPER_MODEL_PATH` thật (đã SKIP từ trước PBI 15, không liên quan thay đổi này).
