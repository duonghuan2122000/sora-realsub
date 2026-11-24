# Test case: T006 — AudioSource mock + thay thế cài đặt

**File**: `app/src-tauri/src/core/audio_source.rs`

| Test | Input | Kỳ vọng | Thực tế | Kết quả |
|---|---|---|---|---|
| `mock_silent_phat_ra_im_lang` | `MockAudioSourceSilent` | mẫu đầu = `0.0` | `0.0` | PASS |
| `mock_noise_phat_ra_nhieu` | `MockAudioSourceNoise` | mẫu đầu = `0.5` | `0.5` | PASS |
| `doi_cai_dat_khong_can_sua_code_goi` | `[MockAudioSourceSilent, MockAudioSourceNoise]` qua `&dyn AudioSource` | `[0.0, 0.5]`, hàm gọi không đổi code | `[0.0, 0.5]` | PASS |

Chạy bằng `cargo test core::audio_source::` — log đầy đủ: [T014-result.txt](T014-result.txt).
