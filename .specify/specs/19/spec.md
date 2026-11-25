# Đặc tả tính năng: Nối STT final → Dịch → subtitle-ready

**Mã PBI**: 19
**Ngày tạo**: 2026-11-25
**Trạng thái**: Nháp

## Mô tả tổng quan

Hiện tại hệ thống đã có luồng audio → VAD → STT phát ra `transcript-final` (câu gốc đã chốt), và có khả năng dịch một câu kèm ngữ cảnh (`Translator`). Nhưng hai phần này chưa được nối với nhau trong phiên thực tế: mỗi câu transcript final chưa được tự động đưa qua bước dịch và chưa có một sự kiện duy nhất mang cả câu gốc và câu dịch để UI hiển thị. Tính năng này nối transcript final vào bước dịch (có ngữ cảnh 1-2 câu trước) và phát ra một sự kiện phụ đề hoàn chỉnh, để người dùng nhận được phụ đề song ngữ theo thời gian thực trong một phiên họp/stream.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** một phiên capture đang chạy và đã cấu hình được STT + dịch,
**When** một câu nói được chốt thành transcript final (câu gốc, ngôn ngữ nguồn),
**Then** hệ thống tự động dịch câu đó (có kèm 1-2 câu transcript final ngay trước nó làm ngữ cảnh nếu có) và phát ra một sự kiện phụ đề mang cả câu gốc và câu dịch, theo đúng thứ tự thời gian các câu đã nói.

### Kịch bản chấp nhận

1. **Given** một segment thoại được STT chốt thành transcript final với text "Hello, how are you?", **When** pipeline xử lý xong segment đó, **Then** hệ thống phát ra đúng 1 sự kiện phụ đề chứa cả câu gốc "Hello, how are you?" và câu dịch tương ứng.
2. **Given** đã có ít nhất 1 câu transcript final trước đó trong phiên, **When** một câu transcript final mới được chốt, **Then** bước dịch câu mới PHẢI dùng tối đa 2 câu transcript final gần nhất (gốc, không phải câu dịch) làm ngữ cảnh, đúng theo hành vi đã có của `Translator`.
3. **Given** nhiều segment liên tiếp đều được chốt thành transcript final, **When** pipeline xử lý xong, **Then** các sự kiện phụ đề phát ra theo đúng thứ tự thời gian của các câu nói gốc (không bị đảo thứ tự, không bị trộn giữa các câu).
4. **Given** transcript chỉ là partial (chưa final), **When** pipeline xử lý segment đó, **Then** hệ thống KHÔNG dịch và KHÔNG phát sự kiện phụ đề cho câu partial đó (chỉ xử lý khi đã final).
5. **Given** câu transcript final rỗng hoặc chỉ chứa khoảng trắng, **When** pipeline xử lý, **Then** hệ thống không coi đó là lỗi, không phát sự kiện phụ đề cho câu rỗng đó, và không đưa câu rỗng vào làm ngữ cảnh cho câu sau.

### Trường hợp biên

- Bước dịch lỗi (ví dụ sidecar dịch không chạy được) cho 1 câu final → hệ thống báo lỗi theo cùng cơ chế báo lỗi runtime đã có (tương tự lỗi STT ở PBI 15), không dừng cả phiên, các câu final tiếp theo vẫn tiếp tục được dịch và phát sự kiện bình thường.
- Câu transcript final rất dài hoặc chứa ký tự đặc biệt → vẫn dịch và phát sự kiện, không crash phiên.
- Phiên chưa cấu hình được bước dịch (ví dụ thiếu model dịch) → hệ thống vẫn phát `transcript-final` như hiện tại, chỉ bỏ qua bước dịch/phát `subtitle-ready` cho câu đó, tương tự cách STT bị bỏ qua khi thiếu model ở các PBI trước.

## Yêu cầu chức năng

- **FR-001**: Khi một segment được STT chốt thành transcript final (không rỗng), hệ thống PHẢI tự động đưa câu đó qua bước dịch, không cần người dùng thực hiện thêm hành động nào.
- **FR-002**: Khi dịch một câu final, hệ thống PHẢI kèm theo tối đa 2 câu transcript final gần nhất (câu gốc, theo thứ tự thời gian) trong cùng phiên làm ngữ cảnh, nếu đã có.
- **FR-003**: Sau khi dịch xong một câu final, hệ thống PHẢI phát ra một sự kiện phụ đề duy nhất mang cả câu gốc và câu dịch tương ứng của câu đó.
- **FR-004**: Các sự kiện phụ đề PHẢI được phát theo đúng thứ tự thời gian của các câu transcript final gốc tương ứng.
- **FR-005**: Hệ thống KHÔNG được dịch hoặc phát sự kiện phụ đề cho transcript partial (chưa final).
- **FR-006**: Hệ thống KHÔNG được coi câu transcript final rỗng/chỉ khoảng trắng là lỗi; không phát sự kiện phụ đề cho câu đó và không dùng câu đó làm ngữ cảnh cho câu sau.
- **FR-007**: Khi bước dịch của một câu final cụ thể gặp lỗi, hệ thống PHẢI báo lỗi đó (không làm crash phiên) và PHẢI tiếp tục xử lý các câu final tiếp theo bình thường.
- **FR-008**: Khi phiên không có bước dịch khả dụng (ví dụ chưa cấu hình được), hệ thống PHẢI vẫn phát `transcript-final` như hành vi hiện có, và được phép bỏ qua việc phát sự kiện phụ đề cho các câu final trong phiên đó.

*Mỗi yêu cầu phải kiểm thử được (testable) và không mơ hồ.*

## Tiêu chí thành công

- **SC-001**: Với một đoạn ghi âm mẫu có nhiều câu nói liên tiếp, 100% câu được STT chốt thành final (không rỗng) đều có đúng 1 sự kiện phụ đề tương ứng chứa cả câu gốc và câu dịch, theo đúng thứ tự đã nói.
- **SC-002**: Với ít nhất 1 cặp câu có đại từ/thiếu ngữ cảnh đã biết trước (kế thừa từ PBI 17), bản dịch trong sự kiện phụ đề phát ra khi chạy qua luồng end-to-end này khớp với bản dịch có ngữ cảnh đã xác nhận ở PBI 17.
- **SC-003**: Khi mô phỏng lỗi dịch cho 1 câu final giữa nhiều câu final liên tiếp, phiên không dừng/crash, và số sự kiện phụ đề phát ra đúng bằng số câu final dịch thành công (câu lỗi không có sự kiện phụ đề tương ứng).

## Giả định

- Bước dịch tái dùng `Translator`/`SidecarTranslator` đã có (PBI 16, 17); PBI này không đổi cách dịch 1 câu, chỉ nối luồng gọi dịch tự động sau transcript final và quản lý ngữ cảnh 1-2 câu gần nhất trong phiên (`RecentTranscriptContext` đã có sẵn từ PBI 17).
- "Sự kiện phụ đề" ở mức đặc tả này tương đương sự kiện Tauri `subtitle-ready` đã nêu trong roadmap (tên sự kiện cụ thể, tên trường payload là chi tiết kỹ thuật, quyết định ở bước lập kế hoạch/plan.md).
- Phạm vi chỉ nối transcript final (không nối partial) vào dịch, theo đúng định nghĩa transcript final/partial đã có từ PBI 14.
- Việc hiển thị sự kiện phụ đề lên một cửa sổ overlay thật (UI) thuộc M5, ngoài phạm vi PBI này — PBI này chỉ đảm bảo Core phát đúng sự kiện.

## Ngoài phạm vi

- Xây dựng cửa sổ phụ đề nổi hiển thị `subtitle-ready` (thuộc M5).
- Thay đổi logic dịch 1 câu đơn lẻ (ghép ngữ cảnh, tách câu dịch) — đã chốt ở PBI 16/17.
- Lưu trữ/xuất transcript sau khi kết thúc phiên (thuộc M7).
