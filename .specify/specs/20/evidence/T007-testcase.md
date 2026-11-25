# Test case: T007 — Kiểm thử thủ công cửa sổ phụ đề overlay

**Môi trường thực tế dùng để implement**: container cloud Linux, không có window manager thật,
chỉ có `Xvfb` (X server ảo, không compositing/WM) + `xwininfo`/`xprop`/`import` (ImageMagick).
Đây **không phải** máy GUI thật như `quickstart.md` yêu cầu — không có window manager nên
**không thể xác nhận trực quan hành vi always-on-top thật (SC-002)**.

## Các bước đã thực hiện (best-effort trong container)

1. Khởi động `Xvfb :77` + chạy `bun run tauri dev` với `DISPLAY=:77`.
2. Chụp ảnh toàn màn hình: `T007-overlay-startup.png`.
3. Liệt kê cây cửa sổ X11: `xwininfo -root -tree` → thấy 2 webview window cấp cao nhất
   kích thước 800x600: label title "app" (cửa sổ `main`, theo `tauri.conf.json`) và
   "Tauri App" (title mặc định — suy ra là `subtitle_overlay` vì không có title custom).
4. Đọc thuộc tính X11 (`xprop`) của từng cửa sổ, lưu tại `T007-xprop-result.log`.
5. Dừng toàn bộ tiến trình (`pkill`), xác nhận không còn `app`/`vite`/`Xvfb` nào chạy (`ps aux`).

## Kết quả đối chiếu với tiêu chí thành công

| Tiêu chí | Kết quả trong container (Xvfb, không WM) | Kết luận |
|---|---|---|
| FR-001: cửa sổ riêng được tạo | 2 window cấp cao nhất tồn tại (main + overlay) | **Xác nhận được** |
| FR-002: không viền/title bar | Window suy ra là overlay ("Tauri App") có `_MOTIF_WM_HINTS = 0x2, 0x0, ...` (flags=2=MWM_HINTS_DECORATIONS, decorations=0 → yêu cầu không trang trí); window `main` ("app") không có `_MOTIF_WM_HINTS` (dùng mặc định có trang trí) | **Xác nhận được qua hint gửi tới WM** — nhưng không có WM thật để xác nhận WM *tuân theo* hint này (hiển thị thực tế không viền) |
| FR-003/SC-002: always-on-top thật khi chuyển đổi ứng dụng | `_NET_WM_STATE` không có giá trị nào trên cả 2 cửa sổ — vì không có window manager chạy để nhận và phản hồi property này | **KHÔNG xác nhận được** trong container này — cần máy GUI thật có WM |
| FR-004: không giành focus | Không có WM nên không có khái niệm "focus" thật để kiểm tra | **KHÔNG xác nhận được** trong container này |
| SC-001: xuất hiện ≤2s | Ảnh chụp cho thấy cửa sổ `main` render xong trong vài giây sau khi chạy `tauri dev` (dev build, có compile/launch time; bản release sẽ nhanh hơn) | Quan sát gián tiếp, không đo chính xác bằng giây — cần xác nhận lại trên máy GUI thật |
| SC-003: không leak process khi đóng app | `pkill` toàn bộ tiến trình `app`, `vite`, `Xvfb`; `ps aux` sau đó không còn tiến trình nào của app | **Xác nhận được** |

## Kết luận

Phần cấu hình có thể xác nhận bằng công cụ dòng lệnh (FR-001, FR-002 ở mức "gửi đúng hint tới
WM", SC-003) đã **PASS** trong container này. Phần hành vi thị giác thật đòi hỏi window manager
thật (FR-003/SC-002 always-on-top, FR-004 focus, SC-001 đo chính xác thời gian xuất hiện bằng
mắt) **CHƯA xác nhận được** — cần người có máy Windows hoặc Linux desktop thật chạy lại theo
đúng các bước trong `quickstart.md`, giống cách PBI 17/18 đã xử lý trước đây trong dự án này.
Không tick dòng roadmap M5 tương ứng cho đến khi có xác nhận trực quan thật.
