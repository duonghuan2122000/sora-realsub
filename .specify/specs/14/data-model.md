# Data model: Event transcript-partial/transcript-final

**Mã PBI**: 14

## `TranscriptChunk` (đã có từ PBI 12/13, `core/types.rs`)

Không đổi field, chỉ đổi cách dùng `is_final`:

| Trường | Kiểu | Ghi chú |
|---|---|---|
| `text` | `String` | nội dung văn bản tại thời điểm chunk được sinh |
| `is_final` | `bool` | `false` → chunk này phát thành event `transcript-partial`; `true` → phát thành `transcript-final` |

## `TranscriptEventPayload` (mới, `lib.rs`, serialize camelCase cho Tauri event)

| Trường | Kiểu | Bắt buộc | Ghi chú |
|---|---|---|---|
| `startFrameIndex` | `usize` | có | lấy từ `SegmentReady.start_frame_index` của segment tương ứng |
| `endFrameIndex` | `usize` | có | lấy từ `SegmentReady.end_frame_index` của segment tương ứng |
| `text` | `String` | có | từ `TranscriptChunk.text` |
| `isFinal` | `bool` | có | từ `TranscriptChunk.is_final`; luôn khớp với tên event đã phát (`transcript-partial` ↔ `false`, `transcript-final` ↔ `true`) |

## Luồng trạng thái 1 segment

```
SegmentReady (VAD)
      │
      ▼
TranscriptPipeline.process_segments  ──(mỗi segment gọi SpeechToText.transcribe)──▶ TranscriptChunk
      │
      ├─ is_final == false ─▶ emit "transcript-partial" (TranscriptEventPayload)
      └─ is_final == true  ─▶ emit "transcript-final"   (TranscriptEventPayload)
```

Với `WhisperSpeechToText` hiện tại (luôn trả `is_final: true`), mỗi segment sinh đúng 1 event `transcript-final`, không có `transcript-partial` (xem research.md quyết định 2).
