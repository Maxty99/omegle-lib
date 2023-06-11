use crate::{
    chat_session::ChatSession,
    status::OmegleStatus,
    types::{check_server, error::OmegleLibError, rand_id::RandID},
};
use futures::TryFutureExt;
use reqwest::Client;
use std::{marker::PhantomData, sync::OnceLock};

static CLIENT: OnceLock<Client> = OnceLock::new();

struct Omegle<'om> {
    rand_id: RandID,
    client: &'om Client,
    status: OmegleStatus,
}

impl<'om> Omegle<'om> {
    fn new(status: OmegleStatus) -> Self {
        Self {
            rand_id: RandID::new(),
            client: CLIENT.get_or_init(|| Client::new()),
            status,
        }
    }

    async fn new_chat(&self) -> Result<ChatSession<'om>, OmegleLibError> {
        let server = self.status.get_chat_server();
        let rand_id = String::from(self.rand_id);
        let check_server = self.status.get_check_server();
        let check_code = self
            .client
            .post(format!("{check_server}/check"))
            .send()
            .await
            .map_err(|_| OmegleLibError::ConnectionError)?
            .text()
            .await
            .map_err(|_| OmegleLibError::ConnectionError)?;
        let resp = self
            .client
            .post(format!("http://{server}.omegle.com/start?"))
            .send()
            .await
            .map_err(|_| OmegleLibError::ConnectionError)?;

        todo!()
    }
}
