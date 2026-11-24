# Quickstart: Xử lý lỗi runtime model STT (PBI 15)

Kịch bản kiểm thử nhanh để xác nhận tính năng hoạt động, không cần chạy UI thật.

## 1. Lỗi nạp model (thiếu file)

```powershell
cd app/src-tauri
cargo test speech_to_text -- --nocapture
```

- Test mới: tạo `WhisperSpeechToText::new("duong-dan-khong-ton-tai.bin")`, kỳ vọng trả `Err(CoreError::Provider(..))`.
- Test ở `lib.rs`-level (hoặc tương đương trong `core`): giả lập `stt = None` do lỗi nạp, xác nhận có đúng 1 lần phát `model-error` (loại `load`) và phiên vẫn `start_capture` thành công (không panic).

## 2. Lỗi infer giữa các segment thành công

```powershell
cargo test transcript_pipeline -- --nocapture
```

- Dùng STT giả lập (`FlakySpeechToText`/tương tự) trả `Err` ở segment thứ 2 trong 3 segment, `Ok` ở segment 1 và 3.
- Kỳ vọng: callback lỗi được gọi đúng 1 lần cho segment 2 (`kind == Infer`), `on_chunk` vẫn được gọi cho segment 1 và 3 với kết quả đúng, hàm `process_segments` không trả `Err` sớm (vẫn chạy hết `segments`).

## 3. Không có lỗi — không phát `model-error`

- Test đã có từ PBI 13/14 (`nhieu_segment_tra_ve_text_dung_thu_tu`, `phat_transcript_final_dung_thu_tu`, ...) tiếp tục pass không đổi hành vi — xác nhận callback lỗi mới không được gọi khi không có lỗi.

## Chạy toàn bộ test liên quan

```powershell
cargo test --manifest-path app/src-tauri/Cargo.toml
```

Chụp màn hình/log kết quả `cargo test` lưu vào `.specify/specs/15/evidence/` làm bằng chứng (yêu cầu của `/sora-implement`).
