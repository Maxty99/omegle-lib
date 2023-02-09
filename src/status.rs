use crate::error::OmegleLibError;
use crate::id::RandID;
use json::JsonValue;
use vec1::Vec1;

pub struct OmegleStatus {
    count: u64,
    servers: Vec1<String>,
    rand_id: RandID,
}

impl OmegleStatus {
    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn get_server(&self) -> &str {
        self.servers.first()
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
    ///     let server_status = OmegleStatus::get_server_status().await.unwrap();
    ///     println!("There are {} users currently active", server_status.get_count())
    /// }
    ///```
    ///
    /// # Errors:
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response contained no text
    /// - The response was unexpected (Ex: Error on omegle's end)
    pub async fn get_server_status() -> Result<OmegleStatus, OmegleLibError> {
        let req = reqwest::get("https://omegle.com/status")
            .await
            .map_err(|_| OmegleLibError::ConnectionError)?;
        let resp = req
            .text()
            .await
            .map_err(|_| OmegleLibError::CouldNotDetermineResponse)?;
        deserialize_response(resp)
    }
}

fn deserialize_response(resp: String) -> Result<OmegleStatus, OmegleLibError> {
    // Parse JSON
    let parsed = json::parse(&resp).map_err(|_| OmegleLibError::UnexpectedRespone(resp.clone()))?;
    let count_json = &parsed["count"];
    let servers_json = &parsed["servers"];

    // Eliminate all bad responses with very specific match
    match (count_json, servers_json) {
        (JsonValue::Number(number), JsonValue::Array(servers_vec))
            if number.is_sign_positive() //Need this for as_fixed_point
                && !servers_vec.is_empty() //Need this for vec1
                && servers_vec.iter().all(|elem| elem.is_string()) =>
        {
            let count = number.as_fixed_point_u64(0).expect("Number is positive");
            let servers: Vec1<String> = servers_vec
                .iter()
                .map(|elem| String::from(elem.as_str().expect("All elems are strings")))
                .collect::<Vec<String>>()
                .try_into()
                .map_err(|_| OmegleLibError::NoServers)?;
            let rand_id = RandID::new();
            Ok(OmegleStatus {
                count,
                servers,
                rand_id,
            })
        }
        _ => Err(OmegleLibError::UnexpectedRespone(resp)),
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
        "servers": [
        "front20",
        "front5"]}"#;
        let status = deserialize_response(resp_text.to_string());
        assert!(status.is_ok())
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
        let status = deserialize_response(resp_text.to_string());
        assert!(status.is_err())
    }

    #[test]
    fn invalid_response_text_should_error() {
        let resp_text = "bad request";
        let status = deserialize_response(resp_text.to_string());
        assert!(status.is_err())
    }
}
