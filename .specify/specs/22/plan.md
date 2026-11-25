# Kế hoạch triển khai: Auto-scroll & fade dòng phụ đề cũ

**Mã PBI**: 22
**Liên kết spec**: .specify/specs/22/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: TypeScript + Vue 3 (Composition API), chạy trong webview Tauri của cửa sổ phụ đề.
- Framework / Thư viện chính: không thêm thư viện mới — dùng `ref`/`reactive` của Vue, CSS transition thuần cho hiệu ứng mờ (fade).
- Lưu trữ dữ liệu: không có, chỉ là state trong bộ nhớ của component (mảng các dòng gần nhất), không persist.
- Kiểm thử: Vitest + `@vue/test-utils` (đã có sẵn trong `SubtitleApp.spec.ts`), mock `@tauri-apps/api/event` như hiện tại.
- Nền tảng triển khai: không đổi — vẫn là `app/src/subtitle/SubtitleApp.vue`, build qua Vite/Tauri như cũ.
- Ràng buộc hiệu năng: không đáng kể — cập nhật mảng tối đa N phần tử (N nhỏ, ví dụ 3) mỗi lần có event, không có vòng lặp nặng.
- Ràng buộc khác: giữ nguyên hành vi đã có ở PBI 20/21 (always-on-top, không border, không hiển thị dòng trống khi `translatedText` rỗng).

## Kiểm tra theo hiến pháp dự án

Không tìm thấy `.specify/memory/constitution.md` trong dự án — bỏ qua bước đối chiếu, chỉ áp dụng nguyên tắc chung trong `CLAUDE.md`/wiki (Core Rust tách biệt UI Vue, PBI này chỉ sửa tầng UI, không chạm Core/Rust, không vi phạm ranh giới kiến trúc).

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core/UI tách biệt (CLAUDE.md) | ✅ | Chỉ sửa `SubtitleApp.vue` (UI), không đụng Core/Rust, không thêm logic nghiệp vụ vào UI — chỉ là trình bày danh sách dòng đã nhận. |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`. Tóm tắt: quyết định giữ state là mảng các dòng phụ đề gần nhất (giới hạn N=3) trong component, dùng CSS `opacity` theo vị trí trong mảng để tạo hiệu ứng mờ, dùng CSS `transition` để cuộn/mờ mượt — không cần thư viện animation riêng.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` — một mảng `SubtitleLine[]` trong state component, không có entity lưu trữ lâu dài.
- Hợp đồng giao diện: không áp dụng — không có API/CLI công khai mới, chỉ tiêu thụ event Tauri `subtitle-ready` đã có sẵn từ PBI 19/21.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src/subtitle/
├── SubtitleApp.vue       # sửa: giữ mảng N dòng gần nhất, render list + CSS fade theo vị trí
└── SubtitleApp.spec.ts   # sửa/bổ sung: test giữ đúng N dòng, đúng thứ tự, loại dòng cũ nhất khi vượt N
```

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc dự án.
- Rủi ro nhỏ: giá trị N cố định (3) là lựa chọn mặc định hợp lý theo giả định trong spec.md, chưa cho người dùng tuỳ chỉnh (để dành M6). Nếu sau này cần đổi, chỉ cần đổi 1 hằng số trong component.
