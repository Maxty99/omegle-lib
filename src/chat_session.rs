use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::error::OmegleLibError;
use crate::status::OmegleStatus;
use futures::{future, Future, Stream, StreamExt};
use reqwest::Client;
use vec1::Vec1;

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
    DIsconnected,
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

impl Stream for ChatSession<'_> {
    type Item = Result<Vec1<Event>, OmegleLibError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.is_connected {
            Poll::Ready(None)
        } else {
            let future = self.get_events();
            let mut task = Box::pin(future);
            match task.as_mut().poll(cx) {
                Poll::Ready(value) => todo!(),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}
