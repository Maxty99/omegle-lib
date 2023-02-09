use crate::error::OmegleLibError;
use rand::distributions::Distribution;
use rand::Rng;

pub struct RandID {
    id: String,
}

impl TryFrom<String> for RandID {
    type Error = OmegleLibError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            Ok(Self { id: value })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl TryFrom<&String> for RandID {
    type Error = OmegleLibError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            Ok(Self {
                id: String::from(value),
            })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl TryFrom<&str> for RandID {
    type Error = OmegleLibError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 8
            && value
                .chars()
                .all(|char| (char != '1' && char != '0' && char != 'I' && char != 'O'))
        {
            Ok(Self {
                id: String::from(value),
            })
        } else {
            Err(OmegleLibError::InvalidID)
        }
    }
}

impl Into<String> for RandID {
    fn into(self) -> String {
        self.id
    }
}

impl RandID {
    pub fn new() -> Self {
        let id: String = rand::thread_rng()
            .sample_iter(OmegleCharset)
            .take(8)
            .map(char::from)
            .collect();
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
}
