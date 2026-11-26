---
description: Thi công toàn bộ task trong tasks.md của một PBI, tương đương /speckit.implement
argument-hint: <mã-pbi>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
model: claude-opus-5-5
effort: low
---

## Đầu vào

```text
$ARGUMENTS
```

Tham số đầu tiên (`$1`) là **Mã PBI** (bắt buộc).

## Kiểm tra đầu vào

1. Nếu `$1` rỗng hoặc không phải số → dừng, báo lỗi cách dùng: `/sora-implement <mã-pbi>`.
2. Kiểm tra `.specify/specs/$1/tasks.md` tồn tại:
   - Nếu **không** → dừng lại, báo: "Chưa có tasks.md cho PBI $1. Hãy chạy `/sora-task $1` trước." Không tự bịa task để thi công.

## Quy trình thực hiện

1. **Kiểm tra checklist (nếu có)**: nếu tồn tại `.specify/specs/$1/checklists/`, quét toàn bộ file trong đó, đếm số dòng `- [ ]` (chưa xong) và `- [x]`/`- [X]` (đã xong) cho từng checklist. Hiển thị bảng trạng thái:

   ```text
   | Checklist       | Tổng | Đã xong | Chưa xong | Trạng thái |
   |-----------------|------|---------|-----------|------------|
   | requirements.md | 12   | 12      | 0         | ✓ ĐẠT      |
   ```

   - Nếu còn checklist chưa hoàn tất: dừng lại và hỏi người dùng "Một số checklist chưa hoàn tất. Bạn có muốn tiếp tục thi công không? (có/không)". Chỉ tiếp tục khi người dùng xác nhận "có".
   - Nếu tất cả đã đạt: tự động tiếp tục sang bước 2.

2. **Nạp toàn bộ ngữ cảnh** từ `.specify/specs/$1/`:
   - **Bắt buộc**: `tasks.md`, `plan.md`
   - **Nếu có**: `data-model.md`, `contracts/`, `research.md`, `quickstart.md`, `.specify/memory/constitution.md`

3. **Kiểm tra thiết lập dự án**: xác định loại dự án (Node.js, Python, .NET, Go...) dựa trên plan.md và các file cấu hình có sẵn (package.json, requirements.txt, *.csproj, go.mod...). Đảm bảo các file ignore cần thiết (.gitignore, .dockerignore...) tồn tại và có pattern phù hợp với công nghệ đang dùng; nếu thiếu thì tạo, nếu đã có thì chỉ bổ sung pattern còn thiếu chứ không ghi đè toàn bộ.

4. **Phân tích tasks.md**: đọc từng pha (Setup, Foundational, User Story..., Polish), xác định:
   - Task nào tuần tự, task nào có thể chạy song song (`[P]`)
   - Task nào đã đánh dấu `[x]`/`[X]` (bỏ qua, không làm lại)

5. **Thi công theo từng pha, đúng thứ tự**:
   - Hoàn thành trọn vẹn một pha trước khi sang pha kế tiếp.
   - Tôn trọng phụ thuộc: task tuần tự làm theo đúng thứ tự; các task `[P]` có thể xử lý cùng lúc nếu không đụng chung file.
   - Nếu tasks.md có task kiểm thử, viết và chạy test trước khi cài đặt phần tương ứng (TDD).
   - Task cùng đụng vào một file phải làm tuần tự, không song song.
   - Sau mỗi task hoàn thành, **đánh dấu `[X]`** ngay trong tasks.md.

6. **Xử lý lỗi khi thi công**:
   - Nếu một task (tuần tự hoặc `[P]`) thất bại → **retry tối đa 3 lần**: mỗi lần đọc lỗi cụ thể, điều chỉnh cách làm rồi thử lại, ghi lại lỗi + cách điều chỉnh của từng lần.
   - Nếu sau 3 lần vẫn thất bại:
     - Task tuần tự → dừng lại, báo lỗi cụ thể kèm log 3 lần retry, không tự ý bỏ qua sang task sau.
     - Task `[P]` → tiếp tục với các task song song khác đã thành công, báo cáo rõ task nào thất bại, log 3 lần retry và lý do.
   - Đề xuất bước khắc phục tiếp theo nếu không thể tiếp tục.

7. **Xác minh khi hoàn tất — bắt buộc có bằng chứng test**:
   - Xác nhận toàn bộ task bắt buộc đã được đánh dấu `[X]`.
   - Đối chiếu kết quả cài đặt với spec.md để đảm bảo đúng yêu cầu.
   - Với mỗi task có bước kiểm thử: viết **file test case** (mô tả input/expected/actual, kết quả pass/fail) lưu tại `.specify/specs/$1/evidence/<tên-task>-testcase.md`.
   - Chạy test, và khi có thể quan sát kết quả trực quan (UI, output chạy app, log) thì **chụp ảnh màn hình kết quả** lưu tại `.specify/specs/$1/evidence/<tên-task>-result.png` (dùng browser tool hoặc màn hình chạy app); với test chỉ có output dạng text (unit test CLI), lưu log output ra `.specify/specs/$1/evidence/<tên-task>-result.log` thay cho ảnh.
   - Nếu một bước kiểm thử lỗi → áp dụng retry tối đa 3 lần như mục 6; nếu vẫn fail sau 3 lần, ghi rõ trong file test case là FAIL + log 3 lần thử, không tự đánh dấu task đó `[X]`.
   - Báo cáo tổng kết cuối cùng: số task hoàn thành/tổng số, các file đã tạo/sửa, danh sách file bằng chứng test (test case + ảnh/log) theo từng task, các vấn đề còn tồn đọng (nếu có).

Nếu `tasks.md` chưa đầy đủ hoặc không tìm thấy pha nào hợp lệ, dừng lại và đề xuất chạy `/sora-task $1` để tạo lại danh sách task.
