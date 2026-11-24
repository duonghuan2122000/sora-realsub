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
- **Vấn đề 1 (ct2rs vs ort)**: crate C++ binding nào build CTranslate2/thư viện ML khác với CRT tĩnh (`static_crt(true)` trong `build.rs`, qua `cmake` crate) **không thể cùng link** với `ort`/onnxruntime (CRT động, dùng cho VAD qua `voice_activity_detector`, [[PBI-9-vad-silero]]) trong cùng 1 binary `app_lib` — lỗi `LNK2038`/`LNK1169`. `RUSTFLAGS=-C target-feature=+crt-static` **không** giải quyết được trong trường hợp này vì object của `ort_sys` dựng sẵn cố định `/MD`, không build lại theo RUSTFLAGS.
- **Vấn đề 2 (ct2rs tự xung đột nội bộ, xác nhận sau — không cần `ort`)**: `ct2rs` với feature mặc định `sentencepiece` (trong `all-tokenizers`) build `sentencepiece-sys` bằng `/MD`, trong khi `ct2rs-sys`/CTranslate2 build `/MT` — tự xung đột ngay trong nội bộ `ct2rs`, không liên quan `ort`. Khắc phục: dùng `default-features = false, features = ["tokenizers", "ruy"]` (bỏ `sentencepiece`, dùng tokenizer HF `tokenizers` pure Rust đọc `tokenizer.json` thay binding sentencepiece native; `ruy` cho backend SGEMM CPU).
- **Khi nào `RUSTFLAGS=-C target-feature=+crt-static` cứu được**: chỉ khi TOÀN BỘ crate C++ native trong cùng 1 binary đều build lại theo flag này (ví dụ `ct2rs` một mình trong 1 crate riêng, không có `ort`). Không cứu được khi một trong các crate ship object nhị phân dựng sẵn cố định CRT (như `ort`/onnxruntime).
- **Case cụ thể đã gặp**: `ct2rs` (CTranslate2) — xem [[PBI-16-dich-local-sidecar]]. Giải quyết cuối cùng bằng kiến trúc **sidecar**: tách `ct2rs` ra 1 Cargo package độc lập hoàn toàn (`app/translate-sidecar/`), build riêng với `RUSTFLAGS=-C target-feature=+crt-static` (không có `ort` để xung đột), rồi gọi qua `std::process::Command`.
- **Ghi chú phụ (Windows MAX_PATH)**: build CTranslate2 + `ruy` tạo path build rất dài, dễ vượt 260 ký tự → lỗi MSBuild `MSB3501`. Fix bằng `CARGO_TARGET_DIR` ngắn.
- **Áp dụng cho tương lai**: trước khi thêm bất kỳ crate binding C++ native nào (nhận diện qua có `build.rs` gọi `cmake`), phải thử `cargo build`/`cargo test` thật (không chỉ `cargo check`, vì `cargo check` không link nên không phát hiện được xung đột này) trong project đã có `ort`. Nếu xung đột, tách crate đó ra 1 Cargo package riêng (không chỉ 1 module/`[[bin]]` trong cùng package — `[[bin]]` cùng package vẫn ổn nếu bin đó không bị lib khác trong cùng package kéo theo, nhưng an toàn nhất là package riêng) và gọi qua `std::process::Command` (kiến trúc sidecar).

## Liên quan
[[Tong-quan-du-an]], [[Pipeline-xu-ly-audio]], [[PBI-16-dich-local-sidecar]], [[PBI-9-vad-silero]]
