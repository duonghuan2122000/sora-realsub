# Test case T005 — unit test LoopbackAudioSource

**Lệnh**: `cargo test --lib loopback_audio_source`

| Bước | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| `stop()` trên handle thật (nếu máy test có thiết bị output mặc định) | Gọi `LoopbackAudioSource::new().start(...)` rồi `handle.stop()` ngay | Không panic, không treo | Test `stop_tren_handle_khong_panic_va_dung_nhan_mau` pass | PASS |

Trường hợp "không có thiết bị output" không mô phỏng được bằng unit test tự động trên máy có sẵn thiết bị output (giống hạn chế đã ghi nhận ở PBI 5 với mic) — được kiểm chứng thủ công một phần qua hành vi không crash khi im lặng ở T008.

Bằng chứng: log chạy toàn bộ test suite tại [T009-result.log](T009-result.log) (bao gồm test này).
