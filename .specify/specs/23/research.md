# Research: Kéo-thả cửa sổ phụ đề (PBI 23)

**Quyết định**: Dùng thuộc tính HTML gốc của Tauri `data-tauri-drag-region` trên vùng gốc của `SubtitleApp.vue`, không viết code Rust mới, không dùng JS API `window.startDragging()`.

**Lý do**: Tauri 2 xử lý kéo-thả cửa sổ native qua webview khi phần tử có `data-tauri-drag-region` — tự phân biệt click (không di chuyển) và drag (di chuyển), đúng luôn FR-005 mà không cần code thêm. Cửa sổ `decorations(false)` (PBI 20) không có title bar nên đây là cách chuẩn để bù lại vùng kéo.

**Phương án khác đã xem xét**:
- Gọi `getCurrentWindow().startDragging()` thủ công trong handler `mousedown` (JS) — cần tự viết listener, tự phân biệt click vs drag, nhiều code hơn mà không có lợi ích thêm so với `data-tauri-drag-region`.
- Thêm command Tauri Rust riêng để set vị trí cửa sổ theo toạ độ chuột — không cần, vì OS/webview đã lo phần kéo-thả, không có yêu cầu nào (FR) cần Rust can thiệp.

Không có điểm NEEDS CLARIFICATION — spec.md đã đủ, không cần thêm phụ thuộc/tích hợp mới.
