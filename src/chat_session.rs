use std::{
    collections::HashMap,
    pin::{pin, Pin},
    task::{Context, Poll},
};

use crate::types::{chat_event::ChatEvent, error::OmegleLibError};
use crate::types::{chat_server::ChatServer, client_id::ClientID};
use futures::{Future, Stream};
use reqwest::Client;
use vec1::Vec1;

// TODO: Figure out how to reuse this effectively
#[derive(Debug, Clone, Copy)]
pub struct ChatSession<'a> {
    pub(crate) client_id: ClientID,
    pub(crate) client: &'a Client,
    pub(crate) server: ChatServer,
}

impl ChatSession<'_> {
    pub async fn send_message<M: ToString>(&self, message: M) -> Result<(), OmegleLibError> {
        let chat_server_string = String::from(self.server);
        let client_id_string = String::from(self.client_id);
        let message_string = message.to_string();

        let mut form = HashMap::new();
        form.insert("id", client_id_string);
        form.insert("msg", message_string);
        let resp = self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/send"))
            .form(&form)
            .send()
            .await?
            .text()
            .await?;

        if resp == "win" {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError)
        }
    }
    pub async fn start_typing(&self) -> Result<(), OmegleLibError> {
        let chat_server_string = String::from(self.server);
        let client_id_string = String::from(self.client_id);

        let mut form = HashMap::new();
        form.insert("id", client_id_string);
        let resp = self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/typing"))
            .form(&form)
            .send()
            .await?
            .text()
            .await?;

        if resp == "win" {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError)
        }
    }
    pub async fn stop_typing(&self) -> Result<(), OmegleLibError> {
        let chat_server_string = String::from(self.server);
        let client_id_string = String::from(self.client_id);

        let mut form = HashMap::new();
        form.insert("id", client_id_string);
        let resp = self
            .client
            .post(format!(
                "http://{chat_server_string}.omegle.com/stoppedtyping"
            ))
            .form(&form)
            .send()
            .await?
            .text()
            .await?;

        if resp == "win" {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError)
        }
    }
    pub async fn get_events(&self) -> Result<Vec1<ChatEvent>, OmegleLibError> {
        let chat_server_string = String::from(self.server);
        let client_id_string = String::from(self.client_id);

        let mut form = HashMap::new();
        form.insert("id", client_id_string);
        let resp = self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/events"))
            .form(&form)
            .send()
            .await?
            .json::<Vec1<ChatEvent>>()
            .await?;

        Ok(resp)
    }
    pub async fn disconnect(&self) -> Result<(), OmegleLibError> {
        let chat_server_string = String::from(self.server);
        let client_id_string = String::from(self.client_id);

        let mut form = HashMap::new();
        form.insert("id", client_id_string);
        let resp = self
            .client
            .post(format!("http://{chat_server_string}.omegle.com/disconnect"))
            .form(&form)
            .send()
            .await?
            .text()
            .await?;

        if resp == "win" {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError)
        }
    }
}
