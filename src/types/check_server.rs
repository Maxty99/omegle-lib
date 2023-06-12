use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct CheckServer(pub(crate) u8);

impl From<CheckServer> for String {
    fn from(value: CheckServer) -> Self {
        format!("waw{}.omegle.com", value.0)
    }
}

impl<'de> Deserialize<'de> for CheckServer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CheckServerVisitor)
    }
}

struct CheckServerVisitor;

impl<'de> Visitor<'de> for CheckServerVisitor {
    type Value = CheckServer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "A string with following the format: 'waw' + `u8` + 'omegle.com'"
        )
    }
    fn visit_str<E>(self, str: &str) -> Result<CheckServer, E>
    where
        E: Error,
    {
        if str.len() == 15 && str.starts_with("waw") && str.ends_with(".omegle.com") {
            let variant_as_str = str.get(3..4).expect(
                "str was checked to have at least one char between 'waw' and '.omegle.com'",
            );
            let variant: u8 = variant_as_str.parse().map_err(|_| {
                E::custom(
                    "expected check server string that starts with 'waw' to be followed by a u8",
                )
            })?;
            Ok(CheckServer(variant))
        } else {
            Err(E::custom(
                "expected check server string to start with 'waw' and end with '.omegle.com', and to be 15 chars long",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use super::CheckServer;

    #[test]
    fn can_deserialize_valid_string() {
        let expected_server = CheckServer(1);
        assert_de_tokens(&expected_server, &[Token::BorrowedStr("waw1.omegle.com")])
    }

    #[test]
    fn cant_deserialize_short_string() {
        assert_de_tokens_error::<CheckServer>(&[Token::BorrowedStr("waw.omegle.com")], 
        "expected check server string to start with 'waw' and end with '.omegle.com', and to be 15 chars long")
    }

    #[test]
    fn cant_deserialize_incorrect_domain() {
        assert_de_tokens_error::<CheckServer>(&[Token::BorrowedStr("www1.omegle.com")], 
        "expected check server string to start with 'waw' and end with '.omegle.com', and to be 15 chars long")
    }
}
