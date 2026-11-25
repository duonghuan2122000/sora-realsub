# Nghiên cứu kỹ thuật — PBI 18: Loopback đa hệ điều hành

## Quyết định 1: Cách gate dependency theo OS trong Cargo.toml

- **Quyết định**: Chuyển `wasapi = "0.26"` từ mục `[dependencies]` chung sang mục
  `[target.'cfg(windows)'.dependencies]`. Thêm các dependency cho Linux loopback
  vào `[target.'cfg(target_os = "linux")'.dependencies]`.
- **Lý do**: Cargo hỗ trợ sẵn cơ chế platform-specific dependencies qua
  `[target.'cfg(...)'.dependencies]` — không cần feature flag thủ công, không
  cần thay đổi code gọi (`cargo build`/`cargo test` tự loại bỏ crate không áp
  dụng cho target hiện tại). Đây là cách chuẩn, tối thiểu thay đổi.
- **Phương án khác đã xem xét**:
  - Dùng Cargo feature (`[features] windows-audio = ["wasapi"]`) rồi bắt người
    dùng tự truyền `--features`: bị loại vì yêu cầu người chạy `cargo build`
    nhớ truyền cờ đúng OS, dễ quên, không tự động.
  - Giữ `wasapi` ở `[dependencies]` và chỉ gate code bằng `#[cfg(windows)]`:
    bị loại vì `wasapi` chỉ build được trên Windows — nếu không gate ở
    Cargo.toml, `cargo build`/`cargo metadata` trên Linux vẫn cố biên dịch
    crate này và lỗi trước khi tới bước `#[cfg]` trong code.

## Quyết định 2: Thư viện bắt loopback trên Linux

- **Quyết định**: Dùng `libpulse-binding` + `libpulse-simple-binding` (binding
  Rust cho PulseAudio client API, qua API đơn giản `psimple::Simple`), mở một
  recording stream từ nguồn `@DEFAULT_SINK@.monitor` (monitor source của sink
  output mặc định).
- **Lý do**:
  - PipeWire trên các distro desktop hiện đại (Ubuntu, Fedora ...) đi kèm lớp
    tương thích `pipewire-pulse`, nên một ứng dụng dùng PulseAudio client API
    vẫn hoạt động được trên cả hệ thống PulseAudio thật và PipeWire — khớp với
    giả định "PulseAudio hoặc PipeWire" trong spec.md.
  - Khái niệm monitor source (`<sink>.monitor`) là cách chuẩn, phổ biến nhất để
    "nghe lại" âm thanh đang phát ra loa trên Linux — tương đương vai trò của
    WASAPI loopback trên Windows.
  - `libpulse-binding`/`libpulse-simple-binding` là binding Rust chủ động
    duy trì, dùng rộng rãi, API đơn giản phù hợp nhu cầu đọc PCM tuần tự.
- **Phương án khác đã xem xét**:
  - Dùng `cpal` (đã có sẵn trong dependencies) với host ALSA mặc định: bị loại
    vì `cpal` trên Linux chỉ expose input device qua ALSA, không có khái niệm
    "monitor source" tường minh — việc chọn đúng monitor source phụ thuộc cấu
    hình PulseAudio/PipeWire, khó làm chắc chắn qua lớp ALSA generic.
  - Gọi trực tiếp PipeWire native API (`pipewire-rs`): bị loại vì phức tạp hơn
    cần thiết cho phạm vi PBI này, và không tương thích hệ thống chỉ chạy
    PulseAudio thuần (không có PipeWire).
  - Shell ra ngoài gọi `parecord`/`pw-record`: bị loại vì phụ thuộc binary bên
    ngoài có thể không có sẵn, khó kiểm soát lỗi và luồng dữ liệu real-time.

## Quyết định 3: Cấu trúc code

- **Quyết định**: Giữ nguyên trait `AudioSource` (không đổi interface). Thêm
  file mới `app/src-tauri/src/core/loopback_audio_source_linux.rs` chứa
  implementation `LoopbackAudioSource` riêng cho Linux (cùng tên struct, khác
  nội dung `start()`), gate bằng `#[cfg(target_os = "linux")]` ở `mod.rs`; file
  hiện có `loopback_audio_source.rs` (bản WASAPI) gate bằng
  `#[cfg(windows)]`.
- **Lý do**: Giữ nguyên FR-005 (cùng giao diện cho cả hai OS) mà không cần
  phần còn lại của pipeline (`capture_session`, `TranscriptPipeline`...) biết
  đang chạy trên OS nào. Tách file theo OS giúp mỗi file chỉ chứa logic của
  1 OS, dễ đọc, dễ test riêng.
- **Phương án khác đã xem xét**:
  - Một file dùng `#[cfg]` nội bộ xen trong cùng hàm `start()`: bị loại vì
    làm code khó đọc, trộn lẫn hai luồng logic không liên quan.
