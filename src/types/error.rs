#[derive(Debug, thiserror::Error)]
pub enum OmegleLibError {
    #[error("invalid id: must not contain 'I', 'O', '1', '0'")]
    InvalidID,
}
