# Kế hoạch triển khai: Cắt audio thành segment theo khoảng lặng

**Mã PBI**: 10
**Liên kết spec**: .specify/specs/10/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (crate `app_lib` trong `app/src-tauri`)
- Framework / Thư viện chính: không thêm dependency mới — xử lý thuần trên `Vec<FrameLabel>` do `core::vad::detect` (PBI 9) trả về
- Lưu trữ dữ liệu: không có (xử lý theo buffer trong bộ nhớ, không persist)
- Kiểm thử: `cargo test` — test tích hợp với file WAV mẫu có nói + lặng xen kẽ (tái dùng fixture đã có từ PBI 9, hoặc fixture mới nếu cần nhiều lượt nói tách biệt rõ hơn)
- Nền tảng triển khai: Windows (giống các PBI audio trước)
- Ràng buộc hiệu năng: xử lý 1 lượt trên toàn bộ `Vec<FrameLabel>` đã có, chưa cần streaming stateful qua nhiều buffer (đó thuộc bước ghép pipeline ở PBI tiếp theo khi expose event `vad-segment`)
- Ràng buộc khác: không có

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong repo. Đối chiếu với CLAUDE.md dự án:

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào UI | ✅ | Logic cắt segment implement hoàn toàn trong `app/src-tauri/src/core`, không export ra UI ở PBI này |
| Trait Core đã định nghĩa (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) | ✅ | PBI này không cần trait mới, là bước xử lý thuần dữ liệu nằm giữa VAD (PBI 9) và STT (M3) |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: thuật toán state machine với silence-hangover (ngưỡng `min_silence_frames`), hàm mặc định + hàm `_with_min_silence` để test/tùy biến, `Segment` chỉ lưu `start_frame_index`/`end_frame_index`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không cần `data-model.md` riêng — thực thể chính chỉ là `Segment { start_frame_index: usize, end_frame_index: usize }`, đã mô tả đủ trong spec/research.
- Hợp đồng giao diện: không có — PBI này chưa expose Tauri command/event ra UI (việc đó thuộc PBI kế tiếp trong M2, phát event `vad-segment`). Chỉ là module Core nội bộ.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  segment.rs           # module mới: struct Segment, hàm segment(labels: &[FrameLabel]) -> Vec<Segment>
  mod.rs                # thêm `pub mod segment;`
```

Test tích hợp đặt cạnh module (`#[cfg(test)]` trong `segment.rs`), dùng `vad::detect` trên file WAV mẫu (tái dùng fixture từ PBI 9 trong `app/src-tauri/tests/`), theo đúng convention test hiện tại của repo (xem `vad.rs`, `mixed_audio_source.rs`).

## Rủi ro & ngoại lệ có lý do

- Rủi ro: fixture WAV hiện có (PBI 9) có thể chỉ có 1 lượt nói liên tục, không đủ để test kịch bản nhiều lượt nói tách biệt (kịch bản chấp nhận #2) — nếu thiếu, cần tạo thêm fixture WAV mới (nói-lặng-nói-lặng) ở bước implement.
- Không có ngoại lệ vi phạm nguyên tắc dự án cần biện minh.
