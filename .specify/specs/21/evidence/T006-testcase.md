# Test case: SubtitleApp hiển thị phụ đề (T006/T007)

**File test**: `app/src/subtitle/SubtitleApp.spec.ts`
**Chạy bằng**: `npm run test` (vitest run) tại `app/`
**Log kết quả**: `evidence/T007-result.log`

## Case 1: Hiển thị text dịch khi nhận 1 event subtitle-ready

- **Input**: mock event `subtitle-ready` với `translatedText: "xin chào"`.
- **Expected**: nội dung component chứa "xin chào".
- **Actual**: pass.

## Case 2: Cập nhật đúng thứ tự khi nhận nhiều event liên tiếp (20 event)

- **Input**: phát lần lượt 20 event `translatedText` = "câu 0".."câu 19", kiểm tra sau mỗi lần phát.
- **Expected**: sau mỗi event, nội dung hiển thị đúng text vừa phát; cuối cùng chỉ còn "câu 19", không còn "câu 18".
- **Actual**: pass — khớp SC-002 (đúng thứ tự, không mất/đảo event).

## Case 3: Không hiển thị dòng trống khi translatedText rỗng

- **Input**: mock event với `translatedText: ""`.
- **Expected**: phần tử `.subtitle-text` không tồn tại trong DOM.
- **Actual**: pass.

## Kết quả tổng

`Test Files 3 passed (3)`, `Tests 8 passed (8)` (bao gồm cả test Rust/khác có sẵn trong suite; 3 test case ở trên thuộc `SubtitleApp.spec.ts`, đều PASS). Xem `T007-result.log`.
