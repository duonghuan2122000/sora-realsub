# Danh sách Task: Chỉnh cỡ chữ phụ đề

**Mã PBI**: 24
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Xác nhận key `localStorage` dùng chung (`subtitle-font-size`), cỡ chữ mặc định (1.5rem, khớp CSS hiện tại) và min/max (ví dụ 1.0rem–3.0rem) trong `app/src/subtitle/SubtitleApp.vue` — không cần file/dependency mới.

## Pha 2: Foundational

*(Không có — tính năng chỉ chạm 1 file UI đã tồn tại, không có hạ tầng chung nào cần dựng trước.)*

## Pha 3: User Story 1 - Chỉnh và lưu cỡ chữ phụ đề (Ưu tiên: P1)

**Mục tiêu**: Người dùng tăng/giảm cỡ chữ qua nút +/-, cỡ chữ áp dụng ngay cho mọi dòng đang hiển thị, được lưu và khôi phục đúng ở lần mở app sau.

**Tiêu chí kiểm thử độc lập**: Chạy `app/src/subtitle/SubtitleApp.spec.ts` (test mới) pass; kiểm thử tay theo `quickstart.md` xác nhận cỡ chữ giữ sau khi đóng-mở lại app.

- [X] T002 [US1] Viết test trong `app/src/subtitle/SubtitleApp.spec.ts`: bấm nút tăng cỡ chữ → `font-size` của `.subtitle-text` tăng; bấm nút giảm → giảm; dừng lại ở min/max khi bấm liên tục vượt ngưỡng.
- [X] T003 [US1] Viết test trong `app/src/subtitle/SubtitleApp.spec.ts`: mock `localStorage` có giá trị cỡ chữ đã lưu trước → mount lại component → cỡ chữ ban đầu khớp giá trị đã lưu (không phải mặc định).
- [X] T004 [US1] Viết test trong `app/src/subtitle/SubtitleApp.spec.ts`: `localStorage` chứa giá trị hỏng/không hợp lệ (vd chuỗi rác) → mount component không lỗi, dùng cỡ chữ mặc định.
- [X] T005 [US1] Thêm state `fontSize` (ref, khởi tạo từ `localStorage.getItem('subtitle-font-size')` hoặc mặc định nếu rỗng/không hợp lệ) trong `app/src/subtitle/SubtitleApp.vue`.
- [X] T006 [US1] Thêm hàm `increaseFontSize`/`decreaseFontSize` clamp trong khoảng min/max, mỗi lần gọi ghi `fontSize` mới vào `localStorage` (`app/src/subtitle/SubtitleApp.vue`).
- [X] T007 [US1] Thêm 2 nút `+`/`-` vào template `.subtitle-overlay` trong `app/src/subtitle/SubtitleApp.vue`, đặt ngoài vùng ảnh hưởng của `data-tauri-drag-region` (thêm `data-tauri-drag-region="false"` hoặc CSS pointer riêng cho nhóm nút) để không bị nuốt event click.
- [X] T008 [US1] Bind `fontSize` vào `.subtitle-text` qua CSS variable (`:style="{ '--subtitle-font-size': fontSize + 'rem' }"` trên `.subtitle-overlay`, dùng `font-size: var(--subtitle-font-size)` trong `<style scoped>` thay giá trị cứng `1.5rem` hiện tại) trong `app/src/subtitle/SubtitleApp.vue`.
- [X] T009 [US1] Chạy `bun run tauri dev`, làm theo `quickstart.md` để xác nhận tay: nút +/- hoạt động đúng, không bị nuốt bởi vùng kéo-thả, cỡ chữ giữ nguyên sau khi đóng và mở lại app.

## Pha cuối: Polish & Cross-cutting

- [X] T010 Chụp ảnh/ghi lại kết quả test (`bun run test` pass) và kết quả kiểm thử tay (bước T009) làm bằng chứng, lưu tại `.specify/specs/24/evidence/`.

## Sơ đồ phụ thuộc

- T001 → T002-T009 (cần chốt key/khoảng min-max trước khi viết test và code).
- T002, T003, T004 (viết test trước) → T005, T006, T007, T008 (code để test pass) → T009 (kiểm thử tay) → T010 (bằng chứng).
- Chỉ có 1 user story (P1) — không có story khác cần sắp thứ tự song song.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (T002-T009) — tính năng không chia nhỏ được nữa, đã là đơn vị nhỏ nhất theo roadmap.
- Thứ tự giao hàng: Setup (T001) → Test trước (T002-T004) → Code (T005-T008) → Kiểm thử tay (T009) → Bằng chứng (T010).
