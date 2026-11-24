use super::error::CoreError;
use super::segment::{self, DEFAULT_MIN_SILENCE_FRAMES};
use super::types::AudioFrame;
use super::vad::{self, FrameLabel, VAD_CHUNK_SIZE, VAD_SAMPLE_RATE};

/// Một đoạn audio vừa được VAD xác định đóng, sẵn sàng phát ra ngoài (xem data-model.md PBI 11).
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentReady {
    pub start_frame_index: usize,
    pub end_frame_index: usize,
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

/// Cắt segment streaming: tích lũy `AudioFrame` liên tục, mỗi lần nhận frame mới chạy lại VAD +
/// cắt segment trên toàn bộ buffer rồi chỉ trả về segment MỚI vừa đóng thật (xem research.md PBI 11,
/// quyết định 1). Segment còn mở ở cuối buffer (đang nói, hoặc lặng chưa đủ ngưỡng) không được trả về
/// cho tới khi đã đóng ở một lần `push_frame` sau.
#[derive(Default)]
pub struct VadPipeline {
    buffer: Vec<AudioFrame>,
    /// Số segment đã chắc chắn đóng và đã phát ra, loại trừ segment còn mở dở ở cuối buffer.
    closed_count: usize,
}

impl VadPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Thêm 1 frame audio mới, trả về các `SegmentReady` MỚI đóng kể từ lần gọi trước (FR-001, FR-004).
    pub fn push_frame(&mut self, frame: AudioFrame) -> Result<Vec<SegmentReady>, CoreError> {
        self.buffer.push(frame);
        let labels = vad::detect(&self.buffer)?;
        let segments = segment::segment(&labels);

        let closed_len = closed_segment_count(&labels, segments.len());
        if closed_len <= self.closed_count {
            return Ok(Vec::new());
        }

        let resampled = vad::resample_all(&self.buffer);
        let ready = segments[self.closed_count..closed_len]
            .iter()
            .map(|seg| SegmentReady {
                start_frame_index: seg.start_frame_index,
                end_frame_index: seg.end_frame_index,
                samples: slice_samples(&resampled, seg.start_frame_index, seg.end_frame_index),
                sample_rate: VAD_SAMPLE_RATE,
            })
            .collect();

        self.closed_count = closed_len;
        Ok(ready)
    }
}

/// Số segment trong kết quả `segment()` đã chắc chắn đóng — loại trừ segment mở dở cuối buffer
/// (đang nói, hoặc lặng chưa đủ `DEFAULT_MIN_SILENCE_FRAMES`), nếu có.
fn closed_segment_count(labels: &[FrameLabel], total_segments: usize) -> usize {
    if total_segments == 0 {
        return 0;
    }
    let trailing_silence = labels.iter().rev().take_while(|l| !l.is_speech).count();
    let last_label_is_speech = labels.last().is_some_and(|l| l.is_speech);
    let last_segment_is_open = last_label_is_speech || trailing_silence < DEFAULT_MIN_SILENCE_FRAMES;
    if last_segment_is_open {
        total_segments - 1
    } else {
        total_segments
    }
}

/// Cắt mẫu 16kHz mono tương ứng [start_frame_index, end_frame_index] (bao gồm) từ buffer đã resample.
fn slice_samples(resampled: &[f32], start_frame_index: usize, end_frame_index: usize) -> Vec<f32> {
    let start = start_frame_index * VAD_CHUNK_SIZE;
    let end = ((end_frame_index + 1) * VAD_CHUNK_SIZE).min(resampled.len());
    resampled.get(start..end).map(|s| s.to_vec()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels_from(pattern: &[bool]) -> Vec<FrameLabel> {
        pattern
            .iter()
            .enumerate()
            .map(|(frame_index, &is_speech)| FrameLabel { frame_index, is_speech })
            .collect()
    }

    #[test]
    fn khong_co_segment_nao_tra_ve_0() {
        assert_eq!(closed_segment_count(&labels_from(&[false; 5]), 0), 0);
    }

    #[test]
    fn nhan_cuoi_la_speech_segment_cuoi_van_mo() {
        // nói(3), còn đang nói ở frame cuối -> segment duy nhất chưa đóng.
        let labels = labels_from(&[true, true, true]);
        assert_eq!(closed_segment_count(&labels, 1), 0);
    }

    #[test]
    fn lang_chua_du_nguong_segment_cuoi_van_mo() {
        // nói(2) - lặng(DEFAULT_MIN_SILENCE_FRAMES - 1): chưa đủ để coi là đóng.
        let mut pattern = vec![true, true];
        pattern.extend(vec![false; DEFAULT_MIN_SILENCE_FRAMES - 1]);
        let labels = labels_from(&pattern);
        assert_eq!(closed_segment_count(&labels, 1), 0);
    }

    #[test]
    fn lang_du_nguong_segment_da_dong() {
        // nói(2) - lặng đủ ngưỡng: segment đã đóng, phải tính vào closed_count.
        let mut pattern = vec![true, true];
        pattern.extend(vec![false; DEFAULT_MIN_SILENCE_FRAMES]);
        let labels = labels_from(&pattern);
        assert_eq!(closed_segment_count(&labels, 1), 1);
    }

    #[test]
    fn hai_segment_dong_het_khi_lang_du_nguong_sau_segment_thu_hai() {
        let mut pattern = vec![true, true];
        pattern.extend(vec![false; DEFAULT_MIN_SILENCE_FRAMES]);
        pattern.extend(vec![true, true]);
        pattern.extend(vec![false; DEFAULT_MIN_SILENCE_FRAMES]);
        let labels = labels_from(&pattern);
        assert_eq!(closed_segment_count(&labels, 2), 2);
    }

    #[test]
    fn slice_samples_cat_dung_khoang_theo_vad_chunk_size() {
        let resampled: Vec<f32> = (0..VAD_CHUNK_SIZE * 4).map(|i| i as f32).collect();
        let sliced = slice_samples(&resampled, 1, 2);
        assert_eq!(sliced.len(), 2 * VAD_CHUNK_SIZE);
        assert_eq!(sliced[0], VAD_CHUNK_SIZE as f32);
    }
}
