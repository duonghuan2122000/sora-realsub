---
title: "PBI 8: Lệnh start_capture/stop_capture"
date: 2026-11-24
tags: [pbi, m1, feature]
sources: [.specify/specs/8/spec.md, .specify/specs/8/plan.md, .specify/specs/8/research.md, .specify/specs/8/tasks.md, .specify/specs/8/evidence, app/src-tauri/src/core/capture_session.rs, app/src-tauri/src/lib.rs]
---

# PBI 8: Lệnh start_capture/stop_capture

Tính năng cuối cùng của M1 (Audio Capture): thêm lớp điều khiển phiên thu âm (start/stop) qua Tauri IPC, nằm trên `MixedAudioSource` (PBI 7). Trước PBI này, nguồn mic+loopback đã trộn được chỉ chạy được qua bin CLI ghi `.wav`, chưa có cách bật/tắt từ UI.

## Yêu cầu chính (spec.md)
- Lệnh bắt đầu phiên thu âm (mic + loopback trộn) và lệnh dừng phiên đang chạy.
- Từ chối bắt đầu phiên mới khi đã có phiên chạy, và từ chối dừng khi chưa có phiên nào — cả hai kèm lỗi rõ nghĩa, không crash.
- Khi dừng, phải giải phóng toàn bộ thread/handle đã cấp cho phiên đó — lặp 10 lần start→stop không tích lũy leak.
- Ngoài phạm vi: UI nút Start/Stop (M6), VAD/STT/dịch (M2-M4).

## Quyết định kỹ thuật (plan.md, research.md)
- **Trạng thái phiên** = `CaptureSessionState { handle: Mutex<Option<StreamHandle>> }`, quản lý qua `tauri::State` — tái dùng nguyên `StreamHandle` đã có (PBI 3/5/6/7) làm nguồn sự thật duy nhất, tránh lệch trạng thái giữa cờ "đang chạy" và handle thật (từng xem xét `AtomicBool` riêng, loại vì có thể race).
- **Nguồn thu âm**: tái dùng `MixedAudioSource::new(MicAudioSource::new(None), LoopbackAudioSource::new())` nguyên trạng từ PBI 7, không đổi.
- **2 lỗi mới** trong `CoreError`: `AlreadyRunning`, `NotRunning` — để `start()`/`stop()` trả lỗi rõ nghĩa theo đúng spec, thay vì dùng chung `Provider`/`Io` mơ hồ.
- **Test không leak**: dùng mock `AudioSource` đếm số handle "sống" qua `AtomicUsize` (tăng khi `start()`, giảm khi `stop_fn` chạy) — tránh phụ thuộc thiết bị âm thanh thật/CI headless, vẫn xác minh đúng hợp đồng start cấp đúng 1 resource / stop giải phóng đúng resource đó.
- **Command Tauri**: `start_capture()` / `stop_capture()` trả `Result<(), String>`, theo đúng convention `list_input_devices_cmd` đã có.

## Cài đặt (code)
- `app/src-tauri/src/core/capture_session.rs` (mới): `CaptureSessionState` với `start(source, on_frame)` và `stop()`; test mock đếm resource sống cho 2 lỗi (AlreadyRunning/NotRunning) + lặp 10 lần start/stop.
- `app/src-tauri/src/core/error.rs`: thêm 2 variant `CoreError::AlreadyRunning`, `CoreError::NotRunning`.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `CaptureSessionState`.
- `app/src-tauri/src/lib.rs`: thêm `#[tauri::command] start_capture`/`stop_capture` dùng `State<CaptureSessionState>`; `.manage(CaptureSessionState::new())` và đăng ký 2 command vào `generate_handler!`.
- Không sửa `Cargo.toml` — không cần dependency mới.

## Bằng chứng test (`.specify/specs/8/evidence/`)
- `cargo test`: 26/26 pass (thêm 5 test mới cho `capture_session`: start thành công khi rảnh, start thất bại khi đã chạy, stop thành công giải phóng handle, stop thất bại khi chưa chạy, lặp 10 lần start/stop không leak). Log: `T005-T006-result.log`, test case: `T005-T006-testcase.md`.
- **Giới hạn ghi nhận rõ**: T009 (kiểm thử tay qua `npm run tauri dev` + `invoke()` trong DevTools, theo `quickstart.md`) **chưa chạy** — môi trường thi công (CLI, không có GUI session) không mở được cửa sổ Tauri dev. 4 kịch bản chấp nhận trong spec.md đã được phủ qua unit test gọi trực tiếp `CaptureSessionState` (không qua IPC thật); cần verify tay trên máy có GUI trước khi coi PBI này kiểm thử đầy đủ end-to-end.

## Liên quan
[[Core-Rust]], [[Kien-truc-core-ui]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[PBI-7-tron-mic-loopback]]
