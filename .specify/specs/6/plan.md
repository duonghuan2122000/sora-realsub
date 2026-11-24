# Kế hoạch triển khai: Capture loopback âm thanh hệ thống (Windows)

**Mã PBI**: 6
**Liên kết spec**: .specify/specs/6/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (edition 2024), core trong `app/src-tauri` |
| Framework / Thư viện chính | `wasapi` (crate mới, cần thêm vào Cargo.toml) — `cpal` không expose API loopback (AUDCLNT_STREAMFLAGS_LOOPBACK), nên dùng crate `wasapi` bọc trực tiếp WASAPI cho riêng tính năng này; `hound` (đã có) để ghi .wav làm bằng chứng test |
| Lưu trữ dữ liệu | Không — dữ liệu PCM chỉ tồn tại trong bộ nhớ (callback stream) |
| Kiểm thử | `cargo test`: test logic nội bộ (thiết bị không khả dụng → lỗi rõ nghĩa, stop không panic) + 1 bin thủ công chạy thật với loa thật, ghi ra `.wav` bằng `hound` làm bằng chứng nghe lại |
| Nền tảng triển khai | Windows only (theo M0); dùng `#[cfg(windows)]` nếu cần tách code khỏi build non-Windows trong tương lai, nhưng hiện dự án target Windows trước nên có thể để thẳng |
| Ràng buộc hiệu năng | Không yêu cầu real-time cứng ở bước này; chỉ cần không drop callback liên tục và dừng capture dưới 1s (SC-002, giống PBI 5) |
| Ràng buộc khác | Phải implement qua trait `AudioSource` đã định nghĩa ở PBI 3 (`core/audio_source.rs`), không phá vỡ interface hiện có; giữ pattern 1 thread riêng giữ stream sống (giống `MicAudioSource`) vì các handle WASAPI cũng không `Send` an toàn qua await point |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Không có `.specify/memory/constitution.md` trong dự án — áp dụng nguyên tắc chung trong `CLAUDE.md`: Core (Rust) tách biệt UI (Vue), giao tiếp qua IPC.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Capture loopback thuần Rust trong `core/`, không đụng UI ở PBI này |
| Giao tiếp qua Tauri IPC | ✅ (chưa cần ở PBI này) | Command `start_capture`/`stop_capture` thuộc PBI riêng (mục cuối M1); PBI 6 chỉ làm phần lõi `AudioSource` impl, test qua `cargo test`/bin mẫu, chưa cần expose IPC |

## Giai đoạn 0 — Nghiên cứu

Xem `research.md`. Tóm tắt các quyết định chính:

- **Quyết định**: Dùng crate `wasapi` để mở loopback capture stream từ thiết bị output mặc định. **Lý do**: `cpal` (đã dùng cho mic ở PBI 5) không hỗ trợ cờ `AUDCLNT_STREAMFLAGS_LOOPBACK` của WASAPI qua API công khai, nên không thể capture loopback bằng cpal; crate `wasapi` bọc trực tiếp Windows Core Audio API, hỗ trợ loopback as a first-class use case, đã được nhiều project Rust khác dùng cho mục đích này. **Phương án khác**: tự gọi WASAPI qua `windows`/`windows-sys` crate thủ công — đúng cơ chế nhưng tốn công lặp lại thứ `wasapi` đã làm sẵn, không mang lại giá trị thêm ở bước này.
- **Quyết định**: Chỉ capture thiết bị output **mặc định**, không cho chọn thiết bị cụ thể. **Lý do**: khớp phạm vi đã chốt ở spec.md (Giả định + Ngoài phạm vi), đơn giản hoá implement bước đầu. **Phương án khác**: cho chọn thiết bị — để lại PBI sau nếu cần.
- **Quyết định**: Giữ sample format đầu ra là `f32` mono, convert tại chỗ nếu thiết bị trả định dạng khác (giống `MicAudioSource::to_mono`). **Lý do**: khớp `AudioFrame.samples: Vec<f32>` đã định nghĩa ở `core/types.rs`, tái dùng được logic convert đã có. **Phương án khác**: tách hàm `to_mono` dùng chung giữa 2 module — cân nhắc nhưng để tránh over-engineering ở bước này, có thể nhân bản nhỏ (duplicate 1 hàm ngắn) hoặc đưa lên module chung nếu refactor không rủi ro; quyết định cụ thể để ở bước implement tuỳ độ phức tạp thực tế.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: không có thực thể dữ liệu bền (persistent); tái sử dụng nguyên vẹn `AudioFrame`/`StreamHandle` đã có từ PBI 3. Không tạo `data-model.md`.
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
├── Cargo.toml                          # thêm dependency "wasapi"
├── src/core/
│   ├── loopback_audio_source.rs        # struct LoopbackAudioSource, impl trait AudioSource dùng wasapi
│   └── mod.rs                          # khai báo module mới
├── examples/ hoặc src/bin/
│   └── capture_loopback_to_wav.rs      # bin thủ công: capture N giây loopback thật, ghi ra .wav bằng hound (bằng chứng test SC-001)
```

## Rủi ro & ngoại lệ có lý do

- Test với loa/thiết bị output thật không thể chạy tự động trong CI (cần thiết bị vật lý + có âm thanh đang phát) → dùng làm bằng chứng thủ công (ảnh chụp kết quả + file .wav), còn logic nội bộ (stop không panic, thiết bị không khả dụng → lỗi rõ nghĩa) vẫn có unit test tự động theo pattern đã có ở `mic_audio_source.rs`.
- Crate `wasapi` là crate mới thêm vào dependencies (chưa có trong Cargo.toml) — chỉ dùng cho Windows, không ảnh hưởng các module khác; nếu sau này hỗ trợ macOS (Backlog), module này sẽ cần thay thế hoàn toàn bằng cơ chế khác (BlackHole/ScreenCaptureKit), không tái dùng được.
