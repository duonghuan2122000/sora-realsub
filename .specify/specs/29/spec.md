# Đặc tả tính năng: Chọn cặp ngôn ngữ nguồn-đích

**Mã PBI**: 29
**Ngày tạo**: 2026-11-27
**Trạng thái**: Đã làm rõ

## Mô tả tổng quan

Người dùng cần chọn ngôn ngữ nguồn (lời nói gốc) và ngôn ngữ đích (bản dịch hiển thị) ngay trên màn hình chính, trước khi bắt đầu phiên dịch phụ đề. Lựa chọn này áp dụng cho phiên dịch tiếp theo, giúp người dùng chủ động dùng app cho các cặp ngôn ngữ khác nhau thay vì bị cố định một cặp.

## Kịch bản & luồng người dùng

### Luồng chính

1. Người dùng mở màn hình chính, thấy ô chọn "Ngôn ngữ nguồn" và "Ngôn ngữ đích" bên cạnh phần chọn thiết bị mic/loopback.
2. Người dùng chọn một cặp ngôn ngữ nguồn-đích khác với giá trị mặc định.
3. Người dùng nhấn Start — phiên dịch mới bắt đầu, phụ đề dịch ra đúng theo cặp ngôn ngữ vừa chọn.

### Kịch bản chấp nhận

1. **Given** màn hình chính đang ở trạng thái `idle`, **When** người dùng đổi ngôn ngữ nguồn hoặc đích sang một cặp được hỗ trợ khác, **Then** lựa chọn mới được giữ lại và hiển thị đúng trên UI.
2. **Given** người dùng đã chọn một cặp ngôn ngữ hợp lệ, **When** người dùng nhấn Start, **Then** phiên dịch mới sử dụng đúng cặp ngôn ngữ đã chọn (văn bản dịch ra đúng ngôn ngữ đích).
3. **Given** phiên dịch đang chạy (`running`), **When** người dùng thử đổi ngôn ngữ, **Then** UI khóa (disable) ô chọn ngôn ngữ, không cho đổi giữa phiên đang chạy.
4. **Given** app vừa mở lần đầu, **When** chưa có lựa chọn nào trước đó, **Then** UI hiển thị sẵn một cặp ngôn ngữ mặc định hợp lệ (nguồn khác đích).

### Trường hợp biên

- Người dùng chọn ngôn ngữ nguồn và đích giống nhau → UI báo lỗi rõ ràng, không cho Start.
- Cặp ngôn ngữ người dùng chọn không được hệ thống hỗ trợ (chưa có model dịch tương ứng) → ô chọn chỉ hiển thị các cặp thực sự được hỗ trợ, không cho chọn cặp không hỗ trợ.
- Người dùng đổi ngôn ngữ giữa lúc trạng thái đang chuyển tiếp (`starting`/`stopping`) → ô chọn vẫn bị khóa như trạng thái `running`.

## Yêu cầu chức năng

- **FR-001**: Màn hình chính PHẢI hiển thị ô chọn ngôn ngữ nguồn và ô chọn ngôn ngữ đích, chỉ liệt kê các cặp ngôn ngữ hệ thống thực sự hỗ trợ.
- **FR-002**: Người dùng PHẢI chọn được ngôn ngữ nguồn và đích độc lập với nhau, miễn cặp kết quả nằm trong danh sách hỗ trợ.
- **FR-003**: Hệ thống PHẢI ngăn người dùng chọn ngôn ngữ nguồn trùng ngôn ngữ đích, báo lỗi rõ ràng ngay trên UI.
- **FR-004**: Hệ thống PHẢI khóa (không cho sửa) lựa chọn ngôn ngữ khi phiên dịch đang ở trạng thái `starting`, `running` hoặc `stopping`.
- **FR-005**: Khi người dùng nhấn Start, hệ thống PHẢI áp dụng đúng cặp ngôn ngữ đang được chọn cho phiên dịch mới đó.
- **FR-006**: Hệ thống PHẢI ghi nhớ cặp ngôn ngữ đã chọn trong phạm vi phiên sử dụng app hiện tại (không yêu cầu chọn lại khi bắt đầu phiên kế tiếp, trừ khi người dùng tự đổi).

## Tiêu chí thành công

- **SC-001**: Người dùng đổi được cặp ngôn ngữ và thấy kết quả dịch đúng cặp đã chọn ở phiên dịch kế tiếp, không cần khởi động lại app.
- **SC-002**: 100% các trường hợp người dùng chọn ngôn ngữ nguồn = đích đều bị chặn kèm thông báo lỗi, không có trường hợp lọt qua.
- **SC-003**: Ô chọn ngôn ngữ bị khóa đúng trong toàn bộ thời gian phiên dịch đang chạy (starting/running/stopping), không có thời điểm nào người dùng đổi được giữa phiên.

## Giả định

- Hiện hệ thống chỉ có sẵn model dịch cho một cặp ngôn ngữ (Anh → Việt, theo M0/PBI 16). Tính năng này xây cơ chế chọn tổng quát, nhưng danh sách cặp hỗ trợ ban đầu có thể chỉ gồm 1 cặp — UI vẫn hiển thị đúng, không hard-code giả định "luôn có ≥ 2 lựa chọn".
- "Áp dụng ngay cho phiên mới" nghĩa là phiên dịch **tiếp theo** (sau khi nhấn Start), không yêu cầu đổi ngôn ngữ real-time giữa lúc đang dịch.
- Lựa chọn ngôn ngữ chỉ cần giữ trong bộ nhớ của phiên làm việc app hiện tại (runtime state), việc lưu xuống file cấu hình thuộc PBI khác (mục "Lưu cấu hình" ở M6, đã tách riêng trong roadmap).

## Ngoài phạm vi

- Lưu cấu hình ngôn ngữ xuống file để giữ qua lần khởi động lại app (thuộc mục roadmap riêng — "Lưu cấu hình").
- Đổi ngôn ngữ ngay giữa lúc phiên dịch đang chạy mà không cần Stop/Start lại.
- Thêm model dịch cho ngôn ngữ/cặp ngôn ngữ mới (thuộc phạm vi kỹ thuật của Translator, không phải UI chọn ngôn ngữ).
