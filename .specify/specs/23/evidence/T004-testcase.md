# Test case: T004 — kéo-thả cửa sổ phụ đề (test tay, Windows thật)

**Input**: `bun run tauri dev`, kéo giữ chuột trái lên vùng cửa sổ phụ đề (trong suốt), di chuyển, thả chuột.
**Expected**: cửa sổ di chuyển theo con trỏ và đứng đúng vị trí thả (SC-001).
**Actual (lần 1)**: FAIL — cửa sổ trong suốt, không kéo được. Nguyên nhân: `app/src-tauri/capabilities/default.json` chỉ gán `windows: ["main"]`, window `subtitle` không có capability nào nên lệnh `core:window:allow-start-dragging` phía dưới `data-tauri-drag-region` bị ACL chặn âm thầm.
**Khắc phục**: thêm `app/src-tauri/capabilities/subtitle.json` — `windows: ["subtitle"]`, `permissions: ["core:window:allow-start-dragging"]`.
**Actual (lần 2, sau fix)**: PASS — người dùng xác nhận "đã kéo được rồi".
**Kết quả**: PASS (sau 1 lần sửa capability)
