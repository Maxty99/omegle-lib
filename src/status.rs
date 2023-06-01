use crate::types::error::OmegleLibError;
use crate::types::rand_id::RandID;
use crate::types::server::Server;
use serde::Deserialize;
use vec1::Vec1;

#[derive(Deserialize, Debug)]
pub struct OmegleStatus {
    count: u64,
    servers: Vec1<Server>,
    #[serde(skip)]
    #[serde(default = "new_randid")]
    rand_id: RandID,
}
fn new_randid() -> RandID {
    RandID::new()
}

impl OmegleStatus {
    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn get_server(&self) -> String {
        self.servers.first().into()
    }

    fn get_rand_id(&self) -> &RandID {
        &self.rand_id
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

    #[test]
    fn valid_response_text_should_give_status() {
        let resp_text = r#"{
        "count": 34658,
        "antinudeservers": [
        "waw2.omegle.com",
        "waw4.omegle.com",
        "waw1.omegle.com",
        "waw3.omegle.com"
        ],
        "spyQueueTime": 105.28910002708434,
        "rtmfp": "rtmfp://p2p.rtmfp.net",
        "antinudepercent": 1,
        "spyeeQueueTime": 255.67500002384185,
        "timestamp": 1675392220.026273,
        "servers": ["front20", "front5"]}"#;
        let resp = serde_json::from_str::<OmegleStatus>(&resp_text);

        assert!(resp.is_ok())
    }

    #[test]
    fn no_servers_given_should_error() {
        let resp_text = r#"{
        "count": 34658,
        "antinudeservers": [
        "waw2.omegle.com",
        "waw4.omegle.com",
        "waw1.omegle.com",
        "waw3.omegle.com"
        ],
        "spyQueueTime": 105.28910002708434,
        "rtmfp": "rtmfp://p2p.rtmfp.net",
        "antinudepercent": 1,
        "spyeeQueueTime": 255.67500002384185,
        "timestamp": 1675392220.026273,
        "servers": []}"#;
        let resp = serde_json::from_str::<OmegleStatus>(&resp_text);
        assert!(resp.is_err())
    }

    #[test]
    fn invalid_response_text_should_error() {
        let resp_text = "bad request";
        let resp = serde_json::from_str::<OmegleStatus>(&resp_text);
        assert!(resp.is_err())
    }
}
