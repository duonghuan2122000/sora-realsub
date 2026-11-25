# Đặc tả tính năng: Dịch có ngữ cảnh câu trước

**Mã PBI**: 17
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Hiện tại `Translator::translate()` nhận tham số `context` (1-2 câu transcript trước câu đang dịch) nhưng cài đặt `SidecarTranslator` bỏ qua tham số này (`_context`), nên những câu có đại từ ("he", "it", "they"...) hoặc thiếu chủ ngữ rõ ràng dễ bị dịch sai hoặc mơ hồ. Tính năng này ghép ngữ cảnh 1-2 câu trước vào yêu cầu dịch, để câu hiện tại được dịch chính xác hơn khi đọc một mình không đủ nghĩa.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** một chuỗi các câu transcript liên tiếp cùng ngôn ngữ nguồn, trong đó câu hiện tại chứa đại từ hoặc thiếu ngữ cảnh,
**When** hệ thống dịch câu hiện tại kèm theo 1-2 câu transcript ngay trước nó làm ngữ cảnh,
**Then** hệ thống trả về bản dịch của riêng câu hiện tại (không lẫn bản dịch của câu ngữ cảnh), và bản dịch đó chính xác hơn so với khi dịch câu hiện tại một mình, không có ngữ cảnh.

### Kịch bản chấp nhận

1. **Given** câu ngữ cảnh "John lost his keys." và câu hiện tại "He was very upset.", **When** gọi dịch câu hiện tại kèm câu ngữ cảnh, **Then** đại từ "He" được dịch đúng/nhất quán với ngữ cảnh (ví dụ ra "Anh ấy"/"John"), và kết quả trả về chỉ là bản dịch của câu hiện tại, không kèm bản dịch của câu ngữ cảnh.
2. **Given** không có câu ngữ cảnh nào (đầu phiên, câu đầu tiên), **When** gọi dịch, **Then** hệ thống vẫn dịch bình thường câu hiện tại, không lỗi, hành vi giống PBI 16 (dịch không ngữ cảnh).
3. **Given** có nhiều hơn 2 câu ngữ cảnh khả dụng trước câu hiện tại, **When** gọi dịch, **Then** hệ thống chỉ dùng tối đa 2 câu gần nhất làm ngữ cảnh, không gửi toàn bộ lịch sử.
4. **Given** cùng một câu hiện tại và cùng ngữ cảnh, **When** gọi dịch nhiều lần, **Then** kết quả dịch ổn định giữa các lần gọi (giữ nguyên FR-004 của PBI 16).

### Trường hợp biên

- Câu ngữ cảnh rỗng hoặc chỉ chứa khoảng trắng → bị bỏ qua, không tính là ngữ cảnh hợp lệ.
- Câu hiện tại rỗng/khoảng trắng nhưng có ngữ cảnh kèm theo → hệ thống vẫn trả về rỗng, không dịch, không lỗi (giữ hành vi FR-003 của PBI 16).
- Ngữ cảnh chứa ký tự đặc biệt, số, tên riêng → không crash, trả về bản dịch tốt nhất có thể cho câu hiện tại.
- Tổng độ dài ngữ cảnh + câu hiện tại rất dài → hệ thống vẫn trả về kết quả, không treo quá lâu.

## Yêu cầu chức năng

- **FR-001**: Khi dịch một câu hiện tại có kèm 1 hoặc 2 câu ngữ cảnh, hệ thống PHẢI gửi các câu ngữ cảnh đó cùng với câu hiện tại tới bước dịch, theo đúng thứ tự thời gian (câu cũ nhất trước).
- **FR-002**: Kết quả trả về từ `translate()` PHẢI là bản dịch của riêng câu hiện tại — không bao gồm bản dịch của (các) câu ngữ cảnh.
- **FR-003**: Nếu danh sách ngữ cảnh truyền vào có nhiều hơn 2 câu, hệ thống PHẢI chỉ sử dụng tối đa 2 câu gần câu hiện tại nhất, bỏ qua các câu còn lại.
- **FR-004**: Nếu danh sách ngữ cảnh rỗng, hệ thống PHẢI dịch câu hiện tại độc lập (không có ngữ cảnh), giữ nguyên hành vi đã có ở PBI 16.
- **FR-005**: Hệ thống PHẢI bỏ qua các câu ngữ cảnh rỗng/chỉ chứa khoảng trắng khi xây dựng ngữ cảnh gửi đi.
- **FR-006**: Với cùng câu hiện tại và cùng ngữ cảnh, hệ thống PHẢI cho kết quả dịch ổn định giữa các lần gọi (không ngẫu nhiên khác nhau).

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Với ít nhất 1 cặp câu có đại từ/thiếu ngữ cảnh đã biết trước, bản dịch khi có ngữ cảnh đúng/nhất quán hơn bản dịch khi không có ngữ cảnh, xác nhận bằng test tự động so khớp với bản dịch tham chiếu.
- **SC-002**: 100% test case gọi dịch kèm ngữ cảnh trả về đúng bản dịch của câu hiện tại (không lẫn nội dung câu ngữ cảnh trong kết quả).
- **SC-003**: Không có trường hợp dịch kèm ngữ cảnh làm crash ứng dụng, kể cả khi ngữ cảnh rỗng, nhiều hơn 2 câu, hoặc chứa ký tự bất thường.

## Giả định

- Cặp ngôn ngữ nguồn-đích và model dịch (Marian-MT qua `SidecarTranslator`, PBI 16) giữ nguyên, không đổi trong PBI này.
- "Ngữ cảnh" ở đây chỉ gồm các câu transcript gốc (ngôn ngữ nguồn) ngay trước câu hiện tại trong cùng phiên; không gồm văn bản đã dịch hoặc ngữ cảnh ngoài phiên.
- Việc lấy 1-2 câu transcript final gần nhất để làm ngữ cảnh (duy trì một bộ đệm ngắn hạn trong pipeline) thuộc phạm vi PBI này, vì `Translator::translate()` chỉ nhận ngữ cảnh làm tham số, không tự quản lý lịch sử.
- Việc ghép toàn bộ luồng STT final → Translator (kèm ngữ cảnh) → event `subtitle-ready` là mục roadmap kế tiếp (M4, mục 3), không thuộc PBI này — PBI này chỉ đảm bảo `translate()` dùng đúng ngữ cảnh khi được truyền vào.

## Ngoài phạm vi

- Nối Translator (có ngữ cảnh) vào event `subtitle-ready` end-to-end (thuộc PBI/mục roadmap sau).
- Ngữ cảnh vượt quá 2 câu trước, hoặc ngữ cảnh hai chiều (câu sau).
- Thay đổi model dịch hoặc cặp ngôn ngữ.
