# Research: Auto-scroll & fade dòng phụ đề cũ (PBI 22)

## Quyết định: Số dòng tối đa hiển thị (N)

- **Quyết định**: N = 3 dòng (dòng mới nhất + 2 dòng cũ hơn), là hằng số cố định trong component.
- **Lý do**: Spec yêu cầu giữ ngữ cảnh vài câu gần nhất mà không chiếm quá nhiều diện tích cửa sổ phụ đề nổi (overlay nhỏ, luôn trên cùng). 3 dòng là mức phổ biến ở các app phụ đề live (CC/caption overlay) — đủ ngữ cảnh, không che màn hình.
- **Phương án khác đã xem xét**: N=1 (hành vi hiện tại, không đạt yêu cầu spec); N=5+ (chiếm nhiều diện tích overlay, dễ che nội dung khác đang chia sẻ màn hình). Để cấu hình được (M6) là việc của PBI sau.

## Quyết định: Cơ chế tạo hiệu ứng mờ (fade) theo độ cũ

- **Quyết định**: Dùng CSS `opacity` giảm dần theo vị trí trong danh sách (dòng mới nhất `opacity: 1`, càng cũ opacity càng thấp), kết hợp `transition: opacity, transform` để cuộn/mờ mượt khi danh sách thay đổi.
- **Lý do**: Vue không cần thư viện animation riêng cho hiệu ứng đơn giản này — CSS transition thuần đủ đáp ứng SC-002 (độ mờ tăng dần quan sát được bằng mắt), giữ bundle nhẹ, không thêm dependency.
- **Phương án khác đã xem xét**: Thư viện animation (vue-transition-group nâng cao, GSAP...) — over-engineering cho use case chỉ cuộn 3 dòng text, không cần thêm dependency mới.

## Quyết định: Cấu trúc state giữ danh sách dòng

- **Quyết định**: Mảng `SubtitleLine[]` (mỗi phần tử `{ id, text }`) trong `ref`, push dòng mới vào cuối, cắt mảng khi vượt N (`slice(-N)`).
- **Lý do**: Đơn giản, đủ để Vue `v-for` render đúng thứ tự và áp dụng CSS theo index; `id` tăng dần dùng làm `:key` ổn định cho transition, tránh Vue tái sử dụng sai DOM node khi danh sách trượt.
- **Phương án khác đã xem xét**: Dùng `startFrameIndex` từ payload làm key — không đảm bảo luôn tăng đơn điệu/duy nhất theo mọi tình huống, nên dùng counter nội bộ tự tăng cho chắc.

Không còn điểm `NEEDS CLARIFICATION` nào.
