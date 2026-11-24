# Sora RealSub

Ứng dụng desktop dịch tự động và liên tục cuộc họp, hiển thị phụ đề dịch theo thời gian thực.

## Tên app

- **Tên chốt:** Sora RealSub
- Ý nghĩa: "Sora" là thương hiệu, "RealSub" = Real(time) + Sub(title), gồm đủ 3 ý subtitle, translate, realtime.
- Gợi ý đặt tên thống nhất: `sora-realsub` (repo, gói cài đặt); nên kiểm tra tên miền trước (ví dụ realsub.app).

## Mục tiêu

Nghe âm thanh cuộc họp (người khác nói + mic của mình), dịch liên tục và hiển thị phụ đề trên cửa sổ nổi, lưu transcript sau họp.

## Luồng xử lý

1. **Lấy âm thanh**: loopback âm thanh hệ thống (Windows: WASAPI loopback; macOS: BlackHole hoặc ScreenCaptureKit) + mic.
2. **VAD**: nhận biết đoạn có tiếng nói, cắt thành đoạn ngắn (ví dụ Silero VAD).
3. **STT streaming**: chuyển giọng nói thành văn bản liên tục.
4. **Dịch**: dịch từng câu/cụm, kèm vài câu trước làm ngữ cảnh.
5. **Hiển thị**: cửa sổ phụ đề luôn nằm trên cùng; lưu transcript sau họp.

## Hai hướng triển khai

| Hướng | Công nghệ | Ưu điểm | Nhược điểm |
|---|---|---|---|
| Cloud | Azure Speech Translation, hoặc Deepgram + LLM | Độ trễ thấp (~1–2 giây), chất lượng tốt, dễ làm | Tốn phí, âm thanh gửi ra ngoài |
| Local/offline | whisper.cpp / faster-whisper + NLLB hoặc LLM nhỏ | Riêng tư, không tốn phí theo giờ | Cần GPU khá, độ trễ cao hơn |

## Framework app (ứng viên)

- Tauri hoặc Electron + Vue cho giao diện.
- Hoặc WPF/.NET + NAudio nếu chỉ chạy trên Windows.

## Câu hỏi cần chốt

1. Hệ điều hành mục tiêu: Windows hay macOS?
2. Cặp ngôn ngữ: dịch từ ngôn ngữ nào sang ngôn ngữ nào (ví dụ Anh → Việt)?
3. Cuộc họp có nhạy cảm không? Nhạy cảm thì chạy local, không thì cloud cho nhanh.

## Bước tiếp theo

- Chốt 3 câu hỏi trên.
- Lên kiến trúc chi tiết.
- Dựng prototype đầu tiên.
