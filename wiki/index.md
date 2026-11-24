# Index

## Overview
- [[Tong-quan-du-an]] — mục tiêu, luồng xử lý, 2 hướng cloud/local

## Concepts
- [[Pipeline-xu-ly-audio]] — audio capture → VAD → STT → dịch → hiển thị
- [[Kien-truc-core-ui]] — tách Core Rust / UI Vue qua IPC Tauri, trait AudioSource/SpeechToText/Translator/SubtitleSink

## Components
- [[Core-Rust]] — crate, thư viện, lưu trữ
- [[UI-Vue]] — framework, state, i18n, cửa sổ
- [[Style-guide-token]] — design token, BEM `sora-*`, quy ước class trạng thái

## Decisions
- [[Cau-hoi-con-mo]] — câu hỏi mở còn lại (đã chốt OS, ngôn ngữ, hướng xử lý)
- [[Quyet-dinh-OS-muc-tieu]] — Windows trước, macOS Backlog (PBI 1, đã chốt)
- [[Quyet-dinh-ngon-ngu-mac-dinh]] — cặp ngôn ngữ mặc định en-US → vi-VN (PBI 2, đã chốt)
- [[Quyet-dinh-hop-dong-core]] — 4 trait Core (AudioSource/SpeechToText/Translator/SubtitleSink), chỉ interface (PBI 3, đã chốt)
- [[Rui-ro-ky-thuat]] — độ trễ, khởi động, chi phí GPU, riêng tư

## PBI đã thi công
- [[PBI-4-liet-ke-thiet-bi-mic]] — liệt kê thiết bị audio input qua Tauri command, UI chọn mic (M1, 2026-11-24)
- [[PBI-5-capture-mic-pcm]] — capture mic thành PCM qua `cpal` (cài đặt thật đầu tiên của trait `AudioSource`), test bằng ghi .wav (M1, 2026-11-24)
- [[PBI-6-capture-loopback-wasapi]] — capture loopback hệ thống qua crate `wasapi` (cpal không hỗ trợ loopback), test bằng ghi .wav (M1, 2026-11-24)
- [[PBI-7-tron-mic-loopback]] — trộn mic + loopback thành 1 stream qua `MixedAudioSource` (buffer trung gian + thread trộn, resample linear), test bằng ghi .wav (M1, 2026-11-24)
- [[PBI-8-start-stop-capture]] — lệnh `start_capture`/`stop_capture` qua Tauri IPC (`CaptureSessionState`), hoàn tất M1 (M1, 2026-11-24)
- [[PBI-9-vad-silero]] — gán nhãn speech/silence theo frame qua Silero VAD v5 (crate `voice_activity_detector`), bắt đầu M2; SC-001/002/003 đã xác nhận bằng fixture giọng nói thật (M2, 2026-11-24)
- [[PBI-10-cat-segment-khoang-lang]] — cắt luồng audio thành segment theo khoảng lặng (state machine silence-hangover, không thêm dependency), xác nhận bằng audio thật ghép 2 lượt nói (M2, 2026-11-24)
- [[PBI-11-vad-segment-event]] — streaming segmenter `VadPipeline` nối VAD+segment vào `start_capture` thật, phát event Tauri `vad-segment` ra UI, hoàn tất M2 (M2, 2026-11-24)
- [[PBI-12-stt-local-whisper]] — cài đặt `WhisperSpeechToText` (whisper.cpp local) cho trait `SpeechToText`, bắt đầu M3; xác nhận bằng model + giọng nói thật, cần cài thêm LLVM/libclang để build (M3, 2026-11-24)
- [[PBI-13-ghep-stt-vao-luong]] — `TranscriptPipeline` nối `VadPipeline` + `SpeechToText` thành luồng end-to-end trên file audio mẫu, giữ đúng thứ tự thời gian (M3, 2026-11-24)
- [[PBI-14-transcript-partial-final-event]] — nối `TranscriptPipeline` thật vào `start_capture`, phát event Tauri `transcript-partial`/`transcript-final` ra UI theo đúng thứ tự (M3, 2026-11-24)

## Trạng thái code hiện tại
- [[Trang-thai-ma-nguon]] — snapshot app/ (2026-11-24, cập nhật sau PBI 14): M1 (Audio Capture) hoàn tất 5/5; M2 (VAD) hoàn tất 3/3; **M3 (STT streaming) đang chạy 3/4** (STT local + ghép luồng VAD→STT + phát event transcript-partial/final xong, chưa xử lý lỗi runtime model); dịch vẫn chưa có cài đặt (M4 trở đi)
