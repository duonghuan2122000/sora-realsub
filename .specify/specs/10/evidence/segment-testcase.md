# Test case: Cắt audio thành segment theo khoảng lặng (PBI 10)

## 1. Unit test (`app/src-tauri/src/core/segment.rs`, `#[cfg(test)]`)

| Test | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| `empty_labels_returns_empty_segments` | `labels = []` | `Vec` rỗng | `Vec` rỗng | PASS |
| `all_silence_returns_empty_segments` | 20 frame toàn silence | `Vec` rỗng (FR-005) | `Vec` rỗng | PASS |
| `short_silence_gap_does_not_split_segment` | nói(3) - lặng(2, < ngưỡng 9) - nói(3) | 1 segment duy nhất `[0,7]` (SC-002) | 1 segment `[0,7]` | PASS |
| `long_silence_gap_splits_into_two_segments` | nói(3) - lặng(9, = ngưỡng) - nói(3) | 2 segment `[0,2]` và `[12,14]` (SC-001) | 2 segment `[0,2]`, `[12,14]` | PASS |
| `open_segment_at_end_of_stream_is_still_returned` | nói(5), hết danh sách, chưa đủ silence đóng | 1 segment `[0,4]` (FR-004) | 1 segment `[0,4]` | PASS |
| `single_speech_frame_is_a_valid_segment` | 1 frame speech đơn lẻ giữa 2 vùng lặng | 1 segment `[5,5]` | 1 segment `[5,5]` | PASS |

## 2. Test tích hợp (`app/src-tauri/tests/segment_real_speech.rs`)

| Test | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| `hai_luot_noi_tach_biet_sinh_dung_hai_segment` | Ghép 2 lần dữ liệu ghi âm thật `tests/fixtures/vad_sample.wav` (PBI 9), cách nhau bởi 2s im lặng tuyệt đối | `vad::detect()` → `segment::segment()` trả về đúng 2 segment, segment 1 kết thúc trước khi segment 2 bắt đầu (SC-001) | 2 segment, thứ tự đúng | PASS |

## Kết quả tổng

7/7 test pass. Log đầy đủ: [segment-test-result.log](segment-test-result.log) (`cargo test segment`).
`cargo clippy --lib`: không warning.
