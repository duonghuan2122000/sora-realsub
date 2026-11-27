# Research: Chọn cặp ngôn ngữ nguồn-đích (PBI 29)

## Quyết định 1: Danh sách cặp ngôn ngữ hỗ trợ lấy từ đâu

**Quyết định**: Thêm 1 hằng số Rust (danh sách tĩnh, mỗi phần tử gồm `id`, `source_lang`, `target_lang`, nhãn hiển thị) trong `core`, hiện tại chỉ có đúng 1 phần tử (`en` → `vi`, khớp model dịch đã có từ PBI 16). Expose qua command Tauri mới `list_language_pairs_cmd` để UI tự lấy, không hard-code trong frontend.

**Lý do**: Hiện hệ thống chỉ có 1 model dịch (Anh→Việt, cấu hình qua env `SORA_TRANSLATE_MODEL_PATH`). Theo YAGNI, không cần cơ chế registry động/file cấu hình phức tạp cho 1 phần tử — khi có thêm model dịch khác (PBI tương lai), chỉ cần thêm phần tử vào danh sách tĩnh này, không đổi hợp đồng command.

**Phương án khác đã xem xét**:
- Đọc danh sách cặp ngôn ngữ từ file cấu hình/scan thư mục model lúc runtime: phức tạp hơn cần thiết khi mới có 1 cặp, không có yêu cầu nào trong spec đòi hỏi thêm/sửa cặp ngôn ngữ không cần build lại app.

## Quyết định 2: UI chọn ngôn ngữ — 2 ô độc lập hay 1 ô chọn cặp

**Quyết định**: 1 ô chọn duy nhất liệt kê các **cặp** ngôn ngữ hợp lệ (ví dụ "Anh → Việt"), lấy từ `list_language_pairs_cmd`.

**Lý do**: FR-001/002 yêu cầu chỉ liệt kê cặp được hỗ trợ và chọn được trong phạm vi đó; FR-003 yêu cầu chặn nguồn = đích. Chọn theo cặp đã-biết-hợp-lệ thỏa cả 2 yêu cầu bằng cấu trúc dữ liệu (không có cặp nào trong danh sách có nguồn = đích), không cần thêm logic validate riêng, đúng tinh thần đơn giản nhất còn đáp ứng yêu cầu. Khi tương lai có ≥ 2 ngôn ngữ nguồn và ≥ 2 ngôn ngữ đích thực sự độc lập (ví dụ matrix N×M), có thể tách lại thành 2 ô — không phải quyết định của PBI này.

**Phương án khác đã xem xét**:
- 2 ô `<select>` độc lập (nguồn/đích) + validate nguồn ≠ đích ở UI: thêm code validate không cần thiết khi danh sách cặp hỗ trợ vốn đã hữu hạn và biết trước; để lại khi có nhu cầu thật (nhiều ngôn ngữ nguồn × nhiều ngôn ngữ đích độc lập).

## Quyết định 3: Áp dụng lựa chọn vào `start_capture`

**Quyết định**: Thêm tham số `pair_id: String` vào command `start_capture`. Backend tra `pair_id` trong danh sách tĩnh (quyết định 1); nếu không khớp → trả lỗi rõ ràng (không start phiên). Nếu khớp, dùng đúng cấu hình model của cặp đó để tạo `SidecarTranslator` (thay cho việc đọc thẳng 2 env var cố định như hiện tại).

**Lý do**: Giữ nguyên luồng hiện có (`SidecarTranslator::new(exe, model_dir)`), chỉ đổi nguồn lấy `model_dir`: trước đây đọc trực tiếp từ env, giờ tra theo `pair_id` trong danh sách — đường sidecar exe vẫn qua env `SORA_TRANSLATE_SIDECAR_PATH` (không đổi, không phải thứ người dùng chọn qua UI).

**Phương án khác đã xem xét**: Giữ nguyên đọc translator cấu hình cố định, chỉ validate `pair_id` ở UI — bị loại vì không thỏa SC-001 (kết quả dịch phải đúng cặp đã chọn khi sau này có ≥ 2 cặp thật).

## Quyết định 4: Lưu lựa chọn trong phạm vi phiên app

**Quyết định**: Giữ `pair_id` đã chọn trong state Vue (`ref`) ở `App.vue`, mặc định là cặp đầu tiên trong danh sách trả về từ `list_language_pairs_cmd`. Không lưu xuống file.

**Lý do**: Đúng phạm vi FR-006 + mục "Ngoài phạm vi" của spec — lưu cấu hình bền vững thuộc PBI "Lưu cấu hình" riêng ở M6.

**Phương án khác đã xem xét**: `localStorage` (như PBI 24 dùng cho cỡ chữ) — không cần vì spec chỉ yêu cầu giữ trong phiên làm việc hiện tại, không yêu cầu giữ qua lần mở app sau.
