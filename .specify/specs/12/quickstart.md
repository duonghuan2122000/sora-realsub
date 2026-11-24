# Quickstart: kiểm thử STT local (PBI 12)

1. Chuẩn bị model whisper ggml/gguf (ví dụ `ggml-base.bin`) tại đường dẫn cấu hình cho test (biến môi trường hoặc hằng số test, ví dụ `WHISPER_MODEL_PATH`).
2. Chuẩn bị các file audio mẫu (16kHz mono hoặc sẽ được resample trước khi test):
   - 1 file giọng rõ, ngắn (vài giây), biết trước nội dung (ground truth).
   - 1 file dài hơn, nhiều câu.
   - 1 file gần như im lặng.
3. Chạy test tích hợp STT:
   ```
   cargo test -p app --test stt_whisper -- --nocapture
   ```
4. Kỳ vọng:
   - File giọng rõ → text khớp ground truth (cho phép sai khác nhỏ dấu câu/hoa-thường).
   - File dài → text đầy đủ, đúng thứ tự.
   - File im lặng → text rỗng/gần rỗng, không panic/lỗi.
