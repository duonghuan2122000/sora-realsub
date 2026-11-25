# Kế hoạch triển khai: Hiển thị phụ đề

**Mã PBI**: 21
**Liên kết spec**: .specify/specs/21/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: TypeScript (Vue 3 `<script setup>`), chạy trong Tauri WebView.
- Framework / Thư viện chính: Vue 3, `@tauri-apps/api/event` (`listen`) để nhận event `subtitle-ready` đã có sẵn từ Core (PBI 19, payload `SubtitlePayload` trong `app/src-tauri/src/lib.rs`, camelCase: `startFrameIndex`, `endFrameIndex`, `sourceText`, `translatedText`).
- Lưu trữ dữ liệu: không cần lưu trữ bền vững, chỉ state trong bộ nhớ (reactive `ref`) của component `SubtitleApp.vue`.
- Kiểm thử: unit test component bằng Vitest + `@vue/test-utils` (đã có sẵn trong `devDependencies`), giả lập event `subtitle-ready` qua mock `@tauri-apps/api/event`.
- Nền tảng triển khai: không đổi — vẫn cửa sổ Tauri `subtitle` đã tạo ở PBI 20.
- Ràng buộc hiệu năng: cập nhật UI trong < 1s sau khi nhận event (SC-001); không rớt/đảo thứ tự khi nhận dồn dập (SC-002).
- Ràng buộc khác: không đổi cấu trúc event/payload phía Core (đã có sẵn, không cần sửa Rust).

## Kiểm tra theo hiến pháp dự án

Không tìm thấy `.specify/memory/constitution.md` trong dự án — bỏ qua bước đối chiếu hiến pháp.

| Nguyên tắc (từ CLAUDE.md) | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) và UI (Vue) tách biệt, không đưa logic nghiệp vụ vào UI | ✅ | Tính năng chỉ hiển thị dữ liệu Core đã xử lý xong (`SubtitlePayload`), UI không tự suy luận/dịch gì thêm |

## Giai đoạn 0 — Kết quả nghiên cứu

Không có điểm `NEEDS CLARIFICATION`. Payload event đã tồn tại sẵn từ PBI 19/20, không cần nghiên cứu thêm công nghệ mới (dùng lại `@tauri-apps/api/event` đã có trong dự án). Xem `research.md`.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` — tái sử dụng `SubtitlePayload` đã có, không tạo entity Core mới.
- Hợp đồng giao diện: không có hợp đồng mới (event `subtitle-ready` là hợp đồng nội bộ đã tồn tại, không phải API công khai) — bỏ qua `contracts/`.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src/subtitle/SubtitleApp.vue   # sửa: thêm listen('subtitle-ready', ...) + hiển thị translatedText
app/src/subtitle/SubtitleApp.spec.ts (mới)  # unit test: mock event, assert nội dung + thứ tự hiển thị
```

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc dự án.
