---
title: Quyết định OS mục tiêu (PBI 1)
date: 2026-11-24
tags: [decision, platform]
sources: [.specify/specs/1/spec.md, .specify/specs/1/plan.md, docs/tech-stack.md]
---

# Quyết định OS mục tiêu (PBI 1)

## Quyết định

**Windows là OS mục tiêu đầu tiên.** macOS triển khai sau, nằm ở Backlog (`roadmap.md`), không block các milestone M1–M8.

## Lý do

- Đối tượng dùng chính của Sora RealSub (họp/stream công ty) phổ biến Windows.
- WASAPI loopback (Windows) là API sẵn có, ổn định, không cần driver phụ — khác với macOS cần BlackHole hoặc ScreenCaptureKit, phức tạp hơn để bắt âm thanh hệ thống.
- Tauri 2 hỗ trợ build đa nền tảng sẵn nên quyết định này chỉ ảnh hưởng **thứ tự ưu tiên**, không loại trừ macOS vĩnh viễn.

## Tác động

- Các PBI thuộc M1 (Audio Capture) có thể lập kế hoạch kỹ thuật ngay dựa trên WASAPI, không cần trừu tượng hoá đa-OS từ đầu.
- Trait `AudioSource` + module `platform/` vẫn giữ để mở rộng macOS sau mà không phải sửa pipeline.

## Nơi ghi nhận

- Spec/plan/tasks: `.specify/specs/1/`
- Tài liệu dự án: `docs/tech-stack.md` → mục "Nền tảng hỗ trợ", dòng "Quyết định (PBI 1)".
- `roadmap.md` M0, mục "Chốt OS mục tiêu" → đã tick.

## Liên quan

[[Cau-hoi-con-mo]], [[Tong-quan-du-an]], [[Rui-ro-ky-thuat]]
