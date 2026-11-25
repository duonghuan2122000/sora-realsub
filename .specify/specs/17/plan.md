# Kế hoạch triển khai: Dịch có ngữ cảnh câu trước

**Mã PBI**: 17
**Liên kết spec**: .specify/specs/17/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core), edition 2024
- Framework / Thư viện chính: không thêm dependency mới. Sửa logic trong `SidecarTranslator::translate()` (`core/translator.rs`) để dùng tham số `context` (hiện bị bỏ qua, `_context`); không đổi `app/translate-sidecar` (giữ nguyên CLI, không thêm flag).
- Lưu trữ dữ liệu: không — chỉ thêm 1 struct nhỏ giữ tối đa 2 câu gần nhất trong bộ nhớ (không persist), xem `data-model.md`.
- Kiểm thử: `cargo test -p app translator` cho logic ghép/tách ngữ cảnh bằng mock; test tích hợp `#[ignore]` với sidecar + model thật cho SC-001 (giống mẫu PBI 16).
- Nền tảng triển khai: desktop Windows (giống các PBI trước), không cần GPU.
- Ràng buộc hiệu năng: không yêu cầu realtime trong PBI này (nối pipeline thật là mục roadmap sau); chỉ cần hoàn tất dịch 1 câu (kèm tối đa 2 câu ngữ cảnh) trong thời gian hợp lý cho test.
- Ràng buộc khác: không đổi model/cặp ngôn ngữ (giữ Marian-MT `opus-mt-en-vi`, English → Vietnamese).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án → áp dụng nguyên tắc chung ở `CLAUDE.md`: Core (Rust) tách biệt UI (Vue), không đưa logic nghiệp vụ vào tầng UI.

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Toàn bộ thay đổi nằm trong `core/translator.rs` (Core); không đụng `app/src` (UI) |
| Giữ nguyên hợp đồng sidecar đã có (PBI 16) | ✅ | Không đổi `app/translate-sidecar/src/main.rs` — xem research.md Quyết định 1 |
| `Translator` vẫn là trait nội bộ, không lộ chi tiết triển khai ra ngoài Core | ✅ | Struct ngữ cảnh mới (`RecentTranscriptContext`) cũng đặt trong `core/`, không export qua Tauri command trong PBI này |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`: 3 quyết định — (1) ghép ngữ cảnh + câu hiện tại thành 1 input gửi sidecar hiện có, không đổi CLI sidecar; (2) tách bản dịch câu hiện tại bằng heuristic dấu kết câu, có fallback dịch lại không ngữ cảnh khi tách thất bại; (3) thêm struct nhỏ quản lý "1-2 câu gần nhất" ở mức unit, chưa nối vào pipeline thật. Không còn điểm NEEDS CLARIFICATION.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md`.
- Hợp đồng giao diện: không cần `contracts/` — không đổi CLI sidecar, không có API/endpoint công khai mới trong PBI này.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/core/
  translator.rs            (sửa SidecarTranslator::translate(): dùng context thật — ghép input,
                             gọi sidecar, tách bản dịch câu hiện tại, fallback khi tách thất bại;
                             + struct RecentTranscriptContext mới, test riêng)
```

## Rủi ro & ngoại lệ có lý do

- Heuristic tách câu theo dấu kết câu (`.`, `!`, `?`) không chính xác 100% với câu viết tắt (ví dụ "Mr.", số thập phân) hoặc câu không có dấu kết thúc rõ ràng → đã có phương án dự phòng (research.md Quyết định 2: dịch lại không ngữ cảnh khi tách thất bại) để không vi phạm FR-002/SC-002, đổi lại một số trường hợp mất lợi ích ngữ cảnh. Ghi nhận là hạn chế biết trước, không phải lỗi cần chặn PBI.
- Việc nối `RecentTranscriptContext` + `Translator` (có ngữ cảnh) vào `TranscriptPipeline`/`lib.rs` thật (event `subtitle-ready`) không thuộc PBI này — rủi ro là PBI sau (M4 mục 3) cần tái sử dụng đúng API của struct này; giảm thiểu bằng cách giữ API struct đơn giản (`push`, `recent(n)`).
- Test tích hợp SC-001 cần sidecar + model thật cục bộ (không commit vào repo), giống hạn chế đã biết từ PBI 16 — không chạy được trên CI nếu chưa có sẵn.
