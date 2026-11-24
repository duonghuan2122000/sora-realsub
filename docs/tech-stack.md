# Sora RealSub - Tech Stack

## Nguyên tắc

- App desktop, **core viết bằng Rust**.
- UI hiện đại, có **style guideline dùng chung** (design tokens) để mọi màn hình nhất quán.
- Hỗ trợ **Windows** trước; macOS/Linux đưa vào roadmap.
- **Khởi động dưới 1 giây.**
- **Core logic và UI tách biệt** (qua IPC Tauri commands/events), không trộn logic nghiệp vụ vào tầng UI, dễ mở rộng/thay engine sau này.

## Tổng quan kiến trúc

```
+---------------------------+      IPC (Tauri commands/events)      +-----------------------+
|  UI (Vue 3 + TypeScript)  | <-----------------------------------> |  Core (Rust)          |
|  Cửa sổ chính, phụ đề nổi |                                       |  audio, VAD, STT,     |
+---------------------------+                                       |  dịch, lưu transcript |
                                                                    +-----------------------+
```

## Core (Rust)

| Thành phần | Lựa chọn | Ghi chú |
|---|---|---|
| Shell desktop | Tauri 2 | Core Rust, UI web nhẹ, đóng gói nhỏ hơn Electron |
| Async runtime | tokio | Pipeline streaming |
| Bắt âm thanh | cpal (WASAPI loopback + mic) | Windows trước; macOS dùng ScreenCaptureKit sau |
| Resample/xử lý | rubato | Chuẩn hoá về 16 kHz mono cho STT |
| VAD | Silero VAD qua ort (ONNX Runtime) | Cắt đoạn có tiếng nói |
| STT local | whisper-rs (whisper.cpp) | Tăng tốc GPU nếu có |
| STT/dịch cloud | HTTP/WebSocket (reqwest, tokio-tungstenite) | Azure Speech Translation, Deepgram + LLM |
| Dịch local | LLM nhỏ hoặc NLLB (ONNX/ct2) | Cho chế độ offline |
| Lưu trữ | SQLite (rusqlite/sqlx) | Transcript, cấu hình, lịch sử họp |
| Cấu hình/bí mật | serde + keyring | API key lưu trong Windows Credential Manager |
| Log | tracing | |

Thiết kế core theo trait để thay engine dễ dàng:
`AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`. Cloud và local chỉ là hai implementation khác nhau.

## UI

| Thành phần | Lựa chọn |
|---|---|
| Framework | Vue 3 + TypeScript |
| Build/package | Vite + Bun |
| Styling | SCSS (thay CSS thuần), cấu hình bằng design tokens, class đặt tên BEM tiền tố `sora-*` |
| Component | shadcn-vue (Reka UI) làm nền, tuỳ biến theo guideline |
| Icon | Lucide |
| State | Pinia |
| i18n | vue-i18n (giao diện Việt/Anh) |

### Cửa sổ

- **Cửa sổ chính**: chọn nguồn âm thanh, ngôn ngữ, engine; xem transcript; lịch sử.
- **Cửa sổ phụ đề nổi**: trong suốt, luôn nằm trên cùng, kéo thả được, click xuyên (tuỳ chọn).

## Style guideline dùng chung

Đặt trong `ui/src/styles/tokens.scss` và `docs/style-guide.md`; mọi component chỉ dùng token, không hardcode giá trị.

- **CSS**: dùng SCSS, không viết CSS thuần. Class đặt tên theo **BEM** (`block__element--modifier`), tất cả bắt đầu bằng tiền tố `sora-` (vd: `sora-subtitle__text--active`).
- **Thống nhất UI**: mọi màn hình/cửa sổ dùng chung bộ component, token, layout pattern; không tự chế style riêng lẻ cho từng màn hình.
- **Màu**: bảng màu semantic (`--color-bg`, `--color-surface`, `--color-text`, `--color-primary`, `--color-danger`...), hỗ trợ light/dark.
- **Typography**: một font UI + một font mono; thang cỡ chữ cố định (xs, sm, base, lg, xl).
- **Spacing**: thang 4px (4, 8, 12, 16, 24, 32).
- **Bo góc, đổ bóng, z-index**: định nghĩa theo token.
- **Phụ đề**: cỡ chữ, độ đậm, màu nền/viền, độ trong suốt chỉnh được; mặc định độ tương phản cao để đọc khi chia sẻ màn hình.
- **Trạng thái**: quy ước chung cho loading, lỗi, rỗng, mất kết nối.
- **Trợ năng**: tương phản tối thiểu WCAG AA, hỗ trợ bàn phím.

## Nền tảng hỗ trợ

| OS | Trạng thái | Ghi chú |
|---|---|---|
| Windows 10/11 | Hỗ trợ (giai đoạn 1) | WASAPI loopback, WebView2 |
| macOS | Roadmap | ScreenCaptureKit hoặc BlackHole để lấy âm thanh hệ thống |
| Linux | Roadmap | PipeWire/PulseAudio monitor |

Giữ phần đặc thù OS sau trait `AudioSource` và module `platform/` để mở rộng sau này không phải sửa pipeline.

## Đóng gói và phân phối

- Tauri bundler: MSI/NSIS cho Windows.
- Model local tải về lần đầu, không đóng gói sẵn trong installer.
- Ký số (code signing) trước khi phát hành.

## Rủi ro cần lưu ý

- Độ trễ end-to-end: mục tiêu dưới 2 giây ở chế độ cloud.
- Thời gian khởi động: mục tiêu dưới 1 giây, cần lazy-load model/engine nặng sau khi UI hiện.
- Chi phí GPU và dung lượng model ở chế độ local.
- Quyền riêng tư: cần thông báo rõ khi âm thanh được gửi lên cloud; cho phép chế độ chỉ chạy local.
- Một số nền tảng họp có thể cảnh báo về việc ghi âm: cần có thông báo cho người tham dự.
