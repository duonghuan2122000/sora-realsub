# Quickstart: Auto-scroll & fade dòng phụ đề cũ (PBI 22)

## Kiểm thử tự động

```powershell
cd app
bun run test -- SubtitleApp.spec.ts
```

Kỳ vọng: pass toàn bộ, bao gồm test mới kiểm tra giữ đúng N=3 dòng gần nhất, đúng thứ tự, và dòng cũ nhất bị loại khi vượt N.

## Kiểm thử tay (tuỳ chọn, cần máy có app chạy được)

1. `bun run tauri dev` để khởi động app + cửa sổ phụ đề.
2. Kích hoạt phiên dịch (start capture) để phát sinh nhiều event `subtitle-ready` liên tiếp (hoặc giả lập bằng cách gọi lệnh test/dev phát event thủ công nếu có).
3. Quan sát cửa sổ phụ đề: dòng mới nhất rõ nét ở dưới, các dòng cũ hơn phía trên mờ dần, tối đa 3 dòng cùng lúc; khi có dòng thứ 4, dòng cũ nhất biến mất khỏi khung nhìn.
