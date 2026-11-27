# Danh sách Task: Ẩn-hiện cửa sổ phụ đề

**Mã PBI**: 25
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Định dạng task

`- [ ] [MãTask] [P?] [Story?] Mô tả kèm đường dẫn file`

## Pha 1: Setup

- [X] T001 Thêm dependency `tauri-plugin-global-shortcut` (v2) vào `app/src-tauri/Cargo.toml`
- [X] T002 Thêm quyền/permission global-shortcut cần thiết vào `app/src-tauri/capabilities/default.json` (nếu plugin yêu cầu capability riêng cho shortcut đã đăng ký)

## Pha 2: Foundational

- [X] T003 Viết hàm thuần `toggle_subtitle_visibility(is_visible: bool) -> bool` tại `app/src-tauri/src/lib.rs` (đảo trạng thái, không phụ thuộc `AppHandle`/Tauri runtime, để `cargo test` được trực tiếp — theo research.md quyết định 4)

## Pha 3: User Story 1 - Ẩn-hiện cửa sổ phụ đề bằng phím tắt (Ưu tiên: P1) 🎯 MVP

**Mục tiêu**: Người dùng nhấn một phím tắt toàn cục để ẩn/hiện cửa sổ phụ đề mà không dừng phiên dịch, giữ nguyên vị trí/cỡ chữ khi hiện lại.

**Tiêu chí kiểm thử độc lập**: Theo `quickstart.md` — nhấn `Ctrl+Alt+H` khi cửa sổ phụ đề không focus → cửa sổ ẩn trong < 0.5s, pipeline vẫn chạy; nhấn lại → cửa sổ hiện lại đúng vị trí/cỡ chữ cũ.

- [X] T004 [US1] Đăng ký plugin `tauri_plugin_global_shortcut` trong `Builder` tại `app/src-tauri/src/lib.rs` (hàm `run()`)
- [X] T005 [US1] Thêm state `AtomicBool` (mặc định `true` = đang hiện) qua `app.manage(...)` tại `app/src-tauri/src/lib.rs`, lưu trạng thái hiện/ẩn hiện tại của cửa sổ `subtitle`
- [X] T006 [US1] Đăng ký shortcut `Ctrl+Alt+H` trong `setup()` tại `app/src-tauri/src/lib.rs`; handler: đọc state hiện tại, gọi `toggle_subtitle_visibility` (T003) để tính trạng thái mới, cập nhật state, rồi gọi `window("subtitle").show()`/`.hide()` tương ứng
- [X] T007 [US1] Test `cargo test` cho `toggle_subtitle_visibility` tại `app/src-tauri/src/lib.rs` (mod tests): `true -> false`, `false -> true`, đảo nhiều lần liên tiếp cho kết quả đúng chu kỳ 2 (trường hợp biên "nhấn liên tục nhiều lần")
- [X] T008 [US1] Test tay trên Windows thật theo `quickstart.md` bước 3–6, chụp ảnh/ghi lại kết quả làm bằng chứng (thực hiện ở bước `/sora-implement`)

## Pha cuối: Polish & Cross-cutting

- [X] T009 Cập nhật comment/docstring ngắn giải thích lý do dùng `hide()`/`show()` thay vì đóng cửa sổ tại nơi đăng ký shortcut trong `app/src-tauri/src/lib.rs` (tham chiếu research.md quyết định 3), tránh người đọc sau nhầm là bug

## Sơ đồ phụ thuộc

```text
Setup (T001-T002) → Foundational (T003) → US1 (T004-T008) → Polish (T009)
```

## Ví dụ chạy song song

```text
# T001 và T002 có thể làm song song (file khác nhau):
T001 Thêm dependency vào Cargo.toml
T002 [P] Thêm permission vào capabilities/default.json
```

## Chiến lược triển khai

- **MVP**: chỉ User Story 1 (toàn bộ tính năng PBI 25 là 1 story duy nhất).
- **Giao hàng**: Setup → Foundational → US1 → Polish, không chia nhỏ thêm vì phạm vi đã nhỏ.
