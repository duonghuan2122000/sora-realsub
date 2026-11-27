# Quickstart kiểm thử: PBI 26 — Màn hình chính chọn thiết bị

1. Chạy `bun run tauri dev` trong `app/`.
2. Màn hình chính hiển thị 2 danh sách: "Chọn microphone" và "Chọn nguồn loopback".
3. Chọn 1 mic + 1 nguồn loopback → bấm "Xác nhận" → không có lỗi hiển thị, lựa chọn được giữ lại trên UI.
4. Bấm "Xác nhận" khi chưa chọn đủ cả hai → thấy thông báo lỗi nêu rõ còn thiếu mic hay loopback.
5. (Thủ công, cần máy thật) Rút/tắt thiết bị mic đang chọn, bấm "Làm mới" → thiết bị đó biến mất khỏi danh sách, lựa chọn bị xoá, hiển thị cảnh báo thiết bị không còn khả dụng.
