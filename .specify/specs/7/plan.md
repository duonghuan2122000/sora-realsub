# Kế hoạch triển khai: Trộn mic + loopback thành 1 stream

**Mã PBI**: 7
**Liên kết spec**: .specify/specs/7/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Core, Tauri 2), crate `src-tauri` tại `app/src-tauri/`
- Framework / Thư viện chính: không thêm crate mới — tái dùng `AudioSource` trait + `AudioFrame`/`StreamHandle` đã có (`core::audio_source`, `core::types`), chạy `MicAudioSource` và `LoopbackAudioSource` đồng thời trên 2 luồng gọi hiện có của chúng.
- Lưu trữ dữ liệu: không có; chỉ xử lý luồng PCM trong bộ nhớ (f32 mono).
- Kiểm thử: `cargo test` (unit test với mock `AudioSource`) + bin thực nghiệm ghi ra .wav để nghe bằng tai (giống `capture_mic_to_wav.rs`/`capture_loopback_to_wav.rs`).
- Nền tảng triển khai: Windows (theo quyết định M0), không đổi.
- Ràng buộc hiệu năng: trộn phải chạy real-time, không được làm trễ/nghẽn luồng audio.
- Ràng buộc khác: `MicAudioSource`/`LoopbackAudioSource` phát `AudioFrame` độc lập, không đồng bộ theo thời gian tuyệt đối với nhau — cần một buffer riêng cho mỗi nguồn để trộn theo khối (frame-by-frame) mà không yêu cầu 2 nguồn đẩy mẫu đồng thời.

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này; đối chiếu trực tiếp với CLAUDE.md: Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Toàn bộ logic trộn nằm trong `core/`, không đụng tới `app/src/` (Vue) |
| Core expose qua trait, không gắn cứng implementation (từ M0) | ✅ | `MixedAudioSource` là 1 `AudioSource` khác, nhận 2 `Box<dyn AudioSource>` làm input, không phá vỡ interface hiện có |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không có entity dữ liệu lưu trữ mới; chỉ có khái niệm xử lý "cặp buffer mic/loopback" mô tả trong `data-model.md`.
- Hợp đồng giao diện: không có API công khai/CLI mới ở bước này (chưa tới `start_capture`/`stop_capture` Tauri command — PBI sau). Không tạo `contracts/`.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  mixed_audio_source.rs   # MỚI: struct MixedAudioSource, impl AudioSource — trộn 2 nguồn con
  mod.rs                  # SỬA: thêm `pub mod mixed_audio_source;`
app/src-tauri/src/bin/
  capture_mixed_to_wav.rs # MỚI: bin thực nghiệm, chạy MixedAudioSource(mic, loopback), ghi ra .wav để nghe kiểm tra (làm bằng chứng test)
.specify/specs/7/evidence/
  *.wav, ảnh/log test     # bằng chứng test do /sora-implement tạo
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: mic và loopback có thể có sample rate khác nhau (ví dụ mic 48kHz, loopback 44.1kHz) → cần resample trước khi cộng mẫu. Giảm thiểu: resample đơn giản bằng linear interpolation về 1 sample rate chung (sample rate cao hơn trong 2 nguồn), đủ cho mục tiêu "nghe được, không bị lỗi" của SC-001..SC-004, không cần resampler chất lượng cao ở bước này (ghi chú trong research.md).
- Rủi ro: clipping khi cộng 2 nguồn cùng lớn → giảm thiểu bằng clamp về [-1.0, 1.0] sau khi cộng (không cần giảm gain phức tạp, chấp nhận trade-off nhỏ về âm lượng theo Giả định trong spec).
- Không có ngoại lệ vi phạm nguyên tắc dự án cần biện minh.
