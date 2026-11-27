# Quickstart: Chọn cặp ngôn ngữ nguồn-đích (PBI 29)

1. Chạy app dev: `bun run tauri dev` (trong `app/`).
2. Trên màn hình chính, quan sát ô chọn ngôn ngữ mới cạnh phần chọn thiết bị — mặc định hiển thị "Anh → Việt".
3. Chọn mic + loopback, nhấn Start.
4. Nói tiếng Anh trước mic/loopback → quan sát cửa sổ phụ đề hiển thị bản dịch tiếng Việt (SC-001).
5. Trong lúc phiên đang `running`, thử bấm vào ô chọn ngôn ngữ → xác nhận ô bị khoá, không đổi được (SC-003).
6. Nhấn Stop → ô chọn ngôn ngữ mở lại, đổi được bình thường.

Kiểm thử tự động tương ứng:
- Rust: test `list_language_pairs_cmd` trả đúng danh sách tĩnh; test `start_capture` nhận `pair_id` không hợp lệ → trả lỗi, không start session.
- Vitest: test ô chọn ngôn ngữ bị khoá khi `sessionState !== "idle"`, test mặc định chọn cặp đầu tiên khi tải danh sách xong.
