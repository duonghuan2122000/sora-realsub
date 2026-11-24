---
description: Chạy lần lượt sora-spec → sora-plan → sora-task → sora-implement cho một PBI mới, sau đó cập nhật wiki dự án qua skill sora-wiki
argument-hint: <mã-pbi> <mô-tả-tính-năng>
allowed-tools: SlashCommand, Skill, Read, Write, Edit, Glob, Grep, Bash
---

## Đầu vào

```text
$ARGUMENTS
```

Tham số đầu tiên (`$1`) là **Mã PBI**. Phần còn lại của `$ARGUMENTS` là **Mô tả tính năng**.

## Kiểm tra đầu vào

Nếu `$1` rỗng hoặc không phải số nguyên → dừng, báo lỗi:
> "Thiếu hoặc sai Mã PBI. Cách dùng: `/sora-new-feature <mã-pbi> <mô-tả-tính-năng>`."

## Quy trình thực hiện

Chạy tuần tự, **dừng ngay nếu bước nào báo lỗi hoặc cần người dùng làm rõ** (ví dụ spec còn điểm `[CẦN LÀM RÕ]` chưa giải quyết) — không tự chuyển bước tiếp theo khi bước trước chưa hoàn tất:

1. `/sora-spec $ARGUMENTS`
2. `/sora-plan $1`
3. `/sora-task $1`
4. `/sora-implement $1`
5. **Cập nhật knowledge-base**: gọi skill `sora-wiki` để ingest kết quả tính năng vừa làm (spec.md, plan.md, tasks.md và các thay đổi code chính) vào wiki dự án — coi các file này là nguồn mới cần ingest, tạo/cập nhật page liên quan, cập nhật `index.md` và `log.md`. Nếu project chưa có wiki, để skill tự hỏi bootstrap; không tự bỏ qua bước này.

Sau khi cả 5 bước hoàn tất, báo cáo tóm tắt: đường dẫn `.specify/specs/$1/`, trạng thái từng bước (spec, plan, tasks, implement), và các page wiki đã tạo/cập nhật.
