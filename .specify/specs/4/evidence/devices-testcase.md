# Test case: PBI 4 — Liệt kê thiết bị âm thanh đầu vào

## Core (Rust, `cargo test`, log: `T007-result.log`)

| # | Input (mock `AudioHost`) | Expected | Actual | Kết quả |
|---|---|---|---|---|
| 1 | 2 thiết bị, Mic B là default | Trả 2 thiết bị, Mic B có `is_default=true`, Mic A `false` | Đúng như kỳ vọng | PASS |
| 2 | Danh sách rỗng | Trả `Vec` rỗng, không lỗi | Đúng | PASS |
| 3 | Thiết bị tên rỗng | Hiển thị `"Thiết bị không tên"` | Đúng | PASS |
| 4 | Host trả lỗi | Trả `CoreError::Io` | Đúng (khớp qua `matches!`) | PASS |

Lệnh: `cargo test devices` trong `app/src-tauri` — 4/4 pass.

## UI (`vitest run`, log: `T009-vitest-result.log`)

| # | Tình huống | Expected | Actual | Kết quả |
|---|---|---|---|---|
| 1 | `invoke` trả 2 thiết bị | Hiển thị tên cả 2, đánh dấu "(mặc định)" cho thiết bị default | Đúng | PASS |
| 2 | `invoke` trả rỗng | Hiển thị "Không tìm thấy thiết bị microphone nào" | Đúng | PASS |
| 3 | `invoke` reject lỗi | Hiển thị nội dung lỗi cho người dùng | Đúng | PASS |
| 4 | Bấm nút "Làm mới" | Gọi lại `invoke` lần 2 | Đúng (`toHaveBeenCalledTimes(2)`) | PASS |

Lệnh: `npm test` trong `app` — 5/5 pass (4 case trên + 1 test mẫu `sample.test.ts` có sẵn).

## Kiểm thử thủ công (chưa chạy — cần máy thật có mic)

Theo `quickstart.md`: chạy `npm run tauri dev`, mở app, xác nhận danh sách mic thật hiển thị đúng và nút làm mới hoạt động khi cắm/rút thiết bị. Bước này cần chạy thủ công trên máy có mic thật, không tự động hoá được trong môi trường agent — đề nghị người dùng xác nhận khi có điều kiện.
