# Test case: VAD gán nhãn speech/silence theo frame (PBI 9)

**Liên quan task**: T005–T012
**File test**: `app/src-tauri/src/core/vad.rs` (`#[cfg(test)] mod tests`)
**Log kết quả**: [T009-T012-result.log](T009-T012-result.log)

## Ghi chú thu hẹp phạm vi so với kế hoạch ban đầu

Kế hoạch ban đầu (T008) dự định dùng 1 file WAV mẫu có giọng nói người thật xen kẽ im lặng để xác nhận SC-001 (≥90% frame nói được nhãn "speech"). Không có sẵn file ghi âm giọng nói thật nào trong repo hoặc trong môi trường thi công, và việc tự tạo file như vậy cần thiết bị ghi âm — ngoài khả năng của bước này. Vì vậy test case thực tế dùng tín hiệu **im lặng tuyệt đối** (toàn 0.0) để xác nhận nhánh "silence" (SC-002) và các đường biên (FR-004), còn nhánh "speech" (SC-001) **chưa được xác nhận bằng dữ liệu thật** — cần làm khi có file mẫu giọng nói (có thể lấy từ fixture thực tế khi làm PBI STT ở M3, hoặc người dùng cung cấp file ghi âm mẫu).

## Test case 1 — `empty_buffer_returns_empty_labels`

- **Input**: `&[]` (không có `AudioFrame` nào)
- **Expected**: `detect()` trả về `Ok(Vec::new())`, không lỗi/panic (FR-004)
- **Actual**: pass

## Test case 2 — `buffer_shorter_than_one_frame_returns_empty_labels`

- **Input**: 1 `AudioFrame` 16kHz, độ dài 256 sample (< 512 sample/frame VAD)
- **Expected**: `detect()` trả về `Ok(Vec::new())`, không lỗi/panic (FR-004)
- **Actual**: pass

## Test case 3 — `pure_silence_is_labeled_silence`

- **Input**: 1 `AudioFrame` 16kHz, 5120 sample toàn 0.0 (= 10 frame VAD)
- **Expected**: trả về đúng 10 `FrameLabel`; ≥90% nhãn `is_speech = false` (SC-002)
- **Actual**: pass — 10/10 frame nhãn silence

## Test case 4 — `labels_keep_original_frame_order`

- **Input**: 1 `AudioFrame` 16kHz, 2560 sample toàn 0.0 (= 5 frame VAD)
- **Expected**: `frame_index` của các `FrameLabel` trả về theo đúng thứ tự 0,1,2,3,4 (FR-003)
- **Actual**: pass

## Kết quả tổng thể

4/4 test pass (`cargo test vad::`). Xem log đầy đủ tại [T009-T012-result.log](T009-T012-result.log).

**Chưa xác nhận bằng dữ liệu thật**: SC-001 (nhánh "speech") và SC-003 (thứ tự nhãn trên dữ liệu nói+lặng xen kẽ thật) — cần file audio mẫu có giọng nói người thật để kiểm thử đầy đủ, hiện chưa có trong repo.
