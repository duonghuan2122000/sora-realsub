# Research: Ghép STT vào luồng VAD → Transcript (PBI 13)

## Quyết định 1: Cơ chế ghép VAD → STT

**Quyết định**: Thêm một bộ điều phối mới (`TranscriptPipeline`, cùng thư mục `core/`) nhận từng `SegmentReady` do `VadPipeline::push_frame` trả ra, chuyển `samples`/`sample_rate` của nó thành `AudioFrame`, gọi `SpeechToText::transcribe` (trait đã có ở PBI 12), và tích luỹ `TranscriptChunk` trả về vào một `Vec` theo đúng thứ tự gọi.

**Lý do**: `VadPipeline` đã trả `Vec<SegmentReady>` theo đúng thứ tự thời gian mỗi lần `push_frame` (PBI 11). `SpeechToText` đã có sẵn trait + cài đặt whisper.cpp (PBI 12), nhận `AudioFrame`. Chỉ cần một lớp mỏng nối hai khả năng đã có, không đổi gì ở VAD hoặc STT.

**Phương án khác đã xem xét**:
- Gọi STT trực tiếp ngay trong `VadPipeline::push_frame` — bị bác vì vi phạm tách biệt trách nhiệm (VAD không nên biết về STT), và test VAD hiện tại (PBI 9-11) sẽ phải mock STT không cần thiết.
- Dùng channel/queue bất đồng bộ (mpsc) giữa VAD và STT — chưa cần ở PBI này vì pipeline vẫn chạy trên file mẫu (đồng bộ, không phải luồng capture realtime); để lại cho PBI nối vào audio capture thời gian thực sau này.

## Quyết định 2: Giữ đúng thứ tự thời gian

**Quyết định**: Xử lý tuần tự từng `SegmentReady` theo đúng thứ tự trong `Vec` trả về từ `push_frame`, và gọi `await` lần lượt (không chạy song song `join_all`), đẩy từng `TranscriptChunk` vào buffer kết quả ngay khi có.

**Lý do**: FR-002 yêu cầu đúng thứ tự thời gian. Xử lý song song (concurrent futures) có thể hoàn thành không theo thứ tự submit, phải sắp xếp lại — không cần thiết ở quy mô PBI này (vài segment mỗi lần push_frame). Tuần tự đơn giản, đúng theo yêu cầu, dễ test.

**Phương án khác đã xem xét**: `futures::future::join_all` chạy song song — nhanh hơn nhưng cần logic sắp xếp lại theo index, over-engineering so với yêu cầu hiện tại.

## Quyết định 3: Input test end-to-end

**Quyết định**: Test end-to-end dùng file audio mẫu (giống cách PBI 9-12 đã làm: audio thật nạp bằng `hound`/tương tự, chạy qua từng `AudioFrame` đẩy vào `VadPipeline`, rồi qua `TranscriptPipeline`), không dựng audio capture mic/loopback thật.

**Lý do**: Spec PBI 13 (Giả định) đã chốt: đầu vào vẫn là file mẫu, chưa yêu cầu luồng capture thời gian thực. Nhất quán với cách test VAD (PBI 9-11) và STT (PBI 12) đã làm.

**Phương án khác đã xem xét**: Dùng audio capture thật qua `AudioSource` — ngoài phạm vi PBI này (thuộc PBI tích hợp pipeline đầy đủ sau).
