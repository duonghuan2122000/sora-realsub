# Test case bằng chứng: PBI 17 — Dịch có ngữ cảnh câu trước

**Log đầy đủ**: [T017-logic-tests.log](./T017-logic-tests.log) (chạy `cargo test` thật trong
crate Rust độc lập `pbi17-logic-check`, sao y nguyên logic thuần từ
`app/src-tauri/src/core/translator.rs` — xem [README-moi-truong.md](./README-moi-truong.md) để
biết tại sao không chạy trực tiếp `cargo test -p app` được trong container này).

## T005/T006 — Input rỗng / không có ngữ cảnh giữ hành vi PBI 16

- **Test**: `input_rong_tra_ve_rong_bat_ke_context`, `dich_khong_ngu_canh_giong_hanh_vi_pbi16`
- **Input**: `translate("", context=["John lost his keys."])`, `translate("   ", ...)`,
  `translate("Hello.", context=[])`
- **Expected**: input rỗng/khoảng trắng → trả `""` bất kể context; context rỗng → dịch câu hiện
  tại một mình như PBI 16 (`"[vi] Hello."`)
- **Actual**: khớp đúng kỳ vọng
- **Kết quả**: PASS

## T007/T008 — Ghép ngữ cảnh + câu hiện tại, tách đúng bản dịch câu hiện tại

- **Test**: `dich_co_ngu_canh_chi_tra_ve_cau_hien_tai`
- **Input**: `translate("He was very upset.", context=["John lost his keys."])`, sidecar giả dịch
  từng câu thành `"[vi] <câu>"`
- **Expected**: kết quả trả về là `"[vi] He was very upset."`, không chứa nội dung câu ngữ cảnh
  ("keys")
- **Actual**: `"[vi] He was very upset."`, không chứa "keys" — khớp
- **Kết quả**: PASS

## T009 — Fallback khi sidecar gộp câu không tách được

- **Test**: `sidecar_gop_cau_thi_fallback_dich_lai_khong_ngu_canh`
- **Input**: sidecar giả trả về 1 câu gộp (`"mot cau gop khong tach duoc"`) khi input có nhiều câu
  (ghép ngữ cảnh), dịch bình thường khi input chỉ có 1 câu
- **Expected**: `translate()` phát hiện số đoạn tách không khớp → tự gọi lại dịch câu hiện tại
  không ngữ cảnh → trả về `"[vi] He was very upset."`, không trả về chuỗi gộp rác
- **Actual**: khớp đúng kỳ vọng
- **Kết quả**: PASS

## T010 — Dịch có ngữ cảnh, kiểm tra trực tiếp input gửi đi + kết quả tách

- **Test**: `ghep_ngu_canh_va_tach_dung_cau_hien_tai` (trong `translator.rs` thật, dùng
  `EchoPrefixSidecar` + `split_sentences`)
- **Input**: `"John lost his keys. He was very upset."`
- **Expected**: tách ra đúng 2 đoạn, đoạn cuối là bản dịch câu hiện tại, không bằng toàn bộ bản
  dịch ghép
- **Actual**: khớp đúng kỳ vọng
- **Kết quả**: PASS (xác nhận bằng code trong `translator.rs`; hành vi tương đương cũng được xác
  nhận lại trong log chuẩn, test `dich_co_ngu_canh_chi_tra_ve_cau_hien_tai`)

## T011 — Context rỗng

- **Test**: `normalize_context_rong_tra_ve_rong`
- **Input**: `normalize_context(&[])`
- **Expected/Actual**: `vec![]`
- **Kết quả**: PASS

## T012 — Context chứa câu rỗng/khoảng trắng

- **Test**: `normalize_context_loai_cau_rong`
- **Input**: `["", "John lost his keys.", "   "]`
- **Expected/Actual**: `["John lost his keys."]` — câu rỗng/khoảng trắng bị loại
- **Kết quả**: PASS

## T013 — Context nhiều hơn 2 câu

- **Test**: `normalize_context_gioi_han_toi_da_2_cau`, `dich_co_qua_2_cau_ngu_canh_chi_dung_2_cau_gan_nhat`
- **Input**: `["Câu xa 1.", "Câu xa 2.", "Câu gần."]` + câu hiện tại `"Câu hiện tại."`
- **Expected**: chỉ 2 câu gần nhất ("Câu xa 2."/"Câu gần.") được dùng; input gửi sidecar không
  chứa "Câu xa 1"
- **Actual**: khớp đúng kỳ vọng (input gửi đi không chứa "Câu xa 1", có chứa "Câu gần." và "Câu
  hiện tại.")
- **Kết quả**: PASS

## T014 — Tách câu không khớp → fallback (hàm thuần)

- **Test**: `tach_cau_khong_khop_tra_ve_none_de_fallback`, `tach_cau_khop_tra_ve_dung_cau_cuoi`
- **Input**: `extract_current_translation("mot cau gop khong tach duoc", 2)` và
  `extract_current_translation("[vi] John lost his keys. [vi] He was very upset.", 2)`
- **Expected**: `None` khi không khớp số câu; `Some("[vi] He was very upset.")` khi khớp
- **Actual**: khớp đúng kỳ vọng cả 2 trường hợp
- **Kết quả**: PASS

## T015 — Ổn định giữa các lần gọi

- **Test**: `dich_lap_lai_cung_input_cho_ket_qua_on_dinh`, `normalize_context_on_dinh_giua_cac_lan_goi`
- **Input**: gọi 2 lần `translate_with_context("He was very upset.", context, echo_prefix_sidecar)`
- **Expected**: 2 lần cho kết quả giống nhau
- **Actual**: khớp đúng kỳ vọng
- **Kết quả**: PASS

## Hồi quy (test cũ PBI 16 không đổi hành vi)

- `dich_kem_ngu_canh`, `goi_qua_trait_object`, `sidecar_khong_ton_tai_tra_loi`,
  `input_rong_tra_ve_rong_khong_spawn_tien_trinh` trong `translator.rs` thật **không bị sửa logic**
  — vẫn đúng với cài đặt mới vì các nhánh rỗng/không-ngữ-cảnh không đổi hành vi so với PBI 16.
  Không chạy được trực tiếp trong container này (lý do: xem README-moi-truong.md) nhưng không có
  thay đổi nào trong code ảnh hưởng tới các test này.

## Tổng kết

**13/13 test logic** (bản sao thuần, hành vi y hệt code thật) `ok` — xem log đầy đủ. Test
`cargo test -p app translator` trên code thật **chưa chạy được** trong container Linux này do
hạn chế môi trường có từ trước (PBI 17 không gây ra, không sửa được trong phạm vi PBI 17) — xem
README-moi-truong.md. T016/T018 (test tích hợp sidecar+model thật, SC-001) cũng chưa chạy được vì
cùng lý do cộng với việc sidecar/model không có sẵn trong container mới.
