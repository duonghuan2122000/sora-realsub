# Danh sách Task: Nhận diện giọng nói local (STT)

**Mã PBI**: 12
**Nguồn**: plan.md, spec.md, research.md, quickstart.md

## Pha 1: Setup

- [X] T001 Thêm dependency `whisper-rs` vào `app/src-tauri/Cargo.toml`
- [X] T002 Tạo thư mục `app/src-tauri/tests/fixtures/` và chuẩn bị/ghi chú nguồn file audio mẫu (giọng rõ ngắn, giọng dài nhiều câu, gần im lặng) dùng cho test tích hợp STT

## Pha 2: Foundational

- [X] T003 Thêm hằng số/helper đọc đường dẫn model ggml/gguf từ biến môi trường (ví dụ `WHISPER_MODEL_PATH`) tại `app/src-tauri/src/core/speech_to_text.rs`
- [X] T004 Thêm biến thể lỗi phù hợp (nếu cần) cho lỗi load/infer model vào `app/src-tauri/src/core/error.rs` (dùng `CoreError::Provider`)

## Pha 3: User Story 1 - Nhận diện giọng nói từ file audio mẫu (Ưu tiên: P1)

**Mục tiêu**: cho một đoạn audio (buffer `AudioFrame`), hệ thống trả về `TranscriptChunk` khớp đúng nội dung đã nói, chạy hoàn toàn local, không crash với audio im lặng.

**Tiêu chí kiểm thử độc lập**: chạy `cargo test -p app --test stt_whisper` với 3 file mẫu (giọng rõ ngắn, giọng dài, gần im lặng) và nhận kết quả đúng như spec.md mô tả (SC-001, SC-002, SC-003).

- [X] T005 [US1] Cài đặt struct `WhisperSpeechToText` (nạp model từ đường dẫn file khi khởi tạo) implement trait `SpeechToText::transcribe` tại `app/src-tauri/src/core/speech_to_text.rs`
- [X] T006 [US1] Chuẩn hoá đầu vào: đảm bảo `AudioFrame.samples` đưa vào whisper-rs đúng định dạng 16kHz mono f32 (resample nếu `sample_rate` khác 16000, dùng `rubato` theo research.md) tại `app/src-tauri/src/core/speech_to_text.rs`
- [X] T007 [US1] Xử lý trường hợp audio gần như im lặng: trả `TranscriptChunk { text: "", is_final: true }` hoặc tương đương khi whisper-rs không nhận diện được text, không panic, tại `app/src-tauri/src/core/speech_to_text.rs`
- [X] T008 [US1] Export `WhisperSpeechToText` qua `app/src-tauri/src/core/mod.rs`
- [X] T009 [US1] Viết test tích hợp tại `app/src-tauri/tests/stt_whisper.rs`: đọc từng file mẫu ở `tests/fixtures/`, gọi `WhisperSpeechToText::transcribe`, so khớp text kỳ vọng (ground truth) cho kịch bản chấp nhận 1 và 2, kiểm tra text rỗng/gần rỗng cho kịch bản chấp nhận 3 — đánh dấu `#[ignore]` kèm comment hướng dẫn nếu máy chạy test không có model (rủi ro đã ghi trong plan.md)

## Pha cuối: Polish & Cross-cutting

- [X] T010 Chạy `cargo clippy` và `cargo fmt` trên các file đã sửa, sửa warning phát sinh
- [X] T011 Chụp ảnh/log kết quả `cargo test -p app --test stt_whisper -- --nocapture` làm bằng chứng test (theo yêu cầu `/sora-implement`)

## Sơ đồ phụ thuộc

Setup (T001–T002) → Foundational (T003–T004) → User Story 1 (T005–T009, theo thứ tự vì cùng sửa 1 file `speech_to_text.rs` trước khi test) → Polish (T010–T011).

Không có user story thứ 2 ở PBI này (1 lát cắt duy nhất, không chia P1/P2/P3 nhỏ hơn).

## Chiến lược triển khai

- MVP đề xuất: toàn bộ User Story 1 (T005–T009) — đây đã là lát cắt nhỏ nhất, hoàn chỉnh, kiểm thử độc lập được.
- Thứ tự giao hàng: Setup → Foundational → US1 → Polish, không có nhánh song song đáng kể do tất cả task US1 cùng sửa một file (`speech_to_text.rs`) nối tiếp nhau.
