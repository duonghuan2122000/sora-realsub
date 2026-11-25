# Quickstart: Hiển thị phụ đề (PBI 21)

Kiểm thử thủ công nhanh sau khi implement:

1. Chạy `bun run tauri dev` (hoặc `npm run tauri dev`) từ `app/`.
2. Mở cửa sổ phụ đề (đã tạo sẵn từ PBI 20).
3. Bắt đầu 1 phiên capture + VAD + STT + dịch (`start_capture`) và nói vài câu trước mic.
4. Xác nhận: mỗi khi có 1 câu transcript final được dịch xong (event `subtitle-ready`), text dịch xuất hiện ngay trên cửa sổ phụ đề, thay thế dòng cũ.
5. Nói liên tục nhiều câu ngắn để kiểm tra event dồn dập — xác nhận nội dung hiển thị đúng thứ tự, không bị đảo/mất dòng nào.

Kiểm thử tự động (unit test, chạy bằng `npm run test` / `vitest`):

- Mock `@tauri-apps/api/event` để giả lập phát nhiều event `subtitle-ready` liên tiếp vào component `SubtitleApp.vue`.
- Assert nội dung hiển thị (`translatedText`) cập nhật đúng theo từng event, đúng thứ tự.
