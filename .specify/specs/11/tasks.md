# Danh sách Task: Phát sự kiện đoạn VAD cho UI

**Mã PBI**: 11
**Nguồn**: plan.md, spec.md, data-model.md, contracts/vad-segment-event.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm `pub mod vad_pipeline;` và export `VadPipeline`, `SegmentReady` tại `app/src-tauri/src/core/mod.rs`

## Pha 2: Foundational

- [X] T002 Định nghĩa struct `SegmentReady { start_frame_index, end_frame_index, samples, sample_rate }` tại `app/src-tauri/src/core/vad_pipeline.rs` (theo `data-model.md`)
- [X] T003 Định nghĩa struct `VadPipeline` với buffer `Vec<AudioFrame>` tích lũy và state "đã phát tới segment nào" tại `app/src-tauri/src/core/vad_pipeline.rs` (theo quyết định 1, `research.md`)

## Pha 3: User Story 1 - Phát sự kiện `vad-segment` đúng thứ tự, không trùng/thiếu (Ưu tiên: P1)

**Mục tiêu**: Core phát sự kiện `vad-segment` ngay khi VAD cắt xong 1 đoạn mới, UI nhận đúng thứ tự, không trùng, không phát thêm sau khi dừng phiên.
**Tiêu chí kiểm thử độc lập**: Chạy `cargo test` cho `vad_pipeline` qua các kịch bản nói-lặng-nói; chạy thủ công theo `quickstart.md` xác nhận UI nhận log đúng thứ tự, không trùng, ngừng sau `stop_capture`.

- [X] T004 [US1] Implement `VadPipeline::push_frame(&mut self, frame: AudioFrame) -> Vec<SegmentReady>`: thêm frame vào buffer, chạy `vad::detect` + `segment::segment_with_min_silence` trên buffer, so khớp với segment đã phát trước đó (theo `end_frame_index`) để chỉ trả về segment MỚI đóng (bỏ qua segment còn mở ở cuối buffer), cắt `samples` đúng theo `start_frame_index`/`end_frame_index` × `VAD_CHUNK_SIZE` tại `app/src-tauri/src/core/vad_pipeline.rs`
- [X] T005 [P] [US1] Viết test cho `VadPipeline::push_frame` tại `app/src-tauri/src/core/vad_pipeline.rs` (mod tests) và `app/src-tauri/tests/vad_pipeline_real_speech.rs` (tích hợp, fixture thật): (a) nhiều frame tạo 1 segment → chỉ 1 `SegmentReady`; (b) nói-lặng dài-nói tạo 2 segment liên tiếp qua nhiều lần `push_frame` → mỗi segment chỉ trả về đúng 1 lần, đúng thứ tự `start_frame_index` tăng dần; (c) segment còn mở (chưa đủ lặng để đóng) không được trả về cho tới khi đủ điều kiện đóng
- [X] T006 [US1] Sửa `start_capture` tại `app/src-tauri/src/lib.rs`: nhận thêm `app_handle: tauri::AppHandle`, tạo `VadPipeline`, trong closure `on_frame` gọi `pipeline.push_frame(frame)` và với mỗi `SegmentReady` trả về gọi `app_handle.emit("vad-segment", payload)` (payload theo `contracts/vad-segment-event.md`, map field sang camelCase)
- [X] T007 [US1] Định nghĩa struct payload serializable (`#[derive(Serialize)]`, camelCase) cho sự kiện `vad-segment` tại `app/src-tauri/src/lib.rs`, khớp đúng `contracts/vad-segment-event.md`
- [X] T008 [US1] Rà soát `CaptureSessionState::stop`/`StreamHandle` (`app/src-tauri/src/core/capture_session.rs`, `mixed_audio_source.rs`): xác nhận `stop()` join thread mixer và không còn gọi `on_frame` sau khi dừng (test `lap_10_lan_start_stop_khong_leak_handle` có sẵn từ PBI 8 đã phủ, không cần thêm) → FR-005 đã được đảm bảo bởi cơ chế có sẵn
- [ ] T009 [US1] Kiểm thử thủ công theo `quickstart.md`: chạy `pnpm tauri dev`, thêm listener debug tạm trong UI, nói-lặng-nói vài lần, xác nhận thứ tự/không trùng, dừng `stop_capture` xác nhận hết sự kiện — chụp ảnh console log làm bằng chứng. **CHƯA THỰC HIỆN**: môi trường thực thi hiện tại không có thiết bị mic thật/cửa sổ Tauri dev tương tác được — xem `evidence/vad-pipeline-testcase.md` Test 3 cho bằng chứng tự động thay thế (tích hợp với file ghi âm thật); cần người dùng tự chạy `quickstart.md` để xác nhận cuối ở UI thật.

## Pha cuối: Polish & Cross-cutting

- [X] T010 Chạy `cargo test` toàn bộ `app/src-tauri`, xác nhận không phá vỡ test cũ của PBI 1-10 (42 unit + 4 tích hợp, tất cả PASS — xem `evidence/T005-T010-cargo-test-result.log`)

## Sơ đồ phụ thuộc

Setup (T001) → Foundational (T002, T003) → User Story 1 (T004 → T005 song song được với T004 sau khi T004 xong vì cùng file nhưng khác phần nên làm T004 trước; T006 cần T004 xong; T007 cần cho T006 nên làm trước hoặc cùng lúc; T008 độc lập có thể làm song song với T004-T007; T009 cần T006-T008 xong) → Polish (T010 cuối cùng).

Chỉ có 1 user story (P1) — không có story khác để song song toàn bộ pha.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, 1 story duy nhất).
- Thứ tự giao hàng: Setup → Foundational → US1 (T004, T007 trước — là nền cho T006; T005, T008 làm song song được; T006 sau cùng trong nhóm code; T009 kiểm thử cuối) → Polish.
