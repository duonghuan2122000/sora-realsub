# Test case T007 — validate_language_pair với pair_id hợp lệ/không hợp lệ

**File test**: `app/src-tauri/src/lib.rs::tests::validate_language_pair_pair_id_hop_le_tra_ok`,
`...validate_language_pair_pair_id_khong_hop_le_tra_loi`

| Input | Expected | Actual | Kết quả |
|---|---|---|---|
| `pair_id = "en-vi"` (cặp hợp lệ) | `Ok(())` | `Ok(())` | PASS |
| `pair_id = "khong-ton-tai"` | `Err` chứa `"khong-ton-tai"` | `Err("cặp ngôn ngữ không được hỗ trợ: khong-ton-tai")` | PASS |

`start_capture` gọi `validate_language_pair(&pair_id)?` trước khi tạo session — `pair_id` không khớp sẽ trả `Err` ngay, không gọi `session.start(...)` (FR-001/FR-005, xem `app/src-tauri/src/lib.rs`).

Xem log đầy đủ: `T013-cargo-test-result.log`.
