# Data Model: Hiển thị phụ đề (PBI 21)

Tái sử dụng payload đã có từ PBI 19/20, không tạo entity Core mới.

## SubtitlePayload (đã tồn tại — `app/src-tauri/src/lib.rs`)

Payload của event Tauri `subtitle-ready` (camelCase khi serialize sang JS):

| Trường | Kiểu | Mô tả |
|---|---|---|
| `startFrameIndex` | `number` | Vị trí bắt đầu segment audio |
| `endFrameIndex` | `number` | Vị trí kết thúc segment audio |
| `sourceText` | `string` | Text gốc (tiếng nguồn) |
| `translatedText` | `string` | Text đã dịch — trường UI sẽ hiển thị ở PBI này |

## State phía UI (`SubtitleApp.vue`, mới)

- `currentSubtitle: Ref<string>` — nội dung dòng phụ đề đang hiển thị (lấy từ `translatedText` của event mới nhất). Khởi tạo rỗng (chưa có event nào).
