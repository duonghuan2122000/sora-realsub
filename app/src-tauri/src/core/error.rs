use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("lỗi I/O: {0}")]
    Io(String),
    #[error("lỗi provider: {0}")]
    Provider(String),
    #[error("đã bị hủy")]
    Cancelled,
}
