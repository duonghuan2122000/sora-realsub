# Nghiên cứu kỹ thuật: PBI 17 — Dịch có ngữ cảnh câu trước

## Quyết định 1: Cách truyền ngữ cảnh tới sidecar dịch mà không đổi executable sidecar

**Quyết định**: Không thêm flag CLI mới cho `translate-sidecar`. Ở phía `SidecarTranslator::translate()` (Core), ghép (nối chuỗi) tối đa 2 câu ngữ cảnh (đã lọc rỗng) với câu hiện tại thành một đoạn văn bản liên tục, theo thứ tự: `[câu ngữ cảnh cũ nhất] [câu ngữ cảnh gần nhất] [câu hiện tại]`, cách nhau bởi khoảng trắng — rồi gửi cả đoạn này qua `--text` như một lần gọi dịch duy nhất tới sidecar hiện có (không đổi `app/translate-sidecar/src/main.rs`).

**Lý do**:
- Model Marian-MT (`opus-mt-en-vi`) hiện dùng là model dịch câu đơn (sentence-level), không có token/flag "ngữ cảnh" riêng như các model multi-sentence/document-level (NLLB không có cơ chế này sẵn, các model context-aware thường cần fine-tune riêng). Model dịch toàn bộ input như một đoạn văn, nên khi ngữ cảnh đứng trước câu hiện tại trong cùng input, attention của encoder/decoder vẫn "thấy" được ngữ cảnh đó khi sinh bản dịch — đây là cách thực dụng nhất để cải thiện dịch đại từ/thiếu ngữ cảnh mà không cần đổi model hay sidecar.
- Giữ nguyên giao diện sidecar hiện có (PBI 16) → giảm rủi ro, không cần build lại/test lại sidecar, không đổi `Cargo.toml` hay dependency.
- Khớp với giả định đã ghi trong spec.md (PBI 16 giữ nguyên, không đổi trong PBI 17).

**Phương án khác đã xem xét**:
- Thêm flag `--context` riêng cho sidecar, dịch ngữ cảnh + câu hiện tại thành 2 lệnh gọi dịch tách biệt rồi hệ thống tự ghép ngữ cảnh ở tầng ngữ nghĩa → bị loại vì dịch tách biệt từng câu không giải quyết được vấn đề đại từ (mỗi câu vẫn dịch độc lập, không có thông tin chéo câu) — không đáp ứng FR-001.
- Đổi hẳn sang model context-aware/document-level → bị loại, ngoài phạm vi PBI này (giả định: không đổi model/cặp ngôn ngữ).

## Quyết định 2: Cách tách bản dịch của câu hiện tại ra khỏi bản dịch ghép (ngữ cảnh + hiện tại)

**Quyết định**: Sau khi nhận bản dịch của toàn đoạn ghép từ sidecar, tách câu bằng heuristic dựa trên dấu kết câu (`.`, `!`, `?`, hoặc dấu tương đương tiếng Việt) — số đoạn tách ra tương ứng với số câu đầu vào (ngữ cảnh + hiện tại); lấy đoạn **cuối cùng** làm bản dịch trả về cho câu hiện tại (đúng với FR-002: chỉ trả bản dịch của câu hiện tại).

Nếu sau khi tách, số đoạn thu được **ít hơn** số câu đầu vào (model gộp câu, không tách đúng dấu câu) → coi toàn bộ bản dịch ghép là không tách được tin cậy, dùng phương án dự phòng: gọi lại sidecar dịch riêng câu hiện tại không kèm ngữ cảnh (như PBI 16), đảm bảo luôn có kết quả hợp lệ, không trả về rác hoặc cả đoạn ghép.

**Lý do**:
- Marian-MT không trả về alignment/offset giữa câu nguồn và câu dịch, nên không có cách tách chính xác 100% — heuristic theo dấu câu là cách đơn giản, không cần đổi sidecar, đủ tốt cho SC-001/SC-002 với các câu ví dụ thông thường (có dấu kết câu rõ ràng).
- Phương án dự phòng (dịch lại không ngữ cảnh khi tách thất bại) đảm bảo FR-002/SC-002 không bị vi phạm (không bao giờ trả về bản dịch lẫn cả ngữ cảnh) và SC-003 (không crash) — đổi lấy việc đôi khi mất lợi ích ngữ cảnh cho câu khó tách, nhưng đây là lựa chọn an toàn hơn so với trả kết quả sai.

**Phương án khác đã xem xét**:
- Gọi sidecar 2 lần: dịch ngữ cảnh trước, rồi dịch câu hiện tại với một "mồi" (prime) nào đó từ bản dịch ngữ cảnh → Marian-MT không hỗ trợ cơ chế priming qua CLI hiện có, cần đổi model/sidecar → loại.
- Luôn chấp nhận rủi ro tách sai, không có phương án dự phòng → loại vì vi phạm FR-002 khi tách thất bại.

## Quyết định 3: Nơi lưu "1-2 câu transcript gần nhất" làm ngữ cảnh

**Quyết định**: Thêm một struct nhỏ `RecentTranscriptContext` (hoặc tương đương) trong Core — không phải trong `Translator`/`SidecarTranslator` — có nhiệm vụ giữ tối đa 2 câu transcript final gần nhất và trả về khi cần dịch câu kế tiếp. PBI này chỉ cung cấp struct này ở mức đơn vị (unit, có test riêng); **không** nối vào `TranscriptPipeline`/`lib.rs` để tự động chạy end-to-end (việc đó thuộc M4 mục 3, PBI sau).

**Lý do**: `Translator::translate()` đã nhận `context: &[String]` làm tham số (đã có từ PBI 16) — nghĩa là việc quản lý lịch sử câu không thuộc trách nhiệm của `Translator`, mà của nơi gọi nó (pipeline). Vì nối vào pipeline thật là ngoài phạm vi, PBI 17 chỉ cần chứng minh *cơ chế* (struct quản lý ngữ cảnh + `translate()` dùng đúng ngữ cảnh) hoạt động đúng qua test, để PBI nối pipeline sau này dùng lại trực tiếp.

**Phương án khác đã xem xét**:
- Không tạo struct riêng, để pipeline PBI sau tự quản lý `Vec<String>` thủ công → bị loại vì spec.md (giả định) đã nêu PBI này cần đảm bảo có cơ chế lấy 1-2 câu gần nhất; tạo struct nhỏ có test rõ ràng giúp PBI sau tái sử dụng, không phải tự thiết kế lại.
