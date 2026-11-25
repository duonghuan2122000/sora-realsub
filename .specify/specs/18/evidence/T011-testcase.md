# Test case — T011: build & test trên Linux không cần `wasapi`

**Mục tiêu kiểm thử**: FR-001, SC-003 — `cargo build`/`cargo test` chạy được
trên Linux không cài `wasapi`.

## Input

- Môi trường: container Linux (Ubuntu 24.04, không cài `wasapi`/thư viện
  Windows-only), đã cài hệ thống lib cần cho build: `libpulse-dev`,
  `libasound2-dev`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`,
  `libayatana-appindicator3-dev`, `librsvg2-dev`, `libsoup-3.0-dev`,
  `libjavascriptcoregtk-4.1-dev` (dependency chuẩn của Tauri 2 trên Linux,
  không liên quan code PBI 18).
- Lệnh: `cd app/src-tauri && cargo build` rồi `cargo test`.

## Expected

- `cargo build` biên dịch xong, không có lỗi liên quan `wasapi`/`windows`.
- `cargo test` chạy toàn bộ test suite, không fail (có thể có test `ignored`
  do thiếu model/sidecar dịch — không liên quan PBI này).

## Actual

- `cargo build`: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1m 01s` — thành công.
- `cargo test`: `test result: ok. 60 passed; 0 failed; 2 ignored` (unit tests
  `app_lib`) + toàn bộ integration test khác đều `ok`. Test mới
  `core::loopback_audio_source_linux::tests::stop_tren_handle_khong_panic_va_dung_nhan_mau`
  pass (`ok`) — container không có PulseAudio/PipeWire server thật nên hàm
  `start()` trả lỗi và test tự bỏ qua theo thiết kế (`let Ok(handle) = ... else { return; }`),
  không fail.
- Log đầy đủ: `T011-cargo-test-result.log` (cùng thư mục).

## Kết quả: **PASS**
