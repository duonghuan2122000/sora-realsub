# Test case: T008 — SpeechToText mock (partial/final)

**File**: `app/src-tauri/src/core/speech_to_text.rs`

| Test | Input | Kỳ vọng | Thực tế | Kết quả |
|---|---|---|---|---|
| `tra_ve_partial_roi_final` | gọi `transcribe()` 2 lần liên tiếp trên `MockSpeechToText` | lần 1: `is_final=false, text="xin"`; lần 2: `is_final=true, text="xin chào"` | đúng như kỳ vọng | PASS |
| `goi_qua_trait_object_khong_biet_cai_dat_cu_the` | gọi qua `Box<dyn SpeechToText>` | `is_final=false` | `is_final=false` | PASS |

Chạy bằng `cargo test core::speech_to_text::` — log đầy đủ: [T014-result.txt](T014-result.txt).
