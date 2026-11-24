---
title: Câu hỏi còn mở
date: 2026-11-24
tags: [decision]
sources: [docs/sora-realsub-overview.md, docs/tech-stack.md]
---

# Câu hỏi còn mở

Từ `docs/sora-realsub-overview.md`, 3 câu hỏi nêu nhưng chưa thấy quyết định ghi lại rõ:

1. ~~Hệ điều hành mục tiêu: Windows hay macOS?~~ → **ĐÃ CHỐT (PBI 1, 2026-11-24)**: xem [[Quyet-dinh-OS-muc-tieu]].
2. Cặp ngôn ngữ dịch (ví dụ Anh → Việt)? → **chưa thấy chốt** trong tài liệu nào.
3. Cuộc họp có nhạy cảm không (quyết định local vs cloud)? → **chưa chốt**, tech-stack.md giữ cả hai hướng (trait implementation riêng).

## Đã chốt (không còn mở)
- Framework: **Tauri 2 + Rust core + Vue 3 UI** (tech-stack.md, mới hơn overview.md).
- Tên app: Sora RealSub / `sora-realsub`.
- Hệ điều hành mục tiêu: **Windows trước, macOS sau (Backlog)** — [[Quyet-dinh-OS-muc-tieu]] (PBI 1).

## Liên quan
[[Tong-quan-du-an]], [[Kien-truc-core-ui]]
