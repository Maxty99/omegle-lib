use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Type to store omegle server (front1, front2,...). Taking
/// advantage of the fact that they all follow the pattern of
/// 'front' + number. It's essentially just a wrapper for [u8].
#[derive(Debug, PartialEq)]
pub struct Server {
    pub(crate) id_number: u8,
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
            if str.get(5..).is_some_and(|val| {!val.is_empty()}) {
                let id_number_as_str = str.get(5..).expect("str was checked to be Some");
                let id_number: u8 = id_number_as_str.parse()
                    .map_err(|_| {E::custom("expected server string that starts with 'front' to be followed by a u8")})?;
                Ok(Server { id_number })
            } else {
                Err(E::custom("expected server string that starts with 'front' to be followed by at least one char"))
            }
        } else {
            Err(E::custom("expected server string to start with 'front'"))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token, assert_de_tokens_error};

    use super::*;

    #[test]
    fn can_deserialize_and_serialize_valid_string() {
        let server = Server { id_number: 15 };
        assert_tokens(&server, &[
            Token::Str("front15")
        ])
    }

    #[test]
    fn can_not_deserialize_string_with_invalid_start() {
        assert_de_tokens_error::<Server>(&[
            Token::Str("fromt15")
        ],
        "expected server string to start with 'front'")
    }

    #[test]
    fn can_not_deserialize_string_with_no_id() {
        assert_de_tokens_error::<Server>(&[
            Token::Str("front")
        ],
        "expected server string that starts with 'front' to be followed by at least one char")
    }
    
    #[test]
    fn can_not_deserialize_string_with_invalid_id() {
        assert_de_tokens_error::<Server>(&[
            Token::Str("front155555")
        ],
        "expected server string that starts with 'front' to be followed by a u8")
    }
}
