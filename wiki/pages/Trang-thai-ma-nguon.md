---
title: Trạng thái mã nguồn (snapshot)
date: 2026-11-25
tags: [status]
sources: [app/]
---

# Trạng thái mã nguồn — snapshot 2026-11-25 (cập nhật sau PBI 24)

Ghi chú: trạng thái code đổi nhanh, chỉ tin phần này tới mốc ngày trên; ingest lại khi code có tiến triển mới.

## app/src-tauri (Core Rust)
- `main.rs` — skeleton Tauri 2 mặc định (`create_tauri_app` boilerplate).
- **PBI 3**: `core/` module đã có 4 trait interface (`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`) + kiểu dữ liệu + `CoreError`, kèm mock test. Chi tiết: [[Quyet-dinh-hop-dong-core]].
- **PBI 4**: `core/devices.rs` — liệt kê thiết bị audio input bằng `cpal` thật; Tauri command `list_input_devices_cmd`. Dependency: `cpal = "0.15"`. Chi tiết: [[PBI-4-liet-ke-thiet-bi-mic]].
- **PBI 5**: `core/mic_audio_source.rs` — `MicAudioSource`, cài đặt thật đầu tiên của trait `AudioSource` (capture mic qua `cpal`, stream giữ trên thread riêng vì `cpal::Stream` không `Send`); bin thủ công `src/bin/capture_mic_to_wav.rs` ghi `.wav` (crate `hound`) làm bằng chứng nghe lại. `lib.rs`: `mod core` đổi thành `pub mod core`. Chi tiết: [[PBI-5-capture-mic-pcm]].
- **Mới (PBI 6)**: `core/loopback_audio_source.rs` — `LoopbackAudioSource`, cài đặt thứ hai của trait `AudioSource` (capture loopback từ thiết bị output mặc định qua crate **`wasapi`**, vì `cpal` không hỗ trợ loopback; stream giữ trên thread riêng cùng pattern với mic). Bin thủ công `src/bin/capture_loopback_to_wav.rs` ghi `.wav`. Dependency mới: `wasapi = "0.26"`. 16/16 test pass, clippy sạch. Chi tiết: [[PBI-6-capture-loopback-wasapi]].
- **Mới (PBI 7)**: `core/mixed_audio_source.rs` — `MixedAudioSource`, trộn 2 `AudioSource` con (mic + loopback) thành 1 stream bằng buffer trung gian + thread trộn chu kỳ 15ms (cộng mẫu + clamp), resample linear khi sample rate khác nhau. Không thêm dependency. Bin thủ công `src/bin/capture_mixed_to_wav.rs`. 21/21 test pass, clippy sạch. Chi tiết: [[PBI-7-tron-mic-loopback]].
- **Mới (PBI 8)**: `core/capture_session.rs` — `CaptureSessionState`, quản lý 1 phiên thu âm duy nhất (`Mutex<Option<StreamHandle>>`), expose qua Tauri command `start_capture`/`stop_capture` trong `lib.rs` (dùng `MixedAudioSource` từ PBI 7). Thêm 2 lỗi `CoreError::AlreadyRunning`/`NotRunning`. Không thêm dependency. 26/26 test pass; kiểm thử tay qua UI thật (quickstart.md) chưa chạy do môi trường không có GUI — xem [[PBI-8-start-stop-capture]]. **M1 (Audio Capture) hoàn tất.**
- **Mới (PBI 9)**: `core/vad.rs` — `detect()`/`detect_with_threshold()` chia buffer frame 512 sample @16kHz, gán nhãn speech/silence qua crate `voice_activity_detector` (Silero VAD v5). Tái dùng `resample_linear` từ `mixed_audio_source.rs` (đổi `pub(super)`). Dependency mới: `voice_activity_detector = "0.2"`. Test tích hợp `tests/vad_real_speech.rs` dùng fixture giọng nói thật `tests/fixtures/vad_sample.wav` (copy từ `capture_mic_to_wav_output.wav`, 8s im lặng-nói-im lặng) xác nhận SC-001/003 bằng đối chiếu RMS năng lượng. 31/31 test pass (30 unit + 1 fixture thật), clippy sạch. Chưa nối vào pipeline streaming, chưa có Tauri command/event. Xem [[PBI-9-vad-silero]].
- **Mới (PBI 10)**: `core/segment.rs` — `segment()`/`segment_with_min_silence()`, state machine silence-hangover cắt `Vec<FrameLabel>` (từ `vad::detect`, PBI 9) thành `Vec<Segment>` theo lượt nói, ngưỡng mặc định `DEFAULT_MIN_SILENCE_FRAMES = 9` (~300ms). Không thêm dependency. Test tích hợp `tests/segment_real_speech.rs` ghép 2 lần audio thật `vad_sample.wav` với khoảng lặng 2s chèn giữa, xác nhận đúng 2 segment. 7/7 test pass (6 unit + 1 fixture thật), clippy sạch. Chưa nối vào pipeline streaming, chưa có Tauri event. Xem [[PBI-10-cat-segment-khoang-lang]].
- **Mới (PBI 11)**: `core/vad_pipeline.rs` — `VadPipeline` nối `vad::detect` (PBI 9) + `segment::segment` (PBI 10) thành streaming segmenter: `push_frame()` tích lũy `AudioFrame`, trả về `SegmentReady` cho segment MỚI chắc chắn đóng. `lib.rs::start_capture` nối closure `on_frame` (trước đó bỏ qua mọi frame, PBI 8) vào `VadPipeline`, phát sự kiện Tauri `vad-segment` (payload camelCase) cho mỗi segment đóng. Không thêm dependency; đổi vài const/fn ở `vad.rs`/`segment.rs` sang `pub(crate)`. 42 unit test + 4 test tích hợp (gồm `tests/vad_pipeline_real_speech.rs` mới, dùng fixture `vad_sample.wav`) — tất cả pass. Kiểm thử tay UI thật qua `pnpm tauri dev` **chưa chạy** (môi trường không có mic/GUI thật, giống PBI 8). **M2 (VAD) hoàn tất.** Xem [[PBI-11-vad-segment-event]].
- **Mới (PBI 12)**: `core/speech_to_text.rs` — `WhisperSpeechToText`, cài đặt thật trait `SpeechToText` bằng `whisper-rs` (whisper.cpp), nạp model ggml/gguf từ file cục bộ, tái dùng `resample_linear` (không thêm `rubato`). Dependency mới: `whisper-rs = "0.16"`. Test tích hợp `tests/stt_whisper.rs` dùng model thật `ggml-tiny.bin` (tải tạm, không commit) + fixture `vad_sample.wav`: giọng nói thật → text không rỗng, audio im lặng → text rỗng (phải sửa code lọc tag `[BLANK_AUDIO]` sau khi phát hiện qua test thật). 42 unit test cũ + 2 test mới pass, clippy sạch. **Bắt đầu M3 (STT streaming)**. Yêu cầu toolchain mới: cần LLVM/libclang để build (`winget install LLVM.LLVM`, set `LIBCLANG_PATH`) — chưa cập nhật vào docs/README. Chưa ghép vào luồng VAD streaming thật, chưa có Tauri event. Xem [[PBI-12-stt-local-whisper]].
- **Mới (PBI 13)**: `core/transcript_pipeline.rs` — `TranscriptPipeline`, bộ điều phối mỏng nối `VadPipeline` (PBI 11) + `SpeechToText` (PBI 12): `process_segments()` nhận `Vec<SegmentReady>`, gọi `transcribe` tuần tự từng segment, trả `Vec<TranscriptChunk>` đúng thứ tự thời gian. Không thêm dependency. Test tích hợp mới `tests/transcript_pipeline_real_speech.rs` (fixture `vad_sample.wav`, tự skip nếu thiếu `WHISPER_MODEL_PATH` — máy chạy hiện tại không có model nên 2 test tích hợp mới chạy ở nhánh skip, chưa xác nhận bằng model thật). 44 unit test + 8 file test tích hợp pass, không crash. Vẫn chạy trên file audio mẫu, CHƯA nối vào luồng capture mic/loopback thời gian thực. Xem [[PBI-13-ghep-stt-vao-luong]].
- **Mới (PBI 14)**: `core/transcript_pipeline.rs` — `process_segments()` thêm tham số `on_chunk` (callback gọi ngay sau mỗi segment có kết quả, thay vì chỉ trả `Vec` gộp như PBI 13). `lib.rs::start_capture` nối `TranscriptPipeline` thật vào luồng VAD: mỗi `SegmentReady` được đưa qua STT (nạp từ `WHISPER_MODEL_PATH`, tuỳ chọn — thiếu thì bỏ qua, `vad-segment` vẫn phát như cũ), emit event Tauri `transcript-final` (hoặc `transcript-partial` nếu `is_final == false`) với payload `TranscriptEventPayload`. Cần `rt.block_on` (dùng `tokio::runtime::Builder::new_current_thread`) để gọi code async từ closure đồng bộ của `CaptureSessionState`. Dependency: `tokio` chuyển từ `[dev-dependencies]` sang `[dependencies]`. `WhisperSpeechToText` hiện tại luôn trả `is_final: true` nên trên thực tế mới chỉ phát `transcript-final`, chưa có `transcript-partial` thật (STT chưa hỗ trợ streaming partial) — đã test được bằng mock. 47 unit test + 8 integration test (55 tổng) pass. Xem [[PBI-14-transcript-partial-final-event]].
- **Mới (PBI 15)**: `core/types.rs` thêm `ModelErrorKind` (`Load`/`Infer`) + `ModelErrorEvent`. `core/transcript_pipeline.rs::process_segments` đổi xử lý lỗi infer: không còn `?` dừng cả batch, mà `match` + gọi `on_error` rồi `continue` sang segment kế tiếp (tham số mới `on_error: impl FnMut(&SegmentReady, &CoreError)`). `lib.rs::start_capture` emit event Tauri `model-error` loại `load` khi `WhisperSpeechToText::new` lỗi (đúng 1 lần), loại `infer` kèm `segmentStartFrameIndex` khi 1 segment lỗi infer; phiên capture/VAD vẫn chạy bình thường trong cả 2 trường hợp. 50 unit test + 8 integration test (58 tổng) pass, không hỏng test PBI 11-14. **Hoàn tất M3 (STT streaming), 4/4.** Xem [[PBI-15-xu-ly-loi-runtime-model-stt]].
- **Mới (PBI 16)**: `core/translator.rs` — `SidecarTranslator` (đổi tên từ `NllbTranslator`), cài đặt thật đầu tiên của trait `Translator` (PBI 3), chạy qua **sidecar executable ngoài** (`std::process::Command`), không phải binding native in-process — đổi hướng giữa implement vì `ct2rs` (CTranslate2) ép CRT tĩnh, xung đột linker với `ort`/`voice_activity_detector` (VAD, PBI 9) đã có trong `app_lib` (xem [[Rui-ro-ky-thuat]], [[PBI-16-dich-local-sidecar]]). `core/translator.rs` không thêm Cargo dependency nào. **Sidecar thật**: crate Cargo riêng `app/translate-sidecar/` (dùng `ct2rs`, feature `sentencepiece`+`ruy`, build với `RUSTFLAGS=+crt-static`), xác nhận SC-001+FR-004 bằng model `gaudi/opus-mt-en-vi-ctranslate2` thật (Marian-MT, **license Apache-2.0, open-source** — đổi từ NLLB-200 non-commercial theo yêu cầu người dùng). 5/5 test pass. Bắt đầu M4 (Dịch), 1/3 mục, chưa ghép ngữ cảnh và chưa nối vào pipeline STT→subtitle.
- **Mới (PBI 17)**: `core/translator.rs` — `SidecarTranslator::translate()` nay dùng thật tham số
  `context` (trước đó bỏ qua): ghép tối đa 2 câu ngữ cảnh + câu hiện tại thành 1 input gửi sidecar
  (không đổi `app/translate-sidecar/`), tách bản dịch câu hiện tại bằng heuristic dấu kết câu,
  fallback dịch lại không ngữ cảnh khi tách thất bại. Thêm struct `RecentTranscriptContext`
  (bounded FIFO tối đa 2 câu) — chỉ ở mức đơn vị, **chưa nối vào `TranscriptPipeline` thật**
  (mục roadmap kế tiếp). Không thêm Cargo dependency. **Chưa xác nhận bằng `cargo test -p app`
  thật** trong container Linux hiện tại (môi trường không build được crate `app` — xem
  [[Rui-ro-ky-thuat]] mục dependency `wasapi`) — chỉ xác nhận 13/13 test logic bằng crate scratch
  độc lập. Xem [[PBI-17-dich-co-ngu-canh]].
- **Mới (PBI 18)**: gate `wasapi = "0.26"` vào
  `[target.'cfg(windows)'.dependencies]` trong `Cargo.toml` (trước đó vô
  điều kiện, chặn build Linux — phát hiện ở PBI 17, xem [[Rui-ro-ky-thuat]]).
  `core/loopback_audio_source.rs` (bản WASAPI, PBI 6) thêm `#![cfg(windows)]`,
  logic không đổi. Thêm `core/loopback_audio_source_linux.rs` (mới):
  `LoopbackAudioSource` bản Linux dùng `libpulse-binding`/`libpulse-simple-binding`,
  mở monitor source mặc định qua pseudo-device `@DEFAULT_MONITOR@` (tương
  thích PulseAudio và PipeWire). `core/mod.rs` export `LoopbackAudioSource`
  theo đúng 1 trong 2 module tuỳ `cfg(windows)`/`cfg(target_os = "linux")` —
  `lib.rs` và các `src/bin/*.rs` dùng `LoopbackAudioSource` không cần sửa.
  **Lần đầu `cargo build`/`cargo test -p app` chạy được thật trên container
  Linux** — còn cần cài thêm hệ thống lib không liên quan code (`libpulse-dev`,
  `libasound2-dev`, GTK/WebKit chuẩn của Tauri 2: `libgtk-3-dev`,
  `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`,
  `libsoup-3.0-dev`, `libjavascriptcoregtk-4.1-dev`), đã ghi lại ở
  [[PBI-18-loopback-linux]] để lần sau không mất công dò lại. 60 unit test +
  toàn bộ integration test pass trên Linux. Capture loopback thật đã xác
  nhận bằng PulseAudio + tone 440Hz test (`T012-result.log`). **Hoàn tất M1
  cho cả Windows và Linux.** **Chưa xác nhận trên Windows thật** (không có
  máy Windows trong môi trường implement) — SC-002 cần người có máy Windows
  xác nhận lại trước khi tick dòng roadmap M1 tương ứng. Xem
  [[PBI-18-loopback-linux]].
- **Mới (PBI 19)**: `core/types.rs` thêm biến thể `ModelErrorKind::Translate`. `core/mod.rs` export
  thêm `SidecarTranslator`, `RecentTranscriptContext` (trước đó chỉ export trait `Translator`).
  `lib.rs`: `SubtitlePayload` (payload event `subtitle-ready`, camelCase) + hàm thuần
  `translate_final_chunk(translator, recent_ctx, chunk, on_subtitle, on_error)` — tách khỏi
  `start_capture` để test được bằng mock `Translator`, không cần `AppHandle`/Tauri runtime.
  `start_capture` nay dựng `translator: Option<SidecarTranslator>` từ
  `SORA_TRANSLATE_SIDECAR_PATH`+`SORA_TRANSLATE_MODEL_PATH` (optional, giống `stt`) và
  `RecentTranscriptContext` mới mỗi phiên; sau khi `TranscriptPipeline` trả `chunks`, gọi
  `translate_final_chunk` cho mỗi chunk — dịch thành công thì emit `subtitle-ready` (câu gốc +
  câu dịch), lỗi thì emit `model-error` kiểu `translate` (không dừng phiên). Không thêm Cargo
  dependency. Test mới: 5 unit test mock (`RecordingTranslator`/`FlakyTranslator`) trong
  `lib.rs::tests` + 1 test tích hợp `#[ignore]` mới `tests/subtitle_pipeline_real.rs` (cần
  model+sidecar thật, chưa chạy được trong container này). **65/65 test pass (2 ignored, không
  liên quan code PBI 19), build thành công** — môi trường lần này build/test được cả phần
  Tauri/GTK (trước đó PBI 17/18 chỉ xác nhận phần `core/` thuần Rust), sau khi cài thêm
  `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libjavascriptcoregtk-4.1-dev`, `libsoup-3.0-dev`,
  `librsvg2-dev`, `libasound2-dev`, `libpulse-dev`. **Hoàn tất M4 (Dịch), 3/3.** Xem
  [[PBI-19-noi-stt-dich-subtitle-ready]].
- **Mới (PBI 20)**: `lib.rs` thêm hàm `create_subtitle_window(app: &tauri::App)` — tạo cửa sổ Tauri
  thứ hai bằng `WebviewWindowBuilder::new(app, "subtitle", WebviewUrl::App("subtitle.html".into()))`
  với `.decorations(false)`, `.always_on_top(true)`, `.transparent(true)`, `.skip_taskbar(true)`,
  kích thước mặc định 800×150; gọi trong `.setup()` của `tauri::Builder::run()`. Dùng API chung
  `always_on_top` của Tauri 2 (cross-platform Windows+Linux), không viết code riêng theo
  `cfg(windows)`/`cfg(target_os = "linux")` như các PBI audio trước. Không thêm Cargo dependency,
  không thêm Tauri command/event mới. **Hoàn tất mục đầu của M5 (Cửa sổ phụ đề), 1/6 mục.**
  `cargo build` pass (`T004-cargo-build.log`). **SC-001/SC-002 đã xác nhận PASS** bằng GUI thật
  trên máy Windows của người dùng (`bun run tauri dev`, quan sát trực tiếp: không viền + luôn nổi
  trên) — roadmap đã tick dòng tương ứng. Xem [[PBI-20-cua-so-phu-de-noi]].
- **Mới (PBI 21)**: không đụng `app/src-tauri` — toàn bộ thay đổi ở `app/src` (xem mục dưới). Tiêu thụ
  nguyên payload `SubtitlePayload`/event `subtitle-ready` đã có từ PBI 19, không sửa Core. Xem
  [[PBI-21-hien-thi-phu-de]].
- **Mới (PBI 23)**: thêm `app/src-tauri/capabilities/subtitle.json` (`windows: ["subtitle"]`,
  permission `core:window:allow-start-dragging`) — window `subtitle` (PBI 20) trước đó không có
  capability nào nên các permission của `default.json` (`windows: ["main"]`) không áp dụng. Phát
  hiện khi test tay kéo-thả cửa sổ không hoạt động dù code UI đúng — xem [[Rui-ro-ky-thuat]] mục
  capability-per-window. Không đụng code Rust logic nào khác. Xem [[PBI-23-keo-tha-cua-so-phu-de]].
- **Vẫn chưa có**: chọn thiết bị output cụ thể cho loopback (cả Windows và
  Linux — vẫn chỉ dùng thiết bị/sink mặc định), cấu hình model path qua
  UI/config thật (hiện đọc tạm từ biến môi trường `WHISPER_MODEL_PATH`/`SORA_TRANSLATE_SIDECAR_PATH`/`SORA_TRANSLATE_MODEL_PATH`), UI hiển thị thông báo lỗi `model-error` (toast/dialog, thuộc M6/M8), tự động retry khi model lỗi, auto-scroll/fade nhiều dòng phụ đề + kéo-thả/resize cửa sổ (M5 còn 4/6 mục), lưu trữ thật trong [[Pipeline-xu-ly-audio]]. Chưa thấy `rusqlite` trong Cargo deps. Dòng roadmap "Ghép ngữ cảnh" (PBI 17) vẫn CHƯA tick vì SC-001 riêng của PBI 17 vẫn chưa xác nhận bằng sidecar+model thật (PBI 19 tái dùng logic đó, không xác nhận lại SC-001 cũ).

## app/src (UI Vue)
- `App.vue`, `main.ts`, `vite-env.d.ts`, `sample.test.ts` — skeleton Vite+Vue 3 mặc định (cửa sổ chính).
- **PBI 4**: `components/DeviceSelector.vue` — component UI thật đầu tiên (chọn mic từ danh sách thật, nút làm mới, thông báo rỗng/lỗi), gắn vào `App.vue`; kèm `DeviceSelector.test.ts` (vitest, 4 test pass).
- **Mới (PBI 20)**: `subtitle/main.ts` + `subtitle/SubtitleApp.vue` — entry point Vue thứ hai cho cửa sổ
  phụ đề nổi, mount vào `subtitle.html` (file mới ở root `app/`). `SubtitleApp.vue` hiện chỉ là khung
  trống (`.subtitle-overlay`, nền trong suốt, full viewport) — chưa hiển thị nội dung `subtitle-ready`
  thật (mục roadmap kế tiếp của M5). `vite.config.ts` thêm `build.rollupOptions.input` liệt kê cả
  `index.html` và `subtitle.html` để Vite build đa trang (multi-window kiểu Tauri 2 khuyến nghị: mỗi
  cửa sổ một entry HTML riêng, không dùng chung SPA route).
- **Mới (PBI 21)**: `subtitle/SubtitleApp.vue` nay có `<script setup>` thật — `listen('subtitle-ready', ...)`
  từ `@tauri-apps/api/event` trong `onMounted`, `unlisten()` trong `onUnmounted`, state `currentSubtitle`
  (chỉ giữ 1 dòng hiện tại, ghi đè mỗi event, không giữ lịch sử). Template hiển thị `translatedText`,
  ẩn hẳn khi rỗng; CSS chống vỡ layout text dài. Test mới `subtitle/SubtitleApp.spec.ts` — **test Vue
  component đầu tiên của dự án** (mock `@tauri-apps/api/event`, 3 case, dùng `vitest`+`@vue/test-utils`
  đã có sẵn). 8/8 test pass, `npm run build` sạch. Xem [[PBI-21-hien-thi-phu-de]].
- **Mới (PBI 22)**: `subtitle/SubtitleApp.vue` đổi `currentSubtitle: Ref<string>` thành
  `lines: Ref<SubtitleLine[]>` (`{ id, text }`, `id` counter tự tăng làm key `v-for` ổn định), giữ tối
  đa `MAX_LINES = 3` (hằng số, `slice(-3)` mỗi khi push dòng mới). Template `v-for` gán class
  `subtitle-text--age-0/1/2` theo khoảng cách tới dòng mới nhất; CSS 3 class opacity giảm dần
  (1/0.6/0.35) + `transition: opacity/transform 0.3s ease`. Không thêm dependency. Test
  `subtitle/SubtitleApp.spec.ts` sửa/thêm: giữ đúng 3 dòng cuối theo thứ tự, loại dòng cũ nhất khi
  vượt N, đúng class độ mờ theo thứ tự cũ→mới. 5/5 test pass — **lưu ý**: phải chạy `bun run test --
  SubtitleApp.spec.ts` (gọi script `vitest run` trong `package.json`), `bun test` (bun's own runner)
  không tương thích `@vue/test-utils` (lỗi `WeakMap keys must be objects`). **Hoàn tất mục 3/6 của
  M5.** Xem [[PBI-22-auto-scroll-fade-phu-de]].
- **Mới (PBI 23)**: `subtitle/SubtitleApp.vue` thêm attribute `data-tauri-drag-region` vào
  `.subtitle-overlay` — không cần code JS kéo-thả tự viết, Tauri 2 xử lý native ở tầng webview.
  Test mới xác nhận attribute tồn tại. 11/11 test pass. Test tay trên Windows thật: cần thêm
  capability Rust (`capabilities/subtitle.json`, xem mục Core Rust trên) mới kéo được — code UI
  một mình không đủ. **Hoàn tất mục 4/6 của M5.** Xem [[PBI-23-keo-tha-cua-so-phu-de]].
- **Mới (PBI 24)**: `subtitle/SubtitleApp.vue` thêm state `fontSize` (ref, khởi tạo từ
  `readStoredFontSize()` đọc + validate `localStorage['subtitle-font-size']`, fallback 1.5rem mặc
  định nếu rỗng/NaN, clamp 1.0-3.0rem), hàm `increaseFontSize`/`decreaseFontSize` (bước 0.25rem,
  ghi lại `localStorage` mỗi lần đổi), 2 nút `+`/`-` trong `data-tauri-drag-region="false"` (không
  bị vùng kéo-thả PBI 23 nuốt click), bind `fontSize` vào `.subtitle-text` qua CSS variable
  `--subtitle-font-size` (thay giá trị cứng `1.5rem` cũ). Không đụng `app/src-tauri` — không cần
  Tauri command/IPC hay file config Rust, chỉ dùng `localStorage` webview (xem research.md: webview
  tự persist qua app data dir giữa các lần mở app, giống browser thật). Test mới: 5 case (tăng/giảm,
  clamp min/max, khôi phục từ localStorage hợp lệ, fallback khi hỏng, ghi lại localStorage). 16/16
  test pass. Kiểm thử tay: container Linux không có máy Windows thật (lặp lại hạn chế PBI 18/20/23)
  — dùng `bun run dev` (vite thuần) + Claude Browser pane mở `subtitle.html` trực tiếp (bỏ qua phần
  `listen('subtitle-ready')` vì thiếu IPC Tauri) để xác nhận UI + persistence: tăng cỡ chữ 1.5→1.75rem,
  reload giữ đúng 1.75rem. **Chưa xác nhận trên webview WebView2 Windows thật.** **Hoàn tất mục 5/6
  của M5.** Xem [[PBI-24-chinh-co-chu-phu-de]].
- Chưa có Pinia/vue-i18n/shadcn-vue/Lucide, chưa có `styles/tokens.scss` theo [[Style-guide-token]].
- **PBI 5, PBI 6, PBI 7, PBI 8 không đụng UI** — chỉ thi công phần lõi Rust capture + điều khiển phiên (mic, loopback, trộn, start/stop), command Tauri đã có nhưng chưa có nút bấm trong Vue.

## Kết luận
M1 (Audio Capture) đã hoàn tất 6/6 mục (PBI 4-8 + PBI 18, nay chạy được trên cả Windows và Linux). M2 (VAD) đã hoàn tất 3/3 mục (PBI 9-11). M3 (STT streaming) đã hoàn tất 4/4 (PBI 12-15). **M4 (Dịch) đã hoàn tất 3/3**: `Translator` có cài đặt thật (`SidecarTranslator`, PBI 16) chạy qua sidecar process riêng (`app/translate-sidecar/`, đổi hướng do xung đột CRT native, xem [[Rui-ro-ky-thuat]]), **SC-001 (PBI 16) đã xác nhận bằng sidecar + model Marian-MT (opus-mt-en-vi, Apache-2.0, open-source) thật**; dùng được ngữ cảnh 1-2 câu trước khi dịch (PBI 17, **SC-001 riêng của PBI 17 vẫn chưa xác nhận bằng sidecar+model thật**, chỉ logic); và đã **nối STT final → Translator (có ngữ cảnh) → event `subtitle-ready`** (PBI 19) — 65/65 test pass, container lần này build/test được cả phần Tauri/GTK. **M5 (Cửa sổ phụ đề) đang tiến triển, 5/6 mục**: cửa sổ phụ đề always-on-top/không viền đã tạo và xác nhận PASS bằng GUI thật (PBI 20), hiển thị `translatedText` thật từ event `subtitle-ready` (PBI 21), giữ 3 dòng gần nhất có auto-scroll/fade (PBI 22), kéo-thả được bằng `data-tauri-drag-region` + capability riêng cho window `subtitle` (PBI 23), và nay **chỉnh được cỡ chữ qua nút +/- + lưu/khôi phục qua `localStorage` webview** (PBI 24, 16/16 test pass, kiểm thử tay qua Claude Browser pane xác nhận UI+persistence, **chưa xác nhận trên webview Windows thật**). Còn lại 1/6 mục M5: ẩn-hiện phím tắt. Kiểm thử tay trên UI thật (có mic/GUI) vẫn là khoảng trống chung từ PBI 8 cho phần audio/STT/dịch; chưa có UI hiển thị thông báo `model-error` (M6/M8).

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Style-guide-token]], [[Quyet-dinh-hop-dong-core]], [[PBI-4-liet-ke-thiet-bi-mic]], [[PBI-5-capture-mic-pcm]], [[PBI-6-capture-loopback-wasapi]], [[PBI-7-tron-mic-loopback]], [[PBI-8-start-stop-capture]], [[PBI-9-vad-silero]], [[PBI-10-cat-segment-khoang-lang]], [[PBI-11-vad-segment-event]], [[PBI-12-stt-local-whisper]], [[PBI-13-ghep-stt-vao-luong]], [[PBI-14-transcript-partial-final-event]], [[PBI-15-xu-ly-loi-runtime-model-stt]], [[PBI-16-dich-local-sidecar]], [[PBI-17-dich-co-ngu-canh]], [[PBI-18-loopback-linux]], [[PBI-19-noi-stt-dich-subtitle-ready]], [[PBI-20-cua-so-phu-de-noi]], [[PBI-21-hien-thi-phu-de]], [[PBI-22-auto-scroll-fade-phu-de]], [[PBI-23-keo-tha-cua-so-phu-de]], [[PBI-24-chinh-co-chu-phu-de]]
