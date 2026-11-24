# Khởi động nhanh: kiểm thử hợp đồng Core (PBI 3)

Mục tiêu: chứng minh 4 trait (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) dùng được bằng cài đặt giả (mock), đúng SC-001..SC-003.

## Bước 1 — Build

```powershell
cd app/src-tauri
cargo build
```

Kỳ vọng: biên dịch thành công, không lỗi (SC-001).

## Bước 2 — Chạy test cài đặt giả

```powershell
cd app/src-tauri
cargo test core::
```

Kỳ vọng: mỗi trait có ít nhất 1 test dùng mock, pass (SC-002).

## Bước 3 — Kiểm tra thay thế cài đặt không cần sửa code gọi (SC-003)

Trong test, gọi hàm nhận `impl Trait` (hoặc `Box<dyn Trait>`) qua tham số generic/trait object, truyền 2 mock khác nhau của cùng 1 trait (ví dụ `MockAudioSourceSilent` và `MockAudioSourceNoise`) — xác nhận hàm gọi không cần sửa, chỉ đổi instance truyền vào vẫn chạy đúng.
