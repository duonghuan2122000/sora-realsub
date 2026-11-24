# Quickstart: kiểm thử nhanh PBI 8

1. Build & chạy app Tauri dev: `npm run tauri dev` (trong `app/`).
2. Trong DevTools console của webview, gọi:
   ```js
   await window.__TAURI__.core.invoke('start_capture')
   ```
   → kỳ vọng không lỗi, phiên thu âm bắt đầu (nghe được qua mic + loopback nếu có sink kiểm tra).
3. Gọi lại `start_capture` lần 2 trong khi phiên vẫn chạy → kỳ vọng lỗi rõ ràng ("đã có phiên đang chạy").
4. Gọi:
   ```js
   await window.__TAURI__.core.invoke('stop_capture')
   ```
   → kỳ vọng không lỗi, phiên dừng.
5. Gọi `stop_capture` lần 2 khi không có phiên nào chạy → kỳ vọng lỗi rõ ràng ("không có phiên nào để dừng").
6. Chạy unit test Rust: `cargo test` trong `app/src-tauri/` — test leak thread/handle (mock `AudioSource`) phải pass.
