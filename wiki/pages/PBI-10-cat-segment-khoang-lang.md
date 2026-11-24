---
title: "PBI 10: Cắt audio thành segment theo khoảng lặng"
date: 2026-11-24
tags: [pbi, m2, feature]
sources: [.specify/specs/10/spec.md, .specify/specs/10/plan.md, .specify/specs/10/research.md, .specify/specs/10/tasks.md, .specify/specs/10/evidence, app/src-tauri/src/core/segment.rs, app/src-tauri/tests/segment_real_speech.rs, app/src-tauri/src/core/mod.rs]
---

# PBI 10: Cắt audio thành segment theo khoảng lặng

Tính năng thứ hai của M2 (VAD). Dùng nhãn speech/silence theo frame đã có từ [[PBI-9-vad-silero]] để cắt luồng audio liên tục thành các segment rời rạc, mỗi segment tương ứng một lượt nói — nền tảng để sau này mỗi segment được gửi riêng đi STT (M3).

## Yêu cầu chính (spec.md)
- Mở segment khi gặp frame speech, đóng segment khi gặp khoảng lặng đủ dài; khoảng lặng ngắn (ngắt hơi giữa câu) không được cắt gãy segment.
- Segment còn mở khi luồng audio kết thúc vẫn phải được trả về, không mất dữ liệu.
- Audio toàn silence hoặc rỗng → danh sách segment rỗng, không lỗi.
- Trả về segment đúng thứ tự thời gian, kèm thời điểm bắt đầu/kết thúc.
- Ngoài phạm vi: event Tauri `vad-segment` ra UI (PBI kế tiếp M2), STT/dịch (M3/M4).

## Quyết định kỹ thuật (plan.md, research.md)
- **Thuật toán**: state machine silence-hangover — quét tuần tự `Vec<FrameLabel>`, mở segment ở frame speech đầu tiên ngoài segment, đóng segment tại frame speech cuối cùng khi số frame silence liên tiếp đạt ngưỡng `min_silence_frames`. Chọn cách này vì không cần thêm dependency, xử lý thuần trên dữ liệu `FrameLabel` đã có từ PBI 9.
- **Ngưỡng mặc định**: `DEFAULT_MIN_SILENCE_FRAMES = 9` (~300ms @ frame VAD 512 sample/16kHz) — tránh cắt gãy câu do khoảng lặng ngắn/nhiễu VAD.
- **Dữ liệu trả về**: `Segment { start_frame_index, end_frame_index }` — chỉ lưu range chỉ số frame, không copy lại PCM (việc ghép PCM theo segment để gửi STT là chi tiết triển khai của bước pipeline sau).
- **API**: theo đúng convention đã có ở `vad.rs` — hàm mặc định `segment()` gọi hàm có tham số `segment_with_min_silence()` để test/tùy biến.

## Cài đặt (code)
- `app/src-tauri/src/core/segment.rs` (mới): `Segment { start_frame_index, end_frame_index }`, `segment()` (ngưỡng mặc định 9 frame) và `segment_with_min_silence()`.
- `app/src-tauri/src/core/mod.rs`: khai báo + export `segment::{Segment, segment, segment_with_min_silence}`.
- Chưa có Tauri command/event nào cho segment — module Core nội bộ, chưa nối vào pipeline streaming thật (đó là việc của PBI kế tiếp M2, phát event `vad-segment`).

## Bằng chứng test (`.specify/specs/10/evidence/`)
- `cargo test segment::`: 6/6 unit test pass — buffer rỗng, toàn silence, khoảng lặng ngắn không cắt gãy (SC-002), khoảng lặng dài tách đúng 2 segment (SC-001), segment còn mở ở cuối stream vẫn trả về (FR-004), frame speech đơn lẻ vẫn hợp lệ.
- **Test tích hợp dùng audio thật**: `tests/segment_real_speech.rs` ghép 2 lần dữ liệu ghi âm thật `tests/fixtures/vad_sample.wav` (đã có từ PBI 9) với 1 khoảng lặng dài (2s) chèn giữa, chạy qua `vad::detect()` → `segment::segment()`, xác nhận đúng 2 segment theo đúng thứ tự thời gian (SC-001). Không tạo fixture WAV mới — tái dùng audio thật đã có.
- Tổng 7/7 test pass. Log: `evidence/segment-test-result.log`, test case: `evidence/segment-testcase.md`. `cargo clippy --lib` không warning. Tasks T001–T016 đã tick xong trong `tasks.md`.

## Liên quan
[[PBI-9-vad-silero]], [[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Quyet-dinh-hop-dong-core]]
