# Kế hoạch triển khai: Lệnh bắt đầu/kết thúc phiên bắt âm thanh

**Mã PBI**: 8
**Liên kết spec**: .specify/specs/8/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (Tauri 2 core) |
| Framework / Thư viện chính | `tauri` (command + `State`), tái dùng `core::{MixedAudioSource, MicAudioSource, LoopbackAudioSource, StreamHandle}` từ PBI 5/6/7 |
| Lưu trữ dữ liệu | Không — trạng thái phiên chỉ giữ trong RAM (`Mutex<Option<StreamHandle>>`) |
| Kiểm thử | `cargo test` — unit test mock `AudioSource` đếm start/stop để xác minh không leak |
| Nền tảng triển khai | Windows desktop (Tauri), không đổi so với PBI trước |
| Ràng buộc hiệu năng | Không có yêu cầu mới ngoài spec (không leak theo thời gian) |
| Ràng buộc khác | Không đưa logic nghiệp vụ vào UI Vue — toàn bộ state máy start/stop nằm ở Core Rust |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Không có `.specify/memory/constitution.md` trong dự án → bỏ qua bước đối chiếu hiến pháp, chỉ tuân theo nguyên tắc tách Core/UI đã nêu trong CLAUDE.md.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Toàn bộ state máy nằm trong `src-tauri/src/core` + `lib.rs`, UI chỉ `invoke` |

## Giai đoạn 0 — Nghiên cứu

Xem `research.md`. Tóm tắt:

- **Quyết định**: Trạng thái phiên = `Mutex<Option<StreamHandle>>` trong `tauri::State`. **Lý do**: tái dùng `StreamHandle` có sẵn, tránh 2 nguồn sự thật. **Phương án khác**: `AtomicBool` riêng — loại vì race.
- **Quyết định**: Tái dùng `MixedAudioSource` nguyên trạng từ PBI 7. **Lý do**: ngoài phạm vi PBI 8. **Phương án khác**: không có.
- **Quyết định**: 2 command Tauri `start_capture`/`stop_capture` trả `Result<(), String>`. **Lý do**: nhất quán với `list_input_devices_cmd` đã có. **Phương án khác**: serialize `CoreError` — chưa cần.
- **Quyết định**: Test leak bằng mock `AudioSource` đếm start/stop, không phụ thuộc thiết bị thật. **Lý do**: chạy được trên CI. **Phương án khác**: đo thread OS thật — loại, thêm dependency không cần.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: Không áp dụng — không có thực thể dữ liệu bền; chỉ có trạng thái runtime `CaptureSessionState { handle: Mutex<Option<StreamHandle>> }`.
- **Hợp đồng giao diện**: 2 lệnh Tauri IPC:
  - `start_capture() -> Result<(), String>`
  - `stop_capture() -> Result<(), String>`
- **Kịch bản khởi động nhanh**: xem `quickstart.md`.

## Kiểm tra theo hiến pháp dự án (sau thiết kế)

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Không đổi so với trước thiết kế |

## Cấu trúc dự án dự kiến

```text
app/src-tauri/src/
├── core/
│   └── capture_session.rs       # MỚI — CaptureSessionState: start()/stop(), bọc MixedAudioSource + StreamHandle
│   └── mod.rs                   # SỬA — export CaptureSessionState
└── lib.rs                       # SỬA — thêm command start_capture/stop_capture, quản lý tauri::State<CaptureSessionState>
```

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc dự án.
- Rủi ro: nếu `start()`/`stop()` bị gọi đồng thời từ nhiều luồng IPC (hiếm với Tauri single-threaded invoke theo command), `Mutex` đã đủ để đảm bảo nhất quán — không cần thêm cơ chế.
