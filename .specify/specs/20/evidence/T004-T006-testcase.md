# Test case: T004–T006 — Cấu hình cửa sổ phụ đề overlay (tự động, trong container)

**File test**: `app/src-tauri/tests/subtitle_overlay_window.rs`
**Log đầy đủ**: `T004-T006-cargo-test-result.log` (toàn bộ `cargo test` trong `app/src-tauri/`)

## Test 1: `cau_hinh_overlay_dung_khong_vien_luon_noi_khong_giành_focus`

- **Input**: hằng số `app_lib::SUBTITLE_OVERLAY_CONFIG` (nguồn cấu hình duy nhất mà `create_subtitle_overlay_window()` dùng để gọi `WebviewWindowBuilder`).
- **Expected**: `decorations == false` (FR-002), `always_on_top == true` (FR-003), `focused == false` (FR-004).
- **Actual**: đúng như kỳ vọng.
- **Kết quả**: PASS.

## Test 2: `tao_cua_so_overlay_thanh_cong_voi_dung_label`

- **Input**: gọi `app_lib::create_subtitle_overlay_window(app.handle())` với `app = tauri::test::mock_app()`.
- **Expected**: trả `Ok(window)`, `window.label() == "subtitle_overlay"`.
- **Actual**: đúng như kỳ vọng.
- **Kết quả**: PASS.

## Ghi chú giới hạn (xem research.md quyết định 5)

`MockRuntime` của Tauri (dùng trong `tauri::test::mock_app()`) có các getter `is_decorated()`/
`is_always_on_top()`/`is_focused()` luôn trả về hằng số cố định (`false`), không phản ánh giá trị
đã truyền vào builder, vì mock không mô phỏng window manager thật. Vì vậy 2 test trên không thể
xác nhận "window manager thật áp dụng đúng always-on-top/no-decoration/no-focus" — chỉ xác nhận
mã nguồn cấu hình đúng giá trị mong muốn và `create_subtitle_overlay_window()` không lỗi. Việc xác
nhận hành vi thật (SC-001, SC-002, SC-003) chuyển sang T007 — kiểm thử thủ công trên máy GUI thật.

## Toàn bộ test suite (65 + 2 + 1 + 2 + 2 + 1 = ...)

`cargo test` chạy toàn bộ workspace: 0 fail, các test `ignored` là các test cần model/sidecar
thật (STT/dịch) đã ignored từ các PBI trước, không liên quan PBI 20 — xem log đầy đủ.
