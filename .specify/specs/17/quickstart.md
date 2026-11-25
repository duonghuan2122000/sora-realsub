# Quickstart: kiểm thử PBI 17 — Dịch có ngữ cảnh câu trước

## Test không cần sidecar/model thật

Chạy toàn bộ test logic (ghép ngữ cảnh, giới hạn tối đa 2 câu, bỏ qua câu rỗng, tách bản dịch câu hiện tại, fallback khi tách thất bại) bằng mock `Translator`/sidecar giả:

```bash
cd app/src-tauri
cargo test -p app translator
```

## Test tích hợp với sidecar + model thật (xác nhận SC-001)

Cần chuẩn bị sẵn executable `translate-sidecar` (build từ `app/translate-sidecar`) và model `gaudi/opus-mt-en-vi-ctranslate2` cục bộ, giống PBI 16 (xem `.specify/specs/16/quickstart.md`).

```bash
export SORA_TRANSLATE_SIDECAR_PATH=/duong-dan/toi/translate-sidecar
export SORA_TRANSLATE_MODEL_PATH=/duong-dan/toi/model-dir
cd app/src-tauri
cargo test -p app translator -- --ignored
```

Kịch bản xác nhận SC-001 (đại từ có ngữ cảnh dịch tốt hơn): dịch câu hiện tại `"He was very upset."` với ngữ cảnh `["John lost his keys."]`, so sánh với bản dịch khi không có ngữ cảnh — ghi lại cả 2 kết quả làm bằng chứng (ảnh/log output test).
