use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Type to store omegle server (front1, front2,...). Taking
/// advantage of the fact that they all follow the pattern of
/// 'front' + number. It's essentially just a wrapper for [u8].
#[derive(Debug)]
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

impl Serialize for Server {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let server_string: String = self.into();
        serializer.serialize_str(&server_string)
    }
}

impl<'de> Deserialize<'de> for Server {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ServerVisitor)
    }
}

struct ServerVisitor;

impl<'de> Visitor<'de> for ServerVisitor {
    type Value = Server;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "A string with following the format: 'front' + `u8`"
        )
    }
    fn visit_str<E>(self, str: &str) -> Result<Server, E>
    where
        E: Error,
    {
        if str.starts_with("front") {
            let id_number_as_str = str.get(5..)
                .ok_or(E::custom("expected server string that starts with 'front' to be followed by at least one char"))?;
            let id_number: u8 = id_number_as_str.parse()
                .map_err(|_| {E::custom("expected server string that starts with 'front' to be followed by numeric chars")})?;
            Ok(Server { id_number })
        } else {
            Err(E::custom("expected server string to start with 'front'"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_valid_string() {
        let server_string = "\"front15\"";
        let server = serde_json::from_str::<Server>(server_string);
        assert!(server.is_ok())
    }
}
