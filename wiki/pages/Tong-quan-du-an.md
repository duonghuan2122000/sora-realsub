---
title: Tổng quan dự án
date: 2026-11-24
tags: [overview]
sources: [docs/sora-realsub-overview.md]
---

# Tổng quan dự án

**Sora RealSub** — app desktop dịch tự động và liên tục cuộc họp, hiển thị phụ đề dịch theo thời gian thực.

- Tên: "Sora" (thương hiệu) + "RealSub" = Real(time) + Sub(title). Repo/gói: `sora-realsub`.
- Mục tiêu: nghe âm thanh cuộc họp (người khác nói + mic), dịch liên tục, hiển thị phụ đề cửa sổ nổi, lưu transcript sau họp.

## Luồng xử lý
Xem [[Pipeline-xu-ly-audio]].

## Hai hướng triển khai
| Hướng | Công nghệ | Ưu điểm | Nhược điểm |
|---|---|---|---|
| Cloud | Azure Speech Translation, hoặc Deepgram + LLM | Độ trễ thấp (~1–2s), chất lượng tốt, dễ làm | Tốn phí, âm thanh gửi ra ngoài |
| Local/offline | whisper.cpp / faster-whisper + NLLB hoặc LLM nhỏ | Riêng tư, không tốn phí giờ | Cần GPU khá, độ trễ cao hơn |

Chọn framework app: Tauri (đã chốt, xem [[Kien-truc-core-ui]]).

## Liên quan
[[Cau-hoi-con-mo]], [[Rui-ro-ky-thuat]], [[Kien-truc-core-ui]]
