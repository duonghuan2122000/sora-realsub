# Danh sách Task: Cắt audio thành segment theo khoảng lặng

**Mã PBI**: 10
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Không cần thêm dependency mới (research.md). Xác nhận `cargo build` trong `app/src-tauri` vẫn chạy sạch trước khi bắt đầu.

## Pha 2: Foundational

- [X] T002 Tạo file `app/src-tauri/src/core/segment.rs` với struct `Segment { start_frame_index: usize, end_frame_index: usize }` và khung hàm `pub fn segment(labels: &[FrameLabel]) -> Vec<Segment>` (dùng `FrameLabel` từ `core::vad`).
- [X] T003 Thêm `pub mod segment;` vào `app/src-tauri/src/core/mod.rs`.
- [X] T004 Định nghĩa hằng số `DEFAULT_MIN_SILENCE_FRAMES` (tương ứng ~300ms silence theo research.md) trong `app/src-tauri/src/core/segment.rs`.

## Pha 3: User Story 1 - Cắt liên tục thành các lượt nói tách biệt (Ưu tiên: P1)

**Mục tiêu**: Từ danh sách `FrameLabel` (VAD), trả về danh sách `Segment` theo đúng thứ tự thời gian, mỗi segment tương ứng 1 lượt nói, không bị cắt gãy bởi khoảng lặng ngắn.
**Tiêu chí kiểm thử độc lập**: Chạy file WAV mẫu có nhiều lượt nói tách biệt qua `vad::detect()` → `segment::segment()`, số segment khớp đúng số lượt nói thực tế, đạt SC-001/SC-002/SC-003 trong spec.md.

- [X] T005 [US1] Cài đặt hàm `segment_with_min_silence(labels: &[FrameLabel], min_silence_frames: usize) -> Vec<Segment>` trong `app/src-tauri/src/core/segment.rs`: state machine quét tuần tự qua `labels`, mở segment khi gặp frame speech ngoài segment, đóng segment khi số frame silence liên tiếp ≥ `min_silence_frames` (theo thuật toán hangover ở research.md).
- [X] T006 [US1] Cài đặt `pub fn segment(labels: &[FrameLabel]) -> Vec<Segment>` gọi `segment_with_min_silence` với `DEFAULT_MIN_SILENCE_FRAMES` (FR-001, FR-002, FR-003).
- [X] T007 [US1] Xử lý segment đang mở khi `labels` kết thúc mà chưa gặp đủ silence để đóng: vẫn trả về segment đó trong kết quả (FR-004).
- [X] T008 [US1] Xử lý `labels` rỗng hoặc không có frame speech nào: trả về `Vec` rỗng, không panic (FR-005).
- [X] T009 [US1] [P] Thay vì fixture WAV mới, ghép 2 lần dữ liệu ghi âm thật `tests/fixtures/vad_sample.wav` (PBI 9) với 1 khoảng lặng dài (2s) chèn giữa, trực tiếp trong `app/src-tauri/tests/segment_real_speech.rs` — tái dùng audio thật đã có thay vì ghi fixture mới.
- [X] T010 [US1] [P] Viết test tích hợp `app/src-tauri/tests/segment_real_speech.rs`: chạy dữ liệu ghép (T009) qua `vad::detect()` → `segment::segment()`, xác nhận số segment trả về khớp đúng 2 lượt nói đã ghép (SC-001).
- [X] T011 [US1] Viết test xác nhận khoảng lặng ngắn hơn `min_silence_frames` không bị cắt thành 2 segment (SC-002) — dùng dữ liệu tổng hợp (speech frame liên tiếp có 1-2 frame silence xen giữa) trong `app/src-tauri/src/core/segment.rs` (`#[cfg(test)]`).
- [X] T012 [US1] Viết test xác nhận biên segment (`start_frame_index`/`end_frame_index`) sai lệch không quá 1 frame so với ranh giới speech/silence đã biết trước trong dữ liệu test (SC-003).
- [X] T013 [US1] Viết test xác nhận `labels` rỗng và `labels` toàn silence đều trả về `Vec` rỗng, không panic (tương ứng T008/FR-005).
- [X] T014 [US1] Viết test xác nhận segment đang mở ở cuối `labels` (chưa gặp đủ silence để đóng) vẫn được trả về đầy đủ (tương ứng T007/FR-004).

## Pha cuối: Polish & Cross-cutting

- [X] T015 Lưu log kết quả `cargo test segment::` và `cargo test --test segment_real_speech` làm bằng chứng tại `.specify/specs/10/evidence/segment-test-result.log` và file mô tả test case tại `.specify/specs/10/evidence/segment-testcase.md`.
- [X] T016 Review code `segment.rs`: không export logic ra Tauri command/UI (chỉ `pub use` trong `core::mod`), chạy `cargo clippy --lib` không warning.

## Sơ đồ phụ thuộc

- Pha 1 (Setup) → Pha 2 (Foundational) → Pha 3 (User Story 1) → Pha cuối (Polish).
- Chỉ có 1 user story (P1) trong PBI này — không có story song song độc lập ở cấp story.
- Trong Pha 3: T005 → T006 → T007 → T008 phải làm tuần tự (cùng file `segment.rs`, cùng hàm lõi); T009 (fixture) có thể làm song song với T005–T008; T010 chạy sau khi T006, T009 xong; T011–T014 làm sau khi T006–T008 ổn định cấu trúc hàm.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (PBI nhỏ, chỉ có 1 story).
- Thứ tự giao hàng: Setup → Foundational → US1 (logic cắt segment trước, test theo sau để xác nhận SC-001/002/003) → Polish (bằng chứng test + review).
