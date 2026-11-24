# Sora RealSub

Desktop app phụ đề thời gian thực cho họp/stream: bắt âm thanh hệ thống/mic, nhận dạng giọng nói, dịch, hiển thị phụ đề nổi. Core viết bằng Rust (Tauri 2), UI Vue 3 + TypeScript.

Chi tiết kiến trúc, tech stack, style guide: xem [docs/](docs/).

## Cấu trúc

```
app/
  src/          # UI (Vue 3 + TypeScript)
  src-tauri/    # Core (Rust)
```

## Phát triển

```bash
cd app
bun install
bun run tauri dev      # chạy app
bun run lint            # lint UI
bun run test            # unit test UI
cd src-tauri
cargo clippy            # lint core
cargo test              # unit test core
```
