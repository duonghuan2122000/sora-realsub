use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("lỗi I/O: {0}")]
    Io(String),
    #[error("lỗi provider: {0}")]
    Provider(String),
    #[error("đã bị hủy")]
    Cancelled,
    #[error("đã có phiên đang chạy")]
    AlreadyRunning,
    #[error("không có phiên nào để dừng")]
    NotRunning,
}
