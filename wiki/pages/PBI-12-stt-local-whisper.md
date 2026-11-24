---
title: "PBI 12: Nhận diện giọng nói local (whisper.cpp)"
date: 2026-11-24
tags: [pbi, m3, feature]
sources: [.specify/specs/12/spec.md, .specify/specs/12/plan.md, .specify/specs/12/research.md, .specify/specs/12/quickstart.md, .specify/specs/12/tasks.md, .specify/specs/12/evidence, app/src-tauri/src/core/speech_to_text.rs, app/src-tauri/tests/stt_whisper.rs, app/src-tauri/Cargo.toml]
---

# PBI 12: Nhận diện giọng nói local (whisper.cpp)

Tính năng đầu tiên của M3 (STT streaming). Cài đặt thật cho trait `SpeechToText` (đã định nghĩa interface-only từ [[Quyet-dinh-hop-dong-core]], PBI 3) bằng model whisper.cpp chạy local, chưa ghép vào luồng capture/VAD thời gian thực — đó là PBI tiếp theo trong M3.

## Yêu cầu chính (spec.md)
- STT chạy hoàn toàn local, không gửi audio ra ngoài qua mạng (FR-001).
- Nhận một đoạn audio (buffer/file mẫu), trả văn bản tương ứng, giữ đúng thứ tự nội dung (FR-002, FR-004).
- Audio im lặng → trả text rỗng/gần rỗng, không crash (FR-003).
- Ngoài phạm vi: ghép vào luồng VAD segment thời gian thực, event Tauri `transcript-partial`/`transcript-final`, xử lý lỗi runtime model chi tiết, dịch (Translation) — tất cả thuộc PBI sau trong M3/M4.

## Quyết định kỹ thuật (plan.md, research.md)
- **Chọn `whisper-rs` (binding whisper.cpp)** thay vì `faster-whisper` (Python) — khớp quyết định local đã chốt ở roadmap (đổi hướng cloud → local, commit `24d1897`) và giữ Core 1 binary Rust, không cần runtime Python. Đã được dự liệu sẵn trong bảng [[Core-Rust]] từ trước khi implement.
- **Tái dùng `resample_linear`** (có sẵn từ [[PBI-7-tron-mic-loopback]], `mixed_audio_source.rs`) để chuẩn hoá `AudioFrame` về 16kHz mono f32 mà whisper.cpp yêu cầu — không thêm dependency `rubato` như research.md ban đầu dự tính (ponytail: đã có sẵn, không cần thêm).
- **Model nạp từ file cục bộ** qua `WhisperSpeechToText::new(model_path)`, không bundle model vào source (model ggml nặng vài chục–vài trăm MB).

## Cài đặt (code)
- `app/src-tauri/Cargo.toml`: thêm dependency `whisper-rs = "0.16"` (bản `0.14` gặp lỗi build do mismatch layout struct với LLVM mới trên máy dev — nâng lên `0.16` giải quyết; API `full_n_segments()`/`get_segment()` cũng đổi giữa 2 bản).
- `app/src-tauri/src/core/speech_to_text.rs`: thêm `WhisperSpeechToText` implement `SpeechToText::transcribe` — resample input, tạo `WhisperState`, chạy `full()` với `SamplingStrategy::Greedy`, ghép text từ các segment, **lọc bỏ tag non-speech dạng `[BLANK_AUDIO]`/`[...]`** mà whisper.cpp trả về khi audio im lặng (phát hiện qua test thật, không phải giả định — xem bằng chứng).
- `app/src-tauri/src/core/mod.rs`: export `WhisperSpeechToText`.
- `app/src-tauri/tests/stt_whisper.rs`: test tích hợp, đọc model từ biến môi trường `WHISPER_MODEL_PATH` — tự `return` sớm (skip, không fail) nếu thiếu, để máy không có model vẫn `cargo test` xanh.

## Môi trường build (phát sinh ngoài kế hoạch)
- `whisper-rs-sys` cần **LLVM/libclang** (bindgen) để build — máy dev ban đầu chưa có, phải cài qua `winget install LLVM.LLVM` (~500MB) và set `LIBCLANG_PATH="C:/Program Files/LLVM/bin"` khi chạy `cargo build`/`cargo test`. Đây là yêu cầu toolchain mới cho ai build Core từ PBI 12 trở đi, chưa được ghi ở `docs/tech-stack.md`/README — cần bổ sung hướng dẫn setup máy mới.

## Bằng chứng test (`.specify/specs/12/evidence/`)
- Tải model thật `ggml-tiny.bin` (whisper.cpp, từ HuggingFace `ggerganov/whisper.cpp`, ~75MB) vào thư mục tạm (không commit vào repo) để chạy test **thật**, không chỉ test skip.
- Test 1 — giọng nói thật (tái dùng fixture `vad_sample.wav` từ [[PBI-9-vad-silero]], ~24s): `transcribe()` trả text không rỗng. **PASS**.
- Test 2 — audio im lặng (buffer 2s giá trị 0.0): lần đầu FAIL vì whisper.cpp trả literal `"[BLANK_AUDIO]"` thay vì rỗng — phát hiện thực nghiệm, không nằm trong research.md ban đầu; sửa code lọc tag `[...]`, chạy lại **PASS**.
- Toàn bộ 42 unit test cũ (M0-M2) + 2 test STT mới — tất cả pass, clippy sạch (1 warning collapsible-if đã sửa bằng let-chains).
- Chi tiết: `evidence/T009-testcase.md`, log: `evidence/T009-result.log`.
- **Giới hạn đã biết**: không có ground-truth transcript đáng tin cho `vad_sample.wav` nên chỉ assert "không rỗng" cho case có giọng nói (đúng mức SC-001 — đọc hiểu được, không yêu cầu khớp tuyệt đối), không assert khớp từng chữ. Model `tiny` dùng cho test nhẹ/nhanh, không phải quyết định model production cuối cùng.

## Liên quan
[[Quyet-dinh-hop-dong-core]], [[PBI-11-vad-segment-event]], [[PBI-9-vad-silero]], [[PBI-7-tron-mic-loopback]], [[Core-Rust]], [[Pipeline-xu-ly-audio]], [[Trang-thai-ma-nguon]]
