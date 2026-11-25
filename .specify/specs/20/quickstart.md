# Quickstart: Kiểm thử thủ công cửa sổ phụ đề always-on-top (PBI 20)

Dành cho người xác nhận lại tính năng trên máy có GUI thật (Windows hoặc Linux có desktop environment).

## Chuẩn bị

```bash
cd app
bun install
bun run tauri dev
```

## Bước kiểm thử

1. **Khởi động app** (`bun run tauri dev`).
   - Kỳ vọng: ngoài cửa sổ chính (`main`, "app"), một cửa sổ thứ hai (`subtitle_overlay`) xuất hiện trong vòng 2 giây.
   - Chụp ảnh màn hình lúc này làm bằng chứng cho SC-001 (quan sát: không có viền, không có title bar).

2. **Kiểm tra không viền/title bar**: nhìn cửa sổ `subtitle_overlay` — không có khung viền hệ điều hành, không có thanh tiêu đề, không có nút minimize/maximize/close.

3. **Kiểm tra always-on-top**: mở ít nhất 2 ứng dụng khác (ví dụ trình duyệt, một ứng dụng văn bản), lần lượt click để focus từng ứng dụng đó, đưa cửa sổ của chúng full-screen hoặc che vị trí cửa sổ `subtitle_overlay`.
   - Kỳ vọng: cửa sổ `subtitle_overlay` luôn hiển thị nổi ở trên, không bị ẩn xuống dưới, trong toàn bộ các lần chuyển đổi đã thử (SC-002).
   - Chụp ảnh ít nhất 1 lần cửa sổ overlay nổi trên 1 ứng dụng khác đang full-screen/focus làm bằng chứng.

4. **Kiểm tra không giành focus**: sau khi app khởi động, gõ thử vào ô nhập liệu của cửa sổ `main` (nếu có) hoặc một ứng dụng khác đang mở — xác nhận ký tự gõ vào đúng ô đang focus, không bị cửa sổ overlay "cướp" bàn phím.

5. **Đóng app** (đóng cửa sổ `main` hoặc Ctrl+C tiến trình `tauri dev`).
   - Kỳ vọng: cửa sổ `subtitle_overlay` đóng theo, không còn tiến trình/cửa sổ nào của app còn tồn tại.
   - Xác nhận bằng Task Manager (Windows) hoặc `ps aux | grep app` / danh sách cửa sổ của window manager (Linux) — không còn process `app`/`app.exe` nào (SC-003).

## Ghi lại kết quả

Lưu ảnh chụp + ghi chú (OS, window manager/desktop environment nếu Linux) vào `.specify/specs/20/evidence/` để làm bằng chứng cho bước `/sora-implement`.
