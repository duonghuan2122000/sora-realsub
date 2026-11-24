# Kế hoạch triển khai: Chốt OS mục tiêu

**Mã PBI**: 1
**Liên kết spec**: .specify/specs/1/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Không áp dụng (PBI tài liệu/quyết định, không viết code) |
| Framework / Thư viện chính | Tauri 2 (hỗ trợ build đa nền tảng sẵn, không bị khoá vào 1 OS) |
| Lưu trữ dữ liệu | Không áp dụng |
| Kiểm thử | Review thủ công nội dung tài liệu (không có test tự động) |
| Nền tảng triển khai | Windows (đầu tiên), macOS (sau, Backlog) |
| Ràng buộc khác | Quyết định phải nêu rõ lý do, không chỉ chọn mặc định |

## Giai đoạn 0 — Nghiên cứu

- **Quyết định**: Windows là OS mục tiêu đầu tiên. **Lý do**: đối tượng dùng chính (họp/stream công ty) phổ biến Windows; WASAPI loopback là API ổn định, có sẵn, không cần driver phụ. **Phương án khác**: macOS trước (loại — cần BlackHole/ScreenCaptureKit phức tạp hơn để bắt loopback), cả hai song song (loại — tăng rủi ro, roadmap đã ưu tiên 1 OS trước ở M0).

## Giai đoạn 1 — Thiết kế

- Không có mô hình dữ liệu, hợp đồng giao diện, hay quickstart — PBI này chỉ ghi quyết định vào tài liệu dự án (`docs/tech-stack.md` và/hoặc wiki), không tạo code mới.

## Cấu trúc dự án dự kiến

```text
docs/tech-stack.md   # bổ sung mục "OS mục tiêu" nếu chưa có, hoặc xác nhận nội dung hiện có khớp quyết định
wiki/                # sora-wiki sẽ ingest quyết định này ở bước cuối của /sora-new-feature
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: nếu sau này đối tượng người dùng đổi sang chủ yếu macOS, quyết định này cần xét lại — chấp nhận được vì chi phí đổi hướng ở M1 còn thấp (chưa viết code audio capture).
