#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

pub struct StreamHandle {
    stop_fn: Box<dyn FnOnce() + Send>,
}

impl StreamHandle {
    pub fn new(stop_fn: impl FnOnce() + Send + 'static) -> Self {
        Self {
            stop_fn: Box::new(stop_fn),
        }
    }

    pub fn stop(self) {
        (self.stop_fn)();
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptChunk {
    pub text: String,
    pub is_final: bool,
}

/// Phân loại lỗi runtime model STT (PBI 15; xem data-model.md): `Load` khi model không nạp
/// được lúc bắt đầu phiên, `Infer` khi lỗi xảy ra lúc xử lý 1 segment cụ thể.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelErrorKind {
    Load,
    Infer,
}

/// Payload mô tả 1 lỗi runtime model STT, gửi cho UI qua event `model-error` (PBI 15).
#[derive(Debug, Clone)]
pub struct ModelErrorEvent {
    pub kind: ModelErrorKind,
    pub message: String,
    pub segment_start_frame_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub source_text: String,
    pub translated_text: String,
    pub timestamp_ms: u64,
}
