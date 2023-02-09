use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum OmegleLibError {
    ConnectionError,
    UnexpectedRespone(String),
    CouldNotDetermineResponse,
    NoServers,
    InvalidID,
}

impl Display for OmegleLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OmegleLibError::ConnectionError => write!(f, "cannot connect to omegle"),
            OmegleLibError::UnexpectedRespone(response) => {
                write!(f, "unexpected response: {response}")
            }
            OmegleLibError::CouldNotDetermineResponse => {
                write!(f, "unexpected response: could not determine response")
            }
            //Not likely to happen but still gotta cover it
            OmegleLibError::NoServers => write!(f, "no valid omegle servers available"),
            OmegleLibError::InvalidID => {
                write!(f, "invalid id: must not contain 'I', 'O', '1', '0'")
            }
        }
    }
}

impl Error for OmegleLibError {}
