# Danh sách Task: Chốt cặp ngôn ngữ mặc định

**Mã PBI**: 2
**Nguồn**: plan.md, spec.md

## Pha 1: Setup

- [X] T001 Không cần setup code (PBI quyết định/tài liệu, không có hạ tầng mới)

## Pha 2: User Story 1 - Ghi quyết định cặp ngôn ngữ mặc định vào tài liệu dự án (Ưu tiên: P1) 🎯 MVP

**Mục tiêu**: Người đọc tài liệu dự án biết rõ ngôn ngữ nguồn/đích mặc định và lý do chọn.
**Tiêu chí kiểm thử độc lập**: Mở `docs/tech-stack.md`, đọc thấy mục "Cặp ngôn ngữ mặc định" nêu rõ `en-US` → `vi-VN`, kèm lý do.

- [X] T002 [US1] Thêm/xác nhận mục "Cặp ngôn ngữ mặc định" trong `docs/tech-stack.md`: nguồn `en-US` (English), đích `vi-VN` (Vietnamese), kèm lý do chọn
- [X] T003 [US1] Xác nhận `roadmap.md` mục M0 "Chốt cặp ngôn ngữ mặc định" đã phản ánh đúng quyết định này (không cần sửa nếu đã khớp)

## Pha 3: Polish & Cross-cutting

- [X] T004 Chụp ảnh/snapshot nội dung `docs/tech-stack.md` sau khi cập nhật làm bằng chứng cho bước implement

## Sơ đồ phụ thuộc

```text
Setup → US1 → Polish
```

## Chiến lược triển khai

- **MVP**: chỉ User Story 1 (ghi quyết định vào tài liệu) — PBI này không có story thứ hai.
