# Quickstart: Chỉnh cỡ chữ phụ đề (PBI 24)

1. Chạy `bun run tauri dev` trong `app/`.
2. Mở cửa sổ phụ đề (subtitle), kích hoạt ít nhất 1 dòng phụ đề (giả lập event `subtitle-ready` hoặc chạy pipeline thật).
3. Bấm nút "+" trên cửa sổ phụ đề → cỡ chữ các dòng đang hiển thị tăng ngay.
4. Bấm nút "-" nhiều lần → cỡ chữ giảm dần rồi dừng lại ở mức tối thiểu, không lỗi.
5. Đóng app (đóng toàn bộ, không chỉ cửa sổ phụ đề), mở lại `bun run tauri dev` → cỡ chữ cửa sổ phụ đề đúng với mức đã chỉnh ở bước 3/4, không về lại mặc định.
