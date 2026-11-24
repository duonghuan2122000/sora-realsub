# Nghiên cứu: Hợp đồng giao diện Core (PBI 3)

## Quyết định 1: Hợp đồng dạng `trait` Rust, đặt trong crate lib hiện có (`app_lib`)

- **Quyết định**: Định nghĩa 4 trait trong module mới `app/src-tauri/src/core/mod.rs` (và các file con `audio_source.rs`, `speech_to_text.rs`, `translator.rs`, `subtitle_sink.rs`), không tạo crate riêng.
- **Lý do**: Dự án còn ở giai đoạn skeleton, chỉ có 1 crate `app_lib`. Tách crate riêng lúc này là over-engineering (YAGNI) — tách sau nếu thật sự cần tái sử dụng ở nhiều binary.
- **Phương án khác đã xem xét**: Tạo workspace nhiều crate (`core`, `app`) ngay từ đầu — bị loại vì chưa có nhu cầu thực tế, tăng độ phức tạp build không cần thiết ở M0.

## Quyết định 2: Audio streaming qua trait với callback/channel, không dùng `async_trait`

- **Quyết định**: `AudioSource` dùng phương thức đồng bộ `start(&self, on_frame: impl FnMut(&[f32]) + Send + 'static) -> Result<StreamHandle, CoreError>` kiểu callback, thay vì async stream.
- **Lý do**: Capture audio (M1) thường chạy trên thread riêng do thư viện native (cpal/WASAPI) điều khiển vòng lặp, không tự nhiên là async. Dùng callback đơn giản hơn, không cần thêm dependency `async-trait` hay runtime async cho tầng này.
- **Phương án khác đã xem xét**: `async_trait` trả `Stream<Item = Vec<f32>>` — bị loại vì thêm dependency (`async-trait`, `futures`) và độ phức tạp runtime không cần thiết khi chưa có cài đặt thật nào yêu cầu async ở tầng capture.

## Quyết định 3: `SpeechToText` và `Translator` dùng `async_trait` vì bản chất I/O chờ kết quả

- **Quyết định**: Hai trait này định nghĩa phương thức `async fn` (qua macro `async_trait`), trả `Result<T, CoreError>`.
- **Lý do**: Cả cloud (gọi API) và local (whisper.cpp/NLLB chạy nền) đều là tác vụ chờ kết quả, phù hợp mô hình async/await đã có sẵn trong Tauri (tokio runtime). Thêm 1 dependency nhỏ (`async-trait`) đổi lại code gọi tự nhiên hơn nhiều so với tự viết `Future` tay — hợp lý.
- **Phương án khác đã xem xét**: Trả `Result` đồng bộ (blocking) — bị loại vì sẽ chặn thread Tauri khi gọi cloud API hoặc model local chạy lâu.

## Quyết định 4: `SubtitleSink` đồng bộ, không async

- **Quyết định**: `fn push(&self, subtitle: Subtitle) -> Result<(), CoreError>`.
- **Lý do**: Sink điển hình (gửi event Tauri, ghi buffer bộ nhớ) là tác vụ nhanh, không cần async. Giữ đơn giản nhất có thể (ponytail: 1 dòng thắng hơn trait async không cần thiết).
- **Phương án khác đã xem xét**: Async `push` — bị loại vì chưa có cài đặt nào (event Tauri, ghi file cuối session ở M7) thực sự cần chờ I/O lâu ở bước này.

## Quyết định 5: Kiểu lỗi chung `CoreError`

- **Quyết định**: Một enum lỗi duy nhất `CoreError` (dùng `thiserror`) dùng chung cho cả 4 trait, với các biến thể tối thiểu: `Io(String)`, `Provider(String)`, `Cancelled`.
- **Lý do**: FR-006 yêu cầu mỗi hợp đồng mô tả được đường lỗi. Một enum chung đơn giản hơn 4 kiểu lỗi riêng, đủ dùng ở giai đoạn chỉ có cài đặt giả; mở rộng biến thể khi có cài đặt thật cụ thể cần.
- **Phương án khác đã xem xét**: Mỗi trait có kiểu lỗi riêng (`AudioError`, `SttError`...) — bị loại, chưa có lý do nghiệp vụ nào đòi hỏi phân biệt lỗi theo từng hợp đồng ở giai đoạn interface-only này.

## Quyết định 6: Thêm dependency `async-trait` và `thiserror`

- **Quyết định**: Thêm 2 dependency nhỏ, ổn định, phổ biến vào `Cargo.toml`.
- **Lý do**: Cả hai là thư viện chuẩn de-facto cho pattern này trong ecosystem Rust, tránh phải tự viết boilerplate (`Pin<Box<dyn Future>>`, `impl std::error::Error` tay).
- **Phương án khác đã xem xét**: Không dùng `async-trait`, viết trait trả `Pin<Box<dyn Future<Output = ...> + Send>>` tay — bị loại vì rườm rà hơn, không có lợi ích thêm ở giai đoạn này.
