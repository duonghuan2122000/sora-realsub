---
title: Hợp đồng giao diện Core (PBI 3)
date: 2026-11-24
tags: [decision, component, architecture]
sources: [.specify/specs/3/spec.md, .specify/specs/3/plan.md, .specify/specs/3/research.md, app/src-tauri/src/core]
---

# Hợp đồng giao diện Core (PBI 3)

Đã cài đặt (chỉ interface, chưa có logic thật) 4 trait Core trong `app/src-tauri/src/core/`:

| Trait | File | Dạng | Dùng cho |
|---|---|---|---|
| `AudioSource` | `audio_source.rs` | đồng bộ, callback (`FnMut(AudioFrame)`) | M1 — mic/loopback |
| `SpeechToText` | `speech_to_text.rs` | `async fn` qua `async_trait` | M3 — STT streaming |
| `Translator` | `translator.rs` | `async fn` qua `async_trait`, nhận `context: &[String]` | M4 — dịch có ngữ cảnh |
| `SubtitleSink` | `subtitle_sink.rs` | đồng bộ, `push(Subtitle)` | M5/M7 — hiển thị/lưu transcript |

## Quyết định thiết kế chính

- `AudioSource` dùng callback đồng bộ thay vì async stream — capture native (cpal/WASAPI) chạy trên thread riêng, không tự nhiên async; tránh thêm runtime phức tạp không cần.
- `SpeechToText`/`Translator` dùng `async fn` (`async-trait` dependency mới) — khớp bản chất chờ I/O (cloud API hoặc model local chạy nền), dùng chung tokio runtime Tauri đã có.
- `SubtitleSink` giữ đồng bộ — sink hiện tại (event Tauri, ghi file) là tác vụ nhanh, chưa có lý do cần async.
- Một `CoreError` chung (`thiserror`, 3 biến thể: `Io`, `Provider`, `Cancelled`) cho cả 4 trait, thay vì lỗi riêng từng trait — đơn giản hơn ở giai đoạn interface-only.
- Kiểu dữ liệu dùng chung: `AudioFrame`, `StreamHandle`, `TranscriptChunk` (có `is_final`), `Subtitle`.
- Mỗi trait không rò rỉ chi tiết provider cụ thể (cloud/local) vào chữ ký hàm — khớp hướng xử lý **local** đã chốt ở [[Cau-hoi-con-mo]].

## Bằng chứng

8 unit test với cài đặt giả (mock) cho cả 4 trait, pass; `cargo build` + `cargo clippy` sạch. Chi tiết: `.specify/specs/3/tasks.md`, `.specify/specs/3/evidence/`.

## Phạm vi

Chỉ định nghĩa interface + mock test — **chưa** cài đặt logic thật (capture, STT, dịch, sink thật thuộc M1/M3/M4/M5).

## Liên quan
[[Kien-truc-core-ui]], [[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]], [[Cau-hoi-con-mo]]
