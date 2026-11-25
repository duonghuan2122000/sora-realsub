---
title: "PBI 20: Cửa sổ phụ đề nổi always-on-top"
date: 2026-11-25
tags: [pbi, m5, feature]
sources: [.specify/specs/20/spec.md, .specify/specs/20/research.md, .specify/specs/20/plan.md, .specify/specs/20/tasks.md, .specify/specs/20/quickstart.md, .specify/specs/20/evidence, app/src-tauri/src/lib.rs, app/src-tauri/capabilities/subtitle-overlay.json, app/src-tauri/tests/subtitle_overlay_window.rs]
---

# PBI 20: Cửa sổ phụ đề nổi always-on-top

Mục roadmap M5 đầu tiên ("Tạo window Tauri riêng, always-on-top, không có border/title bar").
M4 (Dịch) đã hoàn tất, phát event `subtitle-ready` ra UI ([[PBI-19-noi-stt-dich-subtitle-ready]]),
nhưng chưa có nơi nào hiển thị. PBI này tạo cửa sổ Tauri thứ hai dùng để hiển thị phụ đề sau này —
**chỉ tạo cửa sổ với đúng đặc tính (always-on-top, không viền, không giành focus)**, chưa hiển
thị nội dung phụ đề thật (các mục tiếp theo trong M5).

## Yêu cầu chính (spec.md)

- Tạo cửa sổ riêng `subtitle_overlay`, khác cửa sổ `main` (FR-001).
- Không viền/title bar (FR-002), luôn nổi trên các cửa sổ khác (FR-003).
- Không giành focus bàn phím khi tạo/cập nhật (FR-004).
- Đóng cùng vòng đời app chính, không leak process (FR-005).
- Ngoài phạm vi: nội dung phụ đề thật, kéo-thả, resize, cỡ chữ, phím tắt ẩn-hiện, click-through.

## Quyết định kỹ thuật (research.md)

1. Tạo cửa sổ bằng code (`WebviewWindowBuilder` trong hook `setup()`), không khai báo tĩnh trong
   `tauri.conf.json` — để các PBI M5 sau (toggle ẩn/hiện, nhớ vị trí) dễ điều khiển bằng code.
2. Dùng API chuẩn, đa nền tảng của Tauri 2: `.decorations(false)`, `.always_on_top(true)`,
   `.focused(false)`, `.skip_taskbar(true)`, `.resizable(true)` — không gate theo OS.
3. Capability riêng `subtitle-overlay.json` (`"windows": ["subtitle_overlay"]`), quyền tối thiểu
   `core:default` — không gộp vào capability `default` của `main` (tránh thừa quyền).
4. Không cấu hình vòng đời riêng — overlay thoát cùng process khi app đóng (mặc định Tauri).
5. **Phát hiện quan trọng khi thi công**: `MockRuntime` của `tauri::test::mock_app()` có các
   getter `is_decorated()`/`is_always_on_top()`/`is_focused()` luôn trả hằng số cố định (`false`),
   **không track giá trị đã truyền vào builder** — không mô phỏng window manager thật. Giải pháp:
   tách cấu hình mong muốn thành hằng số `pub const SUBTITLE_OVERLAY_CONFIG` trong `lib.rs` (nguồn
   duy nhất mà hàm tạo cửa sổ dùng), test assert trực tiếp vào hằng số này thay vì gọi getter của
   cửa sổ mock.

## Cài đặt (code)

- `lib.rs`:
  - `SubtitleOverlayConfig` (struct `pub`) + hằng số `pub const SUBTITLE_OVERLAY_CONFIG` — nguồn
    cấu hình duy nhất (decorations=false, always_on_top=true, focused=false, skip_taskbar=true,
    resizable=true).
  - `pub fn create_subtitle_overlay_window<R: tauri::Runtime>(app: &AppHandle<R>) ->
    tauri::Result<WebviewWindow<R>>` — hàm thuần generic theo runtime để test được qua
    `tauri::test::mock_app()` (dùng `MockRuntime`) lẫn runtime thật (`Wry`).
  - `.setup()` của `tauri::Builder` gọi hàm trên, log lỗi (không panic) nếu thất bại, không chặn
    `main` khởi động.
- `capabilities/subtitle-overlay.json` (mới): capability riêng cho window `subtitle_overlay`.
- `tests/subtitle_overlay_window.rs` (mới): 2 test — assert `SUBTITLE_OVERLAY_CONFIG` đúng giá
  trị (FR-002/003/004), assert `create_subtitle_overlay_window()` tạo đúng label.
- `Cargo.toml`: thêm `tauri = { version = "2", features = ["test"] }` vào `[dev-dependencies]`
  (cho `tauri::test::mock_app()`).

## Bằng chứng test

- `cargo build -p app` + `cargo test -p app`: **65 + 2 = 67 passed, 0 failed, 2 ignored** (2
  ignored là test PBI 16/17 cần sidecar thật, không liên quan PBI 20) —
  `.specify/specs/20/evidence/T004-T006-cargo-test-result.log`,
  `T004-T006-testcase.md`.
- **Kiểm thử thủ công (T007) best-effort trong container** (Xvfb, không có window manager thật):
  chạy `bun run tauri dev`, chụp ảnh (`T007-overlay-startup.png`), dùng `xwininfo`/`xprop` xác
  nhận 2 webview window cấp cao nhất tồn tại và cửa sổ overlay có `_MOTIF_WM_HINTS` yêu cầu
  "không trang trí" (FR-002, ở mức gửi đúng hint tới WM); `_NET_WM_STATE` rỗng trên cả 2 cửa sổ vì
  **không có window manager** để phản hồi always-on-top — **SC-002 (always-on-top thật khi chuyển
  đổi ứng dụng) và FR-004 (focus thật) CHƯA xác nhận được** trong container này. Không còn process
  nào sau khi dừng (SC-003 xác nhận) — `T007-xprop-result.log`, `T007-testcase.md`.

## ⚠️ Cần xác nhận trên máy GUI thật trước khi tick roadmap

Giống pattern đã áp dụng ở [[PBI-17-dich-co-ngu-canh]]/[[PBI-18-loopback-linux]]: môi trường
container dùng để implement PBI này **không có window manager thật** (`Xvfb` chỉ là X server ảo,
không compositing/WM), nên không thể xác nhận trực quan phần cốt lõi của tính năng — cửa sổ có
thật sự luôn nổi trên các ứng dụng khác khi chuyển đổi hay không. Cần người có máy Windows hoặc
Linux desktop thật chạy lại theo `quickstart.md` (`.specify/specs/20/quickstart.md`), chụp ảnh xác
nhận SC-001/SC-002/SC-003, trước khi tick dòng roadmap M5 tương ứng.

## Liên quan

[[PBI-19-noi-stt-dich-subtitle-ready]], [[PBI-17-dich-co-ngu-canh]], [[PBI-18-loopback-linux]],
[[Kien-truc-core-ui]], [[Rui-ro-ky-thuat]], [[Trang-thai-ma-nguon]]
