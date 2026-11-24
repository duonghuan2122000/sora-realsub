use super::vad::FrameLabel;

/// ~300ms silence @ frame VAD 512 sample/16kHz (~32ms/frame) trước khi coi là điểm cắt (xem research.md PBI 10).
const DEFAULT_MIN_SILENCE_FRAMES: usize = 9;

/// Một lượt nói liên tục, biểu diễn bằng vùng chỉ số frame (theo `FrameLabel.frame_index`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub start_frame_index: usize,
    pub end_frame_index: usize,
}

/// Cắt danh sách nhãn speech/silence thành các segment, dùng ngưỡng silence mặc định.
pub fn segment(labels: &[FrameLabel]) -> Vec<Segment> {
    segment_with_min_silence(labels, DEFAULT_MIN_SILENCE_FRAMES)
}

/// Thuật toán silence-hangover: mở segment ở frame speech đầu tiên, đóng tại frame speech cuối
/// cùng trước khi có `min_silence_frames` frame silence liên tiếp. Segment còn mở khi hết danh
/// sách nhãn vẫn được trả về (FR-004); không có frame speech nào thì trả về rỗng (FR-005).
pub fn segment_with_min_silence(labels: &[FrameLabel], min_silence_frames: usize) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut open_start: Option<usize> = None;
    let mut last_speech: Option<usize> = None;
    let mut silence_run = 0usize;

    for label in labels {
        if label.is_speech {
            if open_start.is_none() {
                open_start = Some(label.frame_index);
            }
            last_speech = Some(label.frame_index);
            silence_run = 0;
        } else if open_start.is_some() {
            silence_run += 1;
            if silence_run >= min_silence_frames {
                segments.push(Segment {
                    start_frame_index: open_start.take().unwrap(),
                    end_frame_index: last_speech.take().unwrap(),
                });
                silence_run = 0;
            }
        }
    }

    if let (Some(start), Some(end)) = (open_start, last_speech) {
        segments.push(Segment {
            start_frame_index: start,
            end_frame_index: end,
        });
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels_from(pattern: &[bool]) -> Vec<FrameLabel> {
        pattern
            .iter()
            .enumerate()
            .map(|(frame_index, &is_speech)| FrameLabel {
                frame_index,
                is_speech,
            })
            .collect()
    }

    #[test]
    fn empty_labels_returns_empty_segments() {
        assert!(segment(&[]).is_empty());
    }

    #[test]
    fn all_silence_returns_empty_segments() {
        let labels = labels_from(&[false; 20]);
        assert!(segment(&labels).is_empty());
    }

    #[test]
    fn short_silence_gap_does_not_split_segment() {
        // nói(3) - lặng ngắn(2, dưới ngưỡng 9) - nói(3): phải là 1 segment liên tục.
        let mut pattern = vec![true; 3];
        pattern.extend(vec![false; 2]);
        pattern.extend(vec![true; 3]);
        let labels = labels_from(&pattern);

        let segments = segment(&labels);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_frame_index, 0);
        assert_eq!(segments[0].end_frame_index, 7);
    }

    #[test]
    fn long_silence_gap_splits_into_two_segments() {
        // nói(3) - lặng dài(9, đạt ngưỡng) - nói(3): phải tách thành 2 segment.
        let mut pattern = vec![true; 3];
        pattern.extend(vec![false; 9]);
        pattern.extend(vec![true; 3]);
        let labels = labels_from(&pattern);

        let segments = segment(&labels);
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], Segment {
            start_frame_index: 0,
            end_frame_index: 2,
        });
        assert_eq!(segments[1], Segment {
            start_frame_index: 12,
            end_frame_index: 14,
        });
    }

    #[test]
    fn open_segment_at_end_of_stream_is_still_returned() {
        // nói liên tục tới hết danh sách, chưa gặp đủ silence để đóng.
        let pattern = vec![true; 5];
        let labels = labels_from(&pattern);

        let segments = segment(&labels);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_frame_index, 0);
        assert_eq!(segments[0].end_frame_index, 4);
    }

    #[test]
    fn single_speech_frame_is_a_valid_segment() {
        let mut pattern = vec![false; 5];
        pattern.push(true);
        pattern.extend(vec![false; 5]);
        let labels = labels_from(&pattern);

        let segments = segment(&labels);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_frame_index, 5);
        assert_eq!(segments[0].end_frame_index, 5);
    }
}
