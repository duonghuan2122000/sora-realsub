//! PBI 20 — T004/T005: test cấu hình cửa sổ phụ đề overlay.
//!
//! `tauri::test::mock_app()` dùng `MockRuntime`: các getter như `is_decorated()`/
//! `is_always_on_top()`/`is_focused()` trên `MockRuntime` luôn trả giá trị cố định, không mô
//! phỏng window manager thật, nên không dùng được để xác nhận cấu hình đã truyền vào builder
//! (xem comment tại `SubtitleOverlayConfig` trong `src/lib.rs`). Vì vậy test này xác nhận
//! trực tiếp giá trị cấu hình hằng số `SUBTITLE_OVERLAY_CONFIG` (nguồn cấu hình builder dùng),
//! và xác nhận cửa sổ tạo ra có đúng label mong đợi.

#[test]
fn cau_hinh_overlay_dung_khong_vien_luon_noi_khong_giành_focus() {
    let cfg = app_lib::SUBTITLE_OVERLAY_CONFIG;

    assert_eq!(cfg.decorations, false, "FR-002: không viền/title bar");
    assert_eq!(cfg.always_on_top, true, "FR-003: luôn nổi ở trên");
    assert_eq!(cfg.focused, false, "FR-004: không giành focus khi tạo");
}

#[test]
fn tao_cua_so_overlay_thanh_cong_voi_dung_label() {
    let app = tauri::test::mock_app();

    let window = app_lib::create_subtitle_overlay_window(app.handle())
        .expect("tạo cửa sổ overlay phải thành công trong môi trường test");

    assert_eq!(window.label(), "subtitle_overlay");
}
