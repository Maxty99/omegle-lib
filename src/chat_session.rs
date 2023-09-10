use std::collections::HashMap;

use crate::types::{chat_event::ChatEvent, error::OmegleLibError};
use crate::types::{chat_server::ChatServer, client_id::ClientID};

use reqwest::Client;
use vec1::Vec1;

static OMEGLE_SUCCESS_RESP: &str = "win";

/// Struct representing a single ongoing chat session
#[derive(Debug, Clone)]
pub struct ChatSession {
    pub(crate) client_id: ClientID,
    pub(crate) client: Client,
    pub(crate) server: ChatServer,
}

impl ChatSession {
    /// Sends a message to the other party.
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response from omegle indicated an error
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

        if resp == OMEGLE_SUCCESS_RESP {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError(resp))
        }
    }

    /// Sends a typing indicator to the server
    ///
    /// # Notes:
    /// This is technically not required to do before you send a message,
    /// however the server might flag you as a bot if you don't. This won't
    /// ban you but may make talking to other humans much more rare.
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response from omegle indicated an error
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

        if resp == OMEGLE_SUCCESS_RESP {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError(resp))
        }
    }

    /// Sends a stopped typing indicator to the server
    ///
    /// # Notes:
    /// It is not necessary to send a stopped typing indicator after you
    /// sent a message as the state is reset on the server after you do.
    /// This is intended for when you suddenly stop typing without
    /// sending the message you were typing.
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response from omegle indicated an error
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

        if resp == OMEGLE_SUCCESS_RESP {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError(resp))
        }
    }

    /// Gets a list of [`ChatEvent`] from the server.
    ///
    /// Omegle uses long-polling to get updates about the state of a chat.
    /// So this function will block untill the server responds with new events.
    /// You should run this on a seperate executor and relaunch it as soon as
    /// the previous call returns
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response from omegle was malformed
    /// - The function was called after the chat ended
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

    /// Sends a disconnect request to the server
    ///
    /// # Errors
    /// This function fails if:
    /// - The omegle server cannot be reached
    /// - The response from omegle indicated an error
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

        if resp == OMEGLE_SUCCESS_RESP {
            Ok(())
        } else {
            Err(OmegleLibError::OmegleError(resp))
        }
    }
}
