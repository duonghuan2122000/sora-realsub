# CLAUDE.md

File này hướng dẫn Claude Code (claude.ai/code) khi làm việc với code trong repo này.

## Dự án

Sora RealSub — app desktop phụ đề thời gian thực cho họp/stream. Bắt âm thanh hệ thống (loopback) + mic, VAD, STT streaming, dịch, cửa sổ phụ đề nổi always-on-top, lưu transcript sau họp.

Core = Rust (Tauri 2), UI = Vue 3 + TypeScript, giao tiếp qua Tauri IPC commands/events. Core và UI phải tách biệt — không đưa logic nghiệp vụ vào tầng UI.

**Nguồn kiến thức dự án — đọc wiki trước:** wiki ở `wiki/` (bắt đầu từ `wiki/index.md`) là nguồn chính, đã tổng hợp và cross-reference sẵn — luôn đọc wiki trước khi cần hiểu lệnh dev, kiến trúc, pipeline, trait Core, quy ước style. `docs/` (`sora-realsub-overview.md`, `tech-stack.md`, `style-guide.md`) chỉ là tài liệu gốc để bổ trợ/đối chiếu khi wiki thiếu chi tiết.

**Trạng thái hiện tại**: `app/` vẫn là skeleton mặc định của `create-tauri-app` — chưa có code pipeline audio/VAD/STT/dịch nào.

## Quy tắc commit

- Commit message theo chuẩn Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, ...), nội dung tiếng Việt có dấu.
- **Không** thêm trailer `Co-Authored-By` vào commit message.
