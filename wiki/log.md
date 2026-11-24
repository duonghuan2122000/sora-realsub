# Log

## [2026-11-24] bootstrap | Khởi tạo wiki từ docs/ và app/ hiện tại
## [2026-11-24] ingest | PBI 1: Chốt OS mục tiêu (spec/plan/tasks/evidence, docs/tech-stack.md cập nhật quyết định)
## [2026-11-24] ingest | PBI 2: Chốt cặp ngôn ngữ mặc định (spec/plan/tasks/evidence, docs/tech-stack.md cập nhật quyết định en-US → vi-VN)
## [2026-11-24] ingest | PBI 3: Định nghĩa trait Core AudioSource/SpeechToText/Translator/SubtitleSink (interface + mock test, app/src-tauri/src/core/), cập nhật Kien-truc-core-ui, Core-Rust, Cau-hoi-con-mo, Trang-thai-ma-nguon

## [2026-11-24] ingest | PBI 4 — Liệt kê thiết bị âm thanh đầu vào (spec/plan/tasks, code core/devices.rs + DeviceSelector.vue, test pass)
## [2026-11-24] ingest | PBI 5 — Capture mic thành PCM buffer (spec/plan/tasks/evidence, code core/mic_audio_source.rs + src/bin/capture_mic_to_wav.rs, hound thêm vào [dependencies], lib.rs pub mod core, 15/15 test pass, cập nhật Core-Rust, Trang-thai-ma-nguon, Pipeline-xu-ly-audio)
## [2026-11-24] ingest | PBI 6 — Capture loopback hệ thống qua WASAPI (spec/plan/tasks/evidence, code core/loopback_audio_source.rs + src/bin/capture_loopback_to_wav.rs, wasapi thêm vào [dependencies] — cpal không hỗ trợ loopback, 16/16 test pass; sửa lại claim sai "cpal (WASAPI loopback + mic)" ở Core-Rust và Pipeline-xu-ly-audio, cập nhật Trang-thai-ma-nguon)
## [2026-11-24] ingest | PBI 7 — Trộn mic + loopback thành 1 stream (spec/plan/tasks/evidence, code core/mixed_audio_source.rs + src/bin/capture_mixed_to_wav.rs, không thêm dependency, 21/21 test pass; cập nhật Core-Rust, Pipeline-xu-ly-audio, Trang-thai-ma-nguon)
## [2026-11-24] ingest | PBI 8 — Lệnh start_capture/stop_capture qua Tauri IPC (spec/plan/tasks/evidence, code core/capture_session.rs + lib.rs, thêm CoreError::AlreadyRunning/NotRunning, không thêm dependency, 26/26 test pass; kiểm thử tay UI thật chưa chạy do môi trường không có GUI; hoàn tất M1; cập nhật Core-Rust, Trang-thai-ma-nguon, index)
