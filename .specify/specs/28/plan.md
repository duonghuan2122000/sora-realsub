# Kế hoạch triển khai: Nút Start/Stop phiên dịch phụ đề

**Mã PBI**: 28
**Liên kết spec**: .specify/specs/28/spec.md
**Ngày tạo**: 2026-11-27

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | TypeScript (Vue 3, `<script setup>`) ở UI; không cần sửa Rust/Core |
| Framework / Thư viện chính | Tauri 2 IPC (`@tauri-apps/api/core` `invoke`), Vue 3 `ref` |
| Lưu trữ dữ liệu | Không — trạng thái nút chỉ sống trong session UI (không persist) |
| Kiểm thử | Vitest (đã dùng ở `DeviceSelector.vue`, `SubtitleApp.vue`) |
| Nền tảng triển khai | Desktop app hiện có (Windows/Linux), không đổi |
| Ràng buộc hiệu năng | Không đáng kể — chỉ 1 lệnh IPC mỗi lần bấm |
| Ràng buộc khác | Lệnh Core `start_capture`/`stop_capture` đã tồn tại, không sửa Core (xem research.md quyết định 1, 3) |

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong repo → bỏ qua bước này; vẫn tuân thủ nguyên tắc tách Core/UI đã nêu ở `CLAUDE.md` (không đưa logic nghiệp vụ vào UI — UI chỉ gọi lệnh có sẵn và hiển thị trạng thái, không tự quyết định hành vi capture).

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`. Tóm tắt:

- **Quyết định**: Tái dùng `start_capture`/`stop_capture` có sẵn, không thêm lệnh Core mới.
- **Quyết định**: State máy 4 giá trị (`idle`/`starting`/`running`/`stopping`) ở frontend để khoá nút đúng lúc.
- **Quyết định**: Lỗi chỉ xử lý qua reject của `invoke()`; case Core tự crash giữa phiên không có event báo, ghi nhận là hạn chế hiện tại, ngoài phạm vi PBI này.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: xem `data-model.md` — enum `SessionButtonState` + bảng chuyển trạng thái.
- **Hợp đồng giao diện**: không có — chỉ dùng lại 2 lệnh Tauri nội bộ đã tồn tại (`start_capture`, `stop_capture`), không có hợp đồng mới cần ghi.
- **Kịch bản khởi động nhanh**: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```text
app/src/App.vue          # sửa: thêm state SessionButtonState + nút Start/Stop + khoá DeviceSelector khi không idle
app/src/App.test.ts      # mới (hoặc mở rộng test file hiện có): test chuyển trạng thái nút, khoá khi transition, hiển thị lỗi
```

Không sửa file Rust/Core — `start_capture`/`stop_capture` đã đủ dùng.

## Rủi ro & ngoại lệ có lý do

- Việc truyền ID mic/loopback đã chọn vào `start_capture` (hiện lệnh này dùng mic mặc định, không nhận tham số thiết bị) nằm ngoài phạm vi PBI 28 theo giả định đã ghi ở spec.md — tasks sẽ không sửa `start_capture` để nhận device ID, chỉ gọi nguyên trạng.
- Core chưa có event báo "phiên tự dừng do lỗi runtime" giữa lúc chạy — nếu điều này xảy ra thật, nút UI sẽ vẫn hiển thị "Stop" dù Core đã dừng, cho tới khi người dùng bấm Stop và nhận lỗi `NotRunning` (lúc đó mới đồng bộ lại). Không có ngoại lệ hiến pháp nào bị vi phạm, chỉ là hạn chế đã biết, ghi lại để cân nhắc ở PBI sau (ví dụ thêm event `capture-stopped`).
