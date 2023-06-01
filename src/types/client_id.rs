use std::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Type to store client id, takes advantage of the fact
/// that the id is a string that follows the pattern of
/// 'central' + [u8] + ':' + [char; 30]
pub struct ClientID {
    central_id: u8,
    user_id: [char; 30],
}

impl Into<String> for ClientID {
    fn into(self) -> String {
        //Avoid allocations
        let mut user_id_string = String::with_capacity(30);

        for elem in self.user_id {
            user_id_string.push(elem);
        }

        format!("central{}:{}", self.central_id, user_id_string)
    }
}

impl Into<String> for &ClientID {
    fn into(self) -> String {
        //Avoid allocations
        let mut user_id_string = String::with_capacity(30);

        for elem in self.user_id {
            user_id_string.push(elem);
        }

        format!("central{}:{}", self.central_id, user_id_string)
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

        if !str.starts_with("central") {
            Err(E::custom(
                "expected client id string to start with 'central'",
            ))
        } else if length < 30 {
            Err(E::custom(
                "expected client id string to be at least 30 chars",
            ))
        } else {
            let central_id_as_str = str.get(7..length - 31)
                .ok_or(E::custom("expected client id string that starts with 'central' to be followed by at least 32 chars"))?;
            let central_id: u8 = central_id_as_str.parse().map_err(|_| {
                E::custom("expected client id string to contain a valid integer after 'central'")
            })?;
            let user_id_as_str = str.get(length - 30..).ok_or(E::custom(
                "expected client id string to end with at least 30 chars",
            ))?;
            let mut user_id = [' '; 30];
            for (idx, char) in user_id_as_str.char_indices() {
                user_id[idx] = char;
            }

            Ok(ClientID {
                central_id,
                user_id,
            })
        }
    }
}
