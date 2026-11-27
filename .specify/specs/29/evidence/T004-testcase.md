# Test case T004 — list_language_pairs_cmd trả đúng danh sách

**File test**: `app/src-tauri/src/lib.rs::tests::list_language_pairs_cmd_tra_dung_danh_sach`

| Input | Expected | Actual | Kết quả |
|---|---|---|---|
| Gọi `list_language_pairs_cmd()` | Số lượng + từng field (`id`/`source_lang`/`target_lang`/`label`) khớp `SUPPORTED_LANGUAGE_PAIRS` | 1 phần tử, field khớp đúng `en-vi`/`en`/`vi`/`Anh → Việt` | PASS |

Xem log đầy đủ: `T013-cargo-test-result.log`.
