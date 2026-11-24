# Quickstart: Cắt audio thành segment theo khoảng lặng (PBI 10)

1. Chuẩn bị 1 file WAV mẫu có ít nhất 2 lượt nói tách biệt bởi khoảng lặng rõ (ví dụ: nói ~1s, lặng ~1s, nói ~1s).
2. Đọc file WAV thành `AudioFrame` (dùng lại helper đọc WAV đã có từ PBI 9/5/6/7 trong `app/src-tauri/tests/`).
3. Gọi `core::vad::detect(&frames)` để lấy `Vec<FrameLabel>`.
4. Gọi `core::segment::segment(&labels)` để lấy `Vec<Segment>`.
5. Kiểm tra: số segment trả về bằng số lượt nói thực tế trong file mẫu, `start_frame_index`/`end_frame_index` của mỗi segment khớp với vùng frame "speech" tương ứng (sai lệch không quá 1 frame).
6. Chạy `cargo test` trong `app/src-tauri` — toàn bộ test trong `segment.rs` phải pass.
