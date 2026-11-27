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

## Capability Tauri 2 không tự kế thừa giữa các window (phát hiện ở PBI 23)

- Mỗi file `app/src-tauri/capabilities/*.json` khai báo `windows: [...]` — chỉ các window có tên nằm trong danh sách đó mới được cấp các `permissions` trong file. `default.json` (tạo sẵn bởi `create-tauri-app`) chỉ có `windows: ["main"]`.
- Khi PBI 20 tạo window thứ hai `"subtitle"` (`create_subtitle_window`), window này **không có bất kỳ capability nào** — mọi permission (kể cả `core:default`) không tự áp dụng sang nó.
- Hệ quả cụ thể (PBI 23): thuộc tính `data-tauri-drag-region` gọi ngầm command `core:window:allow-start-dragging`; vì window `subtitle` không có capability chứa permission này, lệnh kéo **bị ACL chặn âm thầm** — không có lỗi/log, chỉ đơn giản là không kéo được. Dễ nhầm là bug CSS/JS.
- **Fix**: thêm file capability riêng cho window đó, ví dụ `app/src-tauri/capabilities/subtitle.json` với `windows: ["subtitle"]` + đúng permission cần.
- **Áp dụng cho tương lai**: mọi window mới tạo (ngoài `main`) muốn dùng bất kỳ Tauri API nào (kéo-thả, resize, invoke command, event...) phải có capability riêng khai báo đúng tên window đó — không giả định nó kế thừa `default.json`. Kiểm tra `app/src-tauri/gen/schemas/acl-manifests.json` để tìm đúng tên permission identifier khi không chắc (ví dụ `core:window:allow-start-dragging`).

## UI chọn thiết bị loopback chưa nối vào capture thật (phát hiện ở PBI 26)

- `LoopbackAudioSource` (cả bản WASAPI Windows và bản PulseAudio/PipeWire Linux, [[PBI-6-capture-loopback-wasapi]]/[[PBI-18-loopback-linux]]) chỉ nhận capture **thiết bị/sink output mặc định** của hệ điều hành — không có tham số chọn thiết bị cụ thể.
- PBI 26 thêm màn hình chọn "nguồn loopback" ở UI (danh sách thiết bị output qua `list_output_devices_cmd`) nhưng **chỉ dừng ở validate + hiển thị lựa chọn**, chưa truyền lựa chọn đó vào `LoopbackAudioSource`/`start_capture`. Người dùng có thể chọn 1 thiết bị output không phải mặc định trên UI nhưng capture thật vẫn luôn bám theo thiết bị mặc định.
- **Chưa xử lý**: cần mục roadmap riêng (ngoài phạm vi PBI 26, xem `plan.md` PBI 26 mục Rủi ro) để mở rộng `LoopbackAudioSource` nhận tham số thiết bị, rồi nối lựa chọn UI vào `start_capture`.

## Liên quan
[[Tong-quan-du-an]], [[Pipeline-xu-ly-audio]], [[PBI-16-dich-local-sidecar]], [[PBI-9-vad-silero]], [[PBI-6-capture-loopback-wasapi]], [[PBI-17-dich-co-ngu-canh]], [[PBI-20-cua-so-phu-de-noi]], [[PBI-23-keo-tha-cua-so-phu-de]], [[PBI-26-man-hinh-chinh-chon-thiet-bi]]
