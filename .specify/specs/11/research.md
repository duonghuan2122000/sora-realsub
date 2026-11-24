# Nghiên cứu: Phát sự kiện đoạn VAD cho UI (PBI 11)

## Quyết định 1: Cần streaming segmenter mới, không tái dùng trực tiếp `segment()` batch

- **Quyết định**: Thêm `core::vad_pipeline::VadPipeline` — giữ buffer `Vec<AudioFrame>` tích lũy, mỗi lần nhận frame mới sẽ chạy lại `vad::detect` + `segment::segment_with_min_silence` trên buffer, so sánh với segment đã phát trước đó để tìm segment MỚI vừa đóng, rồi gọi callback cho từng segment mới đó.
- **Lý do**: `segment()` hiện tại (PBI 9-10) là hàm batch thuần, nhận toàn bộ `&[FrameLabel]` đã có sẵn — không có khái niệm "mới đóng" theo thời gian thực. `start_capture` hiện tại bỏ qua mọi frame (`Box::new(|_frame| {})`), chưa có nơi nào gọi VAD/segment trong luồng chạy thật. Cần một lớp điều phối incremental để biết "segment nào vừa đóng kể từ lần xử lý trước" và chỉ phát đúng 1 lần cho mỗi segment (FR-004).
- **Phương án khác đã xem xét**:
  - Chạy lại toàn bộ `segment()` trên mỗi frame mới và phát lại tất cả segment tìm được → vi phạm FR-004 (phát trùng lặp), phải thêm bước lọc theo segment đã phát rồi — tương đương vẫn cần state "đã phát tới đâu", nên không đơn giản hơn.
  - Viết lại VAD/segment thành thuật toán streaming "thật" (xử lý từng frame 32ms một, không re-run toàn bộ buffer) → tối ưu hơn về CPU nhưng thay đổi lớn tới `vad.rs`/`segment.rs` đã test kỹ ở PBI 9-10; không cần thiết cho khối lượng audio 1 phiên họp, hoãn tới khi có vấn đề hiệu năng thực tế.

## Quyết định 2: Nơi gọi `AppHandle::emit` là `lib.rs`, không phải `core::`

- **Quyết định**: `VadPipeline` chỉ nhận một closure `FnMut(SegmentReady)` khi tạo, không biết gì về Tauri. `lib.rs::start_capture` tạo `VadPipeline`, truyền closure gọi `app_handle.emit("vad-segment", payload)`.
- **Lý do**: CLAUDE.md yêu cầu Core và UI tách biệt, không đưa logic nghiệp vụ vào tầng UI — áp dụng tương tự cho việc không để Core phụ thuộc trực tiếp framework Tauri, giữ `core::` test được bằng `cargo test` thuần không cần Tauri runtime (nhất quán với toàn bộ code Core hiện tại).
- **Phương án khác đã xem xét**: Gọi `app_handle.emit` ngay trong `core::vad_pipeline` → vi phạm tách biệt, không test được pipeline bằng unit test thuần Rust.

## Quyết định 3: Payload sự kiện mang theo `samples` thô, không chỉ index

- **Quyết định**: `SegmentReady` chứa `start_frame_index`, `end_frame_index`, và `samples: Vec<f32>` (âm thanh resample 16kHz mono của đúng đoạn đó) + `sample_rate`.
- **Lý do**: FR-002 yêu cầu UI dùng được ngay không cần hỏi lại Core; UI (hoặc bước debug) không có quyền truy cập buffer nội bộ của Core để tự cắt lại từ index.
- **Phương án khác đã xem xét**: Chỉ gửi `start_frame_index`/`end_frame_index`, UI tự gọi lệnh khác lấy audio → thêm round-trip không cần thiết, không cần thiết cho phạm vi PBI 11 (chỉ debug/hiển thị), hoãn tới khi có yêu cầu cụ thể.
