# Test case T008 — device id không hợp lệ (SC-003)

**Lệnh**: `cargo run --bin capture_mic_to_wav -- 2 thiet-bi-khong-ton-tai-xyz-123`

| Input | Expected | Actual | Kết quả |
|---|---|---|---|
| `device_id = "thiet-bi-khong-ton-tai-xyz-123"` (không tồn tại) | In lỗi rõ nghĩa ra stderr, exit code khác 0, không treo/crash | In: `Lỗi capture mic (device_id=Some("thiet-bi-khong-ton-tai-xyz-123")): lỗi I/O: không tìm thấy thiết bị mic: thiet-bi-khong-ton-tai-xyz-123`; `exit=1` | PASS |

Bằng chứng log: [T008-result.log](T008-result.log)

Đáp ứng SC-003 (100% lần thử thiết bị không hợp lệ đều báo lỗi rõ ràng, không treo/crash).
