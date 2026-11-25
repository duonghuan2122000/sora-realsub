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
- **Vấn đề 2 (ct2rs tự xung đột nội bộ, xác nhận sau — không cần `ort`)**: `ct2rs` với feature `sentencepiece` (trong default `all-tokenizers`) build `sentencepiece-sys` bằng `/MD`, trong khi `ct2rs-sys`/CTranslate2 build `/MT` — tự xung đột ngay trong nội bộ `ct2rs`, không liên quan `ort`. Có 2 cách khắc phục, cả hai đã thử và dùng thật (tuỳ model/tokenizer cần): (a) tránh `sentencepiece`, dùng `default-features = false, features = ["tokenizers", "ruy"]` (tokenizer HF `tokenizers` pure Rust đọc `tokenizer.json` — dùng cho model NLLB lần đầu); (b) build cả `sentencepiece` lẫn `ct2rs-sys` theo `RUSTFLAGS=-C target-feature=+crt-static` **trong 1 crate riêng không có `ort`** — mọi thứ build lại từ source theo flag này nên hết xung đột (dùng cho model Marian-MT `source.spm`/`target.spm`, xem [[PBI-16-dich-local-sidecar]] phần 3). `ruy` luôn cần cho backend SGEMM CPU.
- **Khi nào `RUSTFLAGS=-C target-feature=+crt-static` cứu được**: khi TOÀN BỘ crate C++ native trong cùng 1 binary đều build lại theo flag này (ví dụ `ct2rs` + `sentencepiece-sys` cùng trong 1 crate riêng, không có `ort`) — đã xác nhận cứu được cả 2 lần (feature `tokenizers` và `sentencepiece`). Không cứu được khi một trong các crate ship object nhị phân dựng sẵn cố định CRT (như `ort`/onnxruntime) — đó là lý do vẫn cần tách `ct2rs` ra khỏi `app_lib` bằng kiến trúc sidecar, không chỉ thêm RUSTFLAGS vào `app_lib`.
- **Case cụ thể đã gặp**: `ct2rs` (CTranslate2) — xem [[PBI-16-dich-local-sidecar]]. Giải quyết cuối cùng bằng kiến trúc **sidecar**: tách `ct2rs` ra 1 Cargo package độc lập hoàn toàn (`app/translate-sidecar/`), build riêng với `RUSTFLAGS=-C target-feature=+crt-static` (không có `ort` để xung đột), rồi gọi qua `std::process::Command`.
- **Ghi chú phụ (Windows MAX_PATH)**: build CTranslate2 + `ruy` tạo path build rất dài, dễ vượt 260 ký tự → lỗi MSBuild `MSB3501`. Fix bằng `CARGO_TARGET_DIR` ngắn.
- **Áp dụng cho tương lai**: trước khi thêm bất kỳ crate binding C++ native nào (nhận diện qua có `build.rs` gọi `cmake`), phải thử `cargo build`/`cargo test` thật (không chỉ `cargo check`, vì `cargo check` không link nên không phát hiện được xung đột này) trong project đã có `ort`. Nếu xung đột, tách crate đó ra 1 Cargo package riêng (không chỉ 1 module/`[[bin]]` trong cùng package — `[[bin]]` cùng package vẫn ổn nếu bin đó không bị lib khác trong cùng package kéo theo, nhưng an toàn nhất là package riêng) và gọi qua `std::process::Command` (kiến trúc sidecar).

## Dependency Windows-only không gate theo target — chặn build/test trên Linux (phát hiện ở PBI 17, đã xử lý ở PBI 18)

**Cập nhật (PBI 18)**: đã gate `wasapi` bằng `[target.'cfg(windows)'.dependencies]` và
thêm `LoopbackAudioSource` bản Linux dùng `libpulse-binding`/`libpulse-simple-binding`
(module riêng `loopback_audio_source_linux.rs`, gate `#[cfg(target_os = "linux")]`,
`mod.rs` export cùng tên `LoopbackAudioSource` theo OS build) — xem
[[PBI-18-loopback-linux]]. `lib.rs` và các `src/bin/*.rs` dùng `LoopbackAudioSource`
không cần sửa vì chỉ gọi qua trait `AudioSource`, không có code riêng theo OS.
- `wasapi = "0.26"` trong `app/src-tauri/Cargo.toml` (từ [[PBI-6-capture-loopback-wasapi]], M1)
  là dependency **vô điều kiện** (không `[target.'cfg(windows)'.dependencies]`), dù bản thân
  crate `wasapi` chỉ build được trên Windows. Trên Linux, build kéo theo `windows` → crate
  `windows-future` lỗi compile `E0425` (`cannot find type IMarshal in module windows_core::imp`,
  `cannot find function submit in crate windows_threading`) — tổ hợp version hiện tại trên
  crates.io của `windows`/`windows-future`/`windows-core` không build được khi chỉ *resolve* cho
  target Linux (dù không có ý định chạy thật trên Linux).
- **Hệ quả**: `cargo build`/`cargo test -p app` **không chạy được trên container Linux**, bất kể
  PBI đang làm có liên quan tới audio/Windows hay không — ví dụ PBI 17 (dịch có ngữ cảnh, thuần
  logic Rust, không đụng audio) vẫn bị chặn hoàn toàn. Xem [[PBI-17-dich-co-ngu-canh]] để biết
  cách lách tạm (sao logic thuần ra crate scratch riêng để vẫn có `cargo test` thật) — không phải
  giải pháp lâu dài.
- **Giải pháp đúng (chưa làm, đề xuất PBI riêng)**: gate `wasapi` bằng
  `[target.'cfg(windows)'.dependencies]` trong `Cargo.toml`, và gate `mod loopback_audio_source`
  + mọi nơi dùng `LoopbackAudioSource` (`lib.rs`, `src/bin/capture_loopback_to_wav.rs`,
  `src/bin/capture_mixed_to_wav.rs`) bằng `#[cfg(windows)]`. Đã thử tạm (PBI 17, không commit) —
  xác nhận hướng đi đúng (build tiến xa hơn qua đúng điểm này) nhưng cần sửa nhiều điểm gọi
  (`lib.rs`, 2 file `src/bin/`), không chỉ 1 dòng — nên làm có kế hoạch riêng, không tiện làm
  "tạm" trong một PBI không liên quan audio.

## Môi trường test Tauri (`tauri::test::mock_app()`) không mô phỏng window manager thật (PBI 20, M5)

`tauri::test::mock_app()`/`MockRuntime` (feature `test` của crate `tauri`) chỉ là stub tối giản
cho IPC/command, **không** mô phỏng hành vi window manager: các getter trạng thái cửa sổ như
`is_decorated()`, `is_always_on_top()`, `is_focused()` luôn trả về **hằng số cố định** (thường
`false`) bất kể giá trị đã truyền vào `WebviewWindowBuilder`. Hệ quả: test viết theo kiểu "tạo
cửa sổ với `.decorations(false)` rồi assert `window.is_decorated() == false`" **pass giả** — sẽ
pass kể cả khi code thực sự gọi `.decorations(true)` (chỉ tình cờ giá trị mong muốn trùng hằng số
mock trả về), không phát hiện được regression thật. Giải pháp đã áp dụng ở PBI 20: tách cấu hình
mong muốn thành hằng số Rust thuần (`SUBTITLE_OVERLAY_CONFIG`), assert trực tiếp vào hằng số đó
thay vì gọi getter của mock window — xem [[PBI-20-overlay-always-on-top]].

Tương tự, container cloud dùng để implement chỉ có `Xvfb` (X server ảo, không compositing/window
manager) — có thể chạy `tauri dev` và chụp ảnh, đọc property X11 thô (`xprop`/`xwininfo`), nhưng
**không có WM nào để phản hồi `_NET_WM_STATE`** (always-on-top thật) hay quản lý focus thật. Mọi
tính năng liên quan hành vi window-manager-dependent (PBI 20 và các mục còn lại của M5: kéo-thả,
resize, ẩn/hiện) sẽ gặp lại giới hạn này — cần máy GUI thật (Windows hoặc Linux có desktop
environment) để xác nhận trực quan, giống pattern "chưa xác nhận trên máy Windows/Linux khác"
đã áp dụng ở PBI 17/18.

## Liên quan
[[Tong-quan-du-an]], [[Pipeline-xu-ly-audio]], [[PBI-16-dich-local-sidecar]], [[PBI-9-vad-silero]], [[PBI-6-capture-loopback-wasapi]], [[PBI-17-dich-co-ngu-canh]], [[PBI-20-overlay-always-on-top]]
