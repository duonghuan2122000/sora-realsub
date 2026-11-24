---
title: Quyết định cặp ngôn ngữ mặc định (PBI 2)
date: 2026-11-24
tags: [decision, language]
sources: [.specify/specs/2/spec.md, .specify/specs/2/plan.md, docs/tech-stack.md]
---

# Quyết định cặp ngôn ngữ mặc định (PBI 2)

## Quyết định

**Cặp ngôn ngữ mặc định (fallback) là nguồn `en-US` (English) → đích `vi-VN` (Vietnamese).** Dùng khi người dùng chưa cấu hình ngôn ngữ; người dùng đổi được qua UI (M6), không bị khoá cứng.

## Lý do

- Đối tượng dùng chính của Sora RealSub (họp/stream) thường nghe nội dung tiếng Anh và cần phụ đề tiếng Việt.
- Mã ngôn ngữ theo chuẩn BCP-47 (`en-US`, `vi-VN`) để các provider STT/Dịch ở M3/M4 dùng trực tiếp.

## Tác động

- Trait `Translator`, `SpeechToText` (M3/M4) có giá trị mặc định/fallback rõ ràng khi lập kế hoạch, không cần hỏi lại "ngôn ngữ mặc định là gì".
- Việc chọn/đổi cặp ngôn ngữ qua UI vẫn thuộc M6, không nằm trong PBI này.
- Không có tự động phát hiện ngôn ngữ nói — ngoài phạm vi hiện tại.

## Nơi ghi nhận

- Spec/plan/tasks: `.specify/specs/2/`
- Tài liệu dự án: `docs/tech-stack.md` → mục "Cặp ngôn ngữ mặc định".
- `roadmap.md` M0, mục "Chốt cặp ngôn ngữ mặc định" → đã tick.

## Liên quan

[[Cau-hoi-con-mo]], [[Tong-quan-du-an]], [[Kien-truc-core-ui]], [[Quyet-dinh-OS-muc-tieu]]
