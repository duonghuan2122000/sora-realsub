# Research: Xử lý lỗi runtime model STT (PBI 15)

## Quyết định 1: Phân loại lỗi runtime model

**Quyết định**: Dùng 1 enum mới `ModelErrorKind` với 2 biến thể `Load` (lỗi nạp model: thiếu file/file hỏng) và `Infer` (lỗi khi infer 1 segment cụ thể), thay vì chỉ tái dùng `CoreError::Provider(String)` chung.

**Lý do**: `CoreError::Provider(String)` hiện chỉ mang 1 chuỗi lỗi kỹ thuật thô (ví dụ "không nạp được model whisper: ..."), không tách được "lỗi nạp" khỏi "lỗi infer" khi gửi payload qua Tauri event cho UI (FR-003 yêu cầu phân loại được ≥ 2 loại). Thêm enum riêng cho phần hiển thị người dùng, không đổi `CoreError` nội bộ dùng để truyền lỗi kỹ thuật giữa các hàm Rust.

**Phương án khác đã xem xét**:
- Tái dùng `CoreError::Provider(String)` và parse chuỗi lỗi ở `lib.rs` để đoán loại lỗi → dễ vỡ (phrase-matching chuỗi lỗi từ thư viện ngoài), không bền khi đổi câu lỗi của `whisper-rs`.
- Thêm biến thể mới vào `CoreError` (`ModelLoad`, `ModelInfer`) → cân nhắc nhưng sẽ phải sửa mọi nơi match `CoreError` hiện có (test PBI 12-14 dùng `CoreError::Provider` trực tiếp); chọn giữ `CoreError::Provider` cho lỗi kỹ thuật, thêm enum hiển thị riêng ở tầng payload event để giảm diện ảnh hưởng.

## Quyết định 2: Không dừng toàn batch khi 1 segment lỗi infer

**Quyết định**: Sửa vòng lặp trong `TranscriptPipeline::process_segments` để khi `self.stt.transcribe(frame).await` trả lỗi, gọi một callback lỗi riêng (`on_error`) với thông tin segment, rồi `continue` sang segment tiếp theo, không `return Err(...)` dừng cả hàm.

**Lý do**: FR-005 yêu cầu lỗi 1 segment không lan ra toàn pipeline. Giữ nguyên happy-path (test PBI 13/14 hiện có) bằng cách thêm tham số callback mới thay vì đổi kiểu trả về `Result<Vec<TranscriptChunk>, CoreError>`.

**Phương án khác đã xem xét**:
- Đổi `Result<Vec<TranscriptChunk>, CoreError>` thành `Vec<Result<TranscriptChunk, CoreError>>` → buộc nơi gọi (`lib.rs`) tự lọc lỗi, thay đổi API nhiều hơn cần thiết, có thể phá test cũ dùng `.unwrap()` trên `Result` toàn hàm.
- Dừng toàn batch khi gặp lỗi (giữ `?` hiện tại) → không đáp ứng FR-005 vì `lib.rs` tương lai có thể gọi với nhiều segment cùng lúc; chọn sửa đúng gốc trong `process_segments` theo nguyên tắc sửa tại nơi dùng chung (xem CLAUDE.md/ponytail: fix root cause tại hàm được gọi chung).

## Quyết định 3: Lỗi nạp model phát 1 lần, không lặp lại mỗi frame

**Quyết định**: Khi `WhisperSpeechToText::new` lỗi trong `lib.rs`, emit event `model-error` (loại `Load`) đúng 1 lần ngay sau khi `start_capture` xác định STT không khả dụng, trước khi tiếp tục phiên capture không có STT.

**Lý do**: Spec FR-004/trường hợp biên yêu cầu "chỉ báo lỗi một lần", tránh spam event mỗi khi có segment VAD mới trong khi STT vẫn `None` suốt phiên.

**Phương án khác đã xem xét**: Emit lại mỗi khi có `vad-segment` mà `stt` là `None` → gây spam event trùng lặp không cần thiết, không đúng yêu cầu "một lần".
