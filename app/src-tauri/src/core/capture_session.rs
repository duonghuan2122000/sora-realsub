use super::audio_source::AudioSource;
use super::error::CoreError;
use super::types::{AudioFrame, StreamHandle};
use std::sync::Mutex;

/// Quản lý 1 phiên thu âm duy nhất: giữ `StreamHandle` của phiên đang chạy
/// (nếu có) để có thể dừng đúng cách, không leak thread/handle.
pub struct CaptureSessionState {
    handle: Mutex<Option<StreamHandle>>,
}

impl Default for CaptureSessionState {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptureSessionState {
    pub fn new() -> Self {
        Self {
            handle: Mutex::new(None),
        }
    }

    /// Bắt đầu phiên thu âm từ `source`. Lỗi `AlreadyRunning` nếu đã có phiên đang chạy.
    pub fn start(
        &self,
        source: &dyn AudioSource,
        on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
    ) -> Result<(), CoreError> {
        let mut slot = self.handle.lock().unwrap();
        if slot.is_some() {
            return Err(CoreError::AlreadyRunning);
        }
        let new_handle = source.start(on_frame)?;
        *slot = Some(new_handle);
        Ok(())
    }

    /// Dừng phiên thu âm đang chạy, giải phóng thread/handle. Lỗi `NotRunning` nếu không có phiên nào.
    pub fn stop(&self) -> Result<(), CoreError> {
        let mut slot = self.handle.lock().unwrap();
        match slot.take() {
            Some(handle) => {
                handle.stop();
                Ok(())
            }
            None => Err(CoreError::NotRunning),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Nguồn giả: khi `start` được gọi tăng `live`, khi `stop_fn` chạy giảm `live`.
    /// Đếm `live` sau khi dừng cho biết handle có được giải phóng đúng không.
    struct MockAudioSource {
        live: Arc<AtomicUsize>,
    }

    impl AudioSource for MockAudioSource {
        fn start(
            &self,
            _on_frame: Box<dyn FnMut(AudioFrame) + Send + 'static>,
        ) -> Result<StreamHandle, CoreError> {
            self.live.fetch_add(1, Ordering::SeqCst);
            let live = self.live.clone();
            Ok(StreamHandle::new(move || {
                live.fetch_sub(1, Ordering::SeqCst);
            }))
        }
    }

    #[test]
    fn start_thanh_cong_khi_chua_co_phien() {
        let live = Arc::new(AtomicUsize::new(0));
        let source = MockAudioSource { live: live.clone() };
        let session = CaptureSessionState::new();

        let result = session.start(&source, Box::new(|_| {}));

        assert!(result.is_ok());
        assert_eq!(live.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn start_that_bai_khi_da_co_phien_dang_chay() {
        let live = Arc::new(AtomicUsize::new(0));
        let source = MockAudioSource { live: live.clone() };
        let session = CaptureSessionState::new();
        session.start(&source, Box::new(|_| {})).unwrap();

        let result = session.start(&source, Box::new(|_| {}));

        assert!(matches!(result, Err(CoreError::AlreadyRunning)));
        assert_eq!(live.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn stop_thanh_cong_giai_phong_handle() {
        let live = Arc::new(AtomicUsize::new(0));
        let source = MockAudioSource { live: live.clone() };
        let session = CaptureSessionState::new();
        session.start(&source, Box::new(|_| {})).unwrap();

        let result = session.stop();

        assert!(result.is_ok());
        assert_eq!(live.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn stop_that_bai_khi_chua_co_phien() {
        let session = CaptureSessionState::new();

        let result = session.stop();

        assert!(matches!(result, Err(CoreError::NotRunning)));
    }

    #[test]
    fn lap_10_lan_start_stop_khong_leak_handle() {
        let live = Arc::new(AtomicUsize::new(0));
        let source = MockAudioSource { live: live.clone() };
        let session = CaptureSessionState::new();

        for _ in 0..10 {
            session.start(&source, Box::new(|_| {})).unwrap();
            assert_eq!(live.load(Ordering::SeqCst), 1);
            session.stop().unwrap();
            assert_eq!(live.load(Ordering::SeqCst), 0);
        }
    }
}
