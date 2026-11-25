# Nghiên cứu kỹ thuật: PBI 19 — Nối STT final → Dịch → subtitle-ready

## Quyết định 1: Nơi nối luồng — trong `lib.rs` (closure `start_capture`), không đổi `TranscriptPipeline`

**Quyết định**: Thêm bước dịch + phát `subtitle-ready` ngay trong callback `on_chunk` hiện có của `TranscriptPipeline::process_segments()` tại `lib.rs::start_capture`, giữ nguyên hoàn toàn `TranscriptPipeline` (không sửa `transcript_pipeline.rs`).

**Lý do**: `on_chunk` đã nhận đúng `(segment, chunk)` theo thứ tự thời gian thực và đã phân biệt được `chunk.is_final`. Đây là điểm duy nhất hiện có vừa biết câu vừa final, vừa biết context thời gian — đúng vị trí tối thiểu để nối thêm dịch mà không đổi hợp đồng của `TranscriptPipeline` (giữ nguyên trách nhiệm: chỉ ghép VAD→STT, không biết về Translator — tách biệt mối quan tâm, không vi phạm nguyên tắc Core tách lớp).

**Phương án khác đã xem xét**:
- Thêm tham số `translator` vào `TranscriptPipeline::process_segments()` để tự dịch bên trong — bị loại vì làm `TranscriptPipeline` biết về `Translator`, tăng coupling không cần thiết trong khi `lib.rs` đã có đủ thông tin để orchestrate ở bên ngoài.
- Tạo 1 struct `SubtitlePipeline` mới bọc cả `TranscriptPipeline` + `Translator` — cân nhắc nhưng quá sớm cho 1 PBI nối đơn giản; để dành nếu sau này cần tái sử dụng logic này ở nơi khác ngoài `lib.rs` (ghi nhận ở "Rủi ro").

## Quyết định 2: Quản lý ngữ cảnh — dùng lại `RecentTranscriptContext` (PBI 17), đặt instance trong closure `start_capture`

**Quyết định**: Tạo 1 `RecentTranscriptContext` mới mỗi lần `start_capture` chạy (sống trong closure, cùng vòng đời với `stt`/`pipeline` hiện có), `push()` câu gốc final (không rỗng) sau khi dịch xong, `recent()` để lấy ngữ cảnh cho câu kế tiếp.

**Lý do**: Struct này đã viết và test xong ở PBI 17 đúng cho mục đích này (giữ tối đa 2 câu gần nhất, bỏ câu rỗng). Tái dùng đúng API có sẵn (`push`, `recent`), không cần thêm logic mới.

**Phương án khác đã xem xét**: Giữ context trong 1 `Mutex`/state Tauri riêng để nhiều lệnh có thể đọc — không cần vì context chỉ dùng nội bộ trong vòng đời 1 phiên capture, closure hiện tại đã đủ (giống cách `stt`, `pipeline` đang được giữ).

## Quyết định 3: Thứ tự ngữ cảnh push — push SAU khi dịch xong, dùng câu gốc (không phải câu dịch)

**Quyết định**: Với mỗi câu final: (1) lấy `context = recent_ctx.recent()`, (2) gọi `translator.translate(text, &context)`, (3) nếu dịch thành công, phát `subtitle-ready`, rồi `recent_ctx.push(text)` (câu gốc) để làm ngữ cảnh cho câu sau.

**Lý do**: Đảm bảo câu hiện tại không tự làm ngữ cảnh cho chính nó (FR-002: ngữ cảnh là câu *trước*), và câu lỗi dịch (nếu `translate()` trả lỗi) — quyết định dưới — vẫn nên được push làm ngữ cảnh vì bản thân câu gốc vẫn hợp lệ, chỉ bước dịch lỗi (không ảnh hưởng FR-002/SC-001 vì ngữ cảnh dùng câu gốc, không dùng câu dịch).

**Phương án khác đã xem xét**: Push trước khi dịch — bị loại vì sẽ khiến câu hiện tại lẫn vào ngữ cảnh của chính nó nếu không cẩn thận thứ tự code; push container riêng là không cần thiết.

## Quyết định 4: Xử lý lỗi dịch 1 câu — báo lỗi qua `model-error` kiểu mới `translate`, không phát `subtitle-ready` cho câu đó

**Quyết định**: Khi `translator.translate()` trả `Err`, phát event `model-error` với `kind = "translate"` (thêm biến thể mới trong `ModelErrorKind`/`ModelErrorPayload`, song song với `load`/`infer` đã có ở PBI 15), không phát `subtitle-ready` cho câu đó, tiếp tục xử lý câu tiếp theo. Câu gốc vẫn được push làm ngữ cảnh (Quyết định 3).

**Lý do**: Nhất quán với cơ chế báo lỗi runtime đã có (PBI 15: lỗi 1 segment không dừng phiên, báo qua `model-error`). Tái dùng đúng pattern đã quen, UI không cần học thêm 1 cơ chế lỗi khác.

**Phương án khác đã xem xét**: Phát `subtitle-ready` với `translatedText` rỗng/placeholder khi lỗi — bị loại vì làm mơ hồ giữa "dịch ra chuỗi rỗng" (câu input rỗng, hành vi hợp lệ) và "lỗi dịch" (cần phân biệt rõ, theo FR-007/SC-003).

## Quyết định 5: Không dịch khi phiên không có `Translator` khả dụng

**Quyết định**: Giống cách `stt` là `Option<WhisperSpeechToText>` (None nếu thiếu `WHISPER_MODEL_PATH`), tạo `translator: Option<SidecarTranslator>` từ biến môi trường tương tự (`SORA_TRANSLATE_SIDECAR_PATH`, `SORA_TRANSLATE_MODEL_PATH`, đã dùng trong test tích hợp PBI 16/17). Nếu `None`, bỏ qua bước dịch, chỉ phát `transcript-final` như hiện tại (không phát `subtitle-ready`).

**Lý do**: Giữ nhất quán với pattern "model optional, phiên vẫn chạy được" đã chốt từ PBI 14/15 cho STT; không bắt buộc người dùng phải có sẵn sidecar dịch để test capture/VAD/STT cơ bản.

**Phương án khác đã xem xét**: Bắt buộc phải có translator mới cho `start_capture` chạy — bị loại vì phá vỡ khả năng test từng phần độc lập đã có, không cần thiết cho PBI này (FR-008).

Không còn điểm NEEDS CLARIFICATION.
