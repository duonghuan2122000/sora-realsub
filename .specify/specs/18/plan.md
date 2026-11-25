# Kế hoạch triển khai: Bắt âm thanh loopback đa hệ điều hành (Windows + Linux)

**Mã PBI**: 18
**Liên kết spec**: .specify/specs/18/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust 2024 edition (crate `app_lib`, Tauri 2), build qua `cargo`.
- Framework / Thư viện chính hiện có liên quan: `wasapi` (Windows), `cpal`, `hound` (ghi .wav).
- Thư viện thêm mới (Linux): `libpulse-binding` + `libpulse-simple-binding`.
- Lưu trữ dữ liệu: không có (không liên quan đến PBI này).
- Kiểm thử: `cargo test` (unit test theo pattern có sẵn trong `loopback_audio_source.rs`, bỏ qua nếu máy test không có thiết bị/monitor source khả dụng, giống cách test Windows hiện tại); kiểm thử thủ công ghi ra .wav theo `quickstart.md`.
- Nền tảng triển khai: desktop app Tauri, build riêng cho Windows và Linux (chưa đóng gói installer ở PBI này — thuộc M8).
- Ràng buộc hiệu năng: không thay đổi so với hiện tại (vẫn capture real-time, không yêu cầu thêm).
- Ràng buộc khác: không được phá vỡ hành vi/test hiện có trên Windows (FR-002, SC-002); `cargo build`/`cargo test` phải chạy được trên Linux không cài `wasapi` (FR-001, SC-003).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này (đối chiếu với CLAUDE.md thay thế):

| Nguyên tắc (từ CLAUDE.md) | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào UI | ✅ | Toàn bộ thay đổi nằm trong `core/` (Rust), không chạm UI |
| Giữ nguyên interface Core đã chốt (`AudioSource`) | ✅ | Không đổi trait `AudioSource`; chỉ thêm implementation theo OS |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`:
1. Gate `wasapi` theo `[target.'cfg(windows)'.dependencies]` trong Cargo.toml.
2. Linux loopback dùng `libpulse-binding`/`libpulse-simple-binding`, đọc từ monitor source `@DEFAULT_SINK@.monitor`.
3. Tách implementation theo file riêng mỗi OS, cùng gọi chung trait `AudioSource`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không áp dụng (không có thực thể dữ liệu mới).
- Hợp đồng giao diện: không áp dụng (trait `AudioSource` nội bộ giữ nguyên, không có API/CLI công khai mới).
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/
├── Cargo.toml                                  # sửa: gate wasapi theo cfg(windows),
│                                                #      thêm libpulse-binding/libpulse-simple-binding
│                                                #      theo cfg(target_os = "linux")
└── src/core/
    ├── mod.rs                                  # sửa: khai báo module loopback theo #[cfg(...)]
    ├── loopback_audio_source.rs                 # sửa: thêm #[cfg(windows)] cho toàn file (bản WASAPI hiện có)
    └── loopback_audio_source_linux.rs           # mới: LoopbackAudioSource bản Linux (PulseAudio/PipeWire monitor source)
```

Các file khác (`capture_session.rs`, `mixed_audio_source.rs`, các bin ghi .wav trong `src/bin/`) không cần sửa vì chỉ phụ thuộc vào trait `AudioSource`, không phụ thuộc OS.

## Rủi ro & ngoại lệ có lý do

- Máy build/test Linux trong container CI có thể không có `libpulse-dev`
  (system lib cần cho `libpulse-binding` build qua `pkg-config`/`bindgen`) →
  cần cài đặt hoặc ghi chú rõ trong hướng dẫn dev nếu build thất bại vì thiếu
  thư viện hệ thống (khác với lỗi logic).
- Container/CI headless có thể không có PulseAudio/PipeWire chạy thật → test
  loopback thật phải tự bỏ qua (return sớm) khi không kết nối được, giống
  pattern test Windows hiện tại (`stop_tren_handle_khong_panic_va_dung_nhan_mau`
  trong `loopback_audio_source.rs`), không fail toàn bộ CI.
- Không có ngoại lệ vi phạm CLAUDE.md cần biện minh.
