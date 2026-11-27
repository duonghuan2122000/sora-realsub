# Test case T007 — `toggle_subtitle_visibility`

**File code**: `app/src-tauri/src/lib.rs`
**Loại**: `cargo test` (unit test, tự động)
**Log**: [T007-cargo-test.log](T007-cargo-test.log)

## Case 1 — `toggle_subtitle_visibility_dao_dung_trang_thai`

- **Input**: `toggle_subtitle_visibility(true)`
- **Expected**: `false`
- **Actual**: `false` — PASS
- **Input**: `toggle_subtitle_visibility(false)`
- **Expected**: `true`
- **Actual**: `true` — PASS

## Case 2 — `toggle_subtitle_visibility_nhan_lien_tuc_dung_chu_ky` (trường hợp biên: nhấn liên tục nhiều lần nhanh)

- **Input**: gọi `toggle_subtitle_visibility` 10 lần liên tiếp bắt đầu từ `true`
- **Expected**: đảo đúng chu kỳ 2 (true, false, true, false, ...)
- **Actual**: đúng chu kỳ 2 ở cả 10 lần gọi — PASS

## Kết quả tổng

`cargo test --lib toggle_subtitle` → 2 passed; 0 failed.
