---
title: Rủi ro kỹ thuật
date: 2026-11-24
tags: [decision, risk]
sources: [docs/tech-stack.md]
---

# Rủi ro kỹ thuật

- **Độ trễ end-to-end**: mục tiêu dưới 2 giây ở chế độ cloud.
- **Thời gian khởi động**: mục tiêu dưới 1 giây — cần lazy-load model/engine nặng sau khi UI hiện.
- **Chi phí GPU & dung lượng model** ở chế độ local.
- **Quyền riêng tư**: cần thông báo rõ khi âm thanh gửi lên cloud; cho phép chế độ chỉ chạy local.
- **Cảnh báo ghi âm**: một số nền tảng họp có thể cảnh báo việc ghi âm — cần thông báo cho người tham dự.

## Đóng gói & phân phối
- Tauri bundler: MSI/NSIS cho Windows.
- Model local tải về lần đầu, không đóng gói sẵn trong installer.
- Ký số (code signing) trước khi phát hành.

## Liên quan
[[Tong-quan-du-an]], [[Pipeline-xu-ly-audio]]
