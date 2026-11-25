# Nghiên cứu kỹ thuật: PBI 20 — Cửa sổ phụ đề nổi always-on-top

## Quyết định 1: Cách tạo cửa sổ phụ đề

- **Quyết định**: Tạo một cửa sổ Tauri 2 thứ hai, label `subtitle_overlay`, bằng `tauri::WebviewWindowBuilder` trong hook `setup()` của `tauri::Builder` (file `app/src-tauri/src/lib.rs`), chạy song song với cửa sổ chính (`main`) đã có sẵn trong `tauri.conf.json`.
- **Lý do**: Tauri 2 hỗ trợ tạo nhiều webview window trong cùng app; tạo bằng code (không khai báo tĩnh trong `tauri.conf.json`) cho phép kiểm soát thời điểm tạo (ví dụ sau khi app sẵn sàng) và dễ mở rộng logic ẩn/hiện ở các PBI sau (M5) mà không phải sửa config tĩnh.
- **Phương án khác đã xem xét**: Khai báo cửa sổ thứ hai tĩnh trong mảng `app.windows` của `tauri.conf.json`. Bị loại vì khó gắn logic điều kiện (ví dụ chỉ tạo khi cần) và các PBI sau trong M5 (toggle ẩn/hiện, nhớ vị trí) sẽ cần điều khiển bằng code từ Rust/IPC, nên tạo bằng code từ đầu nhất quán hơn.

## Quyết định 2: Thuộc tính cửa sổ để đạt "always-on-top, không viền, không title bar"

- **Quyết định**: Dùng các builder method có sẵn của Tauri 2: `.decorations(false)` (bỏ viền + title bar), `.always_on_top(true)`, `.focused(false)` (không giành focus khi tạo), `.resizable(true)` (để các PBI sau có thể resize, không giới hạn phạm vi PBI này), `.skip_taskbar(true)` (ẩn khỏi taskbar/dock vì đây là overlay phụ trợ, không phải cửa sổ chính của người dùng tương tác).
- **Lý do**: Đây là các API chuẩn, đa nền tảng (Windows + Linux/X11/Wayland qua winit) mà Tauri 2 expose trực tiếp, không cần code riêng theo OS — khớp với giả định trong spec.md là không cần gate theo `cfg(windows)`/`cfg(target_os = "linux")` ở mức tính năng này.
- **Phương án khác đã xem xét**: Set always-on-top bằng lệnh hệ điều hành riêng (WinAPI `SetWindowPos` / X11 `_NET_WM_STATE_ABOVE` thủ công). Bị loại vì Tauri/winit đã bọc sẵn, tự viết lại làm tăng rủi ro kỹ thuật không cần thiết (trùng lặp, dễ lỗi trên 1 trong 2 OS) mà không có lợi ích rõ ràng.

## Quyết định 3: Capability/permission cho cửa sổ mới

- **Quyết định**: Thêm 1 file capability mới `app/src-tauri/capabilities/subtitle-overlay.json` với `"windows": ["subtitle_overlay"]`, scope quyền tối thiểu (`core:window:default` hoặc tương đương core:default đã dùng cho `main`), không thêm quyền thừa.
- **Lý do**: Tauri 2 yêu cầu mỗi window phải được gán vào ít nhất 1 capability để các lệnh core (ví dụ đóng cửa sổ khi app đóng) hoạt động đúng; tách riêng capability theo window giúp dễ kiểm soát phạm vi quyền của cửa sổ overlay (không cần quyền của cửa sổ chính như mở dialog, v.v.).
- **Phương án khác đã xem xét**: Gộp `subtitle_overlay` vào capability `default` đã có (sửa `"windows": ["main", "subtitle_overlay"]`). Bị loại vì làm cửa sổ overlay thừa hưởng toàn bộ quyền của cửa sổ chính kể cả khi không cần, vi phạm nguyên tắc tối thiểu quyền.

## Quyết định 4: Vòng đời (đóng cùng app chính)

- **Quyết định**: Không cấu hình `prevent_close`/handler riêng cho `subtitle_overlay`; để hành vi mặc định của Tauri là khi tiến trình app kết thúc (cửa sổ `main` đóng và không còn window nào khác giữ app sống), toàn bộ app (gồm `subtitle_overlay`) thoát theo.
- **Lý do**: Mặc định `tauri.conf.json` không đặt `app.windows[].closable: false` hay cấu hình "ẩn khi đóng window chính" nên hành vi chuẩn là mọi window cùng thoát khi process thoát — khớp FR-005 (không leak handle/process) mà không cần thêm logic.
- **Phương án khác đã xem xét**: Đặt `subtitle_overlay` ẩn (hide) khi `main` đóng thay vì thoát hẳn, để giữ app chạy nền. Bị loại vì spec hiện tại không yêu cầu chạy nền sau khi đóng cửa sổ chính (ngoài phạm vi PBI), có thể xem xét lại ở M6 nếu có yêu cầu "chạy nền/system tray".

## Quyết định 5: Cách kiểm thử

- **Quyết định**: Viết test tích hợp Rust (`tests/subtitle_overlay_window.rs`) dùng `tauri::test::mock_app()` để khởi tạo app ở môi trường test, gọi hàm tạo cửa sổ overlay (tách thành hàm thuần `create_subtitle_overlay_window(app: &AppHandle<R>) -> tauri::Result<WebviewWindow<R>>` để test được không cần hiển thị thật). Thực tế implement phát hiện: `MockRuntime` của Tauri (dùng bởi `mock_app()`) có các getter `is_decorated()`/`is_always_on_top()`/`is_focused()` luôn trả về giá trị cố định (`false`) bất kể builder truyền gì — không mô phỏng window manager thật — nên không dùng được để xác nhận cấu hình đã áp dụng. Giải pháp: tách cấu hình mong muốn thành hằng số `pub const SUBTITLE_OVERLAY_CONFIG: SubtitleOverlayConfig` trong `lib.rs` (nguồn duy nhất mà `create_subtitle_overlay_window()` dùng để gọi builder), test assert trực tiếp giá trị hằng số này (`decorations=false`, `always_on_top=true`, `focused=false`), và assert riêng rằng `create_subtitle_overlay_window()` tạo cửa sổ thành công với đúng label `subtitle_overlay`. Phần hành vi thị giác thật (SC-001, SC-002 — quan sát bằng mắt, chuyển đổi app) được xác nhận thủ công, ghi lại bằng ảnh chụp theo yêu cầu của `/sora-implement`.
- **Lý do**: Tauri test harness không chạy được window manager thật trong môi trường CI/container headless (không có display server đầy đủ cho always-on-top thật), và `MockRuntime` xác nhận đây không chỉ là vấn đề thiếu display mà bản thân mock cũng không track state các thuộc tính này. Assert trực tiếp vào hằng số cấu hình vẫn đảm bảo: nếu sau này ai đó vô tình sửa `SUBTITLE_OVERLAY_CONFIG` (ví dụ đổi `always_on_top` thành `false`), test sẽ bắt được ngay — đạt mục tiêu "catch regression trong code", dù không xác nhận được window manager thật áp dụng đúng. Phần hành vi thị giác thật cần người xác nhận bằng máy có GUI, giống cách PBI 18 (loopback Linux) đã xử lý — ghi rõ trong tasks.md/plan.md rằng phần này cần xác nhận thủ công.
- **Phương án khác đã xem xét**: (1) Bỏ qua test tự động, chỉ test thủ công — bị loại vì vẫn có phần logic (cấu hình, label, capability) kiểm thử được tự động mà không cần GUI thật. (2) Assert qua `window.is_decorated()`/`is_always_on_top()` như dự kiến ban đầu — bị loại sau khi chạy test thật và phát hiện `MockRuntime` luôn trả hằng số cố định, khiến assert đó không có giá trị phát hiện lỗi thật (test có thể "pass giả" dù code sai).
