# Test case T006 — unit test `MixedAudioSource` (logic trộn, dừng, im lặng 1 nguồn)

**Lệnh**: `cargo test core::mixed_audio_source` (chạy trong `app/src-tauri/`)

| Test | Input | Expected | Actual | Kết quả |
|---|---|---|---|---|
| `tron_hai_nguon_cung_co_tieng_cho_ra_tong_co_clamp` | 2 nguồn mock cùng phát `0.6` liên tục | Output = `0.6+0.6=1.2` bị clamp về `1.0` | 32 mẫu đầu đều `≈1.0` | PASS |
| `mot_nguon_im_lang_output_khop_nguon_con_lai` | nguồn A phát `0.3`, nguồn B im lặng hoàn toàn | Output khớp đúng nguồn A (`≈0.3`) | 32 mẫu đầu đều `≈0.3` | PASS |
| `stop_khong_panic_va_dung_nhan_mau_moi` | 2 nguồn im lặng, gọi `handle.stop()` ngay | Không panic | Không panic, join thread trộn + 2 nguồn con thành công | PASS |
| `resample_linear_giu_nguyen_khi_cung_rate` | mẫu cùng sample rate | Trả lại nguyên mẫu | Giống nguyên input | PASS |
| `resample_linear_doi_do_dai_khi_rate_khac_nhau` | 4 mẫu từ 8kHz lên 16kHz | Độ dài output tăng theo tỉ lệ (x2) | `len=8` | PASS |

Log đầy đủ: [T006-result.log](T006-result.log)

Đáp ứng FR-001, FR-002, FR-004, FR-005 (logic trộn + dừng) và FR-003 (resample khi rate khác nhau).
