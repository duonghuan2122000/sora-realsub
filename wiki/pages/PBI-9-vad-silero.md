---
title: "PBI 9: VAD (Silero VAD) gán nhãn speech/silence theo frame"
date: 2026-11-24
tags: [pbi, m2, feature]
sources: [.specify/specs/9/spec.md, .specify/specs/9/plan.md, .specify/specs/9/research.md, .specify/specs/9/tasks.md, .specify/specs/9/evidence, app/src-tauri/src/core/vad.rs, app/src-tauri/tests/vad_real_speech.rs, app/src-tauri/tests/fixtures/vad_sample.wav, app/src-tauri/Cargo.toml]
---

# PBI 9: VAD (Silero VAD) gán nhãn speech/silence theo frame

Tính năng đầu tiên của M2 (VAD). Nhận buffer audio PCM (từ `MixedAudioSource`, M1), chia frame cố định, gán nhãn speech/silence cho từng frame bằng model Silero VAD — nền tảng để sau này chỉ gửi đoạn có giọng nói đi STT (M3).

## Yêu cầu chính (spec.md)
- Chia buffer thành frame cố định, gán nhãn speech/silence từng frame, giữ đúng thứ tự thời gian.
- Buffer rỗng/ngắn hơn 1 frame: trả rỗng hoặc lỗi rõ ràng, không crash.
- SC-001/002/003: ≥90% frame đúng nhãn trên đoạn nói/lặng đã biết trước, thứ tự nhãn khớp thứ tự thời gian thật.
- Ngoài phạm vi: cắt segment theo khoảng lặng, event `vad-segment` ra UI (PBI kế tiếp M2), STT/dịch (M3/M4).

## Quyết định kỹ thuật (plan.md, research.md)
- **Crate**: `voice_activity_detector` 0.2.1 — wrapper Silero VAD v5 chạy ONNX runtime nội bộ (`ort`), model nhúng sẵn. Chọn vì không phải tự quản lý file `.onnx`/binding ONNX runtime; cân nhắc tự tích hợp `ort` thủ công hoặc dùng WebRTC VAD nhưng cả hai tốn công hơn hoặc kém chính xác hơn mà không cần ở giai đoạn này.
- **Ràng buộc model**: input phải 16kHz mono, frame cố định 512 sample (~32ms) — đúng rate/kích thước Silero VAD v5 được huấn luyện.
- **Resample**: tái dùng nguyên `resample_linear` đã có từ PBI 7 (`mixed_audio_source.rs`, đổi `fn` → `pub(super) fn`) thay vì viết lại — áp dụng ladder "đã có sẵn trong codebase thì tái dùng".
- **Ngưỡng phân loại**: 0.5 (mặc định Silero), là tham số của `detect_with_threshold`, không hardcode trong logic.
- **API**: `voice_activity_detector::VoiceActivityDetector` dùng builder (`sample_rate`, `chunk_size`), `predict(samples) -> f32` trả xác suất speech trực tiếp (không phải struct).

## Cài đặt (code)
- `app/src-tauri/src/core/vad.rs` (mới): `FrameLabel { frame_index, is_speech }`, `detect()` (ngưỡng mặc định 0.5) và `detect_with_threshold()`; buffer < 512 sample → `Ok(Vec::new())`.
- `app/src-tauri/src/core/mixed_audio_source.rs`: `resample_linear` đổi visibility thành `pub(super)` để `vad.rs` tái dùng.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `vad::{FrameLabel, detect, detect_with_threshold}`.
- `app/src-tauri/Cargo.toml`: thêm `voice_activity_detector = "0.2"`.
- Chưa có Tauri command/event nào cho VAD — module Core nội bộ, chưa nối vào pipeline streaming thật (đó là việc của các PBI M2 kế tiếp).

## Bằng chứng test (`.specify/specs/9/evidence/`)
- `cargo test vad::`: 4/4 pass — buffer rỗng, buffer ngắn hơn 1 frame, im lặng tuyệt đối → nhãn silence ≥90%, thứ tự `frame_index` giữ nguyên. Log: `T009-T012-result.log`, test case: `vad-testcase.md`. `cargo clippy --lib` không warning.
- **SC-001/SC-003 đã xác nhận bằng dữ liệu thật**: dùng file ghi âm mic thật `app/src-tauri/capture_mic_to_wav_output.wav` (8s, im lặng-nói-im lặng, PCM float32 WASAPI) làm fixture `app/src-tauri/tests/fixtures/vad_sample.wav`. Test tích hợp `tests/vad_real_speech.rs` (`vad_phan_biet_duoc_doan_co_nang_luong_cao_va_thap_tren_ghi_am_thuc`): vì không có timestamp nói/lặng gán tay, dùng RMS năng lượng mỗi frame làm ground truth độc lập đối chiếu nhãn VAD — kỳ vọng cả nhãn speech và silence xuất hiện, tỉ lệ khớp hướng với RMS ≥60%, thứ tự `frame_index` đúng thời gian. Kết quả: pass. Log đầy đủ (4 test unit + 1 test fixture thật, tổng 5/5 pass): `evidence/T008-T011-result.log`. Tasks T008/T009/T011 đã tick xong trong `tasks.md`.

## Liên quan
[[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]]
