# Danh sách Task: Auto-scroll & fade dòng phụ đề cũ

**Mã PBI**: 22
**Nguồn**: plan.md, spec.md, research.md, data-model.md, quickstart.md

## Pha 1: Setup

- [X] T001 Không cần khởi tạo dự án/phụ thuộc mới — xác nhận `app/src/subtitle/SubtitleApp.vue` và `app/src/subtitle/SubtitleApp.spec.ts` là 2 file duy nhất cần sửa (theo plan.md), không thêm dependency.

## Pha 2: Foundational

*(Không có — tính năng không cần hạ tầng nền tảng mới, dùng trực tiếp state Vue hiện có trong component.)*

## Pha 3: User Story 1 - Giữ N dòng gần nhất, cuộn + mờ dần dòng cũ (Ưu tiên: P1)

**Mục tiêu**: Cửa sổ phụ đề giữ tối đa 3 dòng gần nhất, dòng mới nhất rõ nét, dòng cũ hơn mờ dần, dòng vượt quá giới hạn bị loại bỏ — đúng FR-001 → FR-005 và SC-001 → SC-003.

**Tiêu chí kiểm thử độc lập**: Chạy `bun test SubtitleApp.spec.ts` — test phát nhiều hơn 3 event `subtitle-ready` liên tiếp, xác nhận chỉ còn đúng 3 dòng gần nhất, đúng thứ tự, dòng cũ nhất không còn trong DOM.

- [X] T002 [US1] Cập nhật test tại `app/src/subtitle/SubtitleApp.spec.ts`: sửa test "cập nhật đúng thứ tự khi nhận nhiều event liên tiếp" để kỳ vọng đúng 3 dòng gần nhất cùng hiển thị (không chỉ dòng cuối), và thêm test case mới xác nhận dòng thứ (N+1) đẩy dòng cũ nhất (dòng đầu) ra khỏi DOM khi vượt giới hạn N=3.
- [X] T003 [US1] Thêm test mới tại `app/src/subtitle/SubtitleApp.spec.ts` xác nhận dòng mới nhất có class/style độ mờ thấp nhất và dòng cũ hơn có độ mờ cao hơn (kiểm tra qua `style.opacity` hoặc class CSS tương ứng trên từng `.subtitle-text`).
- [X] T004 [US1] Sửa `app/src/subtitle/SubtitleApp.vue`: thay `currentSubtitle: Ref<string>` bằng state danh sách `lines: Ref<{ id: number; text: string }[]>`, mỗi khi nhận event `subtitle-ready` với `translatedText` không rỗng thì push dòng mới (id tự tăng) vào cuối mảng và cắt mảng giữ tối đa N=3 phần tử gần nhất (`slice(-3)`); bỏ qua khi `translatedText` rỗng (giữ hành vi cũ).
- [X] T005 [US1] Sửa template trong `app/src/subtitle/SubtitleApp.vue`: dùng `v-for="(line, idx) in lines" :key="line.id"` để render từng dòng, gán class/style theo vị trí `idx` để dòng cũ hơn (idx nhỏ hơn) có độ mờ cao hơn dòng mới nhất (idx lớn nhất).
- [X] T006 [US1] Sửa CSS trong `app/src/subtitle/SubtitleApp.vue`: thêm CSS cho từng mức độ mờ theo vị trí (ví dụ `.subtitle-line--age-0 { opacity: 1 }`, `.subtitle-line--age-1 { opacity: 0.6 }`, `.subtitle-line--age-2 { opacity: 0.35 }`) và `transition: opacity 0.3s ease, transform 0.3s ease` để cuộn/mờ mượt khi danh sách thay đổi; giữ nguyên style nền/bo góc/max-width đã có.

## Pha cuối: Polish & Cross-cutting

- [X] T007 Chạy `bun test SubtitleApp.spec.ts` trong `app/`, xác nhận toàn bộ test (cũ + mới) pass; chụp ảnh/log kết quả test làm bằng chứng theo yêu cầu `/sora-implement`.

## Sơ đồ phụ thuộc

- T001 không khoá task nào (chỉ là xác nhận phạm vi file).
- T002, T003 (sửa test) nên viết trước hoặc cùng lúc với T004–T006 (TDD nhẹ) nhưng không bắt buộc thứ tự cứng — có thể làm T004→T006 trước rồi mới chỉnh test, miễn T007 chạy cuối cùng để xác nhận toàn bộ pass.
- T004 phải xong trước T005 (template phụ thuộc state `lines` mới).
- T005 phải xong trước T006 (CSS theo class gán trong template).
- T007 chạy sau tất cả T002–T006.

## Ví dụ chạy song song

- T002 và T003 có thể viết song song (cùng 1 file nhưng 2 khối test độc lập — nếu 1 người làm thì làm nối tiếp; nếu phân công 2 người thì tạo 2 nhánh nhỏ rồi merge).
- Không có task nào thuộc 2 story khác nhau (chỉ có 1 user story), nên không có cơ hội chạy song song giữa story.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (T002–T006) — đây là PBI nhỏ, chỉ 1 story, không chia nhỏ thêm.
- Thứ tự giao hàng: T001 (xác nhận phạm vi) → T004 (state) → T005 (template) → T006 (CSS/transition) → T002+T003 (hoàn thiện test theo hành vi mới) → T007 (xác nhận test pass, chụp bằng chứng).
