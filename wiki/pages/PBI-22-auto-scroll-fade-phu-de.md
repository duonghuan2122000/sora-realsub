---
title: PBI 22 - Auto-scroll & fade dòng phụ đề cũ
date: 2026-11-25
tags: [pbi, m5, ui, vue]
sources: [.specify/specs/22/spec.md, .specify/specs/22/plan.md, .specify/specs/22/tasks.md, .specify/specs/22/evidence/]
---

# PBI 22 — Auto-scroll & fade dòng phụ đề cũ (M5, mục 3/6)

Mục thứ ba của **M5 — Cửa sổ phụ đề (Subtitle Overlay UI)**, nối tiếp [[PBI-21-hien-thi-phu-de]] (vốn chỉ giữ đúng 1 dòng hiện tại, ghi đè mỗi khi có event mới).

## Mục tiêu

Giữ lại N=3 dòng phụ đề gần nhất thay vì chỉ 1 dòng; mỗi khi có event `subtitle-ready` mới, dòng mới thêm vào cuối, các dòng cũ cuộn lên và mờ dần theo độ cũ, dòng vượt quá N bị loại khỏi khung nhìn.

## Cài đặt

- `app/src/subtitle/SubtitleApp.vue`: thay `currentSubtitle: Ref<string>` (PBI 21) bằng `lines: Ref<SubtitleLine[]>` (`{ id, text }`, `id` là counter tự tăng dùng làm key ổn định cho `v-for`). Mỗi event `subtitle-ready` không rỗng → push dòng mới, cắt mảng `slice(-3)` (hằng số `MAX_LINES = 3`).
- Template: `v-for="(line, idx) in lines" :key="line.id"`, gán class `subtitle-text--age-N` theo khoảng cách tới dòng mới nhất (`lines.length - 1 - idx`).
- CSS: 3 class `--age-0/1/2` với `opacity` giảm dần (1 → 0.6 → 0.35) + `transition: opacity/transform 0.3s ease` để cuộn/mờ mượt, không dùng thư viện animation ngoài (theo `research.md`, tránh thêm dependency).
- Không đụng Core/Rust — tiếp tục tiêu thụ nguyên event `subtitle-ready` đã có từ [[PBI-19-noi-stt-dich-subtitle-ready]], đúng tách biệt Core/UI theo [[Kien-truc-core-ui]].
- Test `app/src/subtitle/SubtitleApp.spec.ts`: sửa test thứ tự 20 event để kỳ vọng đúng 3 dòng cuối, thêm test loại dòng cũ nhất khi vượt N, thêm test xác nhận class độ mờ theo thứ tự cũ→mới; giữ nguyên test "ẩn khi rỗng" từ PBI 21.

## Trạng thái xác nhận

- `bun run test -- SubtitleApp.spec.ts` (vitest): **5/5 test pass** — log tại `.specify/specs/22/evidence/T007-result.log`, chi tiết case tại `.specify/specs/22/evidence/T007-testcase.md`.
- Lưu ý vận hành: project dùng script `"test": "vitest run"` trong `package.json` — `bun test` (bun's own runner) KHÔNG chạy được vitest config (lỗi `WeakMap keys must be objects`); phải dùng `bun run test` để gọi đúng vitest.
- Chỉ xác nhận bằng unit test jsdom, chưa có xác nhận tay trên GUI thật (tương tự PBI 21, không cần GUI để verify hành vi cuộn/mờ về mặt dữ liệu/class).
- Roadmap M5 mục "Auto-scroll/fade dòng cũ khi có dòng mới..." đã tick.

## Liên quan

- Thuộc [[Pipeline-xu-ly-audio]] ở công đoạn hiển thị cuối cùng, mở rộng từ 1 dòng (PBI 21) sang N=3 dòng có hiệu ứng.
- Tuân [[Kien-truc-core-ui]]: toàn bộ thay đổi ở tầng UI (Vue), không logic nghiệp vụ mới trong UI.
- Giá trị N=3 là hằng số cố định trong phiên bản này — việc cho người dùng tuỳ chỉnh số dòng để dành M6 (Điều khiển & cấu hình).
