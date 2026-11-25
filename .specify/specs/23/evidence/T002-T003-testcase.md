# Test case: T002/T003 — kiểm tra attribute kéo-thả (unit test)

**Input**: mount `SubtitleApp.vue`, lấy attribute `data-tauri-drag-region` của `.subtitle-overlay`.
**Expected**: attribute tồn tại (không undefined), tất cả 11 test trong 3 file test (bao gồm 6 test cũ của `SubtitleApp.spec.ts`) đều pass.
**Actual**: `bun run test` (Vitest) → `Test Files 3 passed (3)`, `Tests 11 passed (11)`. Xem log đầy đủ tại `T003-result.log`.
**Kết quả**: PASS
