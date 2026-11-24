# Danh sách Task: Chốt OS mục tiêu

**Mã PBI**: 1
**Nguồn**: plan.md, spec.md

## Pha 1: Setup

- [x] T001 Không cần setup code (PBI quyết định/tài liệu, không có hạ tầng mới)

## Pha 2: User Story 1 - Ghi quyết định OS mục tiêu vào tài liệu dự án (Ưu tiên: P1) 🎯 MVP

**Mục tiêu**: Người đọc tài liệu dự án biết rõ OS mục tiêu đầu tiên và kế hoạch cho OS thứ hai.
**Tiêu chí kiểm thử độc lập**: Mở `docs/tech-stack.md`, đọc thấy mục "OS mục tiêu" nêu rõ Windows trước, macOS sau (Backlog), kèm lý do.

- [x] T002 [US1] Thêm/xác nhận mục "OS mục tiêu" trong `docs/tech-stack.md`: Windows là OS đầu tiên (lý do: đối tượng dùng chính dùng Windows, WASAPI loopback sẵn có), macOS thuộc Backlog
- [x] T003 [US1] Xác nhận `roadmap.md` mục Backlog "Hỗ trợ macOS" đã phản ánh đúng thứ tự ưu tiên này (không cần sửa nếu đã khớp)

## Pha 3: Polish & Cross-cutting

- [x] T004 Chụp ảnh/snapshot nội dung `docs/tech-stack.md` sau khi cập nhật làm bằng chứng cho bước implement

## Sơ đồ phụ thuộc

```text
Setup → US1 → Polish
```

## Chiến lược triển khai

- **MVP**: chỉ User Story 1 (ghi quyết định vào tài liệu) — PBI này không có story thứ hai.
