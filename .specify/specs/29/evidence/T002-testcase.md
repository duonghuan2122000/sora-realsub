# Test case T002 — SUPPORTED_LANGUAGE_PAIRS không rỗng, nguồn != đích

**File test**: `app/src-tauri/src/core/translator.rs::tests::danh_sach_cap_ngon_ngu_khong_rong_va_moi_cap_nguon_khac_dich`

| Input | Expected | Actual | Kết quả |
|---|---|---|---|
| `SUPPORTED_LANGUAGE_PAIRS` (danh sách tĩnh) | Không rỗng; mọi phần tử có `source_lang != target_lang` | Danh sách có 1 phần tử `en-vi`, `source_lang="en"`, `target_lang="vi"` | PASS |

Xem log đầy đủ: `T013-cargo-test-result.log`.
