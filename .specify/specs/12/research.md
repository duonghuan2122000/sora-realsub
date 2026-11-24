# Research: PBI 12 — STT local

## Quyết định: dùng `whisper-rs` (binding whisper.cpp) làm engine STT

**Lý do**:
- Wiki dự án (`Pipeline-xu-ly-audio.md`) đã chốt hướng STT local bằng `whisper-rs`/whisper.cpp khi roadmap đổi sang local (xem commit `24d1897`).
- Rust-native binding, không cần runtime Python, phù hợp Core Rust/Tauri đóng gói thành 1 binary.
- whisper.cpp hỗ trợ GGUF/ggml quantized model, chạy CPU tốt trên máy người dùng, không cần GPU bắt buộc.

**Phương án khác đã xem xét**:
- `faster-whisper` (Python, CTranslate2): nhanh nhưng cần runtime Python + đóng gói riêng, phá vỡ mục tiêu 1 binary Tauri gọn nhẹ → loại.
- Gọi API cloud (Azure/Deepgram): đã loại ở M0 (quyết định hướng local).

## Quyết định: input 16kHz mono f32 PCM, không phụ thuộc định dạng file

**Lý do**: pipeline capture (M1) đã chuẩn hoá audio về `AudioFrame { samples: Vec<f32>, sample_rate }`; whisper.cpp yêu cầu 16kHz mono f32 — khớp sẵn, không cần resample thêm ở PBI này. File audio mẫu dùng để test sẽ được decode (hound/symphonia) rồi resample nếu cần trước khi đưa vào `transcribe`.

## Quyết định: nạp model ggml/gguf từ đường dẫn file cục bộ, không bundle model vào source

**Lý do**: model whisper (dù bản `tiny`/`base`) nặng vài chục MB–vài trăm MB, không hợp để commit vào repo. PBI này giả định model đã có sẵn ở một đường dẫn cấu hình được (constant/biến môi trường cho test); việc UI chọn/tải model là phạm vi khác (M3 sau hoặc M6).

## Điểm NEEDS CLARIFICATION đã giải quyết

Không còn điểm nào — toàn bộ đã quyết định dựa trên wiki dự án và spec.md.
