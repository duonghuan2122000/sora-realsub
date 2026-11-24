# Hợp đồng sự kiện Tauri: `transcript-partial` / `transcript-final`

**Hướng**: Core → UI (Tauri event, không phải command)
**Tên sự kiện**: `transcript-partial`, `transcript-final`
**Tần suất**: phát mỗi khi `TranscriptPipeline` nhận được `TranscriptChunk` từ `SpeechToText` cho 1 segment thoại, trong lúc phiên capture đang chạy; ngừng sau `stop_capture`

## Payload (JSON) — chung cho cả 2 event

```json
{
  "startFrameIndex": 0,
  "endFrameIndex": 7,
  "text": "xin chào",
  "isFinal": true
}
```

| Trường | Kiểu | Bắt buộc | Ghi chú |
|---|---|---|---|
| `startFrameIndex` | number (int ≥ 0) | có | chỉ số frame VAD bắt đầu segment, khớp `vad-segment` đã phát trước đó cho segment này |
| `endFrameIndex` | number (int ≥ `startFrameIndex`) | có | chỉ số frame VAD kết thúc segment |
| `text` | string | có | nội dung văn bản tại thời điểm chunk được sinh; có thể rỗng nếu STT không nhận diện được |
| `isFinal` | boolean | có | `true` khi phát qua event `transcript-final`, `false` khi phát qua `transcript-partial` (trường này trùng tên event, giữ lại để UI không cần dựa vào tên event nếu lắng nghe chung 1 handler) |

## Đảm bảo thứ tự & quan hệ partial → final (FR-002, FR-003)

- UI nhận các event theo đúng thứ tự thời gian segment phát sinh (khớp `startFrameIndex` tăng dần giữa các segment khác nhau).
- Với cùng 1 segment (cùng `startFrameIndex`/`endFrameIndex`), nếu có `transcript-partial`, event đó luôn đến trước `transcript-final` của chính segment đó.
- Với `SpeechToText` hiện tại (whisper.cpp, luôn trả `is_final: true`), mỗi segment chỉ phát đúng 1 event `transcript-final`, không có `transcript-partial` đi kèm — UI phải xử lý được trường hợp chỉ nhận `transcript-final` mà không có `transcript-partial` trước đó.
- Audio không có segment thoại nào → không event nào thuộc 2 loại này được phát (FR-005).
