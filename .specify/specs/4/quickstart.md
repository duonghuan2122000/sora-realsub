# Quickstart: kiểm thử PBI 4 — Liệt kê thiết bị âm thanh đầu vào

## Kiểm thử Core (Rust, không cần UI)

```powershell
cd app/src-tauri
cargo test list_input_devices
```

Kỳ vọng: test dùng danh sách giả lập (mock) phủ các case — có thiết bị, không có thiết bị, thiết bị không tên, lỗi truy vấn — pass hết, không cần phần cứng thật.

## Kiểm thử thủ công qua UI

1. `cd app && npm run tauri dev`
2. Mở màn hình chọn thiết bị (hoặc gọi command qua DevTools console: `window.__TAURI__.core.invoke('list_input_devices')`)
3. Xác nhận: danh sách hiển thị đúng số mic đang cắm trên máy, có đánh dấu thiết bị mặc định.
4. Rút mic ngoài (nếu có) → bấm làm mới → xác nhận danh sách cập nhật đúng.
