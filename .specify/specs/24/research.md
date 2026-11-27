# Research: Chỉnh cỡ chữ phụ đề (PBI 24)

## Quyết định: Lưu cỡ chữ bằng `localStorage` của webview, không dùng file config Rust

**Lý do**: Cửa sổ phụ đề (`subtitle.html`) là 1 webview Tauri riêng (PBI 20); webview (WebView2 trên Windows, WebKitGTK trên Linux) tự lưu `localStorage` theo app data dir và giữ nguyên qua các lần khởi động app, giống browser thật. Tính năng này chỉ cần giữ một số nguyên (hệ số/cỡ chữ) qua các lần mở app — `localStorage` đáp ứng đủ FR-004/FR-005 mà không cần thêm Tauri command, IPC, hay file I/O phía Rust. Giữ đúng ranh giới "Core/UI tách biệt": đây là state thuần UI, không phải nghiệp vụ Core.

**Phương án khác đã xem xét**:
- Lưu vào file config qua Tauri command (Rust đọc/ghi JSON trong app data dir): đúng hướng dùng cho M6 (cấu hình thiết bị, ngôn ngữ, vị trí cửa sổ — phạm vi lớn hơn, nhiều giá trị, cần Core sở hữu), nhưng là over-engineering cho 1 giá trị cỡ chữ đơn lẻ của riêng UI. Để lại cho M6 khi cấu hình được tập hợp chung.
- Dùng Tauri plugin `tauri-plugin-store`: thêm dependency mới chỉ để lưu 1 số — không cần thiết ở quy mô này.

## Quyết định: Cơ chế điều khiển là nút +/- hiển thị đè lên cửa sổ phụ đề

**Lý do**: Cửa sổ phụ đề không có title bar/border (PBI 20) và toàn bộ vùng đang dùng `data-tauri-drag-region` để kéo-thả (PBI 23). Thêm 2 nút nhỏ (+/-) là cách trực quan nhất, không cần phím tắt global (phức tạp hơn, cần đăng ký global shortcut Tauri) để đạt FR-001/FR-002. Nút phải nằm ngoài vùng `data-tauri-drag-region` để không bị nuốt event click (đúng rủi ro đã ghi trong `plan.md` của PBI 23).

**Phương án khác đã xem xét**:
- Phím tắt toàn cục (Ctrl+/Ctrl-): cần đăng ký global shortcut qua Tauri, phức tạp hơn và không trực quan bằng nút bấm trên chính cửa sổ. Có thể bổ sung sau nếu người dùng cần, ngoài phạm vi PBI này.

Không còn điểm NEEDS CLARIFICATION.
