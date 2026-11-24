# Danh sách Task: Capture mic thành PCM buffer

**Mã PBI**: 5
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm dependency `hound` vào `app/src-tauri/Cargo.toml` (khai ở `[dependencies]` vì bin thường không nhận dev-dependencies)

## Pha 2: Foundational

- [X] T002 Khai báo module mới trong `app/src-tauri/src/core/mod.rs` cho `mic_audio_source` (cũng đổi `mod core;` → `pub mod core;` ở `lib.rs` để bin thủ công dùng được)

## Pha 3: User Story 1 - Capture mic thành PCM buffer (Ưu tiên: P1)

**Mục tiêu**: Người dùng bắt đầu capture từ mic đã chọn, hệ thống liên tục nhận PCM qua callback `AudioSource`, dừng được, báo lỗi rõ khi thiết bị không hợp lệ.
**Tiêu chí kiểm thử độc lập**: Chạy bin thủ công capture vài giây từ mic thật, nghe lại file `.wav` xuất ra khớp nội dung đã nói; `cargo test` pass các test start/stop/lỗi thiết bị.

- [X] T003 [US1] Tạo `struct MicAudioSource` implement trait `AudioSource` dùng `cpal` tại `app/src-tauri/src/core/mic_audio_source.rs` (mở input stream theo device id, convert mẫu về `f32` mono vào `AudioFrame`, trả `StreamHandle` dừng được stream). Ghi chú: `cpal::Stream` không phải `Send`, nên stream được tạo/giữ sống trên 1 thread riêng, giao tiếp qua `mpsc` channel (ready/stop) với `StreamHandle`.
- [X] T004 [US1] Xử lý lỗi thiết bị không hợp lệ/không khả dụng trong `MicAudioSource::start` tại `app/src-tauri/src/core/mic_audio_source.rs`, trả `CoreError` rõ ràng thay vì panic (đáp ứng FR-003)
- [X] T005 [US1] Viết unit test tại `app/src-tauri/src/core/mic_audio_source.rs`: thiết bị không tồn tại trả lỗi rõ nghĩa; `stop()` trên handle thật (nếu máy có mic) không panic. Trường hợp "dừng khi chưa từng start" không cần test runtime vì API không cho gọi `stop()` khi chưa có handle (đảm bảo bởi kiểu dữ liệu) — xem [evidence/T005-testcase.md](evidence/T005-testcase.md)
- [X] T006 [US1] Tạo bin thủ công `app/src-tauri/src/bin/capture_mic_to_wav.rs`: nhận tham số số giây, capture mic thật qua `MicAudioSource`, ghi mẫu ra file `.wav` bằng `hound`, in đường dẫn file ra console (bằng chứng SC-001)
- [X] T007 [US1] Chạy thử `capture_mic_to_wav` với mic thật 5s, lưu file `.wav` kết quả + log console làm bằng chứng test (đáp ứng SC-001, SC-002) — xem [evidence/T007-testcase.md](evidence/T007-testcase.md)
- [X] T008 [US1] Chạy thử `capture_mic_to_wav` với device id không hợp lệ, xác nhận in lỗi rõ ràng không treo/crash, lưu log bằng chứng (đáp ứng SC-003) — xem [evidence/T008-testcase.md](evidence/T008-testcase.md)

## Pha cuối: Polish & Cross-cutting

- [X] T009 Chạy `cargo clippy --all-targets` và `cargo test` toàn bộ `app/src-tauri`, sửa warning (type-complexity ở `capture_mic_to_wav.rs`) — log tại [evidence/T009-result.log](evidence/T009-result.log)

## Sơ đồ phụ thuộc

Setup (T001) → Foundational (T002) → User Story 1 (T003 → T004 → T005; T006 cần T003+T004 xong; T007, T008 cần T006 xong) → Polish (T009).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, chỉ 1 story).
- Thứ tự giao hàng: Setup → Foundational → US1 (code + test tự động) → US1 (bằng chứng test thủ công với mic thật) → Polish.
- Cơ hội song song: không có task `[P]` — các task phụ thuộc tuần tự vào cùng 1-2 file lõi (`mic_audio_source.rs`).
