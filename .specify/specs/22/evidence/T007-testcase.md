# Test case: T007 — xác nhận toàn bộ test SubtitleApp.spec.ts pass sau khi thêm auto-scroll/fade

**PBI**: 22
**Lệnh chạy**: `cd app && bun run test -- SubtitleApp.spec.ts`
**Log đầy đủ**: [T007-result.log](./T007-result.log)

## Input / Expected / Actual

| # | Test case | Input (các event `subtitle-ready` phát ra) | Expected | Actual |
|---|---|---|---|---|
| 1 | hiển thị text dịch khi nhận 1 event subtitle-ready | 1 event `translatedText: 'xin chào'` | Text 'xin chào' xuất hiện trong DOM | PASS |
| 2 | cập nhật đúng thứ tự khi nhận nhiều event liên tiếp, giữ tối đa 3 dòng gần nhất | 20 event liên tiếp `câu 0` → `câu 19` | Chỉ còn đúng 3 dòng cuối `['câu 17', 'câu 18', 'câu 19']`, đúng thứ tự | PASS |
| 3 | loại dòng cũ nhất khỏi DOM khi vượt giới hạn N=3 | `dòng 1`, `dòng 2`, `dòng 3`, sau đó `dòng 4` | Sau dòng 4: không còn 'dòng 1' trong DOM; còn 'dòng 2', 'dòng 3', 'dòng 4' | PASS |
| 4 | dòng mới nhất mờ ít nhất, dòng cũ hơn mờ nhiều hơn | `dòng A`, `dòng B`, `dòng C` | 3 dòng có class `subtitle-text--age-2`, `--age-1`, `--age-0` theo đúng thứ tự cũ→mới | PASS |
| 5 | không hiển thị dòng trống khi translatedText rỗng | 1 event `translatedText: ''` | Không có `.subtitle-text` nào trong DOM | PASS |

## Kết quả tổng thể

**5/5 test pass** (xem log đầy đủ tại `T007-result.log`). Đối chiếu spec.md: FR-001 → FR-005 và SC-001 → SC-003 đều được test case 2–4 xác nhận trực tiếp qua hành vi giữ 3 dòng, đúng thứ tự, loại dòng cũ nhất, độ mờ tăng dần theo độ cũ.
