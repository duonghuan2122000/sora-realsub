# Data Model: Chọn cặp ngôn ngữ nguồn-đích (PBI 29)

## `LanguagePair` (Rust, `core`)

Hằng số tĩnh, không lưu DB/file.

| Thuộc tính | Kiểu | Mô tả |
|---|---|---|
| `id` | `&str` | Khoá ổn định dùng xuyên IPC (ví dụ `"en-vi"`) |
| `source_lang` | `&str` | Mã ngôn ngữ nguồn (ISO 639-1, ví dụ `"en"`) |
| `target_lang` | `&str` | Mã ngôn ngữ đích (ví dụ `"vi"`) |
| `label` | `&str` | Nhãn hiển thị cho UI (ví dụ `"Anh → Việt"`) |

Danh sách khởi tạo: 1 phần tử `{ id: "en-vi", source_lang: "en", target_lang: "vi", label: "Anh → Việt" }`, khớp model dịch hiện có (PBI 16).

**Luật hợp lệ**: mọi phần tử trong danh sách PHẢI có `source_lang != target_lang` (đảm bảo FR-003 bằng cấu trúc dữ liệu, không cần validate runtime).

## State phía UI (`App.vue`)

| Biến | Kiểu | Mô tả |
|---|---|---|
| `languagePairs` | `LanguagePairInfo[]` | Lấy 1 lần từ `list_language_pairs_cmd` lúc mount |
| `selectedPairId` | `string \| null` | Cặp đang chọn, mặc định = `languagePairs[0].id` khi tải xong |

**Chuyển trạng thái**: `selectedPairId` chỉ có thể đổi khi `sessionState === "idle"` (tái dùng computed `devicesLocked`/khoá tương tự cho ô chọn ngôn ngữ — FR-004).
