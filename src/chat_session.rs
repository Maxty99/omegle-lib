use std::{
    pin::{pin, Pin},
    task::{Context, Poll},
};

use crate::error::OmegleLibError;
use crate::status::OmegleStatus;
use futures::{Future, Stream};
use reqwest::Client;
use vec1::Vec1;

// TODO: Figure out how to reuse this effectively
pub struct ChatSession<'a> {
    is_connected: bool,
    client: Client,
    status: &'a OmegleStatus,
}

pub enum Event {
    // Status Events
    Waiting,
    Connected,
    StatusInfo(String),

    // Notifications
    CommonLikes(Vec<String>),

    // Errors
    Error(String),
    ConnectionDied,

    // Chat events
    StartedTyping,
    StoppedTyping,
    Message,
    Disconnected,
}

impl<'a> ChatSession<'a> {
    fn new(status: &'a OmegleStatus) -> Self {
        Self {
            is_connected: false,
            client: Client::new(),
            status,
        }
    }
}

impl ChatSession<'_> {
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub async fn new_chat(&self) {
        todo!()
    }
    pub async fn send_message(&self) -> Result<(), OmegleLibError> {
        todo!()
    }
    pub async fn start_typing(&self) -> Result<(), OmegleLibError> {
        todo!()
    }
    pub async fn stop_typing(&self) -> Result<(), OmegleLibError> {
        todo!()
    }
    async fn get_events(&self) -> Result<Vec1<Event>, OmegleLibError> {
        todo!()
    }
    pub async fn disconnect(&self) {
        todo!()
    }
}

// TODO: Not sure if this should handle error or be able to go on past None,
// No semantics for Stream documented like for iter, allowing the iteration
// to continue after passing None
impl Stream for ChatSession<'_> {
    type Item = Result<Vec1<Event>, OmegleLibError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.is_connected {
            Poll::Ready(None)
        } else {
            let future = self.get_events();
            let mut task = pin!(future);
            match task.as_mut().poll(cx) {
                Poll::Ready(value) => todo!(),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}
