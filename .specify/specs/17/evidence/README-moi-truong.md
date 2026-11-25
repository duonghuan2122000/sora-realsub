# Ghi chú môi trường thi công PBI 17

Container thi công PBI này là **Linux** (Ubuntu 24.04). Dự án `app/src-tauri` (per `Cargo.toml`)
có dependency `wasapi = "0.26"` **không gate theo target** (`cfg(windows)`) — crate `wasapi` chỉ
build được trên Windows (dùng cho `LoopbackAudioSource`, hoàn thành từ PBI thuộc milestone M1).

Khi chạy `cargo test -p app` hoặc `cargo check -p app --lib` trong container Linux này, build thất
bại ở bước compile `windows-future` (phụ thuộc gián tiếp qua `wasapi → windows`), lỗi:

```
error[E0425]: cannot find type `IMarshal` in module `windows_core::imp`
...
error: could not compile `windows-future` (lib) due to 16 previous errors
```

**Xác nhận đây là lỗi môi trường có từ trước (M1), không liên quan đến thay đổi của PBI 17**:
- `cargo tree -i windows-future` cho thấy chuỗi phụ thuộc duy nhất là
  `windows-future → windows → wasapi → app`, không liên quan gì tới `translator.rs`.
- Thử tạm gỡ gate module `loopback_audio_source` (`#[cfg(windows)]`) để kiểm tra — build tiến xa
  hơn (qua được bước webkit/alsa/onnxruntime đã fix bằng cách cài thêm hệ thống lib + cho phép
  host `cdn.pyke.io`) nhưng dừng lại đúng tại lỗi trên khi build `wasapi`/`windows-future` — xác
  nhận lỗi độc lập với code PBI 17. Thay đổi thử nghiệm này đã **revert hoàn toàn**, không có
  trong commit.
- Không có máy Windows nào được gắn vào session này (`list_computer_folders` trả về rỗng) để
  build/test trực tiếp crate `app` trên Windows.

**Hệ quả cho việc kiểm thử PBI 17 trong container này**:
- **Không thể** chạy `cargo test -p app translator` (T017) hay test tích hợp sidecar thật (T018)
  trong container Linux này — đây là hạn chế môi trường có trước PBI 17, không phải lỗi của code
  PBI 17.
- Để vẫn có bằng chứng thật (không phải suy luận), đã tách các hàm **thuần** (không phụ thuộc
  `Command`/sidecar, không phụ thuộc tauri/wasapi) trong `translator.rs` —
  `normalize_context`, `split_sentences`, `extract_current_translation`,
  `RecentTranscriptContext` — và toàn bộ nhánh quyết định của `translate()` (ghép ngữ cảnh → gọi
  dịch → tách câu → fallback) — sao y nguyên logic (không đổi hành vi) vào một crate Rust độc lập
  tại scratchpad (`pbi17-logic-check`, không commit vào repo), rồi chạy `cargo test` thật trong
  container này. Xem [T017-logic-tests.log](./T017-logic-tests.log): **13/13 test pass**.
- Mã nguồn thật trong `app/src-tauri/src/core/translator.rs` (đã sửa cho PBI 17) chứa đúng các
  test case tương đương (T010-T015, dùng mock `Translator`/`EchoPrefixSidecar`) + 2 test tích hợp
  `#[ignore]` (T007 cũ của PBI 16 và T016 mới của PBI 17) — các test này **chưa được chạy thật**
  trong container này (vừa vì build bị chặn bởi `wasapi`, vừa vì sidecar/model không có sẵn trong
  container mới — giống hạn chế đã ghi nhận ở PBI 16).

**Khuyến nghị**: chạy `cargo test -p app translator` (T017) và test tích hợp `--ignored` (T018)
trên máy phát triển Windows có sẵn sidecar + model (theo `quickstart.md`) để xác nhận cuối cùng
trước khi coi PBI 17 là "đã kiểm thử" theo đúng nghĩa CLAUDE.md. Việc gate `wasapi` theo
`cfg(windows)` trong `Cargo.toml`/`core/mod.rs` để dự án có thể build/test một phần trên Linux là
một cải tiến hạ tầng hợp lý nhưng **ngoài phạm vi PBI 17** — đề xuất tạo PBI riêng nếu dự án muốn
hỗ trợ dev/test trên Linux.
