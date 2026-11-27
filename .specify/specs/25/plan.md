# Kế hoạch triển khai: Ẩn-hiện cửa sổ phụ đề

**Mã PBI**: 25
**Liên kết spec**: .specify/specs/25/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (Tauri 2 core), edition 2024 |
| Framework / Thư viện chính | `tauri-plugin-global-shortcut` v2 (mới thêm) |
| Lưu trữ dữ liệu | Không (trạng thái ẩn/hiện chỉ giữ trong bộ nhớ, `AtomicBool` trong Tauri state) |
| Kiểm thử | `cargo test` cho logic toggle thuần (tách khỏi `AppHandle`/plugin để test được không cần runtime Tauri); test tay trên Windows thật cho hành vi global shortcut thực tế (không mock được OS hotkey) |
| Nền tảng triển khai | Windows + Linux (plugin hỗ trợ cả 2, không cần code theo OS) |
| Ràng buộc hiệu năng | SC-002: đổi trạng thái hiển thị < 0.5s sau khi nhấn phím |
| Ràng buộc khác | Không trùng shortcut OS/app phổ biến (xem research.md quyết định 2) |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này, tuân theo quy ước chung ở `CLAUDE.md`/`docs/style-guide.md` (Core/UI tách biệt — tính năng này thuần Core/Rust, không cần sửa UI Vue).

## Giai đoạn 0 — Nghiên cứu

Xem `research.md`. Tóm tắt:

- **Quyết định**: dùng `tauri-plugin-global-shortcut` để bắt phím toàn cục `Ctrl+Alt+H`. **Lý do**: cần hoạt động khi cửa sổ không focus, có sẵn hỗ trợ đa OS. **Phương án khác**: tự bắt raw input theo OS — loại vì phức tạp không cần thiết.
- **Quyết định**: dùng `hide()`/`show()` thay vì đóng/tạo lại cửa sổ. **Lý do**: giữ nguyên vị trí/cỡ chữ đã lưu (FR-004), không cần quản lý state tái tạo window.
- **Quyết định**: logic đảo trạng thái (hàm thuần, nhận trạng thái hiện tại trả trạng thái mới) tách khỏi Tauri API để `cargo test` được, giống pattern `translate_final_chunk` đã dùng ở PBI 19.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: không có — chỉ 1 `AtomicBool` nội bộ, không phải thực thể nghiệp vụ.
- **Hợp đồng giao diện**: không có — không thêm Tauri command/event mới, không đổi giao diện IPC hiện có.
- **Kịch bản khởi động nhanh**: xem `quickstart.md`.

## Kiểm tra theo hiến pháp dự án (sau thiết kế)

Không áp dụng (không có constitution.md). Thiết kế không đưa logic nghiệp vụ vào UI, tuân theo quy ước Core/UI tách biệt của `CLAUDE.md`.

## Cấu trúc dự án dự kiến

```text
app/src-tauri/
  Cargo.toml                 # thêm dependency tauri-plugin-global-shortcut
  capabilities/default.json  # thêm quyền global-shortcut (nếu plugin yêu cầu capability)
  src/lib.rs                 # thêm: đăng ký plugin, hàm toggle thuần (test được),
                              #        handler shortcut gọi window "subtitle".hide()/show()
```

## Rủi ro & ngoại lệ có lý do

- Hành vi global shortcut thật (bắt phím khi mất focus) không test tự động được trong CI/container — chỉ xác nhận bằng test tay trên máy Windows thật, giống các PBI trước (PBI 20, 23).
- Nếu tổ hợp `Ctrl+Alt+H` xung đột với phần mềm khác trên máy người dùng cụ thể, cần đổi hằng số trong code (chưa có UI cấu hình — đã ghi trong "Ngoài phạm vi" của spec.md).
