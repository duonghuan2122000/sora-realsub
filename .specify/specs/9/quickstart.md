# Quickstart: Kiểm thử nhanh VAD (PBI 9)

1. Chuẩn bị file audio mẫu WAV 16kHz mono có đoạn nói + đoạn lặng xen kẽ (có thể lấy từ test fixture của PBI 5/6, hoặc ghi mới).
2. Chạy test tích hợp (`cargo test vad_` trong `app/src-tauri`) để:
   - Nạp file mẫu thành buffer `AudioFrame`.
   - Gọi hàm VAD chia frame + gán nhãn.
   - So khớp nhãn trả về với các mốc thời gian nói/lặng đã biết trước trong file mẫu.
3. Kỳ vọng: nhãn "speech" chiếm ≥90% số frame trong đoạn nói đã biết, nhãn "silence" chiếm ≥90% số frame trong đoạn lặng đã biết, thứ tự nhãn khớp thứ tự thời gian gốc.
