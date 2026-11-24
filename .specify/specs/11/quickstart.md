# Quickstart: kiểm thử nhanh PBI 11 — sự kiện `vad-segment`

1. Chạy app ở chế độ dev: `pnpm tauri dev` (hoặc lệnh dev hiện có của dự án, xem wiki phần lệnh dev) trong `app/`.
2. Mở DevTools của cửa sổ UI (hoặc thêm tạm 1 listener debug trong UI):
   ```ts
   import { listen } from "@tauri-apps/api/event";
   listen("vad-segment", (event) => console.log("vad-segment", event.payload));
   ```
3. Gọi lệnh `start_capture` (qua UI hiện có hoặc gọi trực tiếp `invoke("start_capture")` trong console DevTools).
4. Nói vài câu, dừng lại vài giây, rồi nói tiếp — lặp lại 2-3 lần.
5. Quan sát console: mỗi lần dừng nói đủ lâu (khoảng lặng > ~300ms theo ngưỡng đã chốt ở PBI 10) phải xuất hiện đúng 1 log `vad-segment` mới, theo đúng thứ tự thời gian, không log trùng.
6. Gọi `stop_capture`, xác nhận không còn log `vad-segment` nào xuất hiện thêm sau đó.
7. Chụp ảnh console log làm bằng chứng test (theo yêu cầu của `/sora-implement`).
