# Test case: PBI 16 — T005, T006

## T005 — sidecar không tồn tại trả lỗi rõ ràng

- **Input**: `NllbTranslator::new("duong-dan-khong-ton-tai.exe", "model-dir")`, gọi `translate("Hello", &[])`
- **Expected**: `Err(CoreError::Provider(msg))` với `msg` không rỗng, không panic
- **Actual**: pass — `cargo test` báo `sidecar_khong_ton_tai_tra_loi ... ok`
- **Kết quả**: PASS

## T006 — input rỗng/khoảng trắng không spawn tiến trình, trả rỗng

- **Input**: cùng executable không tồn tại, gọi `translate("", &[])` và `translate("   ", &[])`
- **Expected**: `Ok("".to_string())` cho cả hai, không lỗi (chứng minh không có spawn xảy ra vì executable không tồn tại nhưng không trả lỗi)
- **Actual**: pass — `cargo test` báo `input_rong_tra_ve_rong_khong_spawn_tien_trinh ... ok`
- **Kết quả**: PASS

Xem log đầy đủ tại [T008-result.log](T008-result.log).
