# Quickstart kiểm thử: Nút Start/Stop phiên dịch phụ đề (PBI 28)

1. `cd app && bun run tauri dev`.
2. Ở màn hình chính, chưa chọn mic/loopback → nút Start phải bị vô hiệu hoá.
3. Chọn mic + loopback → nút Start bấm được.
4. Bấm Start → nút hiện "Đang chạy..." rồi chuyển "Stop"; không nghe thấy lỗi, không crash app.
5. Trong lúc đang "running", thử đổi lựa chọn mic/loopback → bị khoá (dropdown disable hoặc không nhận thay đổi).
6. Bấm Stop → nút hiện "Đang dừng..." rồi về "Start".
7. Bấm Start/Stop liên tục nhiều lần nhanh trong lúc đang transition → chỉ 1 lệnh được gửi (kiểm bằng log console hoặc đếm số lần gọi `invoke`).
8. (Giả lập lỗi) Gọi `start_capture` khi đã có phiên chạy (vd. mở devtools gọi `invoke('start_capture')` tay) → nhận lỗi `AlreadyRunning`, nút UI vẫn phản ánh đúng trạng thái `running` hiện có.
