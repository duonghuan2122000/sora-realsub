---
title: "PBI 11: Phát sự kiện vad-segment cho UI"
date: 2026-11-24
tags: [pbi, m2, feature]
sources: [.specify/specs/11/spec.md, .specify/specs/11/plan.md, .specify/specs/11/research.md, .specify/specs/11/data-model.md, .specify/specs/11/contracts/vad-segment-event.md, .specify/specs/11/tasks.md, .specify/specs/11/evidence, app/src-tauri/src/core/vad_pipeline.rs, app/src-tauri/src/lib.rs, app/src-tauri/tests/vad_pipeline_real_speech.rs]
---

# PBI 11: Phát sự kiện vad-segment cho UI

Tính năng thứ ba (cuối) của M2. Nối kết quả cắt segment từ [[PBI-10-cat-segment-khoang-lang]] vào luồng capture thật đang chạy ([[PBI-8-start-stop-capture]]) và phát sự kiện Tauri `vad-segment` ra UI mỗi khi một đoạn nói vừa đóng — hoàn tất M2 (VAD).

## Yêu cầu chính (spec.md)
- Phát sự kiện `vad-segment` đúng 1 lần cho mỗi segment speech vừa đóng, trong lúc phiên capture đang chạy.
- Thứ tự sự kiện khớp đúng thứ tự thời gian các segment phát sinh thực tế; không trùng, không thiếu (FR-003, FR-004).
- Ngừng phát sau khi `stop_capture` (FR-005).
- Payload mang theo đủ dữ liệu (`samples`, index, sample rate) để UI dùng ngay không cần hỏi lại Core.
- Ngoài phạm vi: dùng segment làm input STT (M3), lưu lịch sử segment để replay cho UI mở muộn, UI hiển thị chính thức (M6).

## Quyết định kỹ thuật (plan.md, research.md)
- **Vấn đề cốt lõi**: `segment()` (PBI 10) là hàm batch thuần trên `&[FrameLabel]` có sẵn, không có khái niệm "mới đóng" theo thời gian thực; `start_capture` (PBI 8) trước đó bỏ qua mọi frame (`Box::new(|_frame| {})`) — chưa có nơi nào gọi VAD/segment trong luồng chạy thật.
- **Giải pháp — streaming segmenter mới `core::vad_pipeline::VadPipeline`**: tích lũy `Vec<AudioFrame>`, mỗi lần `push_frame` chạy lại `vad::detect` + `segment::segment` trên toàn bộ buffer, rồi dùng `closed_segment_count()` để xác định segment nào đã **chắc chắn đóng** (loại trừ segment mở dở cuối buffer — còn đang nói, hoặc lặng chưa đủ `DEFAULT_MIN_SILENCE_FRAMES`), chỉ trả về phần mới kể từ lần gọi trước (`closed_count`). Chấp nhận re-run toàn bộ buffer mỗi frame (không tối ưu CPU) vì khối lượng audio 1 phiên họp chưa phải vấn đề thực tế; hoãn streaming "thật" (xử lý từng frame 32ms, không re-run) tới khi có vấn đề hiệu năng.
- **Tách Core/Tauri**: `VadPipeline` không biết gì về Tauri — chỉ trả `Vec<SegmentReady>` từ `push_frame()`. Việc gọi `app_handle.emit("vad-segment", ...)` nằm ở `lib.rs::start_capture`, giữ `core::` test được bằng `cargo test` thuần, nhất quán với toàn bộ code Core hiện tại và nguyên tắc tách Core/UI của CLAUDE.md.
- **Payload đầy đủ**: `SegmentReady { start_frame_index, end_frame_index, samples, sample_rate }` — gửi kèm `samples` thô (16kHz mono) thay vì chỉ index, vì UI không có quyền truy cập buffer nội bộ Core để tự cắt lại.
- **Visibility nội bộ đổi**: `DEFAULT_MIN_SILENCE_FRAMES` (segment.rs), `VAD_SAMPLE_RATE`/`VAD_CHUNK_SIZE`/`resample_all` (vad.rs) đổi từ private sang `pub(crate)` để `vad_pipeline.rs` tái dùng — không nhân bản hằng số/logic.

## Cài đặt (code)
- `app/src-tauri/src/core/vad_pipeline.rs` (mới): `SegmentReady`, `VadPipeline::push_frame()`, hàm thuần `closed_segment_count()` (xác định segment đã đóng dựa trên nhãn cuối buffer) và `slice_samples()` (cắt đúng khoảng `[start, end] × VAD_CHUNK_SIZE` từ buffer đã resample).
- `app/src-tauri/src/core/mod.rs`: export `VadPipeline`, `SegmentReady`.
- `app/src-tauri/src/lib.rs`: `start_capture` nhận thêm `app_handle: tauri::AppHandle`, tạo `VadPipeline` trong closure `on_frame` (thay thế closure rỗng cũ), gọi `app_handle.emit("vad-segment", payload)` cho mỗi `SegmentReady`. Thêm struct `VadSegmentPayload` (`#[derive(Serialize)]`, `camelCase`) khớp `contracts/vad-segment-event.md`.
- `stop_capture` không đổi — FR-005 (ngừng phát sau khi dừng) đã được đảm bảo sẵn bởi cơ chế `StreamHandle::stop()`/join thread mixer có từ [[PBI-8-start-stop-capture]] (PBI 7), không cần code mới.

## Bằng chứng test (`.specify/specs/11/evidence/`)
- Unit test thuần (không gọi model VAD, test trực tiếp logic `closed_segment_count`/`slice_samples` bằng `FrameLabel` dựng tay) — tránh phụ thuộc vào việc tín hiệu tổng hợp (DC/sine) có được Silero VAD phân loại "speech" hay không (lần đầu thử dùng tín hiệu hằng số 0.9 làm "speech" đã FAIL vì model không coi đó là speech thật).
- **Test tích hợp dùng audio thật**: `tests/vad_pipeline_real_speech.rs` — đưa fixture `vad_sample.wav` (tái dùng từ PBI 9-10) vào `VadPipeline::push_frame` theo khối 2048 mẫu/lần (mô phỏng nhận `AudioFrame` liên tục từ capture, khác cách đưa nguyên file 1 lần ở test PBI 9-10), xác nhận: có segment phát ra, đúng thứ tự thời gian, không trùng/chồng lấp; và sau khi đã flush hết, gọi `push_frame` thêm không phát segment mới (mô phỏng FR-005).
- Tổng 42 unit test + 4 test tích hợp (gồm `vad_real_speech`, `segment_real_speech` cũ và 2 test mới của PBI 11) — **tất cả PASS**, không phá vỡ test cũ. Log: `evidence/T005-T010-cargo-test-result.log`, test case: `evidence/vad-pipeline-testcase.md`.
- **T009 (kiểm thử tay UI thật qua `pnpm tauri dev` + console log)**: **chưa thực hiện** — môi trường thực thi hiện tại không có thiết bị mic thật/khả năng tương tác cửa sổ Tauri dev; test tích hợp bằng fixture thật là bằng chứng thay thế cho hành vi streaming. Cần người dùng tự chạy theo `quickstart.md` để xác nhận cuối ở UI thật (giống tình trạng còn lại từ [[PBI-8-start-stop-capture]]).

## Liên quan
[[PBI-10-cat-segment-khoang-lang]], [[PBI-9-vad-silero]], [[PBI-8-start-stop-capture]], [[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]]
