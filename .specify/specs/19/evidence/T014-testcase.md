# Test case bằng chứng: T014 — test tích hợp SC-002 (PBI 19)

Vị trí: `app/src-tauri/tests/subtitle_pipeline_real.rs`, hàm `noi_stt_final_qua_dich_co_ngu_canh_khop_pbi_17`.

- **Input**: 2 câu transcript final mô phỏng liên tiếp trong 1 phiên — `"John lost his keys."` rồi `"He was very upset."` (câu có đại từ phụ thuộc câu trước, giống kịch bản PBI 17 T016) — đưa qua `RecentTranscriptContext` + `SidecarTranslator` thật.
- **Expected**: bản dịch câu 2 lấy ngữ cảnh từ `RecentTranscriptContext.recent()` (sau khi đã `push` câu 1) phải khớp với bản dịch câu 2 khi truyền tay đúng ngữ cảnh `["John lost his keys."]` (không lẫn, không rơi về bản dịch không-ngữ-cảnh).
- **Trạng thái chạy**: `#[ignore]` — môi trường thi công (container này) không có sẵn model STT (`WHISPER_MODEL_PATH`) và sidecar dịch cục bộ (`SORA_TRANSLATE_SIDECAR_PATH`, `SORA_TRANSLATE_MODEL_PATH`); đây là hạn chế đã biết kế thừa từ PBI 12/16/17 (model/sidecar không commit vào repo).
- **Đã xác nhận**: test compile sạch (`cargo test -p app --test subtitle_pipeline_real` → `1 ignored`, không lỗi biên dịch) — xem `T014-compile-result.log`.
- **Chưa xác nhận bằng chạy thật**: cần người có sẵn model/sidecar cục bộ chạy `cargo test -p app --test subtitle_pipeline_real -- --ignored` theo `quickstart.md` để lấy log output làm bằng chứng SC-002 đầy đủ.
- **Kết quả**: PASS (biên dịch + logic review); SC-002 chưa được xác nhận bằng chạy thật trong môi trường này — ghi nhận là hạn chế biết trước, không chặn PBI (tương tự PBI 16 T007, PBI 17 T016).
