# Kế hoạch triển khai: Phát hiện hoạt động giọng nói (VAD)

**Mã PBI**: 9
**Liên kết spec**: .specify/specs/9/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (crate `app_lib` trong `app/src-tauri`)
- Framework / Thư viện chính: crate `voice_activity_detector` (Silero VAD v5, ONNX runtime nội bộ)
- Lưu trữ dữ liệu: không có (xử lý theo buffer trong bộ nhớ, không persist)
- Kiểm thử: `cargo test` — test tích hợp với file WAV mẫu có nói + lặng xen kẽ
- Nền tảng triển khai: Windows (giống các PBI audio trước)
- Ràng buộc hiệu năng: xử lý theo buffer nhận từ `MixedAudioSource`, chưa yêu cầu real-time cứng ở PBI này (đó là việc của M2 bước ghép vào luồng streaming)
- Ràng buộc khác: input audio phải ở 16kHz mono khi vào VAD (resample nếu cần) — xem `research.md`

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong repo. Đối chiếu với CLAUDE.md dự án:

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào UI | ✅ | VAD implement hoàn toàn trong `app/src-tauri/src/core`, không export logic ra UI ở PBI này |
| Trait Core đã định nghĩa (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) | ✅ | PBI này không cần trait mới, VAD là bước xử lý nằm giữa `AudioSource` và `SpeechToText`, implement dưới dạng hàm/struct riêng trong `core` |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: chọn crate `voice_activity_detector` (Silero VAD v5), chuẩn hoá input 16kHz mono / frame 512 sample, ngưỡng phân loại mặc định 0.5.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không cần `data-model.md` riêng — thực thể chính chỉ là nhãn frame (`FrameLabel { speech: bool, frame_index: usize }` hoặc tương đương), đã mô tả đủ trong spec và ngữ cảnh kỹ thuật ở trên.
- Hợp đồng giao diện: không có — PBI này chưa expose Tauri command/event ra UI (việc đó thuộc PBI kế tiếp trong M2, phát event `vad-segment`). Chỉ là module Core nội bộ.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  vad.rs              # module mới: struct VoiceActivityDetector, hàm detect(frames: &[AudioFrame]) -> Vec<FrameLabel>
  mod.rs               # thêm `pub mod vad;`
app/src-tauri/Cargo.toml  # thêm dependency voice_activity_detector
```

Test tích hợp đặt cạnh module (`#[cfg(test)]` trong `vad.rs`) hoặc file test riêng dùng chung fixture WAV mẫu đã có từ PBI 5/6/7, theo đúng convention test hiện tại của repo (xem `mixed_audio_source.rs`, `capture_session.rs`).

## Rủi ro & ngoại lệ có lý do

- Rủi ro: crate `voice_activity_detector` có thể kéo theo dependency `ort` (ONNX runtime) khá nặng về kích thước binary — chấp nhận được ở giai đoạn này vì mục tiêu là có VAD hoạt động đúng trước, tối ưu kích thước đóng gói thuộc M8.
- Không có ngoại lệ vi phạm nguyên tắc dự án cần biện minh.
