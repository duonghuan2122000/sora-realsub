---
title: PBI 29 — Chọn cặp ngôn ngữ nguồn-đích
date: 2026-11-27
tags: [pbi, M6, core-rust, ui-vue]
sources: [.specify/specs/29/spec.md, .specify/specs/29/plan.md, .specify/specs/29/research.md, .specify/specs/29/data-model.md, .specify/specs/29/tasks.md, .specify/specs/29/evidence/]
---

# PBI 29 — Chọn cặp ngôn ngữ nguồn-đích

Mục thứ 3 của **M6 (Điều khiển & cấu hình)**, sau [[PBI-28-nut-start-stop-phien-dich]]. Chọn từ roadmap theo đúng thứ tự xuất hiện — 3 mục đứng trước (PBI 18 Linux, hồi quy Windows, PBI 17 ghép ngữ cảnh) đều đã code xong, chỉ còn chờ xác nhận tay trên máy khác, không phải việc code mới (cùng lý do đã áp dụng ở PBI 26/28).

## Quyết định chính

- **Danh sách cặp ngôn ngữ là hằng số tĩnh trong Core** (`SUPPORTED_LANGUAGE_PAIRS: &[LanguagePair]`, `app/src-tauri/src/core/translator.rs`), hiện chỉ 1 phần tử `en-vi` — khớp đúng model dịch duy nhất đang có từ [[PBI-16-dich-local-sidecar]]. YAGNI: không xây registry động/file cấu hình khi chỉ có 1 phần tử thật.
- **UI dùng 1 `<select>` theo cặp**, không phải 2 ô nguồn/đích độc lập — mọi phần tử trong `SUPPORTED_LANGUAGE_PAIRS` vốn đã có `source_lang != target_lang`, nên yêu cầu "chặn chọn trùng ngôn ngữ" được thỏa bằng cấu trúc dữ liệu, không cần thêm logic validate riêng ở UI hay Core.
- **`start_capture` nhận thêm tham số `pair_id: String`**, validate qua hàm thuần `validate_language_pair()` (tách riêng để test không cần `AppHandle`/`State`, giống lý do tách `translate_final_chunk` ở [[PBI-19-noi-stt-dich-subtitle-ready]]) — `pair_id` không khớp cặp nào → trả `Err` ngay, không tạo session.
- **Chưa tra `model_dir` riêng theo `pair_id`** — vì chỉ có 1 cặp/1 model, `SidecarTranslator` vẫn đọc `model_dir` từ `SORA_TRANSLATE_MODEL_PATH` như cũ, chỉ validate `pair_id` khớp cặp hỗ trợ trước. Khi có cặp ngôn ngữ thứ 2 (model dịch khác), cần thêm bước map `pair_id -> model_dir` (đã ghi chú trong code).
- **Lựa chọn chỉ giữ trong state runtime** (`ref` ở `App.vue`, mặc định = cặp đầu tiên khi tải xong), không persist — việc lưu cấu hình bền vững thuộc PBI "Lưu cấu hình" riêng ở M6 (chưa làm).

## Cài đặt

- `app/src-tauri/src/core/translator.rs`: thêm `struct LanguagePair` + `const SUPPORTED_LANGUAGE_PAIRS`.
- `app/src-tauri/src/core/mod.rs`: export `LanguagePair`, `SUPPORTED_LANGUAGE_PAIRS`.
- `app/src-tauri/src/lib.rs`: thêm `struct LanguagePairInfo` (serialize camelCase) + command `list_language_pairs_cmd`; thêm hàm thuần `validate_language_pair(pair_id: &str) -> Result<(), String>`; `start_capture` nhận thêm `pair_id: String`, gọi `validate_language_pair` trước khi tạo session; đăng ký `list_language_pairs_cmd` trong `generate_handler!`.
- `app/src/components/LanguagePairSelector.vue` (component mới, pattern giống `DeviceSelector.vue`): gọi `list_language_pairs_cmd` lúc mount, `<select>` liệt kê `label` từng cặp, emit `update:selected`/`pairs-loaded`, nhận prop `selected`/`disabled`.
- `app/src/App.vue`: thêm state `selectedPairId`, mặc định chọn cặp đầu khi `LanguagePairSelector` phát `pairs-loaded`; đặt `<LanguagePairSelector :disabled="devicesLocked">` (tái dùng computed đã có từ PBI 28) cạnh 2 `DeviceSelector`; `startSession()` gọi `invoke("start_capture", { pairId: selectedPairId.value })`.

## Bằng chứng kiểm thử

- Rust (`cargo test`): **74/74 pass** (2 ignored, không đổi so với trước PBI 29) — gồm test mới `danh_sach_cap_ngon_ngu_khong_rong_va_moi_cap_nguon_khac_dich` (translator.rs), `list_language_pairs_cmd_tra_dung_danh_sach`, `validate_language_pair_pair_id_hop_le_tra_ok`/`...khong_hop_le_tra_loi` (lib.rs). Xem `evidence/T002-testcase.md`, `T004-testcase.md`, `T007-testcase.md`, `T013-cargo-test-result.log`.
- Vitest (`bun run test`): **37/37 pass** (5 test file) — thêm `LanguagePairSelector.test.ts` (5 test mới) + 4 test mới trong `App.test.ts` (mặc định chọn cặp đầu, khoá/mở khoá theo `sessionState`, gửi đúng `pairId` khi Start); sửa 2 test cũ của PBI 28 để khớp `invoke("start_capture", { pairId })` thay vì gọi không tham số. Xem `evidence/T011-testcase.md`, `T012-testcase.md`, `T013-vitest-result.log`.
- **Chưa xác nhận tay trên máy thật có ≥ 2 cặp ngôn ngữ thật** (chỉ có 1 model dịch hiện nay) — cùng hạn chế hạ tầng đã gặp ở PBI 26/28, ghi rõ trong `plan.md` mục "Rủi ro & ngoại lệ có lý do".

## Liên quan

- [[PBI-16-dich-local-sidecar]] — nguồn model dịch `en-vi` duy nhất mà `SUPPORTED_LANGUAGE_PAIRS` hiện tham chiếu.
- [[PBI-19-noi-stt-dich-subtitle-ready]] — pattern tách hàm thuần khỏi `start_capture` để test không cần Tauri runtime, PBI 29 tái dùng cho `validate_language_pair`.
- [[PBI-28-nut-start-stop-phien-dich]] — mục M6 liền trước, cùng file `App.vue`; PBI 29 tái dùng computed `devicesLocked` để khoá cả ô chọn ngôn ngữ.
- [[Kien-truc-core-ui]] — danh sách cặp ngôn ngữ + validate nằm ở Core (Rust), UI chỉ hiển thị/gọi lệnh, không tự biết logic hợp lệ.
- [[Trang-thai-ma-nguon]] — cập nhật: M6 (3/4).
