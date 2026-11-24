# Danh sách Task: Capture loopback âm thanh hệ thống (Windows)

**Mã PBI**: 6
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [x] T001 Thêm dependency `wasapi` vào `app/src-tauri/Cargo.toml` (`[dependencies]`)

## Pha 2: Foundational

- [x] T002 Khai báo module mới `loopback_audio_source` trong `app/src-tauri/src/core/mod.rs`

## Pha 3: User Story 1 - Capture loopback hệ thống thành PCM buffer (Ưu tiên: P1)

**Mục tiêu**: Hệ thống capture loopback từ thiết bị output mặc định, liên tục nhận PCM qua callback `AudioSource`, dừng được, không lỗi khi im lặng, báo lỗi rõ khi không có thiết bị output.
**Tiêu chí kiểm thử độc lập**: Chạy bin thủ công capture vài giây trong lúc phát nhạc/âm thanh qua loa, nghe lại file `.wav` xuất ra khớp nội dung đã phát; `cargo test` pass các test start/stop/lỗi thiết bị.

- [x] T003 [US1] Tạo `struct LoopbackAudioSource` implement trait `AudioSource` dùng `wasapi` tại `app/src-tauri/src/core/loopback_audio_source.rs` (mở loopback capture stream từ thiết bị output mặc định, convert mẫu về `f32` mono vào `AudioFrame`, trả `StreamHandle` dừng được stream). Giữ pattern thread riêng + `mpsc` channel (ready/stop) giống `mic_audio_source.rs` vì stream/handle WASAPI không `Send` an toàn.
- [x] T004 [US1] Xử lý lỗi không có thiết bị output khả dụng/sự cố thiết bị trong `LoopbackAudioSource::start` tại `app/src-tauri/src/core/loopback_audio_source.rs`, trả `CoreError` rõ ràng thay vì panic (đáp ứng FR-003)
- [x] T005 [US1] Viết unit test tại `app/src-tauri/src/core/loopback_audio_source.rs`: `stop()` trên handle thật (nếu máy có thiết bị output) không panic; test khởi tạo với trạng thái bất thường (nếu mô phỏng được qua API `wasapi`) trả lỗi rõ nghĩa — ghi bằng chứng vào [evidence/T005-testcase.md](evidence/T005-testcase.md)
- [x] T006 [US1] Tạo bin thủ công `app/src-tauri/src/bin/capture_loopback_to_wav.rs`: nhận tham số số giây, capture loopback thật qua `LoopbackAudioSource`, ghi mẫu ra file `.wav` bằng `hound`, in đường dẫn file ra console (bằng chứng SC-001)
- [x] T007 [US1] Chạy thử `capture_loopback_to_wav` với âm thanh/nhạc thật đang phát 5s, lưu file `.wav` kết quả + log console làm bằng chứng test (đáp ứng SC-001, SC-002) — xem [evidence/T007-testcase.md](evidence/T007-testcase.md)
- [x] T008 [US1] Chạy thử `capture_loopback_to_wav` khi không phát gì (im lặng), xác nhận chạy bình thường không lỗi/crash, lưu log bằng chứng (đáp ứng SC-003, trường hợp biên im lặng) — xem [evidence/T008-testcase.md](evidence/T008-testcase.md)

## Pha cuối: Polish & Cross-cutting

- [x] T009 Chạy `cargo clippy --all-targets` và `cargo test` toàn bộ `app/src-tauri`, sửa warning — log tại [evidence/T009-result.log](evidence/T009-result.log)

## Sơ đồ phụ thuộc

Setup (T001) → Foundational (T002) → User Story 1 (T003 → T004 → T005; T006 cần T003+T004 xong; T007, T008 cần T006 xong) → Polish (T009).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, chỉ 1 story).
- Thứ tự giao hàng: Setup → Foundational → US1 (code + test tự động) → US1 (bằng chứng test thủ công với âm thanh thật) → Polish.
- Cơ hội song song: không có task `[P]` — các task phụ thuộc tuần tự vào cùng 1-2 file lõi (`loopback_audio_source.rs`).
