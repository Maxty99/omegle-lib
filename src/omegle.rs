use itertools::Itertools;
use std::collections::HashSet;

use crate::{
    chat_session::ChatSession,
    status::OmegleStatus,
    types::{client_id::ClientID, lang::LangCode, rand_id::RandID},
};

use reqwest::Client;

/// Struct representing an Omegle Client, a factory for creating [`ChatSession`]
pub struct Omegle {
    rand_id: RandID,
    client: Client,
    status: OmegleStatus,
    topics: HashSet<String>,
    lang: LangCode,
}

impl Omegle {
    /// Updates the set language
    pub fn update_lang(&mut self, new_lang: LangCode) {
        self.lang = new_lang
    }

    /// Gets the currently selected langauge
    pub fn get_current_lang(&self) -> LangCode {
        return self.lang;
    }

    /// Add a new interest
    ///
    /// If the interest was already added returns false, otherwise returns true
    pub fn add_interest<M: ToString>(&mut self, new_interest: M) -> bool {
        let new_interest_string = new_interest.to_string();
        self.topics.insert(new_interest_string)
    }

    /// Tries to remove an interest
    ///
    /// If the interest was present returns true, otherwise returns false
    pub fn remove_interest<M: ToString>(&mut self, interest_to_remove: M) -> bool {
        let interest_to_remove_string = interest_to_remove.to_string();
        self.topics.remove(&interest_to_remove_string)
    }

    /// Gets a vec of refrences to the current interests
    pub fn get_current_interests(&self) -> Vec<&String> {
        self.topics.iter().collect_vec()
    }

    /// Gets an interator over currently selected topics
    pub fn get_current_interests_iter(&self) -> std::collections::hash_set::Iter<'_, String> {
        self.topics.iter()
    }

    /// Creates a new instance of [`Omegle`] given some topics and the desired [`LangCode`]
    ///
    /// # Examples
    /// Create a new [`Omegle`] instance with no interests and request to talk in English
    /// ```rust
    /// use omegle_rs::omegle::Omegle;
    /// use omegle_rs::status::OmegleStatus;
    /// use omegle_rs::types::lang::LangCode;
    /// use std::collections::HashSet;
    ///
    /// async fn run() -> Omegle {
    ///     let server_status = OmegleStatus::get_omegle_status().await.unwrap();
    ///     Omegle::new(server_status, HashSet::new(), LangCode::English)
    /// }
    /// ```
    /// ---
    /// Create a new [`Omegle`] instance specifying knitting as an intrest and request to talk in French
    /// ```rust
    /// use omegle_rs::omegle::Omegle;
    /// use omegle_rs::status::OmegleStatus;
    /// use omegle_rs::types::lang::LangCode;
    /// use std::collections::HashSet;
    ///
    /// async fn run() -> Omegle {
    ///     let mut interests = HashSet::new();
    ///     interests.insert("knitting".to_string());
    ///     let server_status = OmegleStatus::get_omegle_status().await.unwrap();
    ///     Omegle::new(server_status, interests, LangCode::French)
    /// }
    /// ```
    pub fn new(status: OmegleStatus, topics: HashSet<String>, lang: LangCode) -> Self {
        Self {
            rand_id: RandID::new(),
            client: Client::new(),
            status,
            topics,
            lang,
        }
    }

    /// Sends a request to start a new chat. If successful returns a new [`ChatSession`]
    /// # Examples
    /// Start a new chat
    /// ```rust
    /// use omegle_rs::omegle::Omegle;
    /// use omegle_rs::status::OmegleStatus;
    /// use omegle_rs::types::lang::LangCode;
    /// use omegle_rs::chat_session::ChatSession;
    /// use std::collections::HashSet;
    ///
    /// async fn run() -> ChatSession {
    ///     let server_status = OmegleStatus::get_omegle_status().await.unwrap();
    ///     let omegle = Omegle::new(server_status, HashSet::new(), LangCode::English);
    ///     omegle.new_chat().await.unwrap()
    /// }
    /// ```
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response was unexpected (Ex: Error on omegle's end or response was malformed)
    pub async fn new_chat(&self) -> Result<ChatSession, reqwest::Error> {
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
            let topics_as_string = self.topics.iter().join(",");
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
            client: self.client.clone(),
            server: chat_server,
        })
    }
}
