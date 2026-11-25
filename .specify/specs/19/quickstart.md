# Quickstart: kiểm thử PBI 19 — Nối STT final → Dịch → subtitle-ready

## Test logic nối pipeline (không cần sidecar/model thật)

Dùng mock `Translator` + mock `SpeechToText` để xác nhận: mỗi câu final có đúng 1 `subtitle-ready`, thứ tự đúng, partial/rỗng bị bỏ qua, lỗi dịch không dừng phiên, ngữ cảnh tối đa 2 câu gốc.

```bash
cd app/src-tauri
cargo test -p app
```

(Test mới của PBI 19 nằm cạnh test hiện có của `lib.rs`/`transcript_pipeline.rs`/`translator.rs` — không cần thiết lập thêm biến môi trường.)

## Test tích hợp end-to-end với sidecar + model dịch thật (xác nhận SC-002)

Cần: model STT (`WHISPER_MODEL_PATH`, xem `.specify/specs/12/quickstart.md`) + sidecar dịch (`SORA_TRANSLATE_SIDECAR_PATH`, `SORA_TRANSLATE_MODEL_PATH`, xem `.specify/specs/16/quickstart.md`) đã có sẵn cục bộ.

```bash
export WHISPER_MODEL_PATH=/duong-dan/toi/model.bin
export SORA_TRANSLATE_SIDECAR_PATH=/duong-dan/toi/translate-sidecar
export SORA_TRANSLATE_MODEL_PATH=/duong-dan/toi/model-dir
cd app/src-tauri
cargo test -p app -- --ignored
```

Kịch bản xác nhận SC-002: đưa 2 segment audio liên tiếp (câu có đại từ phụ thuộc câu trước, ví dụ tương tự PBI 17 T016) qua `start_capture` thật, xác nhận `subtitle-ready` thứ 2 có `translatedText` khớp bản dịch có ngữ cảnh đã xác nhận ở PBI 17 (không phải bản dịch không ngữ cảnh).

## Test thủ công toàn luồng (tuỳ chọn, cần UI)

1. Chạy `npm run tauri dev` trong `app/`.
2. Gọi `start_capture`, nói vài câu liên tiếp bằng tiếng Anh (có câu chứa đại từ phụ thuộc câu trước).
3. Lắng nghe event `subtitle-ready` qua DevTools console (`window.__TAURI__.event.listen('subtitle-ready', console.log)`), xác nhận mỗi câu final có 1 event chứa cả câu gốc và câu dịch, đúng thứ tự.
