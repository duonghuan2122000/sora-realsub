# Kế hoạch triển khai: Phát sự kiện đoạn VAD cho UI

**Mã PBI**: 11
**Liên kết spec**: .specify/specs/11/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Core, `app/src-tauri`), Vue 3 + TypeScript (UI, chỉ nhận event ở mức tối thiểu cho PBI này)
- Framework / Thư viện chính: Tauri 2 (event system `AppHandle::emit`), `voice_activity_detector` (đã dùng ở `core::vad`)
- Lưu trữ dữ liệu: không có (sự kiện tức thời, không persist)
- Kiểm thử: `cargo test` cho logic streaming segmenter trong Core (không phụ thuộc Tauri); kiểm thử UI nhận event bằng test thủ công ghi log console (xem `quickstart.md`)
- Nền tảng triển khai: giống các PBI trước (desktop Windows qua Tauri)
- Ràng buộc hiệu năng: phát sự kiện ngay khi segment đóng, không gây giật khi quan sát trực tiếp (SC-002)
- Ràng buộc khác: Core (Rust) không phụ thuộc trực tiếp vào Tauri API — theo CLAUDE.md "Core và UI phải tách biệt". Việc gọi `emit` phải nằm ở `lib.rs` (tầng command/wiring), không nằm trong module `core::`

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — chỉ đối chiếu với nguyên tắc tách Core/UI trong `CLAUDE.md`.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt, logic nghiệp vụ không ở tầng UI | ✅ | Logic cắt segment streaming nằm trong `core::vad_pipeline`; `lib.rs` chỉ nối callback segment-đóng tới `AppHandle::emit`, không chứa logic cắt |
| Core không phụ thuộc Tauri | ✅ | `core::vad_pipeline` chỉ nhận closure `FnMut(SegmentReady)`, không import `tauri` |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`. Quyết định chính: cần một "streaming segmenter" mới trong Core gói lại `vad::detect` + `segment::segment_with_min_silence` để chạy incremental theo từng `AudioFrame` tới (hiện `on_frame` trong `start_capture` đang bỏ qua frame — `Box::new(|_frame| {})`), vì `segment()` hiện tại chỉ chạy batch trên toàn bộ danh sách nhãn đã có sẵn.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md`
- Hợp đồng giao diện: xem `contracts/vad-segment-event.md` (payload sự kiện Tauri `vad-segment`, không có HTTP/CLI contract vì đây là sự kiện nội bộ IPC)
- Kịch bản khởi động nhanh: xem `quickstart.md`

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  vad_pipeline.rs      # MỚI: streaming segmenter — nhận AudioFrame liên tục, gọi vad::detect +
                        #   segment::segment_with_min_silence trên buffer tích lũy, phát callback
                        #   SegmentReady{start_frame_index, end_frame_index, samples, sample_rate}
                        #   khi có segment MỚI đóng (không lặp lại segment đã phát trước đó)
  mod.rs                # export VadPipeline, SegmentReady
app/src-tauri/src/lib.rs # sửa start_capture: tạo VadPipeline, on_frame đưa frame vào pipeline;
                        #   callback segment-đóng của pipeline gọi app_handle.emit("vad-segment", payload)
```

## Rủi ro & ngoại lệ có lý do

- Rủi ro: tích lũy toàn bộ `AudioFrame` cho tới khi segment đóng có thể tốn bộ nhớ nếu người nói liên tục rất lâu không ngắt — chấp nhận được cho PBI này (M3 sẽ xử lý giới hạn/streaming thật khi ghép STT); không cần tối ưu thêm ở bước này.
- Rủi ro: ngưỡng `min_silence_frames` cố định (9 frame ~300ms) đã chốt từ PBI 10, không đổi ở PBI này.
