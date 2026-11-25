# Kế hoạch triển khai: Cửa sổ phụ đề nổi

**Mã PBI**: 20
**Liên kết spec**: .specify/specs/20/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core), cấu hình qua `tauri.conf.json`
- Framework / Thư viện chính: Tauri 2 (`tauri::WebviewWindowBuilder`), không thêm crate mới
- Lưu trữ dữ liệu: không áp dụng (chưa lưu vị trí/kích thước ở PBI này — thuộc mục sau của M5)
- Kiểm thử: `cargo test` (test đơn vị cấu hình builder — không thể automation hiển thị thật trên UI thread trong CI headless, nên có thêm bước kiểm thử thủ công chụp ảnh làm bằng chứng)
- Nền tảng triển khai: Windows + Linux (theo roadmap hiện tại), macOS ngoài phạm vi
- Ràng buộc hiệu năng: không áp dụng
- Ràng buộc khác: không đưa logic hiển thị nội dung phụ đề vào PBI này (ngoài phạm vi theo spec)

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong repo. Đối chiếu theo CLAUDE.md (Core/UI tách biệt, giao tiếp qua Tauri IPC):

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) tách biệt UI (Vue) | ✅ | Cửa sổ phụ đề tạo/cấu hình ở tầng Rust (`lib.rs`/`run()`), không cần logic nghiệp vụ ở Vue cho PBI này |
| Giao tiếp qua Tauri IPC commands/events | ✅ | Chưa cần command/event mới ở PBI này; cửa sổ được tạo khi app khởi động trong `setup()` |

## Giai đoạn 0 — Kết quả nghiên cứu

Không có điểm `NEEDS CLARIFICATION` — quyết định kỹ thuật:

- **Quyết định**: Tạo cửa sổ phụ đề bằng `tauri::WebviewWindowBuilder::new(app, "subtitle", WebviewUrl::App("subtitle.html".into()))` gọi trong `.setup()` của `tauri::Builder`, thay vì khai báo tĩnh trong mảng `windows` của `tauri.conf.json`.
  - **Lý do**: Cần set `.decorations(false)`, `.always_on_top(true)`, `.transparent(true)`, `.skip_taskbar(true)` và vị trí/kích thước mặc định bằng code — linh hoạt hơn khai báo JSON tĩnh, và tránh cửa sổ phụ đề xuất hiện trước khi app sẵn sàng.
  - **Phương án khác đã xem xét**: Khai báo thêm 1 entry trong mảng `app.windows` của `tauri.conf.json` — đơn giản hơn nhưng không linh hoạt để mở rộng sau này (ẩn/hiện theo lệnh, resize lưu config ở M6).
- **Quyết định**: Trang HTML đích của cửa sổ phụ đề là `subtitle.html` riêng (entry point Vite thứ hai), nội dung tối thiểu rỗng/placeholder ở PBI này.
  - **Lý do**: Tách biệt rõ ràng với `index.html` (cửa sổ điều khiển chính), chuẩn bị cho UI phụ đề thật ở mục tiếp theo của M5, đúng khuyến nghị multi-window chính thức của Tauri (mỗi window một entry HTML).
  - **Phương án khác đã xem xét**: Dùng chung `index.html` với route riêng (SPA) — phức tạp hơn không cần thiết vì 2 cửa sổ có vai trò khác hẳn nhau (control UI vs overlay).
- **Quyết định**: always-on-top dùng API chung `always_on_top(true)` của Tauri (wrap WinAPI `HWND_TOPMOST` trên Windows, `gtk_window_set_keep_above` qua wry/gtk trên Linux) — không tự gọi API hệ điều hành thủ công.
  - **Lý do**: Tauri 2 đã hỗ trợ cross-platform cho Windows + Linux, đúng 2 OS mục tiêu hiện tại của dự án, giảm code đặc thù OS.
  - **Phương án khác đã xem xét**: Gọi trực tiếp WinAPI/GTK theo `cfg(windows)`/`cfg(target_os = "linux")` như các PBI audio trước — không cần thiết vì Tauri đã trừu tượng hoá đủ cho always-on-top.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không áp dụng (không có entity dữ liệu mới ở PBI này).
- Hợp đồng giao diện: không áp dụng (chưa thêm Tauri command/event mới; nếu cần nút ẩn-hiện ở mục sau mới cần command).
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/
├── src-tauri/
│   └── src/
│       └── lib.rs          # thêm hàm tạo WebviewWindowBuilder "subtitle" trong .setup()
└── src/
    subtitle.html            # entry point mới cho cửa sổ phụ đề (placeholder, chưa có nội dung phụ đề thật)
    subtitle/
        main.ts               # bootstrap Vue tối thiểu cho cửa sổ phụ đề
vite.config.ts                # thêm subtitle.html vào rollupOptions.input (multi-page)
```

## Rủi ro & ngoại lệ có lý do

- Không thể verify hiển thị always-on-top thật bằng `cargo test` tự động (cần tương tác UI thật) → bù bằng kiểm thử thủ công + ảnh chụp màn hình làm bằng chứng (yêu cầu của `/sora-implement`).
- Môi trường implement hiện tại (container) có thể không có GUI/display thật để chụp ảnh — nếu vậy, ghi rõ trong tasks.md là cần xác nhận lại trên máy có GUI thật (tương tự PBI 17/18/Windows-regression), không tick ẩu.
