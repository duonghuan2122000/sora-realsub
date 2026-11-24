# Đặc tả tính năng: Trộn mic + loopback thành 1 stream

**Mã PBI**: 7
**Ngày tạo**: 2026-11-24
**Trạng thái**: Nháp

## Mô tả tổng quan

Hệ thống đã thu được riêng mic (PBI 5) và riêng loopback hệ thống (PBI 6) thành hai luồng PCM độc lập. Tính năng này trộn (mix) hai luồng đó thành một luồng âm thanh duy nhất, để các bước xử lý sau (VAD, STT) chỉ cần làm việc với một nguồn audio tổng hợp bao gồm cả tiếng người dùng nói và tiếng phát ra từ loa.

## Kịch bản & luồng người dùng

### Luồng chính

**Given** mic và loopback đều đang được capture đồng thời, **When** hệ thống trộn hai luồng PCM thành một, **Then** luồng kết quả chứa cả nội dung âm thanh từ mic và từ loopback, nghe được cả hai khi phát lại.

### Kịch bản chấp nhận

1. **Given** mic đang thu tiếng nói và loopback đang thu một đoạn nhạc/âm thanh phát ra loa cùng lúc, **When** trộn hai luồng và ghi ra file .wav, **Then** nghe lại file đó nhận ra được cả tiếng nói từ mic và âm thanh từ loopback, không nguồn nào bị mất hoàn toàn.
2. **Given** chỉ mic có âm thanh, loopback im lặng, **When** trộn hai luồng, **Then** output nghe được đúng nội dung mic, không bị câm hay lỗi do nguồn còn lại im lặng.
3. **Given** chỉ loopback có âm thanh, mic im lặng, **When** trộn hai luồng, **Then** output nghe được đúng nội dung loopback.
4. **Given** đang trộn hai luồng, **When** dừng capture, **Then** việc trộn dừng ngay, không còn khối mẫu mới nào được tạo ra sau đó.

### Trường hợp biên

- Mic và loopback có sample rate hoặc số kênh khác nhau → hệ thống phải xử lý để trộn ra một định dạng chung hợp lệ, không crash.
- Một trong hai nguồn dừng đột ngột (ví dụ rút mic) trong khi đang trộn → hệ thống phải báo lỗi rõ ràng hoặc tiếp tục với nguồn còn lại, không crash.
- Cả hai nguồn cùng im lặng → output là im lặng, không lỗi.
- Âm lượng tổng sau khi trộn vượt ngưỡng (clipping) khi cả hai nguồn đều lớn → không được gây méo nghiêm trọng tới mức không nghe được nội dung.

## Yêu cầu chức năng

- **FR-001**: Hệ thống PHẢI trộn luồng PCM từ mic và luồng PCM từ loopback thành một luồng PCM duy nhất, hoạt động liên tục trong khi cả hai đang capture.
- **FR-002**: Luồng đã trộn PHẢI giữ được nội dung nghe được từ cả hai nguồn khi cả hai cùng có âm thanh.
- **FR-003**: Hệ thống PHẢI xử lý được trường hợp định dạng (sample rate/số kênh) giữa hai nguồn không giống nhau, quy về một định dạng chung hợp lệ cho luồng kết quả.
- **FR-004**: Hệ thống PHẢI cho phép dừng việc trộn theo yêu cầu (khi dừng capture), và việc tạo mẫu mới PHẢI chấm dứt ngay sau đó.
- **FR-005**: Hệ thống PHẢI tránh méo tiếng (clipping) nghiêm trọng làm mất khả năng nghe hiểu nội dung khi trộn hai nguồn cùng có âm lượng lớn.

## Tiêu chí thành công

- **SC-001**: Phát thử mic và loopback cùng lúc trong 5-10 giây, nghe lại file .wav trộn ra nhận biết được rõ cả hai nguồn, không nguồn nào bị mất hoàn toàn.
- **SC-002**: Khi chỉ một nguồn có âm thanh, nội dung nghe được ở output khớp đúng với nguồn đó, không bị suy giảm rõ rệt so với thu riêng.
- **SC-003**: Dừng capture luôn dừng việc trộn trong vòng dưới 1 giây.
- **SC-004**: 100% các lần thử với mic/loopback có sample rate hoặc số kênh khác nhau đều ra được output hợp lệ, không crash.

## Giả định

- Mic (PBI 5) và loopback (PBI 6) đã có sẵn dưới dạng hai nguồn PCM độc lập, có thể bắt đầu/dừng riêng.
- Trộn bằng phép cộng biên độ (sample-by-sample mixing) sau khi quy về cùng sample rate/số kênh, có giới hạn (clamp) để chống clipping — đây là kỹ thuật trộn audio thông dụng nhất, không cần thuật toán phức tạp hơn ở bước này.
- Việc ghi ra .wav chỉ dùng để kiểm thử/làm bằng chứng, không phải tính năng cuối cho người dùng.
- Cân bằng âm lượng (gain) giữa hai nguồn dùng tỉ lệ mặc định 1:1, chưa cho người dùng tuỳ chỉnh ở bước này.

## Ngoài phạm vi

- Cho người dùng tuỳ chỉnh tỉ lệ âm lượng (gain) giữa mic và loopback.
- Command Tauri `start_capture`/`stop_capture` thống nhất — thuộc PBI tiếp theo trong M1.
- Xử lý VAD, STT, hoặc bất kỳ xử lý ngữ nghĩa nào trên luồng đã trộn.
- Hỗ trợ macOS/Linux.
