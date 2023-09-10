/// Enum describing all possible errors of the library
#[derive(Debug, thiserror::Error)]
pub enum OmegleLibError {
    /// Error returned when the ID does not follow convention
    #[error("invalid id: must not contain 'I', 'O', '1', '0'")]
    InvalidID,

    /// Transparent error for reqwest
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// An error response returned if a particular action failed during a chat session
    #[error("omegle server responded with '{0}'")]
    OmegleError(String),
}
