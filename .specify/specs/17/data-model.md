# Mô hình dữ liệu: PBI 17 — Dịch có ngữ cảnh câu trước

## `RecentTranscriptContext` (mới)

Struct nhỏ trong `core/translator.rs` (hoặc module riêng cùng cấp), giữ tối đa 2 câu transcript final gần nhất trong phiên hiện tại, dùng làm ngữ cảnh khi dịch câu kế tiếp. Không persist, chỉ sống trong bộ nhớ của một phiên; không nối vào pipeline thật trong PBI này (xem research.md Quyết định 3).

**Thuộc tính**:
- Danh sách tối đa 2 câu gần nhất (ví dụ `VecDeque<String>` giới hạn độ dài 2), thứ tự: câu cũ hơn ở đầu, câu mới nhất ở cuối.

**Hành vi (luật hợp lệ)**:
- `push(câu mới)`: thêm câu mới vào cuối; nếu đã có 2 câu, loại câu cũ nhất (đầu danh sách) trước khi thêm.
- `push` với câu rỗng/khoảng trắng: bỏ qua, không thêm vào danh sách (giữ ngữ cảnh luôn là câu có nội dung thật).
- `recent() -> Vec<String>` (hoặc `&[String]`): trả về các câu hiện có, đúng thứ tự thời gian (cũ → mới), tối đa 2 câu.

**Không có chuyển đổi trạng thái phức tạp** — chỉ là buffer giới hạn kích thước (bounded FIFO).

## Tham số `context: &[String]` của `Translator::translate()` (đã có từ PBI 16)

Không đổi kiểu dữ liệu. PBI 17 chỉ thay đổi cách `SidecarTranslator` *sử dụng* tham số này:

- Lọc bỏ phần tử rỗng/khoảng trắng trước khi dùng (FR-005).
- Chỉ giữ tối đa 2 phần tử cuối (gần câu hiện tại nhất) nếu danh sách dài hơn 2 (FR-003).
- Thứ tự phần tử trong slice được hiểu là cũ → mới (câu đầu slice là câu xa câu hiện tại nhất).
