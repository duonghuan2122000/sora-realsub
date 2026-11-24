# Kế hoạch triển khai: Capture mic thành PCM buffer

**Mã PBI**: 5
**Liên kết spec**: .specify/specs/5/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (edition 2024), core trong `app/src-tauri` |
| Framework / Thư viện chính | `cpal` (đã có trong Cargo.toml, dùng để liệt kê thiết bị ở PBI 4) cho capture audio; thêm `hound` để ghi .wav làm bằng chứng test. Ghi chú: Cargo không cấp dev-dependencies cho `[[bin]]` thường, nên `hound` phải khai ở `[dependencies]` dù chỉ dùng trong bin test thủ công |
| Lưu trữ dữ liệu | Không — dữ liệu PCM chỉ tồn tại trong bộ nhớ (callback stream), không ghi file ở tính năng chính |
| Kiểm thử | `cargo test`: test logic nội bộ bằng `AudioSource` mock (đã có pattern ở `audio_source.rs`) + 1 test/bin thủ công chạy thật với mic thật, ghi ra `.wav` bằng `hound` làm bằng chứng nghe lại |
| Nền tảng triển khai | Windows trước (theo M0), dùng cpal default host |
| Ràng buộc hiệu năng | Không yêu cầu real-time cứng ở bước này; chỉ cần không drop callback liên tục và dừng capture dưới 1s (SC-002) |
| Ràng buộc khác | Phải implement qua trait `AudioSource` đã định nghĩa ở PBI 3 (`core/audio_source.rs`), không phá vỡ interface hiện có |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Không có `.specify/memory/constitution.md` trong dự án — áp dụng nguyên tắc chung trong `CLAUDE.md`: Core (Rust) tách biệt UI (Vue), giao tiếp qua IPC.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Capture mic thuần Rust trong `core/`, không đụng UI ở PBI này |
| Giao tiếp qua Tauri IPC | ✅ (chưa cần ở PBI này) | Command `start_capture`/`stop_capture` thuộc PBI riêng (M1, mục sau); PBI 5 chỉ làm phần lõi `AudioSource` impl, test độc lập qua `cargo test`/bin mẫu, chưa cần expose IPC |

## Giai đoạn 0 — Nghiên cứu

Xem `research.md`. Tóm tắt các quyết định chính:

- **Quyết định**: Dùng `cpal` để mở input stream từ thiết bị mic. **Lý do**: đã có sẵn trong dependencies (PBI 4), cross-platform, hỗ trợ callback mẫu trực tiếp phù hợp interface `AudioSource::start`. **Phương án khác**: viết trực tiếp WASAPI — phức tạp hơn, không cần ở bước này vì cpal đã đủ cho mic (loopback mới cần tiếp cận riêng ở PBI sau).
- **Quyết định**: Dùng `hound` làm thư viện ghi `.wav` cho test/bằng chứng. **Lý do**: nhỏ, phổ biến, chỉ cần cho dev-dependency, không vào bản release. **Phương án khác**: tự viết WAV header — không cần thiết, tốn công không mang lại giá trị.
- **Quyết định**: Giữ sample format là `f32` mono, resample/convert tại chỗ nếu thiết bị trả định dạng khác. **Lý do**: khớp với `AudioFrame.samples: Vec<f32>` đã định nghĩa ở `core/types.rs`. **Phương án khác**: giữ định dạng gốc thiết bị trả về rồi convert ở tầng sau — làm phân tán logic convert, khó test tại nguồn.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: không có thực thể dữ liệu bền (persistent); cấu trúc runtime `AudioFrame`/`StreamHandle` đã có sẵn từ PBI 3, tái sử dụng nguyên vẹn. Không tạo `data-model.md`.
- **Hợp đồng giao diện**: không có API/CLI công khai mới ở bước này (chưa expose Tauri command). Không tạo `contracts/`.
- **Kịch bản khởi động nhanh**: xem `quickstart.md`.

## Kiểm tra theo hiến pháp dự án (sau thiết kế)

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Không đổi |
| Giao tiếp qua Tauri IPC | ✅ | Không vi phạm; IPC để sau |

## Cấu trúc dự án dự kiến

```text
app/src-tauri/
├── Cargo.toml                     # thêm dev-dependency "hound"
├── src/core/
│   ├── mic_audio_source.rs        # struct MicAudioSource, impl trait AudioSource dùng cpal
│   └── mod.rs                     # khai báo module mới
├── examples/ hoặc src/bin/
│   └── capture_mic_to_wav.rs      # bin thủ công: capture N giây từ mic thật, ghi ra .wav bằng hound (bằng chứng test SC-001)
```

## Rủi ro & ngoại lệ có lý do

- Test với mic thật không thể chạy tự động trong CI (cần thiết bị vật lý + người nói) → dùng làm bằng chứng thủ công (ảnh chụp kết quả + file .wav), còn logic nội bộ (start/stop, lỗi thiết bị không hợp lệ) vẫn có unit test tự động qua mock như pattern đã có ở `audio_source.rs`.
- `cpal` trên Windows mặc định dùng WASAPI shared mode — đủ cho mic, không cần exclusive mode ở bước này.
