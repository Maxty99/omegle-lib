use crate::error::OmegleLibError;
use serde::{Deserialize, Serialize};

/// Type to store omegle server (front1, front2,...). Taking
/// advantage of the fact that they all follow the pattern of
/// 'front' + number. It's essentially just a wrapper for [u8].
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    id_number: u8,
}

impl From<u8> for Server {
    fn from(value: u8) -> Self {
        Self { id_number: value }
    }
}

impl Into<String> for Server {
    fn into(self) -> String {
        format!("front{}", self.id_number)
    }
}

impl Into<String> for &Server {
    fn into(self) -> String {
        format!("front{}", self.id_number)
    }
}

impl Server {
    pub fn get_id_from_server_string(server_string: impl ToString) -> Result<u8, OmegleLibError> {
        let string = server_string.to_string();
        if string.starts_with("front") && string.is_ascii() {
            let num_as_str = string.get(5..).ok_or(OmegleLibError::InvalidServerString)?;
            let id_number: u8 = num_as_str
                .parse()
                .map_err(|_| OmegleLibError::InvalidServerString)?;
            Ok(id_number)
        } else {
            Err(OmegleLibError::InvalidServerString)
        }
    }
}
