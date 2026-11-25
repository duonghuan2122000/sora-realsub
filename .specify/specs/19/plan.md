# Kế hoạch triển khai: Nối STT final → Dịch → subtitle-ready

**Mã PBI**: 19
**Liên kết spec**: .specify/specs/19/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core), edition 2024
- Framework / Thư viện chính: không thêm dependency mới. Sửa `start_capture` (`app/src-tauri/src/lib.rs`) để nối `Translator`/`SidecarTranslator` (PBI 16/17) + `RecentTranscriptContext` (PBI 17) vào callback `on_chunk` của `TranscriptPipeline`; thêm payload `SubtitleReadyEvent` và biến thể `ModelErrorKind::Translate`.
- Lưu trữ dữ liệu: không — chỉ dùng lại `RecentTranscriptContext` trong bộ nhớ phiên (không persist), xem `data-model.md`.
- Kiểm thử: `cargo test -p app` cho logic nối pipeline bằng mock STT + mock Translator (không cần model/sidecar thật); test tích hợp `#[ignore]` với STT + sidecar dịch thật cho SC-002 (giống mẫu PBI 14/16/17).
- Nền tảng triển khai: desktop Windows/Linux (giống các PBI trước), không cần GPU.
- Ràng buộc hiệu năng: không yêu cầu benchmark realtime chính thức trong PBI này; chỉ cần mỗi câu final có `subtitle-ready` phát ra trong thời gian hợp lý để test tự động không timeout.
- Ràng buộc khác: không đổi hành vi `transcript-final`/`transcript-partial`/`vad-segment` đã có; không đổi model/cặp ngôn ngữ dịch (English → Vietnamese, Marian-MT).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án → áp dụng nguyên tắc chung ở `CLAUDE.md`: Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào tầng UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Toàn bộ thay đổi nằm trong `app/src-tauri/src` (Core); không đụng `app/src` (UI Vue) |
| Không đổi hợp đồng hiện có (`transcript-final`, `vad-segment`, PBI 14/15/11) | ✅ | Chỉ thêm event mới (`subtitle-ready`) + biến thể lỗi mới (`translate`), không sửa payload/behavior các event cũ |
| `TranscriptPipeline` giữ đúng 1 trách nhiệm (ghép VAD→STT) | ✅ | Bước dịch nối ở `lib.rs` (orchestration), không sửa `transcript_pipeline.rs` — xem research.md Quyết định 1 |
| Model/dịch vụ ngoài là optional, phiên vẫn chạy khi thiếu (đã chốt từ PBI 14) | ✅ | `translator` là `Option`, bỏ qua bước dịch khi thiếu — FR-008, research.md Quyết định 5 |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`: 5 quyết định — (1) nối luồng trong closure `start_capture` của `lib.rs`, không sửa `TranscriptPipeline`; (2) tái dùng `RecentTranscriptContext` (PBI 17) trong vòng đời phiên; (3) push câu gốc vào context sau khi xử lý (không tự làm ngữ cảnh cho chính nó); (4) lỗi dịch 1 câu báo qua `model-error` kiểu `translate` mới, không dừng phiên; (5) translator là optional giống STT, bỏ qua dịch nếu thiếu cấu hình. Không còn điểm NEEDS CLARIFICATION.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` (payload `subtitle-ready`, biến thể `ModelErrorKind::Translate`, luồng xử lý 1 câu final).
- Hợp đồng giao diện: không cần `contracts/` — `subtitle-ready` là event Tauri nội bộ Core↔UI, không phải API/CLI công khai mới ra ngoài hệ thống.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/
  lib.rs                      (sửa: thêm SubtitlePayload, mở rộng ModelErrorKind/ModelErrorPayload
                                với biến thể "translate"; trong start_capture, tạo translator
                                (Option<SidecarTranslator> từ env) + RecentTranscriptContext,
                                nối vào callback on_chunk của TranscriptPipeline)
  core/
    error.rs hoặc core/mod.rs  (nếu ModelErrorKind định nghĩa ở đây: thêm biến thể Translate)
```

(Không sửa `core/transcript_pipeline.rs`, `core/translator.rs` — tái dùng nguyên trạng từ PBI 13-17.)

## Rủi ro & ngoại lệ có lý do

- Việc tạo `translator`/`RecentTranscriptContext` mới mỗi lần `start_capture` nghĩa là ngữ cảnh dịch bị reset khi dừng/chạy lại phiên — đây là hành vi mong đợi (ngữ cảnh chỉ trong 1 phiên, đúng giả định ở spec.md), không phải rủi ro cần xử lý thêm.
- Test tích hợp SC-002 cần cả model STT và sidecar dịch thật cục bộ (không commit vào repo), kế thừa hạn chế đã biết từ PBI 12/16/17 — không chạy được trên CI nếu chưa có sẵn; test logic (mock) vẫn đủ để xác nhận FR-001 đến FR-008 và SC-001/SC-003.
- Thêm biến thể enum `ModelErrorKind::Translate` là thay đổi public API của `core` — rà soát mọi nơi `match` trên `ModelErrorKind` (hiện chỉ có `lib.rs::ModelErrorPayload::from`) để đảm bảo không thiếu nhánh khi build.
