# Test case T011 — LanguagePairSelector.vue

**File test**: `app/src/components/LanguagePairSelector.test.ts`

| Kịch bản | Expected | Actual | Kết quả |
|---|---|---|---|
| `invoke` trả 1 cặp `en-vi` | Hiển thị nhãn "Anh → Việt" | Hiển thị đúng | PASS |
| `invoke` trả danh sách rỗng | Hiển thị "Không có cặp ngôn ngữ nào được hỗ trợ." | Hiển thị đúng | PASS |
| `invoke` reject | Hiển thị thông báo lỗi chứa nội dung lỗi | Hiển thị đúng | PASS |
| Chọn cặp khác trong `<select>` | Emit `update:selected` với đúng `id` | `[["vi-en"]]` | PASS |
| Prop `disabled = true` | `<select>` có attribute `disabled` | Có | PASS |

Xem log đầy đủ: `T013-vitest-result.log`.
