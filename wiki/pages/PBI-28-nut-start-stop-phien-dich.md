---
title: PBI 28 — Nút Start/Stop phiên dịch phụ đề
date: 2026-11-27
tags: [pbi, M6, ui-vue]
sources: [.specify/specs/28/spec.md, .specify/specs/28/plan.md, .specify/specs/28/research.md, .specify/specs/28/tasks.md, .specify/specs/28/evidence/]
---

# PBI 28 — Nút Start/Stop phiên dịch phụ đề

Mục thứ 2 của **M6 (Điều khiển & cấu hình)**, sau [[PBI-26-man-hinh-chinh-chon-thiet-bi]]. Chọn PBI này sau khi bỏ qua 2 mục roadmap M1 còn lại (PBI 18 Linux, hồi quy Windows) vì cả hai chỉ cần xác nhận tay trên máy khác, không phải việc code được — cùng lý do đã áp dụng khi chọn PBI 26.

## Quyết định chính

- **Không thêm lệnh Core mới**: tái dùng trực tiếp `start_capture`/`stop_capture` đã có từ [[PBI-8-start-stop-capture]] (qua `CaptureSessionState`, trả lỗi `AlreadyRunning`/`NotRunning`). UI chỉ gọi và phản ánh kết quả, giữ đúng ranh giới Core/UI ([[Kien-truc-core-ui]]).
- **State máy 4 giá trị ở frontend** (`idle`/`starting`/`running`/`stopping`) thay vì boolean đơn — cần để khoá nút đúng lúc giữa khoảng chờ `invoke()` async, tránh gửi lệnh trùng khi người dùng bấm nhiều lần.
- **Khoá `DeviceSelector` bằng attribute HTML `inert`** khi phiên không ở `idle` — tái dùng tính năng native của browser/webview thay vì thêm prop `disabled` xuyên qua component con.
- **Hạn chế đã biết, ghi nhận thay vì mở rộng phạm vi**: Core hiện không phát event báo "phiên tự dừng do lỗi runtime" giữa lúc chạy (`model-error` chỉ báo lỗi STT/dịch, không dừng phiên) — nên nếu Core crash thật giữa phiên, nút UI vẫn hiển thị "Stop" cho tới khi người dùng bấm Stop và nhận lỗi `NotRunning` mới đồng bộ lại. Không sửa Core để thêm event này trong PBI 28.
- **Chưa nối ID thiết bị đã chọn (PBI 26) vào `start_capture`** — lệnh này vẫn dùng mic/loopback mặc định hệ điều hành, giống hạn chế đã ghi ở PBI 26, PBI 6/18.

## Cài đặt

- `app/src/App.vue`: thêm `type SessionButtonState = "idle" | "starting" | "running" | "stopping"`, `ref<SessionButtonState>` + `sessionError`; `startSession()`/`stopSession()` gọi `invoke("start_capture"/"stop_capture")`; computed `sessionLabel`/`sessionButtonDisabled`/`devicesLocked`; nút mới `.session-btn`; `.device-panels` nhận `:inert="devicesLocked"`.
- Đổi nút "Xác nhận" (PBI 26) thêm class `.confirm-btn` để phân biệt với `.session-btn` mới — không đổi hành vi.
- Không đụng `app/src-tauri` (Core Rust).

## Bằng chứng kiểm thử

- Vitest: **28/28 pass** (10 test cũ + sửa 3 selector do thêm nút mới, 7 test mới cho nút Start/Stop: disabled khi thiếu thiết bị, chuyển trạng thái khi resolve, chống bấm trùng lúc đang chờ, hiển thị lỗi khi reject, Stop quay về Start, khoá `device-panels` qua `inert`). Xem `evidence/T008-testcase.md`, `evidence/T008-result.log`.
- **Chưa làm thủ công `quickstart.md` trên `bun run tauri dev` với thiết bị audio thật** — môi trường implement không có GUI/thiết bị thật, cùng hạn chế đã gặp ở [[PBI-26-man-hinh-chinh-chon-thiet-bi]], [[PBI-18-loopback-linux]], PBI 24.

## Liên quan

- [[PBI-8-start-stop-capture]] — nguồn gốc lệnh `start_capture`/`stop_capture` mà PBI này tái dùng nguyên trạng.
- [[PBI-26-man-hinh-chinh-chon-thiet-bi]] — mục M6 liền trước, cùng file `App.vue`; để lại hạn chế "chưa nối lựa chọn thiết bị vào capture thật" mà PBI 28 vẫn giữ nguyên.
- [[Kien-truc-core-ui]] — UI chỉ gọi lệnh có sẵn + hiển thị trạng thái, không tự quyết định hành vi capture.
- [[Trang-thai-ma-nguon]] — cập nhật: M6 (2/4).
- [[Rui-ro-ky-thuat]] — bổ sung hạn chế: Core chưa có event báo tự dừng do lỗi runtime giữa phiên.
