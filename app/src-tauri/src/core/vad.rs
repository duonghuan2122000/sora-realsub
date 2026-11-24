use super::error::CoreError;
use super::mixed_audio_source::resample_linear;
use super::types::AudioFrame;
use voice_activity_detector::VoiceActivityDetector;

/// Silero VAD v5 chỉ nhận 512 sample/frame ở 16kHz (xem research.md PBI 9).
pub(crate) const VAD_SAMPLE_RATE: u32 = 16_000;
pub(crate) const VAD_CHUNK_SIZE: usize = 512;
const DEFAULT_SPEECH_THRESHOLD: f32 = 0.5;

/// Nhãn speech/silence cho 1 frame cố định, giữ đúng thứ tự thời gian gốc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameLabel {
    pub frame_index: usize,
    pub is_speech: bool,
}

/// Gán nhãn speech/silence cho từng frame của một hoặc nhiều `AudioFrame` nối tiếp.
/// Dùng ngưỡng xác suất mặc định; buffer rỗng/ngắn hơn 1 frame trả về `Vec` rỗng.
pub fn detect(frames: &[AudioFrame]) -> Result<Vec<FrameLabel>, CoreError> {
    detect_with_threshold(frames, DEFAULT_SPEECH_THRESHOLD)
}

pub fn detect_with_threshold(
    frames: &[AudioFrame],
    speech_threshold: f32,
) -> Result<Vec<FrameLabel>, CoreError> {
    let samples = resample_all(frames);
    if samples.len() < VAD_CHUNK_SIZE {
        return Ok(Vec::new());
    }

    let mut vad = VoiceActivityDetector::builder()
        .sample_rate(VAD_SAMPLE_RATE)
        .chunk_size(VAD_CHUNK_SIZE)
        .build()
        .map_err(|e| CoreError::Provider(format!("khởi tạo VAD lỗi: {e}")))?;

    let labels = samples
        .chunks_exact(VAD_CHUNK_SIZE)
        .enumerate()
        .map(|(frame_index, chunk)| {
            let probability = vad.predict(chunk.to_vec());
            FrameLabel {
                frame_index,
                is_speech: probability >= speech_threshold,
            }
        })
        .collect();

    Ok(labels)
}

/// Nối mẫu của toàn bộ `AudioFrame` (resample về 16kHz mono nếu cần) thành 1 buffer liên tục.
pub(crate) fn resample_all(frames: &[AudioFrame]) -> Vec<f32> {
    let mut out = Vec::new();
    for frame in frames {
        out.extend(resample_linear(
            &frame.samples,
            frame.sample_rate,
            VAD_SAMPLE_RATE,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silence_frame(sample_rate: u32, len: usize) -> AudioFrame {
        AudioFrame {
            samples: vec![0.0; len],
            sample_rate,
        }
    }

    #[test]
    fn empty_buffer_returns_empty_labels() {
        let labels = detect(&[]).expect("không lỗi với buffer rỗng");
        assert!(labels.is_empty());
    }

    #[test]
    fn buffer_shorter_than_one_frame_returns_empty_labels() {
        let frame = silence_frame(VAD_SAMPLE_RATE, VAD_CHUNK_SIZE / 2);
        let labels = detect(&[frame]).expect("không lỗi với buffer ngắn hơn 1 frame");
        assert!(labels.is_empty());
    }

    #[test]
    fn pure_silence_is_labeled_silence() {
        // 10 frame im lặng tuyệt đối (biên độ 0) phải được nhãn silence.
        let frame = silence_frame(VAD_SAMPLE_RATE, VAD_CHUNK_SIZE * 10);
        let labels = detect(&[frame]).expect("vad chạy thành công");
        assert_eq!(labels.len(), 10);
        let silence_count = labels.iter().filter(|l| !l.is_speech).count();
        assert!(
            silence_count as f32 / labels.len() as f32 >= 0.9,
            "kỳ vọng >=90% silence, thực tế {}/{}",
            silence_count,
            labels.len()
        );
    }

    #[test]
    fn labels_keep_original_frame_order() {
        let frame = silence_frame(VAD_SAMPLE_RATE, VAD_CHUNK_SIZE * 5);
        let labels = detect(&[frame]).expect("vad chạy thành công");
        let indices: Vec<usize> = labels.iter().map(|l| l.frame_index).collect();
        let expected: Vec<usize> = (0..labels.len()).collect();
        assert_eq!(indices, expected);
    }
}
