#[derive(Debug, thiserror::Error)]
pub enum OmegleLibError {
    #[error("invalid id: must not contain 'I', 'O', '1', '0'")]
    InvalidID,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("omegle server responded with '{0}'")]
    OmegleError(String),
}
