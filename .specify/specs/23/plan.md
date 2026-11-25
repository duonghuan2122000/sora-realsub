# Kế hoạch triển khai: Kéo-thả cửa sổ phụ đề

**Mã PBI**: 23
**Liên kết spec**: .specify/specs/23/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: TypeScript (Vue 3) cho UI phụ đề, không cần đổi Rust/Tauri core.
- Framework / Thư viện chính: Tauri 2 (thuộc tính webview gốc `data-tauri-drag-region`), Vue 3 SFC.
- Lưu trữ dữ liệu: không có (vị trí chỉ giữ trong runtime của webview, không persist).
- Kiểm thử: Vitest + @vue/test-utils (test template có attribute kéo-thả); `bun run tauri dev` kiểm thử tay theo `quickstart.md`.
- Nền tảng triển khai: Windows (và Linux, dùng chung code UI — tính năng không phụ thuộc OS).
- Ràng buộc hiệu năng: không có ràng buộc đặc biệt, kéo-thả do webview/OS xử lý native.
- Ràng buộc khác: không phá vỡ `decorations(false)`/`always_on_top`/`transparent` đã có từ PBI 20; không ảnh hưởng render dòng phụ đề (PBI 21, 22).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này.

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`: chọn `data-tauri-drag-region` (thuộc tính webview gốc của Tauri) thay vì tự viết JS/Rust xử lý kéo-thả. Không còn điểm NEEDS CLARIFICATION.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không áp dụng (tính năng không có entity/dữ liệu mới).
- Hợp đồng giao diện: không áp dụng (không có API/IPC command mới — chỉ thay đổi UI tĩnh).
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src/subtitle/SubtitleApp.vue      # thêm attribute data-tauri-drag-region vào .subtitle-overlay
app/src/subtitle/SubtitleApp.spec.ts  # thêm test: template chứa data-tauri-drag-region
```

Không chỉnh `app/src-tauri/src/lib.rs` — `create_subtitle_window` (PBI 20) giữ nguyên.

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc nào (không có constitution.md để đối chiếu).
- Rủi ro nhỏ: `data-tauri-drag-region` có thể nuốt sự kiện click/hover tương lai nếu thêm nút điều khiển vào cửa sổ phụ đề (M5 các mục sau) — khi đó cần loại trừ vùng nút khỏi attribute kéo-thả; ngoài phạm vi PBI này.
