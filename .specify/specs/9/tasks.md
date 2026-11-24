# Danh sách Task: Phát hiện hoạt động giọng nói (VAD)

**Mã PBI**: 9
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm dependency `voice_activity_detector` vào `app/src-tauri/Cargo.toml`, chạy `cargo build` xác nhận tải/compile được.

## Pha 2: Foundational

- [X] T002 Tạo file `app/src-tauri/src/core/vad.rs` với struct `FrameLabel { frame_index: usize, is_speech: bool }` và khung hàm `pub fn detect(frames: &[AudioFrame]) -> Result<Vec<FrameLabel>, CoreError>` (tái dùng kiểu lỗi hiện có trong `app/src-tauri/src/core/error.rs`).
- [X] T003 Thêm `pub mod vad;` vào `app/src-tauri/src/core/mod.rs`.
- [X] T004 Cài đặt hàm resample về 16kHz mono (nếu `sample_rate` của `AudioFrame` khác 16000) và chia buffer thành frame 512 sample trong `app/src-tauri/src/core/vad.rs` — tái dùng `resample_linear` đã có từ `mixed_audio_source.rs` (đổi thành `pub(super)`) thay vì viết lại.

## Pha 3: User Story 1 - Gán nhãn speech/silence theo frame (Ưu tiên: P1)

**Mục tiêu**: Với một buffer audio PCM, hệ thống trả về danh sách nhãn speech/silence cho từng frame, đúng thứ tự thời gian.
**Tiêu chí kiểm thử độc lập**: Chạy file WAV mẫu có nói + lặng xen kẽ qua `detect()`, so khớp nhãn trả về với mốc thời gian nói/lặng đã biết trước, đạt ngưỡng ở SC-001/SC-002/SC-003 trong spec.md.

- [X] T005 [US1] Khởi tạo model Silero VAD (qua crate `voice_activity_detector`) trong `app/src-tauri/src/core/vad.rs`, áp ngưỡng xác suất mặc định 0.5 (tham số có thể cấu hình qua field struct, không hardcode trong logic).
- [X] T006 [US1] Hoàn thiện `detect()`: chạy model trên từng frame 512 sample, map xác suất → `FrameLabel.is_speech`, giữ đúng thứ tự `frame_index` theo buffer gốc.
- [X] T007 [US1] Xử lý buffer rỗng hoặc ngắn hơn 1 frame: trả về `Vec` rỗng, không panic/crash (theo FR-004).
- [X] T008 [US1] Dùng file ghi âm mic thật `app/src-tauri/capture_mic_to_wav_output.wav` (8s, im lặng-nói-im lặng) làm `tests/fixtures/vad_sample.wav`.
- [X] T009 [US1] [P] Xác nhận SC-001 bằng dữ liệu giọng nói thật trong `tests/vad_real_speech.rs` (đối chiếu nhãn VAD với RMS năng lượng, agreement ≥60%).
- [X] T010 [US1] [P] Viết test tích hợp xác nhận ≥90% frame trong đoạn lặng được nhãn "silence" (SC-002) — dùng tín hiệu im lặng tuyệt đối tổng hợp thay cho fixture WAV thật (xem `pure_silence_is_labeled_silence` trong `vad.rs`).
- [X] T011 [US1] Xác nhận SC-003 trên dữ liệu nói+lặng xen kẽ thật: `vad_real_speech.rs` kiểm tra có cả nhãn speech và silence trên fixture 8s, cộng test thứ tự nhãn (`labels_keep_original_frame_order`).
- [X] T012 [US1] Viết test xác nhận buffer rỗng/ngắn hơn 1 frame không panic, trả `Vec` rỗng (`empty_buffer_returns_empty_labels`, `buffer_shorter_than_one_frame_returns_empty_labels` trong `vad.rs`).

## Pha cuối: Polish & Cross-cutting

- [X] T013 Lưu log kết quả `cargo test vad::` (4/4 pass) làm bằng chứng tại `.specify/specs/9/evidence/T009-T012-result.log` và test case tại `.specify/specs/9/evidence/vad-testcase.md`.
- [X] T014 Review code `vad.rs`: không export logic VAD ra Tauri command/UI (chỉ `pub use` trong `core::mod`), tái dùng `resample_linear` có sẵn, chạy `cargo clippy --lib` không warning.

## Sơ đồ phụ thuộc

- Pha 1 (Setup) → Pha 2 (Foundational) → Pha 3 (User Story 1) → Pha cuối (Polish).
- Chỉ có 1 user story (P1) trong PBI này — không có story song song độc lập ở cấp story.
- Trong Pha 3: T005 → T006 → T007 phải làm tuần tự (cùng file `vad.rs`); T008 (chuẩn bị fixture) có thể làm song song với T005–T007; T009 và T010 có thể chạy song song với nhau sau khi T006, T008 xong (khác assertion, cùng đọc fixture, không sửa cùng đoạn code); T011, T012 làm sau khi T009/T010 ổn định cấu trúc test.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, chỉ có 1 story).
- Thứ tự giao hàng: Setup → Foundational → US1 (model + logic trước, test theo sau để xác nhận SC-001/002/003) → Polish (bằng chứng test + review).
