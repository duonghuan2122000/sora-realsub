# Test case T005: Cửa sổ phụ đề always-on-top, không viền

**Input**: Chạy `bun run tauri dev` từ `app/`, quan sát cửa sổ phụ đề (`subtitle.html`) khi focus lần lượt sang ≥3 ứng dụng khác.

**Expected**:
- Cửa sổ phụ đề không có border/title bar (SC-002).
- Cửa sổ phụ đề luôn nằm trên ứng dụng vừa focus (SC-001).

**Actual (lần 1, môi trường implement)**: KHÔNG THỰC HIỆN ĐƯỢC — môi trường là container không có GUI/display thật, không thể chạy `tauri dev` và quan sát cửa sổ native hay chụp ảnh màn hình thật.

**Actual (lần 2, 2026-11-25, máy Windows thật)**: Chạy `bun run tauri dev` trong terminal thật trên máy Windows của người dùng (session desktop app), app khởi động thành công không crash (`Finished dev profile`, `Running target\debug\app.exe`). Người dùng xác nhận trực tiếp trên màn hình thật: cửa sổ phụ đề không có border/title bar, và luôn nổi trên các ứng dụng khác (always-on-top) — cả SC-001 và SC-002 đạt.

**Kết quả**: PASS (xác nhận bởi người dùng trên máy Windows thật, không phải ảnh chụp do công cụ không chụp được cửa sổ desktop native). Code đã biên dịch sạch trước đó (T004 — xem `T004-cargo-build.log`).
