# Quickstart: kiểm thử nhanh pipeline VAD → STT (PBI 13)

1. Chuẩn bị file audio mẫu `.wav` đã biết trước nội dung từng đoạn nói (tái dùng mẫu đã có từ PBI 9-12 nếu phù hợp), gồm ít nhất: 1 file nhiều đoạn nói xen lặng, 1 file chỉ 1 đoạn nói, 1 file toàn im lặng.
2. Nạp file `.wav` thành các `AudioFrame` (theo cách đã dùng ở test VAD PBI 9-11), đẩy tuần tự vào `VadPipeline::push_frame` để lấy ra các `Vec<SegmentReady>`.
3. Gộp toàn bộ `SegmentReady` thu được (theo đúng thứ tự) và gọi `TranscriptPipeline::process_segments`, dùng `WhisperSpeechToText` (PBI 12) với model ggml/gguf cục bộ đã có.
4. Kiểm tra `Vec<TranscriptChunk>` trả về: đúng số lượng đoạn, đúng thứ tự, nội dung từng đoạn khớp nghĩa với đoạn đã nói.
5. Chạy lại bước 2-4 với file toàn im lặng → kỳ vọng `Vec` kết quả rỗng, không panic/lỗi.

Test tích hợp cụ thể đặt tại `app/src-tauri/tests/` (theo cách `vad_pipeline_real_speech.rs` đã làm ở PBI 11), ví dụ `transcript_pipeline_real_speech.rs`.
