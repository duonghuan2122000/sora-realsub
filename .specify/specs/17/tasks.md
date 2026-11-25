# Danh sách Task: Dịch có ngữ cảnh câu trước

**Mã PBI**: 17
**Nguồn**: plan.md, spec.md, data-model.md, research.md, quickstart.md

## Pha 1: Setup

- [x] T001 Đọc lại `core/translator.rs` hiện có (`app/src-tauri/src/core/translator.rs`) để xác nhận chữ ký `Translator::translate(text, context)` và cài đặt `SidecarTranslator` hiện tại trước khi sửa (không cần tạo file/dependency mới — xem plan.md).

## Pha 2: Foundational

- [x] T002 Thêm struct `RecentTranscriptContext` (giữ tối đa 2 câu gần nhất, bounded FIFO) trong `app/src-tauri/src/core/translator.rs`, với các hành vi: `push(câu)` (bỏ qua câu rỗng/khoảng trắng, loại câu cũ nhất khi đã đầy 2) và `recent()` trả về tối đa 2 câu theo thứ tự cũ→mới — theo data-model.md.
- [x] T003 [P] Viết hàm nội bộ (không public) trong `app/src-tauri/src/core/translator.rs` để chuẩn hoá tham số `context: &[String]` đầu vào của `translate()`: lọc bỏ phần tử rỗng/khoảng trắng, chỉ giữ tối đa 2 phần tử cuối (FR-003, FR-005).
- [x] T004 [P] Viết hàm nội bộ tách câu theo dấu kết câu (`.`, `!`, `?`) trong `app/src-tauri/src/core/translator.rs`, trả về số đoạn tách được, dùng cho bước ghép/tách ngữ cảnh (research.md Quyết định 2).

## Pha 3: User Story 1 - Dịch câu hiện tại kèm ngữ cảnh câu trước (Ưu tiên: P1)

**Mục tiêu**: `SidecarTranslator::translate()` dùng đúng ngữ cảnh truyền vào để dịch câu hiện tại chính xác hơn (đại từ, chủ ngữ ẩn), đồng thời chỉ trả về bản dịch của riêng câu hiện tại — khớp FR-001 đến FR-006, kịch bản chấp nhận 1-4 trong spec.md.

**Tiêu chí kiểm thử độc lập**: Chạy `cargo test -p app translator` (mock sidecar/translator) xác nhận: có ngữ cảnh → input gửi đi là ngữ cảnh+câu hiện tại ghép lại, kết quả trả về chỉ là phần câu hiện tại; không có ngữ cảnh → hành vi giống PBI 16; ngữ cảnh rỗng/khoảng trắng bị lọc; context > 2 câu chỉ giữ 2 câu gần nhất; gọi lặp lại cho kết quả ổn định.

- [x] T005 [US1] Sửa `SidecarTranslator::translate()` trong `app/src-tauri/src/core/translator.rs`: khi `text` rỗng/khoảng trắng, trả `Ok(String::new())` ngay (giữ hành vi PBI 16, không phụ thuộc context) — FR-004 nhánh rỗng.
- [x] T006 [US1] Sửa `SidecarTranslator::translate()`: khi ngữ cảnh (sau chuẩn hoá ở T003) rỗng, gọi sidecar với `--text` là câu hiện tại y như PBI 16, không đổi hành vi — FR-004.
- [x] T007 [US1] Sửa `SidecarTranslator::translate()`: khi có 1-2 câu ngữ cảnh hợp lệ, ghép theo thứ tự cũ→mới + câu hiện tại thành một chuỗi (phân tách bởi khoảng trắng) và gọi sidecar với `--text` là chuỗi ghép đó — FR-001 (research.md Quyết định 1).
- [x] T008 [US1] Sau khi nhận bản dịch ghép từ sidecar, dùng hàm tách câu (T004) để lấy đoạn **cuối cùng** làm kết quả trả về của `translate()` khi số đoạn tách được bằng số câu đầu vào (ngữ cảnh + hiện tại) — FR-002.
- [x] T009 [US1] Thêm nhánh dự phòng trong `translate()`: nếu số đoạn tách được (T008) không khớp số câu đầu vào, gọi lại sidecar dịch riêng câu hiện tại không kèm ngữ cảnh (như T006) và dùng kết quả đó — đảm bảo luôn trả đúng bản dịch câu hiện tại, không lẫn ngữ cảnh, không crash (FR-002, SC-002, SC-003; research.md Quyết định 2).
- [x] T010 [US1] Viết test mock trong `app/src-tauri/src/core/translator.rs` (`#[cfg(test)]`): dịch có 1 câu ngữ cảnh → input gửi tới sidecar giả gồm cả ngữ cảnh + câu hiện tại, kết quả trả về chỉ là phần câu hiện tại (dùng mock trả chuỗi ghép có dấu câu rõ ràng để kiểm chứng tách đúng).
- [x] T011 [US1] Viết test: dịch với danh sách context rỗng → hành vi giống PBI 16 (gọi sidecar chỉ với câu hiện tại).
- [x] T012 [US1] Viết test: dịch với context chứa câu rỗng/khoảng trắng lẫn câu hợp lệ → câu rỗng bị loại trước khi ghép (FR-005).
- [x] T013 [US1] Viết test: dịch với context có 3 câu trở lên → chỉ 2 câu gần câu hiện tại nhất được dùng (FR-003).
- [x] T014 [US1] Viết test: giả lập sidecar trả bản dịch ghép mà số đoạn tách được không khớp số câu đầu vào → `translate()` tự fallback gọi lại không ngữ cảnh, kết quả trả về là bản dịch câu hiện tại một mình, không lỗi (FR-002, SC-003).
- [x] T015 [US1] Viết test: gọi `translate()` 2 lần với cùng câu hiện tại + cùng context → cùng kết quả (FR-006, giữ FR-004 PBI 16).
- [x] T016 [US1] Viết test tích hợp `#[ignore]` (giống mẫu PBI 16, dùng `SORA_TRANSLATE_SIDECAR_PATH`/`SORA_TRANSLATE_MODEL_PATH`) trong `app/src-tauri/src/core/translator.rs`: dịch câu hiện tại "He was very upset." kèm ngữ cảnh "John lost his keys." bằng sidecar + model thật, so sánh với bản dịch không ngữ cảnh — xác nhận đại từ được dịch nhất quán hơn khi có ngữ cảnh (SC-001; xem quickstart.md).

## Pha cuối: Polish & Cross-cutting

- [ ] T017 Chạy `cargo test -p app translator` (toàn bộ test mock, không cần sidecar thật) và xác nhận pass toàn bộ, chụp/log kết quả làm bằng chứng cho `/sora-implement`.
- [ ] T018 Chạy test tích hợp `#[ignore]` (T016) với sidecar + model thật theo quickstart.md, lưu log/kết quả so sánh có-ngữ-cảnh vs không-ngữ-cảnh làm bằng chứng SC-001.
- [x] T019 Rà soát comment/docstring trong `app/src-tauri/src/core/translator.rs` theo style-guide dự án (comment chỉ giải thích WHY, không lặp lại code), cập nhật nếu cần cho các hàm mới ở T002-T009.

## Sơ đồ phụ thuộc

- Pha 1 (T001) → Pha 2 (T002-T004) → Pha 3 (T005-T016) → Pha cuối (T017-T019).
- Trong Pha 2: T002 độc lập; T003 và T004 có thể làm song song với nhau (khác hàm, không phụ thuộc nhau) nhưng cả hai cần xong trước khi vào Pha 3.
- Trong Pha 3: T005 → T006 → T007 → T008 → T009 phải làm tuần tự (cùng sửa 1 hàm `translate()`, các nhánh logic phụ thuộc nhau). T010-T015 (viết test) phụ thuộc T005-T009 đã xong; các test này có thể viết song song với nhau (khác test case, cùng file nhưng không phụ thuộc lẫn nhau về logic) — đánh dấu không `[P]` vì cùng sửa 1 file test module, tránh xung đột khi áp dụng tuần tự. T016 (test tích hợp thật) độc lập, có thể làm sau cùng trong Pha 3.
- Chỉ có 1 user story (US1) trong PBI này — không có phụ thuộc chéo story.

## Ví dụ chạy song song

- T003 và T004 (hai hàm nội bộ độc lập) có thể giao cho 2 người/2 lần thực hiện song song trước khi bắt đầu Pha 3.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (đây là PBI nhỏ, chỉ có 1 story) — không chia nhỏ thêm.
- Thứ tự giao hàng: Setup → Foundational (struct ngữ cảnh + 2 hàm nội bộ) → sửa `translate()` theo từng nhánh logic (rỗng → không ngữ cảnh → có ngữ cảnh → tách câu → fallback) → test mock đầy đủ → test tích hợp thật (bằng chứng SC-001) → polish.
