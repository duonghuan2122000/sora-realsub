# Test case bằng chứng: T009–T013 (PBI 19)

Vị trí: `app/src-tauri/src/lib.rs` (module `tests`), hàm được kiểm thử: `translate_final_chunk`.
Chạy: `cargo test -p app --lib` — log đầy đủ: `T009-T013-lib-tests-result.log`.

## T009 — `nhieu_cau_final_phat_subtitle_dung_thu_tu` (kịch bản chấp nhận 1, 3; FR-003, FR-004)

- **Input**: 3 chunk final liên tiếp `"Cau mot."`, `"Cau hai."`, `"Cau ba."`, dùng `RecordingTranslator` (echo).
- **Expected**: đúng 3 sự kiện subtitle, `source_text` theo đúng thứ tự `"Cau mot."` → `"Cau hai."` → `"Cau ba."`.
- **Actual**: `subtitles.len() == 3`, đúng thứ tự như kỳ vọng.
- **Kết quả**: PASS.

## T010 — `cau_final_sau_nhan_dung_toi_da_2_cau_context_gan_nhat` (kịch bản chấp nhận 2; FR-002)

- **Input**: 4 chunk final liên tiếp `"Cau 1."`..`"Cau 4."`.
- **Expected**: context gửi cho Translator mỗi lần là tối đa 2 câu gần nhất, thứ tự cũ→mới — câu 1: `[]`, câu 2: `["Cau 1."]`, câu 3: `["Cau 1.", "Cau 2."]`, câu 4: `["Cau 2.", "Cau 3."]`.
- **Actual**: khớp chính xác 4 lời gọi ghi lại được.
- **Kết quả**: PASS.

## T011 — `chunk_partial_khong_goi_dich_khong_phat_subtitle` (kịch bản chấp nhận 4; FR-005)

- **Input**: 1 chunk `is_final = false`, text `"dang noi..."`.
- **Expected**: không gọi `translate()`, không phát subtitle, context rỗng sau đó.
- **Actual**: `translator.calls` rỗng, `subtitle_count == 0`, `recent_ctx.recent()` rỗng.
- **Kết quả**: PASS.

## T012 — `chunk_final_rong_khong_goi_dich_khong_phat_subtitle_khong_push_context` (kịch bản chấp nhận 5; FR-006)

- **Input**: 2 chunk final với text `""` và `"   "`.
- **Expected**: không gọi `translate()`, không phát subtitle, không push context.
- **Actual**: `translator.calls` rỗng, `subtitle_count == 0`, `recent_ctx.recent()` rỗng.
- **Kết quả**: PASS.

## T013 — `loi_dich_1_cau_khong_lam_dung_cac_cau_khac` (trường hợp biên; FR-007; SC-003)

- **Input**: 3 chunk final `"Cau 1."`, `"Cau 2."`, `"Cau 3."`, `FlakyTranslator` lỗi đúng ở lần gọi thứ 2 (0-based index 1, tức câu "Cau 2.").
- **Expected**: không panic; 2 subtitle phát ra (câu 1, câu 3); đúng 1 lỗi (câu 2); câu 2 vẫn được push vào context dù lỗi dịch (vì câu gốc vẫn hợp lệ).
- **Actual**: `subtitle_count == 2`, `error_count == 1`, `recent_ctx.recent() == ["Cau 2.", "Cau 3."]`.
- **Kết quả**: PASS.

## Tổng kết

65 passed, 0 failed, 2 ignored (test tích hợp PBI 16/17 cần sidecar/model thật, không liên quan T009-T013) — xem `T009-T013-lib-tests-result.log`.
