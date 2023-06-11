use crate::types::chat_server::ChatServer;
use crate::types::check_server::CheckServer;
use crate::types::error::OmegleLibError;
use serde::Deserialize;
use vec1::Vec1;

//TODO: Add omegle antinude Check
#[derive(Deserialize, Debug, PartialEq)]
pub struct OmegleStatus {
    pub(crate) count: u64,
    pub(crate) servers: Vec1<ChatServer>,
    pub(crate) antinudeservers: Vec1<CheckServer>,
}

impl OmegleStatus {
    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn get_chat_server(&self) -> String {
        let chat_server = self.servers.first();
        (*chat_server).into()
    }

    pub fn get_check_server(&self) -> String {
        let check_server = self.antinudeservers.first();
        (*check_server).into()
    }
    /// Send request to omegle to fetch the current status of the server.   
    /// This is needed before doing anything else
    ///
    /// # Example:
    ///
    /// ```rust
    /// use omegle_rs::status::OmegleStatus;
    /// async fn run() {
    ///     let server_status = OmegleStatus::get_omegle_status().await.unwrap();
    ///     println!("There are {} users currently active", server_status.get_count())
    /// }
    ///```
    ///
    /// # Errors:
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response contained no text
    /// - The response was unexpected (Ex: Error on omegle's end)
    pub async fn get_omegle_status() -> Result<OmegleStatus, OmegleLibError> {
        let req = reqwest::get("https://omegle.com/status")
            .await
            .map_err(|_| OmegleLibError::ConnectionError)?;
        let resp = req
            .text()
            .await
            .map_err(|_| OmegleLibError::CouldNotDetermineResponse)?;
        let omegle_status: OmegleStatus =
            serde_json::from_str(&resp).map_err(OmegleLibError::DeserializationError)?;
        Ok(omegle_status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use vec1::vec1;

    #[test]
    fn valid_response_text_should_give_status() {
        let expected_val = OmegleStatus {
            count: 34658,
            servers: vec1![ChatServer { id_number: 20 }, ChatServer { id_number: 5 }],
            antinudeservers: vec1![
                CheckServer(2),
                CheckServer(4),
                CheckServer(1),
                CheckServer(3)
            ],
        };
        assert_de_tokens(
            &expected_val,
            &[
                Token::Map { len: Some(8) },
                Token::BorrowedStr("count"),
                Token::I64(34658),
                Token::BorrowedStr("antinudeservers"),
                Token::Seq { len: Some(4) },
                Token::BorrowedStr("waw2.omegle.com"),
                Token::BorrowedStr("waw4.omegle.com"),
                Token::BorrowedStr("waw1.omegle.com"),
                Token::BorrowedStr("waw3.omegle.com"),
                Token::SeqEnd,
                Token::BorrowedStr("spyQueueTime"),
                Token::F64(105.28910002708434),
                Token::BorrowedStr("rtmfp"),
                Token::BorrowedStr("rtmfp://p2p.rtmfp.net"),
                Token::BorrowedStr("antinudepercent"),
                Token::F32(1.0),
                Token::BorrowedStr("spyeeQueueTime"),
                Token::F64(229.33259999752045),
                Token::BorrowedStr("timestamp"),
                Token::F64(1685331229.225212),
                Token::BorrowedStr("servers"),
                Token::Seq { len: Some(2) },
                Token::BorrowedStr("front20"),
                Token::BorrowedStr("front5"),
                Token::SeqEnd,
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn no_servers_given_should_error() {
        assert_de_tokens_error::<OmegleStatus>(
            &[
                Token::Map { len: Some(8) },
                Token::BorrowedStr("count"),
                Token::I64(34658),
                Token::BorrowedStr("antinudeservers"),
                Token::Seq { len: Some(4) },
                Token::BorrowedStr("waw2.omegle.com"),
                Token::BorrowedStr("waw4.omegle.com"),
                Token::BorrowedStr("waw1.omegle.com"),
                Token::BorrowedStr("waw3.omegle.com"),
                Token::SeqEnd,
                Token::BorrowedStr("spyQueueTime"),
                Token::F64(105.28910002708434),
                Token::BorrowedStr("rtmfp"),
                Token::BorrowedStr("rtmfp://p2p.rtmfp.net"),
                Token::BorrowedStr("antinudepercent"),
                Token::F32(1.0),
                Token::BorrowedStr("spyeeQueueTime"),
                Token::F64(229.33259999752045),
                Token::BorrowedStr("timestamp"),
                Token::F64(1685331229.225212),
                Token::BorrowedStr("servers"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
            ],
            "Cannot produce a Vec1 with a length of zero.",
        )
    }

    #[test]
    fn invalid_response_text_should_error() {
        assert_de_tokens_error::<OmegleStatus>(
            &[Token::BorrowedStr("bad request")],
            "invalid type: string \"bad request\", expected struct OmegleStatus",
        )
    }
}
