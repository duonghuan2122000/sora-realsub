use super::error::CoreError;
use super::types::Subtitle;

/// Nơi nhận phụ đề đã sẵn sàng (overlay UI, file, log...).
pub trait SubtitleSink: Send + Sync {
    fn push(&self, subtitle: Subtitle) -> Result<(), CoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Cài đặt giả: lưu lại mọi subtitle nhận được vào bộ nhớ.
    struct MockSubtitleSink {
        received: Mutex<Vec<Subtitle>>,
    }

    impl MockSubtitleSink {
        fn new() -> Self {
            Self {
                received: Mutex::new(Vec::new()),
            }
        }
    }

    impl SubtitleSink for MockSubtitleSink {
        fn push(&self, subtitle: Subtitle) -> Result<(), CoreError> {
            self.received.lock().unwrap().push(subtitle);
            Ok(())
        }
    }

    #[test]
    fn nhan_dung_du_lieu_subtitle() {
        let sink = MockSubtitleSink::new();
        sink.push(Subtitle {
            source_text: "hello".into(),
            translated_text: "xin chào".into(),
            timestamp_ms: 1000,
        })
        .unwrap();

        let received = sink.received.lock().unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].translated_text, "xin chào");
    }
}
