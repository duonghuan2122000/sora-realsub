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
- [[Cau-hoi-con-mo]] — OS mục tiêu, cặp ngôn ngữ, cloud vs local chưa chốt
- [[Rui-ro-ky-thuat]] — độ trễ, khởi động, chi phí GPU, riêng tư

## Trạng thái code hiện tại
- [[Trang-thai-ma-nguon]] — snapshot app/ lúc ingest (2026-11-24): chỉ là skeleton Tauri 2 + Vue 3 mặc định, chưa có logic nghiệp vụ
