use crate::types::error::OmegleLibError;
use rand::distributions::Distribution;
use rand::Rng;
use serde::{Serialize, Serializer};

/// Omegle's Random ID used internally by Omegle to assign you with
/// (relatively) new chatters each time. Meant to be used througout
/// the chat session to ensure behavior consisten to that of the website.
/// Can be manually overridden as long as it follows convention.
///
/// # Convention
/// The Random ID is a 8 letter string that contains chars A-Z and 1-9
/// **with the exception of** 'O', 'I', '1', and '0'
///
/// # Examples
/// Generate new random ID that is guarenteed to follow convention:
/// ```rust
/// use omegle_rs::types::rand_id::RandID;
/// let rand_id = RandID::new();
/// ```
/// ---
/// Generate manual ID:
/// ```rust
/// use omegle_rs::types::rand_id::RandID;
/// let id = "ABCDEFGH";
/// let rand_id = RandID::try_from(id).expect("Follows convention");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandID {
    // Better than storing in string since we know it must be 8 chars
    id: [char; 8],
}

impl TryFrom<String> for RandID {
    type Error = OmegleLibError;
    /// Tries to create a new [`RandID`]
    /// # Errors:
    /// Returns [Err] if it doesnt follow the convention
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            let mut id: [char; 8] = Default::default();
            for element in value.char_indices() {
                let (idx, chr) = element;
                id[idx] = chr;
            }
            Ok(Self { id })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl TryFrom<&String> for RandID {
    type Error = OmegleLibError;
    /// Tries to create a new [`RandID`]
    /// # Errors:
    /// Returns [Err] if it doesnt follow the convention
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            let mut id: [char; 8] = Default::default();
            for element in value.char_indices() {
                let (idx, chr) = element;
                id[idx] = chr;
            }
            Ok(Self { id })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl TryFrom<&str> for RandID {
    type Error = OmegleLibError;
    /// Tries to create a new [`RandID`]
    /// # Errors:
    /// Returns [Err] if it doesnt follow the convention
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            let mut id: [char; 8] = Default::default();
            for element in value.char_indices() {
                let (idx, chr) = element;
                id[idx] = chr;
            }
            Ok(Self { id })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl Into<String> for RandID {
    /// Performs the conversion of [`RandID`] into a [String].
    /// Optimized to allocate all 8 bytes right away.
    fn into(self) -> String {
        //Avoid allocations
        let mut string = String::with_capacity(8);

        for elem in self.id {
            string.push(elem);
        }
        string
    }
}

impl Into<String> for &RandID {
    /// Performs the conversion of [`RandID`] into a [String].
    /// Optimized to allocate all 8 bytes right away.
    fn into(self) -> String {
        //Avoid allocations
        let mut string = String::with_capacity(8);

        for elem in self.id {
            string.push(elem);
        }
        string
    }
}

impl Serialize for RandID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let randid_string: String = self.into();
        serializer.serialize_str(&randid_string)
    }
}

impl RandID {
    /// Creates a new [`RandID`] randomly using the convention
    pub fn new() -> Self {
        let mut id: [char; 8] = Default::default();
        for element in (0..8 as usize).zip(
            rand::thread_rng()
                .sample_iter(OmegleCharset)
                .take(8)
                .map(char::from),
        ) {
            let (idx, chr) = element;
            id[idx] = chr;
        }
        Self { id }
    }
}

// Same idea as the Alphanumeric implementation from the rand crate
// except the size of the sample set is exactly 32 so we can take the
// top 5 bits.
struct OmegleCharset;
impl Distribution<u8> for OmegleCharset {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 32;
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        loop {
            let var = rng.next_u32() >> (32 - 5);
            if var < RANGE {
                return CHARSET[var as usize];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_ser_tokens};

    use super::*;

    #[test]
    fn errors_on_invalid_id_string() {
        let id_string = "ABCDE1IO";
        let id = RandID::try_from(id_string);
        assert!(id.is_err())
    }

    #[test]
    fn allows_valid_id_string() {
        let id_string = "ABCDEFGH";
        let id = RandID::try_from(id_string);
        assert!(id.is_ok())
    }

    #[test]
    fn errors_on_long_id_string() {
        let id_string = "AAAAAAAAAAA";
        let id = RandID::try_from(id_string);
        assert!(id.is_err())
    }

    #[test]
    fn errors_on_short_id_string() {
        let id_string = "ABC";
        let id = RandID::try_from(id_string);
        assert!(id.is_err())
    }

    #[test]
    fn new_generates_valid_id() {
        let id_rand = RandID::new();
        let id_string: String = id_rand.into();
        let id = RandID::try_from(id_string);
        assert!(id.is_ok())
    }

    #[test]
    fn serializes_to_valid_id() {
        let rand_id = RandID::try_from("ABCDEFGH").expect("Is valid id");
        assert_ser_tokens(&rand_id, &[Token::Str("ABCDEFGH")])
    }
}
