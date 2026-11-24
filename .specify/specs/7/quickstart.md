# Quickstart: Trộn mic + loopback thành 1 stream

## Mục tiêu

Kiểm tra bằng tai rằng luồng đã trộn chứa cả nội dung từ mic và từ loopback.

## Bước thực hiện

1. Build & chạy bin thực nghiệm:
   ```
   cargo run --bin capture_mixed_to_wav
   ```
   (chạy trong `app/src-tauri/`)
2. Trong lúc bin đang chạy (vài giây):
   - Nói vào mic.
   - Đồng thời phát một đoạn nhạc/âm thanh qua loa (thiết bị output mặc định).
3. Dừng bin (Ctrl+C hoặc tự dừng theo timeout cài trong bin), xem file `.wav` được ghi ra.
4. Nghe lại file `.wav`:
   - Phải nghe được cả tiếng nói (từ mic) và đoạn nhạc (từ loopback) cùng lúc.
   - Thử lại với chỉ 1 nguồn có âm thanh (chỉ nói, không phát nhạc / chỉ phát nhạc, không nói) — output phải khớp đúng nguồn đang có âm thanh.

## Kết quả mong đợi

- File `.wav` nghe được cả 2 nguồn khi cả 2 cùng phát.
- Không bị câm, không bị mất hẳn 1 nguồn, không bị méo tới mức không nghe được nội dung.
