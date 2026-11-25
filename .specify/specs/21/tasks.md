# Danh sách Task: Hiển thị phụ đề

**Mã PBI**: 21
**Nguồn**: plan.md, spec.md, data-model.md, quickstart.md

## Pha 1: Setup

- [X] T001 Xác nhận devDependencies test (`vitest`, `@vue/test-utils`, `jsdom`) đã có trong `app/package.json`, không cần cài thêm gì.

## Pha 2: Foundational

- [X] T002 Đọc lại payload `SubtitlePayload` (`startFrameIndex`, `endFrameIndex`, `sourceText`, `translatedText`) tại `app/src-tauri/src/lib.rs` để đối chiếu đúng tên trường camelCase khi viết UI (không sửa file Rust).

## Pha 3: User Story 1 — Hiển thị phụ đề khi có event `subtitle-ready` (Ưu tiên: P1)

**Mục tiêu**: Cửa sổ phụ đề hiển thị đúng, đúng thứ tự nội dung dịch mỗi khi Core phát event `subtitle-ready`.
**Tiêu chí kiểm thử độc lập**: Phát giả lập 1 hoặc nhiều event `subtitle-ready` liên tiếp vào component, xác nhận nội dung hiển thị khớp `translatedText` mới nhất, đúng thứ tự, không rớt sự kiện.

- [X] T003 [US1] Thêm state `currentSubtitle` (`ref<string>('')`) và `onMounted`/`onUnmounted` subscribe/unsubscribe event `subtitle-ready` qua `listen` từ `@tauri-apps/api/event` trong `app/src/subtitle/SubtitleApp.vue`.
- [X] T004 [US1] Cập nhật `currentSubtitle.value = payload.translatedText` trong callback của `listen`, xử lý trường hợp `translatedText` rỗng (không hiển thị dòng trống gây khó chịu — có thể ẩn phần tử khi rỗng) tại `app/src/subtitle/SubtitleApp.vue`.
- [X] T005 [US1] Render `currentSubtitle` trong template của `app/src/subtitle/SubtitleApp.vue`, thêm CSS đảm bảo text dài (~200 ký tự) wrap/ẩn tràn thay vì vỡ layout (`word-wrap`, `overflow`) trong `<style scoped>` cùng file.
- [X] T006 [P] [US1] Tạo `app/src/subtitle/SubtitleApp.spec.ts`: mock `@tauri-apps/api/event` (`listen`), test case phát 1 event → hiển thị đúng text; test case phát 20 event liên tiếp → hiển thị đúng nội dung + đúng thứ tự (event cuối cùng là nội dung hiển thị sau cùng); test case `translatedText` rỗng → không hiển thị dòng trống.
- [X] T007 [US1] Chạy `npm run test` (hoặc `bun run test`) trong `app/`, xác nhận toàn bộ test ở T006 pass; chụp ảnh/log kết quả làm bằng chứng theo yêu cầu của `/sora-implement`.

## Pha cuối: Polish & Cross-cutting

- [X] T008 Chạy `npm run build` (gồm `vue-tsc --noEmit`) trong `app/` để xác nhận không có lỗi type/biên dịch sau khi sửa `SubtitleApp.vue`.

## Sơ đồ phụ thuộc

T001 → T002 → T003 → T004 → T005 → (T006 song song được với T005 vì khác file, nhưng nên viết test sau khi có component để biết chính xác hành vi) → T007 → T008.
Chỉ có 1 user story (P1) nên không có phụ thuộc chéo giữa các story.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, 1 story duy nhất).
- Thứ tự giao hàng: Setup → Foundational → US1 (component + test) → Polish (build check).
