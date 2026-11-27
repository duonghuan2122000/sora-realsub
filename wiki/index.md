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
- [[PBI-15-xu-ly-loi-runtime-model-stt]] — phát event Tauri `model-error` (loại `load`/`infer`) khi model STT lỗi nạp hoặc lỗi infer 1 segment, lỗi 1 segment không làm dừng pipeline, hoàn tất M3 (M3, 2026-11-24)
- [[PBI-16-dich-local-sidecar]] — cài `SidecarTranslator` (đổi tên từ `NllbTranslator`) cho trait `Translator`, bắt đầu M4; đổi từ binding native in-process sang **sidecar process** (crate riêng `app/translate-sidecar/`) do xung đột CRT với `ort` (VAD) — xem [[Rui-ro-ky-thuat]]; SC-001 đã xác nhận bằng sidecar + model Marian-MT `opus-mt-en-vi` (Apache-2.0, open-source — đổi từ NLLB-200 non-commercial theo yêu cầu người dùng), roadmap đã tick (M4, 2026-11-24)
- [[PBI-17-dich-co-ngu-canh]] — `SidecarTranslator::translate()` nay dùng thật tham số `context`: ghép tối đa 2 câu ngữ cảnh + câu hiện tại thành 1 input gửi sidecar (không đổi sidecar), tách bản dịch câu hiện tại bằng heuristic dấu kết câu + fallback khi tách thất bại; thêm `RecentTranscriptContext` (mức đơn vị, chưa nối pipeline thật); **phát hiện quan trọng**: container Linux không build được crate `app` (dependency `wasapi` không gate theo target, có từ PBI 6) — SC-001 của PBI 17 chỉ xác nhận bằng 13/13 test logic trên crate scratch độc lập, chưa chạy `cargo test -p app` thật; roadmap CHƯA tick dòng này (M4, 2026-11-25)
- [[PBI-18-loopback-linux]] — Gate `wasapi` vào `[target.'cfg(windows)'.dependencies]` (giải quyết đúng gốc rủi ro đã ghi ở PBI 17/[[Rui-ro-ky-thuat]]); thêm `LoopbackAudioSource` bản Linux (PulseAudio/PipeWire qua `libpulse-binding`, monitor source `@DEFAULT_MONITOR@`); lần đầu `cargo build`/`cargo test -p app` chạy được thật trên Linux, 60+ test pass; capture loopback thật xác nhận bằng PulseAudio + tone 440Hz test; hoàn tất M1 cho cả Windows+Linux; chưa xác nhận SC-002 trên Windows thật (M1, 2026-11-25)
- [[PBI-19-noi-stt-dich-subtitle-ready]] — nối `TranscriptPipeline` (STT final) → `Translator` (có ngữ cảnh, PBI 17) qua hàm thuần mới `translate_final_chunk` trong `lib.rs::start_capture`, phát event Tauri `subtitle-ready` (câu gốc + câu dịch); thêm `ModelErrorKind::Translate`; export `SidecarTranslator`/`RecentTranscriptContext` từ `core`; hoàn tất M4 (3/3); 65/65 test pass (2 ignored cần sidecar thật), container Linux lần này build/test Tauri/GTK thành công ngay sau khi cài đủ system lib (M4, 2026-11-25)
- [[PBI-20-cua-so-phu-de-noi]] — tạo cửa sổ Tauri thứ hai `create_subtitle_window()` (always-on-top, không viền/title bar, trong suốt, ẩn taskbar), entry point Vue mới `subtitle.html`/`subtitle/main.ts`/`SubtitleApp.vue` (khung trống, chưa gắn `subtitle-ready`), `vite.config.ts` build đa trang; bắt đầu M5 (1/6); `cargo build` pass và **SC-001/SC-002 đã xác nhận PASS bằng GUI thật trên máy Windows**, roadmap đã tick (M5, 2026-11-25)
- [[PBI-21-hien-thi-phu-de]] — `SubtitleApp.vue` nay subscribe event `subtitle-ready` (`listen` từ `@tauri-apps/api/event`), hiển thị `translatedText` mới nhất, ẩn khi rỗng, CSS chống vỡ layout text dài; thêm test Vue đầu tiên của dự án (`SubtitleApp.spec.ts`, vitest + `@vue/test-utils`), 8/8 test pass, build sạch; M5 (2/6), roadmap đã tick (M5, 2026-11-25)
- [[PBI-22-auto-scroll-fade-phu-de]] — `SubtitleApp.vue` giữ N=3 dòng gần nhất (thay vì 1 dòng, PBI 21) qua `lines: Ref<SubtitleLine[]>`, cuộn + mờ dần dòng cũ bằng CSS `opacity` theo class `--age-0/1/2` + transition, không thêm dependency; 5/5 test pass (vitest, phải dùng `bun run test` chứ không phải `bun test`); hoàn tất M5 (3/6), roadmap đã tick (M5, 2026-11-25)
- [[PBI-23-keo-tha-cua-so-phu-de]] — `SubtitleApp.vue` thêm `data-tauri-drag-region` vào `.subtitle-overlay`; cần thêm capability riêng `capabilities/subtitle.json` (`core:window:allow-start-dragging`) vì window `subtitle` không tự kế thừa `default.json` (`windows: ["main"]`) — ACL chặn âm thầm, phát hiện khi test tay. 11/11 test pass, xác nhận PASS trên Windows thật sau fix. Hoàn tất M5 (4/6), roadmap đã tick (M5, 2026-11-25)
- [[PBI-24-chinh-co-chu-phu-de]] — Nút +/- chỉnh cỡ chữ `SubtitleApp.vue` (1.0-3.0rem, bước 0.25), lưu/khôi phục qua `localStorage` webview (không thêm Tauri command/file config — để dành M6); nút đặt ngoài `data-tauri-drag-region` để không bị nuốt click. 16/16 test pass; kiểm thử tay qua Claude Browser pane (vite dev thuần, chưa phải webview Windows thật) xác nhận tăng cỡ chữ + persist qua reload. Hoàn tất M5 (5/6), roadmap đã tick (M5, 2026-11-25)
- [[PBI-25-an-hien-cua-so-phu-de]] — Phím tắt toàn cục `Ctrl+Alt+H` ẩn/hiện cửa sổ phụ đề qua `tauri-plugin-global-shortcut`, dùng `hide()`/`show()` (không đóng cửa sổ) để giữ nguyên vị trí/cỡ chữ; thuần Core Rust, không đụng UI Vue. 67/67 test pass (2 mới cho hàm thuần `toggle_subtitle_visibility`). **Hoàn tất M5 (6/6), xác nhận PASS trên Windows thật, roadmap đã tick** (M5, 2026-11-25)

- [[PBI-26-man-hinh-chinh-chon-thiet-bi]] — Màn hình chính thay skeleton mặc định, chọn mic + nguồn loopback (loopback = danh sách thiết bị output, tái dùng pattern `AudioHost`), validate đủ 2 lựa chọn, phát hiện thiết bị mất khả dụng khi tải lại danh sách; bắt đầu M6 (1/4); 70/70 test Rust + 22/22 test Vitest pass, chưa xác nhận tay trên máy thật (M6, 2026-11-27)
- [[PBI-28-nut-start-stop-phien-dich]] — Nút Start/Stop phiên dịch phụ đề ở `App.vue`, state máy 4 giá trị (idle/starting/running/stopping) đồng bộ đúng với kết quả `invoke("start_capture"/"stop_capture")`, khoá chọn thiết bị bằng `inert` khi phiên không idle; không sửa Core, chỉ tái dùng lệnh đã có từ PBI 8; M6 (2/4); 28/28 test Vitest pass, chưa xác nhận tay trên máy thật (M6, 2026-11-27)

## Trạng thái code hiện tại
- [[Trang-thai-ma-nguon]] — snapshot app/ (2026-11-27, cập nhật sau PBI 28): M1 hoàn tất 6/6 (cả Windows + Linux); M2 hoàn tất 3/3; M3 hoàn tất 4/4; M4 hoàn tất 3/3; M5 hoàn tất 6/6; **M6 (Điều khiển & cấu hình) 2/4** — màn hình chính chọn mic + loopback (PBI 26), nút Start/Stop phiên dịch phụ đề (PBI 28); SC-001 PBI 17 (ngữ cảnh) và PBI 24 (chỉnh cỡ chữ) vẫn chờ xác nhận tay trên máy Windows thật riêng của chúng; PBI 26 và PBI 28 cũng chờ xác nhận tay trên máy thật
