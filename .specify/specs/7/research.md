# Research: Trộn mic + loopback thành 1 stream

## Quyết định: Cách trộn (mix) 2 luồng PCM

**Quyết định**: Cộng biên độ từng mẫu (sample-by-sample additive mixing) sau khi quy 2 nguồn về cùng sample rate, rồi clamp kết quả về [-1.0, 1.0].

**Lý do**: Đây là cách trộn audio tiêu chuẩn, đơn giản, đủ cho yêu cầu "nghe được cả 2 nguồn" (FR-002) mà không cần xử lý tần số/pha phức tạp. Phù hợp vì cả mic và loopback đều đã là PCM mono f32 (theo cài đặt hiện có của `MicAudioSource`/`LoopbackAudioSource`).

**Phương án khác đã xem xét**:
- Trộn theo tỉ lệ trung bình (average) thay vì cộng: làm giảm âm lượng cả 2 nguồn khi chỉ 1 nguồn có tiếng, không đạt SC-002/SC-003 (nội dung 1 nguồn phải khớp với thu riêng). Loại bỏ.
- Dùng thư viện xử lý audio chuyên dụng (ví dụ `dasp`): thêm dependency không cần thiết cho phép cộng + clamp đơn giản. Loại bỏ theo tinh thần "không thêm crate khi vài dòng code đủ".

## Quyết định: Đồng bộ 2 luồng không cùng tốc độ sinh mẫu

**Quyết định**: Mỗi nguồn con (mic, loopback) chạy trên thread riêng của chính nó (như hiện có), đẩy `AudioFrame` vào 1 buffer riêng (VecDeque<f32>) có khoá (Mutex) cho nguồn đó. Một thread trộn riêng, chạy theo chu kỳ cố định (ví dụ mỗi 10-20ms), lấy ra số mẫu tương ứng từ 2 buffer (nếu buffer thiếu mẫu thì coi là im lặng — fill 0.0), cộng + clamp, rồi gọi `on_frame` ra ngoài.

**Lý do**: `MicAudioSource`/`LoopbackAudioSource` đã tự chạy thread nội bộ và gọi callback bất đồng bộ, không có cách nào bắt chúng đồng bộ hoá thời điểm sinh mẫu. Dùng buffer trung gian + đọc theo chu kỳ là cách đơn giản nhất để ghép 2 nguồn không đồng bộ mà không sửa 2 class nguồn đã có (test pass, không muốn đụng lại).

**Phương án khác đã xem xét**:
- Gọi `start()` của 1 nguồn trong callback của nguồn khác: tạo phụ thuộc chặt, phức tạp hoá lifetime, không cần thiết. Loại bỏ.
- Dùng channel (mpsc) thay buffer: cũng được, nhưng buffer dạng VecDeque đơn giản hơn để "lấy đúng N mẫu mỗi chu kỳ", chọn buffer.

## Quyết định: Resample khi sample rate khác nhau

**Quyết định**: Nếu 2 nguồn có sample rate khác nhau, resample luồng có rate thấp hơn lên rate cao hơn bằng linear interpolation đơn giản, trước khi đưa vào buffer trộn.

**Lý do**: Đủ để tránh lỗi/crash và giữ nội dung nghe được (SC-004), không cần resampler chất lượng cao (ví dụ sinc) cho mục tiêu thử nghiệm ở bước này — đúng giả định đã ghi trong spec.md.

**Phương án khác đã xem xét**:
- Bỏ qua resample, ép cộng mẫu bất kể rate: sai về mặt kỹ thuật (lệch thời gian, tiếng bị méo nhanh/chậm). Loại bỏ.
- Dùng crate resample chuyên dụng (`rubato`): over-engineering cho yêu cầu hiện tại. Loại bỏ.
