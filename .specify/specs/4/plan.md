# Kế hoạch triển khai: Liệt kê thiết bị âm thanh đầu vào

**Mã PBI**: 4
**Liên kết spec**: .specify/specs/4/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core) + Vue 3/TypeScript (UI)
- Framework / Thư viện chính: `cpal` (enumerate audio device), `serde`/`serde_json` (đã có), Tauri IPC command
- Lưu trữ dữ liệu: không có (truy vấn tức thời, không lưu trữ)
- Kiểm thử: `cargo test` cho logic Core (mock host audio), kiểm thử thủ công qua UI cho phần hiển thị
- Nền tảng triển khai: Windows trước (theo M0)
- Ràng buộc hiệu năng: trả kết quả trong < 1 giây (SC-001)
- Ràng buộc khác: không đưa logic liệt kê vào trait `AudioSource` hiện có (xem research.md quyết định 2)

## Kiểm tra theo hiến pháp dự án

Dự án chưa có `.specify/memory/constitution.md`. Đối chiếu với nguyên tắc nêu trong `CLAUDE.md`: Core (Rust) và UI (Vue) phải tách biệt, giao tiếp qua Tauri IPC — kế hoạch tuân thủ (logic liệt kê thiết bị nằm trong `core`, expose qua 1 Tauri command, UI chỉ gọi và hiển thị).

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt, giao tiếp qua IPC | ✅ | Logic nằm trong `core`, không có business logic trong UI |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: chọn `cpal`, không mở rộng trait `AudioSource`, dùng Tauri command request/response (không cần event).

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` (struct `AudioDeviceInfo`)
- Hợp đồng giao diện: Tauri command `list_input_devices() -> Result<Vec<AudioDeviceInfo>, String>` (không cần thư mục `contracts/` riêng — 1 command đơn giản, mô tả đủ trong plan này)
- Kịch bản khởi động nhanh: xem `quickstart.md`

## Cấu trúc dự án dự kiến

```
app/src-tauri/
  Cargo.toml                    # thêm dependency `cpal`
  src/core/
    devices.rs                  # mới: AudioDeviceInfo + hàm list_input_devices(host: &dyn ...) có thể test được qua trait host giả lập
    mod.rs                      # thêm `pub mod devices;`
  src/lib.rs                    # đăng ký Tauri command `list_input_devices`
app/src/
  (màn hình/thành phần UI chọn thiết bị — tuỳ cấu trúc Vue hiện có, gọi invoke('list_input_devices'))
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: `cpal` trên CI/máy không có thiết bị audio có thể trả lỗi khi khởi tạo host — xử lý bằng cách bọc lỗi host thành `CoreError::Io`/thông báo rõ nghĩa (đã có trong FR-005), và tách logic enumerate ra sau 1 trait nhỏ để test bằng mock, không phụ thuộc phần cứng thật khi chạy `cargo test`.
- Không có ngoại lệ vi phạm nguyên tắc dự án.
