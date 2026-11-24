---
title: Kiến trúc Core/UI
date: 2026-11-24
tags: [concept, architecture]
sources: [docs/tech-stack.md]
---

# Kiến trúc Core/UI

```
+---------------------------+      IPC (Tauri commands/events)      +-----------------------+
|  UI (Vue 3 + TypeScript)  | <-----------------------------------> |  Core (Rust)          |
|  Cửa sổ chính, phụ đề nổi |                                       |  audio, VAD, STT,     |
+---------------------------+                                       |  dịch, lưu transcript |
                                                                     +-----------------------+
```

## Nguyên tắc cứng
- Core logic và UI **tách biệt hoàn toàn**, giao tiếp qua IPC Tauri commands/events — không trộn logic nghiệp vụ vào UI.
- Core viết bằng Rust, thiết kế theo trait để thay engine (cloud/local) dễ dàng: `AudioSource`, `SpeechToText`, `Translator`, `SubtitleSink`.
- Phần đặc thù OS nằm sau trait `AudioSource` + module `platform/` để mở rộng macOS/Linux sau không phải sửa pipeline.
- Khởi động dưới 1 giây: UI hiện ngay, model/engine nặng lazy-load sau.

## Shell desktop
Tauri 2 — core Rust, UI web nhẹ, đóng gói nhỏ hơn Electron. (Overview doc còn nêu phương án Electron+Vue hoặc WPF/.NET+NAudio, nhưng tech-stack.md đã **chốt Tauri 2** — xem lưu ý mâu thuẫn dưới.)

> **Mâu thuẫn tài liệu**: `docs/sora-realsub-overview.md` (giai đoạn ý tưởng) còn để mở "Tauri hoặc Electron... hoặc WPF/.NET"; `docs/tech-stack.md` (mới hơn, chi tiết hơn) đã chốt Tauri 2 + Rust + Vue 3. Coi tech-stack.md là nguồn đúng hiện tại.

## Liên quan
[[Core-Rust]], [[UI-Vue]], [[Pipeline-xu-ly-audio]], [[Cau-hoi-con-mo]]
