# Sora RealSub Wiki — schema (LLM-wiki)

Wiki cho app Sora RealSub (desktop, dịch phụ đề realtime, Tauri 2 + Rust core + Vue 3 UI).

## Nguồn

- `docs/*.md` (overview, tech-stack, style-guide) — raw, bất biến.
- Mã nguồn app (`app/src`, `app/src-tauri`) — đọc trực tiếp từ repo khi cần, không copy vào wiki/raw.
- README.md root.

## Cấu trúc page

- `pages/concepts/` — khái niệm kiến trúc (pipeline audio→VAD→STT→dịch→hiển thị, trait core, IPC).
- `pages/components/` — thành phần cụ thể (Core Rust, UI Vue, Style tokens, Tauri shell).
- `pages/decisions/` — quyết định kỹ thuật đã chốt + câu hỏi còn mở.

## Quy ước

- Page có frontmatter: title, date, tags, sources.
- Cross-reference bằng `[[Tên Page]]`.
- Trạng thái mã nguồn (app/) thay đổi nhanh — mỗi khi ingest lại, ghi rõ ngày kiểm tra code để tránh page cũ nói sai hiện trạng.

## Workflow

Theo SKILL.md sora-wiki: ingest nguồn mới → cập nhật page liên quan + index + log. Query → đọc index.md trước.
