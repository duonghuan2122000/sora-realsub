# Test case: T010 — Translator mock (kèm ngữ cảnh)

**File**: `app/src-tauri/src/core/translator.rs`

| Test | Input | Kỳ vọng | Thực tế | Kết quả |
|---|---|---|---|---|
| `dich_kem_ngu_canh` | `translate("hello", ["câu trước"])` | `"[dịch:1] hello"` | `"[dịch:1] hello"` | PASS |
| `goi_qua_trait_object` | `translate("hi", [])` qua `Box<dyn Translator>` | `"[dịch:0] hi"` | `"[dịch:0] hi"` | PASS |

Chạy bằng `cargo test core::translator::` — log đầy đủ: [T014-result.txt](T014-result.txt).
