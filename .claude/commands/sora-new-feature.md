---
description: Chạy lần lượt sora-spec → sora-plan → sora-task → sora-implement cho một PBI mới (lấy từ roadmap.md nếu không truyền tay), sau đó cập nhật wiki và roadmap.md
argument-hint: [mã-pbi] [mô-tả-tính-năng]
allowed-tools: SlashCommand, Skill, Read, Write, Edit, Glob, Grep, Bash
---

## Đầu vào

```text
$ARGUMENTS
```

Tham số đầu tiên (`$1`) là **Mã PBI** (tùy chọn). Phần còn lại của `$ARGUMENTS` là **Mô tả tính năng** (tùy chọn).

## Xác định PBI cần làm

- Nếu `$ARGUMENTS` có đủ mã PBI (số nguyên) + mô tả → dùng trực tiếp, bỏ qua bước chọn từ roadmap.
- Nếu `$ARGUMENTS` rỗng hoặc thiếu mô tả → đọc `roadmap.md` ở gốc repo:
  1. Lấy mục `- [ ]` **đầu tiên chưa tick** theo thứ tự xuất hiện trong file (ưu tiên milestone nhỏ hơn trước, bỏ qua mục trong "Backlog" nếu còn mục ở milestone M0–M8 chưa tick).
  2. Dùng nội dung dòng đó làm **mô tả tính năng**.
  3. Sinh **mã PBI** mới = số lớn nhất đang có trong `.specify/specs/<n>/` + 1 (nếu chưa có PBI nào, bắt đầu từ `1`).
  4. Thông báo cho người dùng mục đã chọn: "Chọn từ roadmap.md: `<dòng checkbox>` → PBI `<mã>`." trước khi chạy tiếp.
  - Nếu roadmap.md không còn mục nào chưa tick → dừng, báo: "Không còn tính năng nào chưa đáp ứng trong roadmap.md."

## Kiểm tra đầu vào

Sau bước xác định PBI ở trên, nếu vẫn không có mã PBI hợp lệ (số nguyên) → dừng, báo lỗi:
> "Thiếu hoặc sai Mã PBI. Cách dùng: `/sora-new-feature <mã-pbi> <mô-tả-tính-năng>` hoặc không truyền gì để tự lấy từ roadmap.md."

## Quy trình thực hiện

Chạy tuần tự. **Retry từng bước tối đa 3 lần** nếu bước đó báo lỗi (lỗi kỹ thuật: command/tool thất bại, không phải lỗi "cần người dùng làm rõ"):

- Giữa mỗi lần retry, đọc lỗi cụ thể và thử điều chỉnh (ví dụ sửa input, tạo lại file thiếu) trước khi gọi lại bước đó.
- Ghi lại mỗi lần thử: lần thứ mấy, lỗi gặp phải, cách đã điều chỉnh.
- Nếu sau 3 lần vẫn lỗi → **dừng toàn bộ quy trình**, báo cáo cho người dùng: bước nào lỗi, log 3 lần retry (lỗi + cách đã thử ở mỗi lần), và không tự chuyển sang bước tiếp theo.
- Nếu bước báo "cần người dùng làm rõ" (ví dụ spec còn điểm `[CẦN LÀM RÕ]`) → dừng ngay, không retry, hỏi người dùng.

Các bước:

1. `/sora-spec <mã-pbi> <mô-tả-tính-năng>`
2. `/sora-plan $1`
3. `/sora-task $1`
4. `/sora-implement $1` (bước này đã bao gồm yêu cầu chụp ảnh kết quả test + file test case làm bằng chứng, xem `/sora-implement`)
5. **Cập nhật knowledge-base**: gọi skill `sora-wiki` để ingest kết quả tính năng vừa làm (spec.md, plan.md, tasks.md, bằng chứng test và các thay đổi code chính) vào wiki dự án — coi các file này là nguồn mới cần ingest, tạo/cập nhật page liên quan, cập nhật `index.md` và `log.md`. Nếu project chưa có wiki, để skill tự hỏi bootstrap; không tự bỏ qua bước này.
6. **Cập nhật roadmap.md**: tìm đúng dòng `- [ ]` tương ứng với tính năng vừa thi công xong (đã implement + test pass) và tick thành `- [x]`. Nếu tính năng vừa làm không khớp 1:1 với 1 dòng có sẵn (ví dụ bị tách nhỏ hơn khi lập spec/task), cập nhật dòng gần nhất cho đúng thực tế và ghi chú ngắn nếu cần. Không tick khi còn task/test chưa pass.

Sau khi cả 6 bước hoàn tất, báo cáo tóm tắt: đường dẫn `.specify/specs/$1/`, trạng thái từng bước (spec, plan, tasks, implement, wiki, roadmap), bằng chứng test (đường dẫn ảnh + file test case), và dòng roadmap.md đã tick.
