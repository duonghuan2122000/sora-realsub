# Kế hoạch triển khai: Hợp đồng giao diện Core

**Mã PBI**: 3
**Liên kết spec**: .specify/specs/3/spec.md
**Ngày tạo**: 2026-11-24

## Ngữ cảnh kỹ thuật

| Hạng mục | Giá trị |
|---|---|
| Ngôn ngữ / Runtime | Rust (edition 2024), chạy trong Tauri 2 (tokio runtime có sẵn) |
| Framework / Thư viện chính | `async-trait`, `thiserror` (mới thêm); không thêm dependency ngoài 2 cái này |
| Lưu trữ dữ liệu | Không — chỉ kiểu dữ liệu trong bộ nhớ |
| Kiểm thử | `cargo test`, test với cài đặt giả (mock) ngay trong module `core` |
| Nền tảng triển khai | Không áp dụng (chưa đóng gói ở PBI này) |
| Ràng buộc hiệu năng | Không áp dụng — chưa có cài đặt thật |
| Ràng buộc khác | Core phải tách biệt UI (CLAUDE.md); hướng xử lý đã chốt local (roadmap M0), hợp đồng không được rò rỉ chi tiết provider cụ thể |

## Kiểm tra theo hiến pháp dự án (trước thiết kế)

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này, chỉ đối chiếu với CLAUDE.md:

| Nguyên tắc (CLAUDE.md) | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt, không logic nghiệp vụ ở UI | ✅ | Toàn bộ trait định nghĩa trong `app/src-tauri/src/core/`, không đụng tới UI Vue |
| Core = Rust/Tauri | ✅ | Dùng crate lib hiện có `app_lib` |

## Giai đoạn 0 — Nghiên cứu

Xem `research.md`. Tóm tắt quyết định chính:

- **Quyết định**: 4 trait đặt trong module `core` của crate `app_lib` hiện có, không tách crate riêng. **Lý do**: dự án còn skeleton, tách crate lúc này là over-engineering. **Phương án khác**: workspace nhiều crate — loại vì chưa cần.
- **Quyết định**: `AudioSource` dùng callback đồng bộ; `SpeechToText`/`Translator` dùng `async fn` (qua `async_trait`); `SubtitleSink` đồng bộ. **Lý do**: khớp bản chất I/O của từng tầng (capture theo callback thread riêng, STT/dịch là I/O chờ kết quả, sink là tác vụ nhanh). **Phương án khác**: toàn bộ async hoặc toàn bộ sync — loại vì không khớp thực tế hoặc thêm phức tạp không cần.
- **Quyết định**: Một `CoreError` chung (3 biến thể: `Io`, `Provider`, `Cancelled`) cho cả 4 trait. **Lý do**: đủ dùng ở giai đoạn interface-only, tránh 4 kiểu lỗi riêng chưa có lý do nghiệp vụ. **Phương án khác**: lỗi riêng từng trait — loại, chưa cần.

## Giai đoạn 1 — Thiết kế

- **Mô hình dữ liệu**: xem `data-model.md` — `CoreError`, `AudioFrame`, `StreamHandle`, `TranscriptChunk`, `Subtitle`.
- **Hợp đồng giao diện**: không có `contracts/` — PBI này không có API/CLI hướng ra ngoài hệ thống, chỉ là trait nội bộ Rust.
- **Kịch bản khởi động nhanh**: xem `quickstart.md` — build, `cargo test core::`, kiểm tra thay thế mock không cần sửa code gọi.

## Kiểm tra theo hiến pháp dự án (sau thiết kế)

| Nguyên tắc (CLAUDE.md) | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt | ✅ | Không file nào trong thiết kế thuộc UI |
| Không cài đặt logic thật ở PBI này (FR-007) | ✅ | Thiết kế chỉ gồm trait + struct dữ liệu + mock test, không gọi API/model thật |

## Cấu trúc dự án dự kiến

```text
app/src-tauri/src/
  core/
    mod.rs              # khai báo module, re-export trait + kiểu dữ liệu
    error.rs            # CoreError
    types.rs            # AudioFrame, StreamHandle, TranscriptChunk, Subtitle
    audio_source.rs      # trait AudioSource + test với mock
    speech_to_text.rs    # trait SpeechToText + test với mock
    translator.rs        # trait Translator + test với mock
    subtitle_sink.rs      # trait SubtitleSink + test với mock
  lib.rs                 # thêm `mod core;`
app/src-tauri/Cargo.toml  # thêm dependency async-trait, thiserror (+ dev dep nếu cần cho test)
```

## Rủi ro & ngoại lệ có lý do

- Không có rủi ro/ngoại lệ đáng kể — PBI phạm vi nhỏ, chỉ định nghĩa interface và mock test, không đụng tới pipeline thật.
