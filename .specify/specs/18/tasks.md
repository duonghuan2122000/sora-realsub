# Danh sách Task: Bắt âm thanh loopback đa hệ điều hành (Windows + Linux)

**Mã PBI**: 18
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

Spec.md không tách nhiều user story theo mức ưu tiên riêng — toàn bộ 3 kịch
bản chấp nhận thuộc cùng một tính năng, nên gộp thành **User Story 1 (P1)**
duy nhất.

## Pha 1: Setup

- [X] T001 Gate `wasapi = "0.26"` trong `app/src-tauri/Cargo.toml` vào mục
  `[target.'cfg(windows)'.dependencies]` (xóa khỏi `[dependencies]` chung).
- [X] T002 Thêm `libpulse-binding` và `libpulse-simple-binding` vào mục
  `[target.'cfg(target_os = "linux")'.dependencies]` trong
  `app/src-tauri/Cargo.toml`.
- [X] T003 Cập nhật `wiki/pages/Rui-ro-ky-thuat.md` (nếu tồn tại) để đổi trạng
  thái rủi ro "`wasapi` không gate theo target" sang "đã xử lý ở PBI 18"
  (ghi chú ngắn, không viết lại toàn bài).

## Pha 2: Foundational

- [X] T004 Thêm `#[cfg(windows)]` ở đầu file
  `app/src-tauri/src/core/loopback_audio_source.rs` (toàn bộ nội dung file
  hiện có — bản WASAPI — chỉ compile trên Windows).
- [X] T005 Tạo file mới `app/src-tauri/src/core/loopback_audio_source_linux.rs`
  với `#[cfg(target_os = "linux")]` ở đầu file, khai báo struct
  `LoopbackAudioSource` rỗng (chưa implement `start()`) để chuẩn bị cho US1.
- [X] T006 Cập nhật `app/src-tauri/src/core/mod.rs`: khai báo module
  `loopback_audio_source` với `#[cfg(windows)]` và module
  `loopback_audio_source_linux` với `#[cfg(target_os = "linux")]`; `pub use`
  `LoopbackAudioSource` từ module tương ứng theo từng `#[cfg(...)]` (chỉ một
  trong hai được export tùy OS build).

## Pha 3: User Story 1 — Loopback hoạt động trên cả Windows và Linux (Ưu tiên: P1)

**Mục tiêu**: Người dùng/lập trình viên build, test và capture loopback được
trên cả Windows và Linux, không ai bị chặn bởi dependency chỉ dành riêng cho
OS khác (FR-001 đến FR-005).

**Tiêu chí kiểm thử độc lập**: Chạy đúng các bước trong `quickstart.md` —
`cargo test` pass trên Linux không cần `wasapi`; ghi `.wav` loopback trên
Linux nghe đúng nội dung đã phát; test cũ trên Windows không đổi kết quả.

- [X] T007 [US1] Implement `AudioSource for LoopbackAudioSource` trong
  `app/src-tauri/src/core/loopback_audio_source_linux.rs`: dùng
  `libpulse_simple_binding::Simple` mở recording stream từ nguồn
  `@DEFAULT_SINK@.monitor` (mono, 16-bit hoặc float theo mẫu có sẵn của
  `cpal`/`hound` trong dự án), decode mỗi khối đọc được thành `AudioFrame` và
  gọi `on_frame`, chạy trên 1 thread riêng (theo pattern `thread::spawn` +
  `mpsc` đã dùng trong `loopback_audio_source.rs` bản Windows).
- [X] T008 [US1] Trong cùng file `loopback_audio_source_linux.rs`: khi mở
  stream PulseAudio thất bại (không có service, không có monitor source khả
  dụng), trả về `Err(CoreError::Io(...))` với thông báo rõ ràng, không panic
  (đáp ứng FR-004 và trường hợp biên "không có PulseAudio/PipeWire chạy").
- [X] T009 [US1] Trong `StreamHandle` trả về từ `start()` (file
  `loopback_audio_source_linux.rs`): đảm bảo hàm dừng (`stop`) gửi tín hiệu
  dừng thread và `join()` an toàn, không leak thread/handle — theo cùng
  nguyên tắc đã kiểm thử ở PBI 5 cho Windows.
- [X] T010 [P] [US1] Thêm test đơn vị trong
  `app/src-tauri/src/core/loopback_audio_source_linux.rs`
  (`#[cfg(test)] mod tests`): test `stop` trên handle không panic và dừng
  nhận mẫu — tự bỏ qua (return sớm) nếu máy test không kết nối được
  PulseAudio/PipeWire (theo đúng pattern
  `stop_tren_handle_khong_panic_va_dung_nhan_mau` đã có ở bản Windows).
- [X] T011 [P] [US1] Chạy `cargo build` và `cargo test` tại `app/src-tauri`
  trên máy Linux (container hiện tại) — xác nhận build/test pass, không có
  lỗi liên quan `wasapi`; nếu thiếu `libpulse-dev`/`pkg-config` ở hệ thống,
  cài đặt trước khi chạy lại.
- [X] T012 [US1] Thực hiện kịch bản thủ công trong `quickstart.md` mục "Trên
  Linux": phát âm thanh test, chạy `capture_loopback_to_wav` (file
  `app/src-tauri/src/bin/capture_loopback_to_wav.rs`, không cần sửa vì chỉ
  phụ thuộc trait `AudioSource`), nghe lại file `.wav` ghi ra để xác nhận nội
  dung đúng — chụp ảnh/log kết quả làm bằng chứng test.
- [X] T013 [US1] Nếu có máy Windows khả dụng: chạy lại `cargo test` và kịch
  bản `quickstart.md` mục "Trên Windows" để xác nhận SC-002 (0 regression).
  Nếu không có máy Windows trong phạm vi hiện tại, ghi rõ trong báo cáo
  implement là "chưa xác nhận trên Windows thật" (tương tự cách PBI 17 đã ghi
  chú) — không tick dòng roadmap tương ứng cho tới khi xác nhận được.
  → **Không có máy Windows trong môi trường hiện tại** (container Linux
  cloud session). Chưa xác nhận SC-002 bằng máy Windows thật; code bản
  WASAPI (`loopback_audio_source.rs`) chỉ thay đổi bằng việc thêm
  `#[cfg(windows)]` bọc ngoài, không sửa logic bên trong, nên về lý thuyết
  hành vi không đổi — nhưng cần người có máy Windows xác nhận thật trước khi
  tick dòng roadmap M1 tương ứng.

## Pha cuối: Polish & Cross-cutting

- [X] T014 Rà lại toàn bộ comment/docstring trong 2 file
  `loopback_audio_source.rs` và `loopback_audio_source_linux.rs` để đảm bảo
  nhất quán văn phong tiếng Việt như các module khác trong `core/`.
- [X] T015 Chạy `cargo clippy --all-targets` tại `app/src-tauri` trên Linux,
  sửa các warning mới phát sinh từ code thêm ở PBI này (không sửa warning có
  từ trước, ngoài phạm vi). → Chỉ có 1 warning tiền tồn tại ở
  `tests/vad_real_speech.rs` (không liên quan PBI 18), không có warning mới
  từ `loopback_audio_source.rs`/`loopback_audio_source_linux.rs`/`Cargo.toml`
  — không cần sửa gì thêm.

## Sơ đồ phụ thuộc

- Pha 1 (Setup) → Pha 2 (Foundational) → Pha 3 (US1) → Pha cuối.
- T001–T003 độc lập với nhau nhưng phải xong trước T004–T006 (Cargo.toml cần
  đúng trước khi module biên dịch được).
- T004, T005, T006 phải xong trước mọi task trong Pha 3 (mod.rs phải khai báo
  đúng module/cfg trước khi implement bên trong).
- T007 → T008 → T009 theo thứ tự (cùng file, cùng hàm `start()`/`StreamHandle`).
- T010, T011 có thể chạy song song với nhau sau khi T007–T009 xong (khác mục
  tiêu: một là unit test trong file, một là lệnh build/test tổng).
- T012 cần T011 pass trước. T013 độc lập, chạy khi có máy Windows.
- Chỉ có 1 user story (US1) nên không có phụ thuộc giữa nhiều story.

## Ví dụ chạy song song

Sau khi hoàn tất T007–T009 (implement xong `LoopbackAudioSource` cho Linux),
có thể giao song song:
- T010 (viết/chạy unit test trong file)
- T011 (chạy `cargo build`/`cargo test` toàn crate)

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (không chia nhỏ hơn được nữa — đây đã là
  lát cắt nhỏ nhất mang lại giá trị: Linux build/test được + loopback hoạt
  động).
- Thứ tự giao hàng: Setup → Foundational → US1 (implement + test Linux) →
  xác nhận Windows không regression → Polish.
