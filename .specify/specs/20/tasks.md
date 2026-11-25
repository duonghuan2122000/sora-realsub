# Danh sách Task: Cửa sổ phụ đề nổi always-on-top

**Mã PBI**: 20
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm capability mới `app/src-tauri/capabilities/subtitle-overlay.json` với `"windows": ["subtitle_overlay"]`, quyền tối thiểu (`core:default`), theo quyết định 3 trong `research.md`.

## Pha 2: Foundational

- [X] T002 Thêm hàm thuần `create_subtitle_overlay_window(app: &tauri::AppHandle) -> tauri::Result<tauri::WebviewWindow>` trong `app/src-tauri/src/lib.rs`, dùng `tauri::WebviewWindowBuilder` với label `subtitle_overlay`, `.decorations(false)`, `.always_on_top(true)`, `.focused(false)`, `.skip_taskbar(true)`, `.resizable(true)` (theo quyết định 2 trong `research.md`); hàm tách riêng, không gọi trực tiếp trong closure của `setup()` để test được không cần hiển thị thật.
- [X] T003 Gọi `create_subtitle_overlay_window()` trong hook `.setup()` của `tauri::Builder` tại `app/src-tauri/src/lib.rs`, log lỗi (không panic) nếu tạo cửa sổ thất bại, không chặn app khởi động cửa sổ `main`.

## Pha 3: User Story 1 - Cửa sổ phụ đề nổi always-on-top, không viền (Ưu tiên: P1)

**Mục tiêu**: Khi app khởi động, một cửa sổ riêng `subtitle_overlay` được tạo, không viền/title bar, luôn nổi trên các cửa sổ khác, không giành focus, và đóng cùng vòng đời app chính.

**Tiêu chí kiểm thử độc lập**: Chạy `bun run tauri dev`, quan sát cửa sổ `subtitle_overlay` xuất hiện đúng đặc điểm theo `quickstart.md`; đồng thời `cargo test -p app subtitle_overlay_window` pass phần cấu hình builder.

- [X] T004 [US1] Viết test tích hợp `app/src-tauri/tests/subtitle_overlay_window.rs`: dùng `tauri::test::mock_app()` để khởi tạo app test, gọi `create_subtitle_overlay_window()`, assert label = `subtitle_overlay` — ứng với FR-001. **Điều chỉnh khi thi công**: `window.is_decorated()`/`is_always_on_top()` trên `MockRuntime` luôn trả hằng số cố định bất kể builder truyền gì (không mô phỏng window manager thật) nên không dùng được để assert FR-002/FR-003; thay vào đó assert trực tiếp hằng số cấu hình `app_lib::SUBTITLE_OVERLAY_CONFIG` — xem `research.md` quyết định 5.
- [X] T005 [US1] Trong cùng test `app/src-tauri/tests/subtitle_overlay_window.rs`, assert cấu hình `focused: false` trong `app_lib::SUBTITLE_OVERLAY_CONFIG` — ứng với FR-004 (lý do không dùng `window.is_focused()` của `MockRuntime`: xem ghi chú ở T004).
- [X] T006 [US1] Chạy `cargo build` và `cargo test` trong `app/src-tauri/` để xác nhận T002–T005 không lỗi biên dịch, test mới pass, không phá vỡ test hiện có (ứng với FR-001, FR-002, FR-003, FR-004).
- [ ] T007 [US1] Kiểm thử thủ công theo đúng các bước trong `.specify/specs/20/quickstart.md` trên máy có GUI thật (Windows hoặc Linux desktop), chụp ảnh từng bước, lưu vào `.specify/specs/20/evidence/` — xác nhận SC-001 (xuất hiện ≤2s, không viền), SC-002 (luôn nổi khi chuyển đổi ứng dụng), SC-003 (không leak process khi đóng app) và FR-005.

## Pha cuối: Polish & Cross-cutting

- [X] T008 Rà soát lại `app/src-tauri/src/lib.rs`: đảm bảo logic tạo `subtitle_overlay` không rẽ nhánh theo OS (`cfg(windows)`/`cfg(target_os = "linux")`) trừ khi phát sinh vấn đề thực tế khi kiểm thử T007, khớp giả định trong `spec.md`.
- [X] T009 Cập nhật `wiki/pages/` (hoặc ghi chú trong `research.md`) nếu kiểm thử thủ công T007 phát hiện giới hạn của window manager (ví dụ `skip_taskbar` không hoạt động trên 1 số WM Linux) — ghi lại để tham chiếu ở các PBI M5 tiếp theo. **Đã ghi chú**: môi trường implement (container, `Xvfb` không có window manager) không có khả năng xác nhận always-on-top/focus thật — xem `evidence/T007-testcase.md` và `research.md` quyết định 5.

## Sơ đồ phụ thuộc

```text
T001 (capability) ──┐
                     ├─> T002 (hàm tạo window) ─> T003 (gọi trong setup())
                     │                                   │
                     └───────────────────────────────────┘
                                                           │
                                                           v
                                      T004, T005 (test tự động, có thể viết song song với T003 nếu mock sẵn ký hiệu hàm)
                                                           │
                                                           v
                                                         T006 (build + test)
                                                           │
                                                           v
                                                         T007 (kiểm thử thủ công có ảnh chụp)
                                                           │
                                                           v
                                              T008, T009 (polish, ghi chú)
```

- T001 không phụ thuộc T002/T003 nhưng cần có trước khi `setup()` tạo window thật chạy không lỗi quyền (nên làm trước hoặc song song, miễn xong trước T006).
- T004, T005 cùng sửa 1 file test → chạy tuần tự, không gắn `[P]`.
- T007 chỉ thực hiện được khi có máy GUI thật — nếu không có trong lúc implement, ghi rõ trạng thái "chưa xác nhận trực quan" (tương tự PBI 18) và không tick dòng roadmap M5 tương ứng cho đến khi có xác nhận.

## Chiến lược triển khai

- **MVP đề xuất**: Toàn bộ User Story 1 (đây là PBI nhỏ, chỉ có 1 story) — T001–T007 là phần phải hoàn thành để coi tính năng "xong".
- **Thứ tự giao hàng tăng dần**:
  1. T001–T003: cửa sổ overlay tồn tại, cấu hình đúng theo code.
  2. T004–T006: xác nhận bằng test tự động trong container (không cần GUI thật).
  3. T007: xác nhận trực quan trên máy GUI thật (có thể trễ hơn nếu môi trường implement không có GUI, giống PBI 18/17 — ghi chú rõ, không block các PBI M5 sau nếu không ảnh hưởng).
  4. T008–T009: polish, cập nhật ghi chú kỹ thuật nếu phát sinh giới hạn window manager.
