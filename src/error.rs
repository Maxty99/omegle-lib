use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum OmegleLibError {
    ConnectionError,
    CouldNotDetermineResponse,
    DeserializationError(serde_json::Error),
    InvalidID,
    InvalidServerString,
}

impl Display for OmegleLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OmegleLibError::ConnectionError => write!(f, "cannot connect to omegle"),

            OmegleLibError::CouldNotDetermineResponse => {
                write!(f, "unexpected response: could not determine response")
            }
            OmegleLibError::InvalidID => {
                write!(f, "invalid id: must not contain 'I', 'O', '1', '0'")
            }
            OmegleLibError::InvalidServerString => write!(f, "the given server string was invalid"),
            OmegleLibError::DeserializationError(err) => {
                write!(f, "error deserializing response from omegle: {err}")
            }
        }
    }
}

impl Error for OmegleLibError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OmegleLibError::DeserializationError(err) => Some(err),
            _ => None,
        }
    }
}
