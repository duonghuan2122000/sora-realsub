# Danh sách Task: Trộn mic + loopback thành 1 stream

**Mã PBI**: 7
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [x] T001 Kiểm tra `app/src-tauri/Cargo.toml` không cần thêm dependency mới (chỉ dùng `std`) — nếu đúng, bỏ qua, không sửa file

## Pha 2: Foundational

- [x] T002 Khai báo module mới `mixed_audio_source` trong `app/src-tauri/src/core/mod.rs`

## Pha 3: User Story 1 - Trộn mic + loopback thành 1 AudioSource (Ưu tiên: P1)

**Mục tiêu**: Một `AudioSource` mới nhận 2 nguồn con (mic, loopback), chạy đồng thời, cộng mẫu theo chu kỳ để tạo 1 luồng PCM duy nhất chứa cả 2 nội dung, dừng được, không crash khi rate/channel khác nhau hoặc khi 1 nguồn im lặng.
**Tiêu chí kiểm thử độc lập**: Chạy bin thủ công trộn vài giây trong lúc vừa nói vào mic vừa phát nhạc qua loa, nghe lại file `.wav` xuất ra có cả 2 nội dung; `cargo test` pass các test trộn/dừng/im lặng một nguồn.

- [x] T003 [US1] Tạo `struct MixedAudioSource` implement trait `AudioSource` tại `app/src-tauri/src/core/mixed_audio_source.rs`, nhận 2 `Box<dyn AudioSource>` (mic, loopback) trong constructor
- [x] T004 [US1] Trong `MixedAudioSource::start`, khởi động 2 nguồn con qua `AudioSource::start`, mỗi nguồn đẩy `AudioFrame` vào 1 `VecDeque<f32>` riêng (có `Mutex`) kèm sample_rate đã ghi nhận; resample bằng linear interpolation khi 2 nguồn có sample_rate khác nhau (quy về rate cao hơn) tại `app/src-tauri/src/core/mixed_audio_source.rs` (đáp ứng FR-001, FR-003)
- [x] T005 [US1] Thêm thread trộn định kỳ (10-20ms/lần) trong `MixedAudioSource::start`: rút N mẫu từ mỗi buffer (thiếu thì coi là 0.0), cộng từng mẫu rồi `clamp(-1.0, 1.0)`, gọi `on_frame` ra ngoài; trả `StreamHandle` dừng cả 2 nguồn con + thread trộn tại `app/src-tauri/src/core/mixed_audio_source.rs` (đáp ứng FR-002, FR-004, FR-005)
- [x] T006 [US1] Viết unit test tại `app/src-tauri/src/core/mixed_audio_source.rs` dùng `AudioSource` giả (mock) để kiểm: trộn 2 nguồn có tiếng cho ra tổng đúng (có clamp khi vượt ngưỡng), 1 nguồn im lặng thì output khớp nguồn còn lại, `stop()` không panic và dừng nhận mẫu mới — ghi bằng chứng vào [evidence/T006-testcase.md](evidence/T006-testcase.md)
- [x] T007 [US1] Tạo bin thủ công `app/src-tauri/src/bin/capture_mixed_to_wav.rs`: nhận tham số số giây, chạy `MixedAudioSource::new(MicAudioSource, LoopbackAudioSource)` thật, ghi mẫu ra file `.wav` bằng `hound`, in đường dẫn file ra console (bằng chứng SC-001)
- [x] T008 [US1] Chạy thử `capture_mixed_to_wav` khi vừa nói vào mic vừa phát nhạc qua loa cùng lúc 5-10s, lưu file `.wav` + log console làm bằng chứng nghe được cả 2 nguồn (đáp ứng SC-001) — xem [evidence/T008-testcase.md](evidence/T008-testcase.md)
- [x] T009 [US1] Chạy thử `capture_mixed_to_wav` với chỉ 1 nguồn có âm thanh (chỉ mic, rồi chỉ loopback), xác nhận output khớp đúng nguồn đang có tiếng, lưu bằng chứng (đáp ứng SC-002) — xem [evidence/T009-testcase.md](evidence/T009-testcase.md)

## Pha cuối: Polish & Cross-cutting

- [x] T010 Chạy `cargo clippy --all-targets` và `cargo test` toàn bộ `app/src-tauri`, sửa warning — log tại [evidence/T010-result.log](evidence/T010-result.log)

## Sơ đồ phụ thuộc

Setup (T001) → Foundational (T002) → User Story 1 (T003 → T004 → T005 → T006; T007 cần T003-T005 xong; T008, T009 cần T007 xong) → Polish (T010).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (PBI nhỏ, chỉ 1 story).
- Thứ tự giao hàng: Setup → Foundational → US1 (code + test tự động) → US1 (bằng chứng test thủ công với audio thật) → Polish.
- Cơ hội song song: không có task `[P]` — các task phụ thuộc tuần tự vào cùng 1 file lõi (`mixed_audio_source.rs`).
