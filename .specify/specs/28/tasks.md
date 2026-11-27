# Danh sách Task: Nút Start/Stop phiên dịch phụ đề

**Mã PBI**: 28
**Nguồn**: plan.md, spec.md, data-model.md, quickstart.md

## Pha 1: Setup

- [X] T001 Đọc `app/src/App.vue` và `app/src/components/DeviceSelector.vue` hiện có, xác nhận prop/emit để biết cách khoá chọn thiết bị (không tạo file mới, chỉ khảo sát trước khi sửa).

## Pha 2: Foundational

- [X] T002 Thêm `ref<SessionButtonState>` (`'idle' | 'starting' | 'running' | 'stopping'`, khởi tạo `'idle'`) và `sessionError` (string) vào `app/src/App.vue`.

## Pha 3: User Story 1 - Nút Start/Stop đồng bộ trạng thái Core (Ưu tiên: P1)

**Mục tiêu**: Người dùng bấm Start/Stop, nút luôn phản ánh đúng trạng thái thật của Core; khoá chọn thiết bị khi phiên đang chạy/chuyển tiếp.
**Tiêu chí kiểm thử độc lập**: Chạy `bun run tauri dev`, làm theo `quickstart.md` bước 2–8; hoặc chạy test Vitest mới ở `App.test.ts`.

- [X] T003 [US1] Viết hàm `startSession()` trong `app/src/App.vue`: set state `'starting'`, gọi `invoke('start_capture')`, `.then` → state `'running'`, `.catch` → set `sessionError` + state về `'idle'`.
- [X] T004 [US1] Viết hàm `stopSession()` trong `app/src/App.vue`: set state `'stopping'`, gọi `invoke('stop_capture')`, `.then`/`.catch` (kể cả lỗi `NotRunning`) → set state về `'idle'` (chỉ `.catch` ghi `sessionError` nếu lỗi khác `NotRunning`).
- [X] T005 [US1] Thêm nút Start/Stop vào template `app/src/App.vue`: nhãn theo bảng ở `data-model.md` (`Start`/`Đang chạy...`/`Stop`/`Đang dừng...`), `:disabled` khi state ∈ `{starting, stopping}` hoặc (state === 'idle' và thiếu mic/loopback — tái dùng điều kiện đã có ở `confirmSelection`), `@click` gọi `startSession`/`stopSession` tùy state hiện tại.
- [X] T006 [US1] Hiển thị `sessionError` trong template `app/src/App.vue` (dùng lại class `.error` đã có).
- [X] T007 [US1] Khoá 2 `DeviceSelector` trong `app/src/App.vue` (prop `disabled` hoặc tương đương — kiểm tra component có hỗ trợ sẵn chưa ở T001, thêm prop nếu thiếu tại `app/src/components/DeviceSelector.vue`) khi state khác `'idle'`.
- [X] T008 [US1] [P] Viết test `app/src/App.test.ts` (mock `@tauri-apps/api/core` `invoke`): test chuyển `idle→starting→running` khi resolve, `starting→idle` + có `sessionError` khi reject, `running→stopping→idle` khi Stop resolve, nút bị disable đúng lúc transition, bấm nhiều lần lúc transition chỉ gọi `invoke` 1 lần.

## Pha cuối: Polish & Cross-cutting

- [X] T009 Chạy `bun run test` (Vitest) — 28/28 PASS, log ở `evidence/T008-result.log`. **Chưa** làm thủ công `quickstart.md` trên `bun run tauri dev` với thiết bị audio thật trong môi trường này (không có GUI/thiết bị thật); cần người có máy Windows thật xác nhận bước 2–8 của `quickstart.md` trước khi coi là xác nhận đầy đủ trên thực tế.

## Sơ đồ phụ thuộc

T001 → T002 → (T003, T004 có thể làm song song vì sửa 2 hàm độc lập trong cùng file, nhưng khuyến nghị làm tuần tự để tránh conflict khi edit cùng file) → T005 → T006 → T007 → T008 → T009.

Chỉ có 1 user story (P1) — không có story khác để phụ thuộc chéo.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ US1 (feature nhỏ, không chia nhỏ thêm được).
- Thứ tự giao hàng: Setup → Foundational → US1 (T003→T008 tuần tự do cùng sửa `App.vue`) → Polish (T009).
