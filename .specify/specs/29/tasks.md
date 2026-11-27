# Danh sách Task: Chọn cặp ngôn ngữ nguồn-đích

**Mã PBI**: 29
**Nguồn**: plan.md, spec.md, data-model.md, research.md

## Pha 1: Setup

(Không có — dùng chung toolchain/dependency hiện có của `app/`, không cần khởi tạo gì mới.)

## Pha 2: Foundational

- [X] T001 Thêm `struct LanguagePair` + hằng số `SUPPORTED_LANGUAGE_PAIRS: &[LanguagePair]` (khởi tạo 1 phần tử `en-vi`) vào `app/src-tauri/src/core/translator.rs`, export qua `core::mod.rs` nếu cần.
- [X] T002 Viết test Rust tại `app/src-tauri/src/core/translator.rs` xác nhận `SUPPORTED_LANGUAGE_PAIRS` không rỗng và mọi phần tử có `source_lang != target_lang`.

## Pha 3: User Story 1 - Chọn và áp dụng cặp ngôn ngữ (Ưu tiên: P1)

**Mục tiêu**: Người dùng chọn được cặp ngôn ngữ hỗ trợ trên UI, lựa chọn áp dụng đúng cho phiên dịch tiếp theo, và bị khoá khi phiên đang chạy.
**Tiêu chí kiểm thử độc lập**: Chạy `cargo test` + `bun run test` (Vitest) pass; chạy tay app: đổi cặp ngôn ngữ ở `idle`, nhấn Start thấy phụ đề dịch đúng cặp, nhấn giữa lúc `running` thì ô chọn bị khoá.

- [X] T003 [US1] Thêm command `list_language_pairs_cmd() -> Vec<LanguagePairInfo>` vào `app/src-tauri/src/lib.rs` (struct `LanguagePairInfo` serialize `id`/`source_lang`/`target_lang`/`label` từ `LanguagePair`), đăng ký trong `tauri::generate_handler!`.
- [X] T004 [US1] Viết test Rust cho `list_language_pairs_cmd`/mapping sang `LanguagePairInfo` tại `app/src-tauri/src/lib.rs` (test module cuối file, theo pattern test hiện có).
- [X] T005 [US1] Sửa command `start_capture` tại `app/src-tauri/src/lib.rs` nhận thêm tham số `pair_id: String`; tra `pair_id` trong `SUPPORTED_LANGUAGE_PAIRS`, trả `Err` rõ ràng nếu không khớp cặp nào trước khi tạo session.
- [X] T006 [US1] Sửa phần tạo `SidecarTranslator` trong `start_capture` (`app/src-tauri/src/lib.rs`) để dùng `model_dir` tra theo `pair_id` đã khớp ở T005 thay cho đọc trực tiếp `SORA_TRANSLATE_MODEL_PATH` (giữ nguyên hành vi optional: thiếu cấu hình sidecar/model vẫn bỏ qua bước dịch, không chặn start).
- [X] T007 [US1] Viết test Rust cho `start_capture` với `pair_id` không khớp cặp nào → trả lỗi, session không start (test module cuối `app/src-tauri/src/lib.rs`, dùng pattern mock như `RecordingTranslator`/`FlakyTranslator` đã có).
- [X] T008 [US1] Tạo component `app/src/components/LanguagePairSelector.vue` (pattern giống `DeviceSelector.vue`): gọi `list_language_pairs_cmd` lúc mount, emit `update:selected` với `id` của cặp được chọn, nhận prop `selected`/`disabled`.
- [X] T009 [US1] Sửa `app/src/App.vue`: thêm state `selectedPairId`, mặc định = cặp đầu tiên khi `LanguagePairSelector` phát sự kiện tải xong; đặt `LanguagePairSelector` cạnh `DeviceSelector`, khoá (prop `disabled`/nằm trong khối `:inert="devicesLocked"`) khi `sessionState !== "idle"`.
- [X] T010 [US1] Sửa `startSession()` trong `app/src/App.vue` để gọi `invoke("start_capture", { pairId: selectedPairId.value, ... })` kèm tham số hiện có, không phá vỡ test cũ của PBI 26/28.
- [X] T011 [US1] [P] Viết test Vitest tại `app/src/components/LanguagePairSelector.vue` (file test riêng cạnh component, theo pattern PBI 26/28): render danh sách cặp từ mock `invoke`, emit đúng `id` khi chọn.
- [X] T012 [US1] [P] Viết test Vitest cho `app/src/App.vue`: ô chọn ngôn ngữ bị khoá khi `sessionState !== "idle"`, mặc định chọn cặp đầu tiên khi danh sách tải xong, `startSession` gửi đúng `pairId` đang chọn.

## Pha cuối: Polish & Cross-cutting

- [X] T013 Chạy toàn bộ `cargo test` (app/src-tauri) + `bun run test` (app) xác nhận không có test cũ bị vỡ; chụp ảnh/ghi lại kết quả làm bằng chứng theo yêu cầu `/sora-implement`.
- [X] T014 Cập nhật comment/doc ngắn trong `app/src-tauri/src/lib.rs` tại chỗ `start_capture` giải thích nguồn `model_dir` mới lấy theo `pair_id` (tránh gây hiểu lầm khi đọc lại code sau này), theo đúng quy ước comment tối thiểu của dự án.

## Sơ đồ phụ thuộc

- Pha 2 (T001, T002) phải xong trước mọi task Pha 3 (cần `LanguagePair`/`SUPPORTED_LANGUAGE_PAIRS` tồn tại trước).
- T003 → T004; T005 → T006 → T007 (cùng sửa `start_capture`, làm tuần tự trên 1 file).
- T008 độc lập với T003–T007 (file khác), có thể làm song song với backend.
- T009, T010 phụ thuộc T008 (cần component tồn tại) và T003/T005 (cần command tồn tại để gọi đúng tham số).
- T011 phụ thuộc T008; T012 phụ thuộc T009, T010. T011/T012 có thể chạy song song với nhau (`[P]`, khác file).
- T013, T014 chạy sau cùng, sau khi toàn bộ US1 hoàn tất.

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (chỉ có 1 story, không chia nhỏ hơn — tính năng vốn đã nhỏ, 1 lát cắt dùng được ngay).
- Thứ tự giao hàng: Pha 2 (nền tảng Rust) → backend command/validate (T003–T007) → UI component + tích hợp (T008–T010) → test (T011, T012) → polish (T013, T014).
- Cơ hội chạy song song: T011 và T012 (`[P]`); T008 có thể làm song song với T003–T007 nếu có 2 người/2 luồng làm việc.
