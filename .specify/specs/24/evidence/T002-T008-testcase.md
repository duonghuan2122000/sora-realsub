# Test case: T002-T008 — Chỉnh cỡ chữ phụ đề (PBI 24)

**File test**: `app/src/subtitle/SubtitleApp.spec.ts` (describe block `SubtitleApp - chỉnh cỡ chữ (PBI 24)`)
**Lệnh chạy**: `bun run test` (vitest run)
**Log output**: [T002-T008-unit-test-result.log](./T002-T008-unit-test-result.log)

| # | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| 1 | Bấm nút `.font-size-btn--increase` 1 lần, sau đó `.font-size-btn--decrease` 2 lần | cỡ chữ sau increase > ban đầu; cỡ chữ sau 2 lần decrease < cỡ sau increase | Đúng như expected | PASS |
| 2 | Bấm `.font-size-btn--decrease` 50 lần liên tục rồi `.font-size-btn--increase` 50 lần liên tục | cỡ chữ dừng ở min (>0) rồi dừng ở max, bấm tăng thêm 1 lần nữa không vượt max | Đúng như expected | PASS |
| 3 | `localStorage.setItem('subtitle-font-size', '2.25')` trước khi mount | cỡ chữ ban đầu = 2.25rem | `--subtitle-font-size` = `2.25rem` | PASS |
| 4 | `localStorage.setItem('subtitle-font-size', 'không-phải-số')` trước khi mount | cỡ chữ ban đầu = mặc định 1.5rem, không lỗi | `--subtitle-font-size` = `1.5rem` | PASS |
| 5 | Bấm `.font-size-btn--increase` 1 lần | `localStorage.getItem('subtitle-font-size')` khớp giá trị cỡ chữ hiện tại | Khớp | PASS |

**Tổng kết**: 16/16 test trong suite (bao gồm 5 test mới của PBI 24 + 11 test cũ của PBI 20-23) PASS. Không có test fail, không cần retry.
