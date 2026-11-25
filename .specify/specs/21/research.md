# Research: Hiển thị phụ đề (PBI 21)

Không có điểm `NEEDS CLARIFICATION` nào cần nghiên cứu — công nghệ, event, payload đều đã tồn tại sẵn trong codebase từ các PBI trước.

- **Quyết định**: Dùng `listen` từ `@tauri-apps/api/event` để subscribe event `subtitle-ready` trong `SubtitleApp.vue`, lưu `translatedText` mới nhất vào `ref` và render trong template.
  - **Lý do**: `@tauri-apps/api` đã là dependency có sẵn của dự án; event `subtitle-ready` + payload (`SubtitlePayload`) đã được Core phát ra từ PBI 19, không cần đổi gì phía Rust.
  - **Phương án khác đã xem xét**: Polling trạng thái qua Tauri command — bị loại vì tốn tài nguyên hơn và không cần thiết khi đã có cơ chế event đẩy (push) sẵn.

- **Quyết định**: Chỉ giữ 1 dòng phụ đề hiện tại trong state (không giữ lịch sử nhiều dòng).
  - **Lý do**: Đúng phạm vi PBI này (xem spec.md mục Giả định/Ngoài phạm vi); auto-scroll/nhiều dòng là mục roadmap riêng kế tiếp.
  - **Phương án khác đã xem xét**: Giữ mảng lịch sử — bị loại vì ngoài phạm vi, tăng rủi ro lệch với tính năng auto-scroll sẽ làm sau.
