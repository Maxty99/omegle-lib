use crate::{
    chat_session::ChatSession,
    status::OmegleStatus,
    types::{client_id::ClientID, lang::LangCode, rand_id::RandID},
};

use reqwest::Client;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub struct Omegle<'om> {
    rand_id: RandID,
    client: &'om Client,
    status: OmegleStatus,
    topics: Vec<String>,
    lang: LangCode,
}

impl Omegle<'_> {
    pub fn new(status: OmegleStatus, topics: Vec<String>, lang: LangCode) -> Self {
        Self {
            rand_id: RandID::new(),
            client: CLIENT.get_or_init(|| Client::new()),
            status,
            topics,
            lang,
        }
    }
}

impl<'om> Omegle<'om> {
    pub async fn new_chat(&self) -> Result<ChatSession<'om>, reqwest::Error> {
        let chat_server = self.status.get_chat_server();
        let chat_server_string = String::from(chat_server);
        let rand_id = String::from(self.rand_id);
        let check_server = self.status.get_check_server();
        let check_server_string = String::from(check_server);

        let check_code = self
            .client
            .post(format!("http://{check_server_string}/check"))
            .send()
            .await?
            .text()
            .await?;
        let lang_code = self.lang.to_string();

        let resp = if self.topics.is_empty() {
            self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/start?caps=recaptcha2,t3&spid=&randid={rand_id}&cc={check_code}&lang={lang_code}"))
            .send()
            .await?
            .json::<ClientID>()
            .await?
        } else {
            let topics_as_string = self.topics.join(",");
            self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/start?caps=recaptcha2,t3&spid=&randid={rand_id}&cc={check_code}&topics={topics_as_string}&lang={lang_code}"))
            .send()
            .await?
            .json::<ClientID>()
            .await?
        };

        Ok(ChatSession {
            client_id: resp,
            client: self.client,
            server: chat_server,
        })
    }
}
