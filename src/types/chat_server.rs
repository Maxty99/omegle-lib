use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Type to store omegle server (front1, front2,...). Taking
/// advantage of the fact that they all follow the pattern of
/// 'front' + number. It's essentially just a wrapper for [u8].
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct ChatServer {
    pub(crate) id_number: u8,
}

impl From<u8> for ChatServer {
    fn from(value: u8) -> Self {
        Self { id_number: value }
    }
}

impl From<ChatServer> for String {
    fn from(val: ChatServer) -> Self {
        format!("front{}", val.id_number)
    }
}

impl Serialize for ChatServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let server_string: String = (*self).into();
        serializer.serialize_str(&server_string)
    }
}

impl<'de> Deserialize<'de> for ChatServer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChatServerVisitor)
    }
}

struct ChatServerVisitor;

impl<'de> Visitor<'de> for ChatServerVisitor {
    type Value = ChatServer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "A string with following the format: 'front' + `u8`"
        )
    }
    fn visit_str<E>(self, str: &str) -> Result<ChatServer, E>
    where
        E: Error,
    {
        if str.starts_with("front") {
            if str.get(5..).is_some_and(|val| !val.is_empty()) {
                let id_number_as_str = str.get(5..).expect("str was checked to be Some");
                let id_number: u8 = id_number_as_str.parse().map_err(|_| {
                    E::custom(
                        "expected server string that starts with 'front' to be followed by a u8",
                    )
                })?;
                Ok(ChatServer { id_number })
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
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    use super::*;

    #[test]
    fn can_deserialize_and_serialize_valid_string() {
        let server = ChatServer { id_number: 15 };
        assert_tokens(&server, &[Token::Str("front15")])
    }

    #[test]
    fn can_not_deserialize_string_with_invalid_start() {
        assert_de_tokens_error::<ChatServer>(
            &[Token::Str("fromt15")],
            "expected server string to start with 'front'",
        )
    }

    #[test]
    fn can_not_deserialize_string_with_no_id() {
        assert_de_tokens_error::<ChatServer>(
            &[Token::Str("front")],
            "expected server string that starts with 'front' to be followed by at least one char",
        )
    }

    #[test]
    fn can_not_deserialize_string_with_invalid_id() {
        assert_de_tokens_error::<ChatServer>(
            &[Token::Str("front155555")],
            "expected server string that starts with 'front' to be followed by a u8",
        )
    }
}
