# Kế hoạch triển khai: Chỉnh cỡ chữ phụ đề

**Mã PBI**: 24
**Liên kết spec**: .specify/specs/24/spec.md
**Ngày tạo**: 2026-11-25

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: TypeScript (Vue 3) cho UI phụ đề; không cần đổi Rust/Tauri core.
- Framework / Thư viện chính: Vue 3 SFC, `localStorage` webview (API trình duyệt gốc, không thêm dependency).
- Lưu trữ dữ liệu: `localStorage` của webview `subtitle.html` (1 key, giá trị cỡ chữ theo rem/px) — không dùng file config Rust (xem `research.md`).
- Kiểm thử: Vitest + @vue/test-utils (test tăng/giảm cỡ chữ, test clamp min/max, test đọc giá trị đã lưu khi mount); `bun run tauri dev` kiểm thử tay theo `quickstart.md`.
- Nền tảng triển khai: Windows + Linux (UI dùng chung, không phụ thuộc OS).
- Ràng buộc hiệu năng: không có ràng buộc đặc biệt.
- Ràng buộc khác: không phá vỡ `data-tauri-drag-region` (PBI 23) — nút +/- phải nằm ngoài vùng kéo-thả hoặc dùng cơ chế loại trừ event kéo; không ảnh hưởng auto-scroll/fade (PBI 22).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — bỏ qua bước này.

## Giai đoạn 0 — Kết quả nghiên cứu

Xem `research.md`: chọn `localStorage` (không file config Rust, không thêm plugin) + nút +/- trên cửa sổ phụ đề (không global shortcut). Không còn điểm NEEDS CLARIFICATION.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: không áp dụng (chỉ 1 giá trị số nguyên — cỡ chữ — lưu trong `localStorage`, không phải entity nghiệp vụ).
- Hợp đồng giao diện: không áp dụng (không có Tauri command/IPC mới, không có API bên ngoài).
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src/subtitle/SubtitleApp.vue      # thêm state fontSize (ref), nút +/-, đọc/ghi localStorage, bind font-size vào .subtitle-text qua CSS variable
app/src/subtitle/SubtitleApp.spec.ts  # thêm test: tăng/giảm cỡ chữ, clamp min/max, khôi phục từ localStorage khi mount
```

Không chỉnh `app/src-tauri/src/lib.rs` hay bất kỳ file Rust nào — tính năng hoàn toàn ở tầng UI.

## Rủi ro & ngoại lệ có lý do

- Không có ngoại lệ vi phạm nguyên tắc nào (không có constitution.md để đối chiếu).
- Rủi ro nhỏ: nút +/- đặt đè lên `data-tauri-drag-region` có thể bị nuốt click nếu không khai báo trừ vùng đúng cách — cần kiểm thử tay (`quickstart.md` bước 3-4) để xác nhận click hoạt động, không chỉ test đơn vị.
- Rủi ro nhỏ: `localStorage` của webview phụ thuộc việc Tauri giữ cùng app data dir giữa các lần chạy (mặc định có, nhưng cần xác nhận tay ở bước 5 `quickstart.md` thay vì chỉ tin theo tài liệu).
