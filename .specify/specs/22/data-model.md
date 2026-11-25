# Data model: Auto-scroll & fade dòng phụ đề cũ (PBI 22)

## SubtitleLine (state nội bộ component, không persist)

| Thuộc tính | Kiểu | Mô tả |
|---|---|---|
| `id` | `number` | Counter tự tăng, dùng làm key ổn định cho `v-for`/transition. |
| `text` | `string` | Nội dung text dịch (`translatedText`) của 1 event `subtitle-ready`. |

## Quy tắc hợp lệ

- Không tạo `SubtitleLine` khi `translatedText` rỗng (kế thừa FR đã có từ PBI 21).
- Danh sách `lines: SubtitleLine[]` luôn có độ dài ≤ N (N=3); khi push phần tử mới vượt N, loại phần tử đầu mảng (cũ nhất).
- Thứ tự mảng = thứ tự thời gian nhận event (phần tử cuối mảng = mới nhất).

## Trạng thái chuyển đổi

Không có máy trạng thái phức tạp — chỉ là hàng đợi giới hạn kích thước (bounded queue/ring buffer đơn giản) cập nhật mỗi khi có event `subtitle-ready` mới.
