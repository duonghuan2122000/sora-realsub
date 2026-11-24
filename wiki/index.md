# Index

## Overview
- [[Tong-quan-du-an]] — mục tiêu, luồng xử lý, 2 hướng cloud/local

## Concepts
- [[Pipeline-xu-ly-audio]] — audio capture → VAD → STT → dịch → hiển thị
- [[Kien-truc-core-ui]] — tách Core Rust / UI Vue qua IPC Tauri, trait AudioSource/SpeechToText/Translator/SubtitleSink

## Components
- [[Core-Rust]] — crate, thư viện, lưu trữ
- [[UI-Vue]] — framework, state, i18n, cửa sổ
- [[Style-guide-token]] — design token, BEM `sora-*`, quy ước class trạng thái

## Decisions
- [[Cau-hoi-con-mo]] — câu hỏi mở còn lại (đã chốt OS, ngôn ngữ, hướng xử lý)
- [[Quyet-dinh-OS-muc-tieu]] — Windows trước, macOS Backlog (PBI 1, đã chốt)
- [[Quyet-dinh-ngon-ngu-mac-dinh]] — cặp ngôn ngữ mặc định en-US → vi-VN (PBI 2, đã chốt)
- [[Quyet-dinh-hop-dong-core]] — 4 trait Core (AudioSource/SpeechToText/Translator/SubtitleSink), chỉ interface (PBI 3, đã chốt)
- [[Rui-ro-ky-thuat]] — độ trễ, khởi động, chi phí GPU, riêng tư

## Trạng thái code hiện tại
- [[Trang-thai-ma-nguon]] — snapshot app/ (2026-11-24, cập nhật sau PBI 3): interface Core xong (4 trait), chưa có logic nghiệp vụ thật
