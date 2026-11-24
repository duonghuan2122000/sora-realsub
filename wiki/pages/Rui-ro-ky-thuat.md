---
title: Rủi ro kỹ thuật
date: 2026-11-24
tags: [decision, risk]
sources: [docs/tech-stack.md]
---

# Rủi ro kỹ thuật

- **Độ trễ end-to-end**: mục tiêu dưới 2 giây ở chế độ cloud.
- **Thời gian khởi động**: mục tiêu dưới 1 giây — cần lazy-load model/engine nặng sau khi UI hiện.
- **Chi phí GPU & dung lượng model** ở chế độ local.
- **Quyền riêng tư**: cần thông báo rõ khi âm thanh gửi lên cloud; cho phép chế độ chỉ chạy local.
- **Cảnh báo ghi âm**: một số nền tảng họp có thể cảnh báo việc ghi âm — cần thông báo cho người tham dự.

## Đóng gói & phân phối
- Tauri bundler: MSI/NSIS cho Windows.
- Model local tải về lần đầu, không đóng gói sẵn trong installer.
- Ký số (code signing) trước khi phát hành.
- **Mới (PBI 16)**: dịch local chạy qua sidecar executable riêng (không in-process) — khi đóng gói (M8) cần kèm executable sidecar này trong installer, ngoài model.

## Xung đột CRT native trên Windows MSVC (phát hiện ở PBI 16)
- **Vấn đề**: crate C++ binding nào build CTranslate2/thư viện ML khác với CRT tĩnh (`static_crt(true)` trong `build.rs`, qua `cmake` crate) **không thể cùng link** với `ort`/onnxruntime (CRT động, dùng cho VAD qua `voice_activity_detector`, [[PBI-9-vad-silero]]) trong cùng 1 binary `app_lib` — lỗi `LNK2038`/`LNK1169`. `RUSTFLAGS=-C target-feature=+crt-static` không giải quyết được vì object của `ort_sys` dựng sẵn cố định.
- **Case cụ thể đã gặp**: `ct2rs` (CTranslate2) — xem [[PBI-16-dich-local-sidecar]]. Nghi ngờ `sentencepiece`/`sentencepiece-sys` (dùng cho tokenizer NLLB/Marian) cũng dính lỗi tương tự (thấy object của nó lẫn trong log lỗi).
- **Áp dụng cho tương lai**: trước khi thêm bất kỳ crate binding C++ native nào (nhận diện qua có `build.rs` gọi `cmake`), phải thử `cargo build`/`cargo test` thật (không chỉ `cargo check`, vì `cargo check` không link nên không phát hiện được xung đột này) trong project đã có `ort`. Nếu xung đột, ưu tiên kiến trúc sidecar process (spawn executable ngoài qua `std::process::Command`) thay vì binding in-process.

## Liên quan
[[Tong-quan-du-an]], [[Pipeline-xu-ly-audio]], [[PBI-16-dich-local-sidecar]], [[PBI-9-vad-silero]]
