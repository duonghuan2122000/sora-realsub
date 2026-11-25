# Danh sách Task: Kéo-thả cửa sổ phụ đề

**Mã PBI**: 23
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

(Không cần — tái dùng cấu trúc `app/src/subtitle/` đã có từ PBI 20-22, không thêm dependency.)

## Pha 2: Foundational

(Không có task nền tảng riêng — thay đổi chỉ nằm trong 1 user story duy nhất của PBI này.)

## Pha 3: User Story 1 - Kéo-thả di chuyển cửa sổ phụ đề (Ưu tiên: P1)

**Mục tiêu**: Người dùng kéo-thả được cửa sổ phụ đề đến vị trí mong muốn; vị trí giữ nguyên khi phụ đề cập nhật; click không kéo không làm cửa sổ di chuyển.

**Tiêu chí kiểm thử độc lập**: Chạy `bun run tauri dev`, kéo cửa sổ phụ đề sang vị trí khác và thả — cửa sổ nằm đúng vị trí mới, không nhảy về chỗ cũ khi có phụ đề mới (theo `quickstart.md`).

- [X] T001 [US1] Thêm thuộc tính `data-tauri-drag-region` vào phần tử gốc `.subtitle-overlay` trong `app/src/subtitle/SubtitleApp.vue` (FR-001, FR-002, FR-003, FR-005)
- [X] T002 [US1] Thêm test kiểm tra template có attribute `data-tauri-drag-region` trên vùng gốc cửa sổ trong `app/src/subtitle/SubtitleApp.spec.ts`
- [X] T003 [US1] Chạy `bun run test` (Vitest) trong `app/`, xác nhận toàn bộ test SubtitleApp pass
- [X] T005 [US1] Thêm capability `app/src-tauri/capabilities/subtitle.json` cấp quyền `core:window:allow-start-dragging` cho window `subtitle` (phát sinh khi test tay T004: `data-tauri-drag-region` bị ACL chặn âm thầm vì `default.json` chỉ gán cho window `main`)
- [X] T004 [US1] Kiểm thử tay theo `.specify/specs/23/quickstart.md` trên Windows thật (`bun run tauri dev`): kéo-thả cửa sổ — người dùng xác nhận kéo được (SC-001); FR-005/SC-002 theo hành vi native `data-tauri-drag-region` của Tauri, không cần code riêng

## Pha cuối: Polish & Cross-cutting

(Không cần — thay đổi nhỏ, không có phần dọn dẹp/tối ưu riêng ngoài US1.)

## Sơ đồ phụ thuộc

- T001 → T002 → T003 → T004 (tuần tự, cùng US1, không chạy song song vì cùng sửa 1 cặp file liên quan trực tiếp).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (PBI này chỉ có 1 story).
- Thứ tự giao hàng: T001 → T002 → T003 → T004, hoàn tất là xong PBI 23.
