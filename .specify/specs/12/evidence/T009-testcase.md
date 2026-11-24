# Test case: T009 — STT local bằng whisper.cpp (`stt_whisper.rs`)

**Model dùng test**: `ggml-tiny.bin` (whisper.cpp, tải từ https://huggingface.co/ggerganov/whisper.cpp)
**Fixture**: `app/src-tauri/tests/fixtures/vad_sample.wav` (tái dùng từ PBI 9-10, ghi âm thật, 16kHz mono f32, ~24s) + 1 buffer im lặng 2s sinh trong test.

## Case 1: `giong_noi_thuc_tra_ve_text_khong_rong`

- **Input**: `AudioFrame` đọc từ `vad_sample.wav` (có giọng nói thật).
- **Expected**: `transcribe()` không lỗi, `is_final = true`, `text` không rỗng.
- **Actual**: pass. Model nhận diện ra text (có hallucination do model `tiny` chất lượng thấp — không phủ nhận yêu cầu spec vì spec chỉ cam kết text đọc hiểu được, không cam kết chính xác 100% với model nhỏ).
- **Kết quả**: PASS.

## Case 2: `audio_im_lang_tra_ve_text_rong_khong_crash`

- **Input**: `AudioFrame` 2 giây toàn giá trị `0.0` (im lặng).
- **Expected**: `transcribe()` không lỗi/panic, `text` rỗng hoặc gần rỗng.
- **Actual (lần 1, trước fix)**: whisper.cpp trả segment text literal `"[BLANK_AUDIO]"` — không rỗng → FAIL.
  - Điều chỉnh: thêm bước lọc segment dạng tag `[...]` (non-speech marker của whisper.cpp) khi ghép text tại `speech_to_text.rs`.
- **Actual (lần 2, sau fix)**: `text` rỗng (`""`) → PASS.
- **Kết quả**: PASS (sau 1 lần điều chỉnh).

## Lệnh chạy

```
cargo test -p app --test stt_whisper -- --nocapture
```

Log đầy đủ: [T009-result.log](T009-result.log)

## Giới hạn đã biết

- Test không assert khớp chính xác ground-truth text của `vad_sample.wav` (không có transcript gốc đáng tin cho file này) — chỉ assert "không rỗng" cho case có giọng nói, theo đúng mức SC-001 (đọc hiểu được, không yêu cầu khớp tuyệt đối).
- Model `tiny` dùng cho test nhẹ/nhanh, không phải model production cuối cùng — việc chọn model cụ thể cho sản phẩm thuộc quyết định khác ngoài phạm vi PBI này.
