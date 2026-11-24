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

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub source_text: String,
    pub translated_text: String,
    pub timestamp_ms: u64,
}
