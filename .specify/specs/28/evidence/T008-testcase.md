# Test case: Nút Start/Stop phiên dịch phụ đề (T008)

**File test**: `app/src/App.test.ts`, describe `"App - nút Start/Stop phiên dịch phụ đề"`
**Lệnh chạy**: `bun run test` (vitest run), log đầy đủ ở `T008-result.log`

| # | Input / hành động | Kỳ vọng | Kết quả |
|---|---|---|---|
| 1 | Mount App, chưa chọn mic/loopback | Nút Start hiển thị "Start", bị `disabled` | PASS |
| 2 | Chọn đủ mic+loopback, bấm Start, `start_capture` chưa resolve | Nút hiện "Đang chạy...", bị disable | PASS |
| 3 | `start_capture` resolve | Nút chuyển "Stop", `invoke` gọi đúng `start_capture` | PASS |
| 4 | Bấm Start 3 lần liên tiếp lúc đang chờ resolve | `start_capture` chỉ được gọi 1 lần | PASS |
| 5 | `start_capture` reject `"đã có phiên đang chạy"` | Nút về "Start", hiển thị thông báo lỗi đúng nội dung | PASS |
| 6 | Đang "Stop", bấm Stop, `stop_capture` resolve | Nút về "Start", `invoke` gọi đúng `stop_capture` | PASS |
| 7 | Bấm Start khi `start_capture` chưa resolve | `.device-panels` có attribute `inert` (khoá chọn thiết bị) | PASS |

Toàn bộ 7/7 trường hợp PASS, tổng 28/28 test toàn bộ suite Vitest PASS (bao gồm `App - màn hình chính chọn thiết bị` đã sửa selector `.confirm-btn` để không xung đột với nút mới, `DeviceSelector.test.ts`, `sample.test.ts`).
