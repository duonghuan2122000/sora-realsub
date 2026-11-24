# Test case: VadPipeline streaming segment (T001-T008, T010)

**PBI**: 11 — Phát sự kiện đoạn VAD cho UI
**File test**: `app/src-tauri/src/core/vad_pipeline.rs` (mod tests), `app/src-tauri/tests/vad_pipeline_real_speech.rs`
**Lệnh chạy**: `cargo test` tại `app/src-tauri`
**Log đầy đủ**: [T005-T010-cargo-test-result.log](T005-T010-cargo-test-result.log)

## Test 1 — `closed_segment_count` xác định đúng segment nào đã đóng thật (unit, không phụ thuộc model)

| Kịch bản | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| Không có segment | `total_segments=0` | `0` | `0` | PASS |
| Nhãn cuối là speech | nói(3), chưa lặng | segment cuối vẫn mở → `0` | `0` | PASS |
| Lặng chưa đủ ngưỡng | nói(2) + lặng(`DEFAULT_MIN_SILENCE_FRAMES - 1`) | segment cuối vẫn mở → `0` | `0` | PASS |
| Lặng đủ ngưỡng | nói(2) + lặng(`DEFAULT_MIN_SILENCE_FRAMES`) | segment đã đóng → `1` | `1` | PASS |
| 2 lượt nói tách biệt, cả 2 đều đủ lặng sau | nói-lặng-nói-lặng (đủ ngưỡng cả 2 lần) | cả 2 đã đóng → `2` | `2` | PASS |

## Test 2 — `slice_samples` cắt đúng khoảng theo `VAD_CHUNK_SIZE`

Input: buffer 4×`VAD_CHUNK_SIZE` mẫu tăng dần, cắt `[1, 2]` → Expected: độ dài `2×VAD_CHUNK_SIZE`, mẫu đầu = `VAD_CHUNK_SIZE as f32`. Actual: khớp. **PASS**.

## Test 3 — `VadPipeline::push_frame` trên ghi âm thật, đưa vào theo khối nhỏ (tích hợp, dùng lại fixture `vad_sample.wav` từ PBI 9-10)

- `streaming_tung_khoi_nho_phat_segment_dung_thu_tu_khong_trung`: đưa fixture vào theo khối 2048 mẫu/lần (mô phỏng nhận `AudioFrame` liên tục từ capture, khác cách đưa nguyên file 1 lần ở test PBI 9-10) + thêm lặng dài cuối. Expected: ≥1 `SegmentReady`, mỗi segment có `samples` không rỗng, `sample_rate = 16000`, và segment sau bắt đầu sau khi segment trước kết thúc (không trùng/chồng lấp — FR-003, FR-004). Actual: khớp. **PASS**.
- `sau_khi_het_audio_push_frame_khong_phat_them_segment_trung`: sau khi đã flush hết bằng lặng dài, gọi thêm `push_frame` với lặng lần 2 → Expected: không có segment mới nào được phát ra (mô phỏng FR-005 — không phát thêm sau khi dừng phiên). Actual: rỗng. **PASS**.

## Test 4 — Không phá vỡ test cũ của PBI 1-10

`cargo test` toàn bộ workspace: **42 test unit + 4 test tích hợp (`vad_real_speech`, `segment_real_speech`, `vad_pipeline_real_speech` ×2) — tất cả PASS**, không có test cũ nào bị phá vỡ (xem log đầy đủ).
