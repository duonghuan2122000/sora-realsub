# Kế hoạch triển khai: Chọn cặp ngôn ngữ nguồn-đích

**Mã PBI**: 29
**Liên kết spec**: .specify/specs/29/spec.md
**Ngày tạo**: 2026-11-27

## Ngữ cảnh kỹ thuật

- Ngôn ngữ / Runtime: Rust (Tauri 2 core) + Vue 3/TypeScript (UI), giao tiếp qua Tauri IPC commands.
- Framework / Thư viện chính: không thêm dependency mới — tái dùng pattern `<select>` như `DeviceSelector.vue` (PBI 26), tái dùng `SidecarTranslator` (PBI 16/17/19).
- Lưu trữ dữ liệu: không có — danh sách cặp ngôn ngữ là hằng số tĩnh trong code Rust; lựa chọn hiện tại chỉ giữ trong state Vue (runtime, không persist).
- Kiểm thử: Rust unit test (`cargo test`) cho command mới + validate `pair_id`; Vitest cho component/khoá UI (như PBI 26/28).
- Nền tảng triển khai: không đổi — vẫn Tauri desktop Windows/Linux hiện có.
- Ràng buộc hiệu năng: không đáng kể (chọn từ danh sách tĩnh ≤ vài phần tử).
- Ràng buộc khác: không phá vỡ hành vi hiện tại khi chưa cấu hình `SORA_TRANSLATE_SIDECAR_PATH`/model (dịch vẫn optional như PBI 19 — thiếu cấu hình thì bỏ qua bước dịch, phiên vẫn chạy).

## Kiểm tra theo hiến pháp dự án

Không có `.specify/memory/constitution.md` trong dự án — áp dụng trực tiếp nguyên tắc tách biệt Core/UI nêu trong `CLAUDE.md`:

| Nguyên tắc | Tuân thủ? | Ghi chú |
|---|---|---|
| Core (Rust) và UI (Vue) tách biệt, giao tiếp qua Tauri IPC | ✅ | Danh sách cặp ngôn ngữ + validate nằm ở `core`/`lib.rs` (Rust), UI chỉ hiển thị + gọi command, không tự biết logic hợp lệ |
| Không đưa logic nghiệp vụ vào tầng UI | ✅ | Validate `pair_id` hợp lệ, tra `model_dir` theo cặp đều ở Rust; UI chỉ khoá ô chọn theo `sessionState` (đã có pattern từ PBI 26/28) |

## Giai đoạn 0 — Kết quả nghiên cứu

Xem chi tiết tại `research.md`: 4 quyết định — (1) danh sách cặp ngôn ngữ là hằng số tĩnh trong `core`, (2) UI dùng 1 ô chọn theo cặp (không phải 2 ô độc lập) để thỏa FR-003 bằng cấu trúc dữ liệu, (3) `start_capture` nhận thêm `pair_id` để tra đúng model dịch, (4) lựa chọn chỉ giữ trong state runtime của phiên app, không persist.

## Giai đoạn 1 — Thiết kế

- Mô hình dữ liệu: xem `data-model.md` — `LanguagePair` (Rust, tĩnh) + state `languagePairs`/`selectedPairId` (Vue).
- Hợp đồng giao diện: 2 command Tauri mới/sửa (coi là hợp đồng nội bộ Core↔UI, không có API bên ngoài nên không tách riêng `contracts/`, theo đúng pattern các PBI trước — PBI 26/28 cũng không có thư mục `contracts/`):
  - `list_language_pairs_cmd() -> Vec<LanguagePairInfo>` (mới) — trả danh sách cặp hỗ trợ.
  - `start_capture(pair_id: String, ...)` (sửa) — thêm tham số `pair_id`; trả `Err` nếu không khớp cặp nào trong danh sách tĩnh.
- Kịch bản khởi động nhanh: xem `quickstart.md`.

## Cấu trúc dự án dự kiến

```
app/src-tauri/src/
  core/
    translator.rs        # thêm hằng số SUPPORTED_LANGUAGE_PAIRS + struct LanguagePair
  lib.rs                  # thêm command list_language_pairs_cmd; sửa start_capture nhận pair_id,
                          # tra model_dir theo pair_id thay vì đọc thẳng SORA_TRANSLATE_MODEL_PATH
app/src/
  components/
    LanguagePairSelector.vue   # (mới) ô chọn cặp ngôn ngữ, pattern giống DeviceSelector.vue
  App.vue                      # thêm state selectedPairId, khoá theo sessionState, truyền pair_id
                                # khi gọi invoke("start_capture", ...)
```

## Rủi ro & ngoại lệ có lý do

- Hiện chỉ có 1 cặp ngôn ngữ thật (Anh→Việt) nên không thể test tay việc "đổi sang cặp khác và nghe kết quả dịch đổi theo" bằng model thật trong môi trường này — kiểm thử SC-001 cho trường hợp ≥ 2 cặp sẽ cần xác nhận lại khi có model dịch thứ hai. Không phải ngoại lệ vi phạm nguyên tắc dự án, chỉ là giới hạn hạ tầng hiện tại (ghi nhận để /sora-task đưa test phù hợp với 1 cặp đang có + test logic chọn lỗi khi `pair_id` không khớp).
