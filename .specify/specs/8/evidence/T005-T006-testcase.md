# Test case: T005-T006 — CaptureSessionState start/stop không leak

**Vị trí test**: `app/src-tauri/src/core/capture_session.rs` (`#[cfg(test)] mod tests`)
**Log chạy**: `T005-T006-result.log` (lệnh `cargo test` trong `app/src-tauri/`)

| # | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| 1 | `start()` khi chưa có phiên nào | `Ok(())`, `live == 1` | `Ok(())`, `live == 1` | PASS |
| 2 | `start()` khi đã có phiên đang chạy | `Err(AlreadyRunning)`, `live` không đổi (vẫn 1) | `Err(AlreadyRunning)`, `live == 1` | PASS |
| 3 | `stop()` khi đang có phiên chạy | `Ok(())`, `live == 0` (handle giải phóng) | `Ok(())`, `live == 0` | PASS |
| 4 | `stop()` khi chưa có phiên nào | `Err(NotRunning)` | `Err(NotRunning)` | PASS |
| 5 | Lặp 10 lần `start()` → `stop()` | Sau mỗi `stop()`, `live == 0`; không tích lũy leak | Đúng như kỳ vọng ở cả 10 lần | PASS |

**Tổng kết**: 5/5 kịch bản PASS (ánh xạ FR-001 → FR-005, SC-001, SC-002, SC-003 trong spec.md). Toàn bộ `cargo test` của workspace: 26/26 PASS.
