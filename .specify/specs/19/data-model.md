# Mô hình dữ liệu: PBI 19 — Nối STT final → Dịch → subtitle-ready

## `SubtitleReadyEvent` (mới, phía Core `lib.rs`)

Payload sự kiện Tauri `subtitle-ready`, phát sau khi dịch xong 1 câu transcript final.

| Trường | Kiểu | Mô tả |
|---|---|---|
| `start_frame_index` | `usize` | Vị trí bắt đầu segment gốc (giống `TranscriptEventPayload`, để UI/transcript sau này đối chiếu thời gian) |
| `end_frame_index` | `usize` | Vị trí kết thúc segment gốc |
| `source_text` | `String` | Câu gốc (ngôn ngữ nguồn) — chính là `chunk.text` của transcript final |
| `translated_text` | `String` | Câu đã dịch (ngôn ngữ đích) |

Ràng buộc:
- Chỉ phát sinh cho `chunk.is_final == true` và `chunk.text` không rỗng/không chỉ khoảng trắng (FR-005, FR-006).
- Thứ tự các sự kiện trên 1 phiên PHẢI khớp thứ tự thời gian các `transcript-final` gốc tương ứng (FR-004) — tự nhiên đúng vì phát trong cùng callback `on_chunk` tuần tự của `TranscriptPipeline`.

## `ModelErrorKind::Translate` (mở rộng enum đã có từ PBI 15)

Biến thể mới trong enum lỗi runtime hiện có (`Load`, `Infer`), dùng khi bước dịch 1 câu final lỗi (FR-007). Payload `ModelErrorPayload` dùng lại nguyên trạng (không đổi cấu trúc), `segment_start_frame_index` set theo segment của câu lỗi.

## Luồng dữ liệu trong 1 lần xử lý 1 câu final

```
SegmentReady + TranscriptChunk(final, text)
        │
        ├─ text rỗng/khoảng trắng? ──► bỏ qua (không dịch, không push context, không event)
        │
        ▼ (text hợp lệ)
  context = RecentTranscriptContext.recent()   (0..=2 câu gốc, cũ→mới)
        │
        ▼
  translator.translate(text, &context)
        │
        ├─ Err ──► emit "model-error" { kind: "translate", ... } ; vẫn push(text) vào context
        │
        ▼ Ok(translated_text)
  emit "subtitle-ready" { start_frame_index, end_frame_index, source_text: text, translated_text }
        │
        ▼
  RecentTranscriptContext.push(text)
```

Không có entity mới cần persist — toàn bộ sống trong bộ nhớ của 1 phiên capture (giống `RecentTranscriptContext` ở PBI 17).
