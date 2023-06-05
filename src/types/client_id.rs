use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub  enum ServerType {
    Central,
    Shard,
}

impl From<ServerType> for String {
    fn from(value: ServerType) -> Self {
        match value {
            ServerType::Central => String::from("central"),
            ServerType::Shard =>  String::from("shard"),
        }
    }
}

impl From<&ServerType> for String {
    fn from(value: &ServerType) -> Self {
        match value {
            ServerType::Central => String::from("central"),
            ServerType::Shard =>  String::from("shard"),
        }
    }
}

/// Type to store client id, takes advantage of the fact
/// that the id is a string that follows the pattern of
/// 'central' + [u8] + ':' + [char; 30]
#[derive(Debug, PartialEq)]
pub struct ClientID {
    pub(crate) server_type: ServerType,
    pub(crate) server_id: u8,
    pub(crate) user_id: [char; 30],
}

impl Into<String> for ClientID {
    fn into(self) -> String {
        //Avoid allocations
        let mut user_id_string = String::with_capacity(30);
        let server_type_string = String::from(self.server_type);
        for elem in self.user_id {
            user_id_string.push(elem);
        }

        format!("{}{}:{}", server_type_string, self.server_id, user_id_string)
    }
}

impl Into<String> for &ClientID {
    fn into(self) -> String {
        //Avoid allocations
        let mut user_id_string = String::with_capacity(30);
        let server_type_string = String::from(self.server_type);
        for elem in self.user_id {
            user_id_string.push(elem);
        }

        format!("{}{}:{}", server_type_string, self.server_id, user_id_string)
    }
}

impl Serialize for ClientID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let client_id_string: String = self.into();
        serializer.serialize_str(&client_id_string)
    }
}

impl<'de> Deserialize<'de> for ClientID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ClientIDVisitor)
    }
}

struct ClientIDVisitor;

impl<'de> Visitor<'de> for ClientIDVisitor {
    type Value = ClientID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "A string with following the format: 'central' + `u8` + ':' + 30 ascii chars"
        )
    }
    fn visit_str<E>(self, str: &str) -> Result<ClientID, E>
    where
        E: Error,
    {
        let length = str.len();

        if !str.starts_with("central") && !str.starts_with("shard") {
            Err(E::custom(
                "expected client id string to start with 'central' or 'shard'",
            ))
        } else if length < 30 {
            Err(E::custom(
                "expected client id string to be at least 30 chars",
            ))
        } else {
            let (server_type, central_id_as_str) = 
                if str.starts_with("central") { 
                    (ServerType::Central, str.get(7..length - 31)
                        .ok_or(E::custom("expected client id string that starts with 'central' to have at least one char before ':' and the user id"))?)} 
                else {
                    (ServerType::Shard, str.get(5..length - 31)
                    .ok_or(E::custom("expected client id string that starts with 'shard' to have at least one char before ':' and the user id"))?)};
            let central_id: u8 = central_id_as_str.parse().map_err(|_| {
                E::custom("expected client id string to contain a valid u8 after 'central' or 'shard'")
            })?;
            let user_id_as_str = str.get(length - 30..).ok_or(E::custom(
                "expected client id string to end with at least 30 chars",
            ))?;
            let mut user_id = [' '; 30];
            for (idx, char) in user_id_as_str.char_indices() {
                user_id[idx] = char;
            }

            Ok(ClientID {
                server_type,
                server_id: central_id,
                user_id,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token, assert_de_tokens_error};

    use super::*;

    #[test]
    fn can_deserialize_and_serialize_valid_central_string() {
        let client_id = ClientID {
            server_type: ServerType::Central,
            server_id: 3, 
            user_id: ['a'; 30]
        };
        assert_tokens(&client_id, &[
            Token::Str("central3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        ])
    }

    #[test]
    fn can_deserialize_and_serialize_valid_shard_string() {
        let client_id = ClientID {
            server_type: ServerType::Shard,
            server_id: 3, 
            user_id: ['a'; 30]
        };
        assert_tokens(&client_id, &[
            Token::Str("shard3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        ])
    }

    #[test]
    fn can_not_deserialize_string_with_invalid_start() {
        assert_de_tokens_error::<ClientID>(&[
            Token::Str("sard15")
        ],
        "expected client id string to start with 'central' or 'shard'")
    }

    #[test]
    fn can_not_deserialize_string_less_than_thirty_chars() {
        assert_de_tokens_error::<ClientID>(&[
            Token::Str("central")
        ],
        "expected client id string to be at least 30 chars")
    }


    #[test]
    fn can_not_deserialize_central_string_with_no_id() {
        assert_de_tokens_error::<ClientID>(&[
            Token::Str("central:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        ],
        "expected client id string to contain a valid u8 after 'central' or 'shard'")
    }

    #[test]
    fn can_not_deserialize_shard_string_with_no_id() {
        assert_de_tokens_error::<ClientID>(&[
            Token::Str("shard:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        ],
        "expected client id string to contain a valid u8 after 'central' or 'shard'")
    }
    
    #[test]
    fn can_not_deserialize_string_with_invalid_id() {
        assert_de_tokens_error::<ClientID>(&[
            Token::Str("central155555:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        ],
        "expected client id string to contain a valid u8 after 'central' or 'shard'")
    }
}