# Test case T005 — unit test logic start/stop (mock + thiết bị thật nếu có)

**File test**: `app/src-tauri/src/core/mic_audio_source.rs` (mod tests)

| Test | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| `thiet_bi_khong_ton_tai_tra_loi_ro_nghia` | `MicAudioSource::new(Some("thiết-bị-chắc-chắn-không-tồn-tại-xyz"))`, gọi `start` | `Err(CoreError::Io(_))` | `Err(CoreError::Io(_))` | PASS |
| `stop_tren_handle_khong_panic_va_dung_nhan_mau` | `MicAudioSource::new(None)` (thiết bị mặc định), `start` rồi `stop()` ngay | Không panic; nếu máy không có thiết bị thì bỏ qua êm | Không panic, chạy xong | PASS |

Log đầy đủ: [T005-result.log](T005-result.log) (gồm toàn bộ 15 test của crate, có 2 test trên).

**Trường hợp "dừng khi chưa từng start không crash"**: không áp dụng được như một test riêng vì API được thiết kế sao cho không thể gọi `stop()` mà chưa có `StreamHandle` (chỉ `start()` mới trả ra handle) — nên trường hợp biên này được đảm bảo bởi kiểu dữ liệu (type-safe), không cần test runtime.
