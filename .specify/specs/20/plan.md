# Kế hoạch triển khai: Cửa sổ phụ đề nổi always-on-top

**Mã PBI**: 20
**Liên kết spec**: .specify/specs/20/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (core, Tauri 2) |
| Framework / Thư viện chính | `tauri` 2 (`WebviewWindowBuilder`), không cần crate mới |
| Lưu trữ dữ liệu | Không áp dụng (không có dữ liệu bền/entity ở PBI này) |
| Kiểm thử | `tauri::test` mock app cho phần cấu hình builder (tự động); kiểm thử thủ công có ảnh chụp cho hành vi always-on-top/visual (xem `quickstart.md`) |
| Nền tảng triển khai | Windows + Linux (dùng chung API Tauri/winit, không gate theo OS ở mức tính năng này) |
| Ràng buộc hiệu năng | Cửa sổ overlay xuất hiện trong ≤ 2 giây sau khi app khởi động (SC-001) |
| Ràng buộc khác | Không giành focus bàn phím khi tạo/cập nhật (FR-004); đóng cùng vòng đời app chính, không leak process (FR-005) |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Dự án chưa có `.specify/memory/constitution.md`. Áp dụng các ràng buộc đã nêu trong `CLAUDE.md`: Core (Rust) và UI (Vue) phải tách biệt, không đưa logic nghiệp vụ vào tầng UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Toàn bộ logic tạo/cấu hình cửa sổ overlay nằm trong Rust (`lib.rs`/module core), UI (Vue) không cần biết hoặc điều khiển việc tạo cửa sổ này ở PBI này |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`. Tóm tắt các quyết định chính:

- **Quyết định**: Tạo window thứ hai `subtitle_overlay` bằng `WebviewWindowBuilder` trong `setup()`. **Lý do**: linh hoạt điều khiển bằng code cho các PBI sau trong M5. **Phương án khác**: khai báo tĩnh trong `tauri.conf.json` — loại vì khó mở rộng logic điều kiện.
- **Quyết định**: Dùng `.decorations(false)`, `.always_on_top(true)`, `.focused(false)`, `.skip_taskbar(true)` của Tauri 2. **Lý do**: API đa nền tảng có sẵn, không cần code riêng theo OS. **Phương án khác**: gọi WinAPI/X11 thủ công — loại vì trùng lặp, rủi ro cao hơn.
- **Quyết định**: Thêm capability riêng `subtitle-overlay.json` cho window mới, quyền tối thiểu. **Lý do**: nguyên tắc tối thiểu quyền. **Phương án khác**: gộp vào capability `default` — loại vì thừa quyền.
- **Quyết định**: Không cấu hình vòng đời riêng — overlay thoát cùng process khi `main` đóng. **Lý do**: khớp FR-005, không cần logic thêm. **Phương án khác**: ẩn thay vì thoát (chạy nền) — ngoài phạm vi, để lại cho M6 nếu cần.
- **Quyết định**: Test tự động cho phần cấu hình builder (`tauri::test` mock), test thủ công có ảnh chụp cho phần hành vi thị giác/always-on-top thật. **Lý do**: môi trường CI/container không có display server đầy đủ để xác nhận always-on-top thật.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: Không áp dụng — PBI này không có entity/dữ liệu bền mới.
- **Hợp đồng giao diện**: Không áp dụng — không thêm Tauri IPC command mới ở PBI này (cửa sổ được tạo tự động khi app khởi động, không cần UI gọi lệnh).
- **Kịch bản khởi động nhanh**: xem `quickstart.md`.

## Kiểm tra theo hiến pháp dự án (sau thiết kế)

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Không có thay đổi gì ở phía UI (Vue); toàn bộ trong `app/src-tauri/src/lib.rs` (hoặc module `core` nếu phù hợp cấu trúc hiện có) |

## Cấu trúc dự án dự kiến

```text
app/src-tauri/
  src/
    lib.rs                          # thêm hàm create_subtitle_overlay_window() + gọi trong setup()
  capabilities/
    subtitle-overlay.json           # mới — capability cho window "subtitle_overlay"
  tests/
    subtitle_overlay_window.rs      # mới — test cấu hình builder qua tauri::test mock app

.specify/specs/20/
  spec.md
  plan.md
  research.md
  quickstart.md
  checklists/requirements.md
  evidence/                         # ảnh chụp + ghi chú kiểm thử thủ công (tạo ở bước implement)
```

## Rủi ro & ngoại lệ có lý do

- **Rủi ro**: Container dùng để implement (môi trường cloud, có thể không có display server đầy đủ hoặc chỉ có headless/Xvfb) không thể xác nhận trực quan hành vi always-on-top thật như PBI 18 đã gặp với loopback Linux. → Xử lý: phần cấu hình (builder, capability) có test tự động pass trong container; phần hành vi thị giác (SC-001, SC-002, SC-003) cần xác nhận thủ công trên máy có GUI thật trước khi tick dòng roadmap tương ứng, ghi rõ trạng thái "chưa xác nhận trực quan" nếu không có máy GUI khi implement — tương tự cách ghi chú đã áp dụng cho PBI 18/17.
- **Rủi ro**: `skip_taskbar(true)` có thể không được hỗ trợ đồng nhất trên một số window manager Linux (ví dụ một số tiling WM không có taskbar/dock). → Không gây lỗi nghiêm trọng (spec cho phép always-on-top "có thể không đảm bảo tuyệt đối do giới hạn window manager"), chấp nhận là hành vi best-effort, không block việc tick tính năng nếu phần còn lại (decorations, always-on-top trên WM phổ biến) hoạt động đúng.
