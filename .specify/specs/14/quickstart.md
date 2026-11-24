# Quickstart: kiểm thử event transcript-partial/transcript-final

**Mã PBI**: 14

## Mục tiêu

Xác nhận `TranscriptPipeline` phát đúng event (`transcript-partial`/`transcript-final`) theo đúng thứ tự khi xử lý một chuỗi segment thoại, không cần chạy Tauri app thật.

## Bước thực hiện (test tích hợp, không cần UI/mic thật)

1. Chạy `cargo test transcript_pipeline` trong `app/src-tauri` — test dùng `MockSpeechToText` trả lần lượt text đã biết trước, kiểm tra callback nhận chunk đúng thứ tự.
2. Thêm/park test case: với 1 segment mock trả `TranscriptChunk { is_final: false }` trước, rồi gọi lại trả `is_final: true` cho cùng segment → xác nhận callback nhận đúng 2 lần, đúng thứ tự partial → final.
3. Test case audio toàn im lặng: `process_segments` với `segments = vec![]`, callback không bị gọi lần nào.
4. (Tuỳ chọn, nếu có audio mẫu thật từ PBI 12/13) chạy test tích hợp end-to-end: audio nhiều đoạn nói xen lặng → list event nhận được đúng thứ tự, đúng nội dung so với ground truth.

## Kiểm tra thủ công qua UI (khi cần, không bắt buộc cho PBI này)

1. `npm run tauri dev` trong `app/`.
2. Gọi command `start_capture`, nói vài câu có khoảng lặng giữa.
3. Dùng `window.__TAURI__.event.listen('transcript-final', console.log)` trong devtools UI để xác nhận nhận được event đúng thứ tự, nội dung khớp lời nói.
4. Gọi `stop_capture`, xác nhận không còn event nào phát ra sau đó.
