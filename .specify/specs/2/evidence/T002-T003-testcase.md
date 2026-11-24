# Test case: Ghi quyết định cặp ngôn ngữ mặc định vào tài liệu

**Task**: T002, T003
**Loại kiểm thử**: Review thủ công nội dung tài liệu (SC-001 trong spec.md)

## Input

- File: `docs/tech-stack.md`, mục mới "Cặp ngôn ngữ mặc định"
- File: `roadmap.md`, dòng M0 "Chốt cặp ngôn ngữ mặc định (ví dụ English → Vietnamese)"

## Expected

- `docs/tech-stack.md` có mục "Cặp ngôn ngữ mặc định" nêu rõ nguồn `en-US`, đích `vi-VN`, kèm lý do.
- `roadmap.md` M0 đã phản ánh đúng quyết định (dòng mô tả ví dụ English → Vietnamese khớp quyết định đã chọn).

## Actual

- `docs/tech-stack.md:83-85` — đã thêm mục "Cặp ngôn ngữ mặc định" với nội dung: nguồn `en-US` (English) → đích `vi-VN` (Vietnamese), lý do đối tượng dùng chính nghe tiếng Anh cần phụ đề tiếng Việt. Xác nhận bằng `grep -n "Cặp ngôn ngữ mặc định" docs/tech-stack.md` → có kết quả tại dòng 83, 85.
- `roadmap.md:10` — dòng gốc "Chốt cặp ngôn ngữ mặc định (ví dụ English → Vietnamese)" đã khớp với quyết định chọn (en-US → vi-VN), không cần sửa nội dung dòng này (chỉ tick ở bước 6 của /sora-new-feature).

## Kết quả

**PASS** — tài liệu đã ghi rõ cặp ngôn ngữ mặc định và lý do, khớp roadmap.md.
