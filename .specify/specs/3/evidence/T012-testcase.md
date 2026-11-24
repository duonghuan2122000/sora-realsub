# Test case: T012 — SubtitleSink mock

**File**: `app/src-tauri/src/core/subtitle_sink.rs`

| Test | Input | Kỳ vọng | Thực tế | Kết quả |
|---|---|---|---|---|
| `nhan_dung_du_lieu_subtitle` | `push(Subtitle{source_text:"hello", translated_text:"xin chào", timestamp_ms:1000})` | sink lưu đúng 1 bản ghi, `translated_text == "xin chào"` | đúng như kỳ vọng | PASS |

Chạy bằng `cargo test core::subtitle_sink::` — log đầy đủ: [T014-result.txt](T014-result.txt).
