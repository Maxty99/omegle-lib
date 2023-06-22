use vec1::Vec1;

use crate::{status::OmegleStatus, types::chat_event::ChatEvent};

pub trait EventHandler {
    fn handle_waiting(&self);
    fn handle_connected(&self);
    fn handle_status_info_update(&self, info: OmegleStatus);
    fn handle_count_update(&self, count: u64);
    fn handle_common_likes(&self, likes: Vec1<String>);
    fn handle_server_message(&self, msg: String);
    fn handle_ident_digests(&self, digests: String);
    fn handle_error(&self, err: String);
    fn handle_connection_died(&self);
    fn handle_banned(&self);
    fn handle_started_typing(&self);
    fn handle_stopped_typing(&self);
    fn handle_message(&self, msg: String);
    fn handle_disconnected(&self);

    fn handle_event(&self, evt: ChatEvent) {
        match evt {
            ChatEvent::Waiting => self.handle_waiting(),
            ChatEvent::Connected => self.handle_connected(),
            ChatEvent::StatusInfo(info) => self.handle_status_info_update(info),
            ChatEvent::Count(count) => self.handle_count_update(count),
            ChatEvent::CommonLikes(likes) => self.handle_common_likes(likes),
            ChatEvent::ServerMessage(msg) => self.handle_server_message(msg),
            ChatEvent::IdentDigests(digests) => self.handle_ident_digests(digests),
            ChatEvent::Error(err) => self.handle_error(err),
            ChatEvent::ConnectionDied => self.handle_connection_died(),
            ChatEvent::Banned => self.handle_banned(),
            ChatEvent::StartedTyping => self.handle_started_typing(),
            ChatEvent::StoppedTyping => self.handle_stopped_typing(),
            ChatEvent::Message(msg) => self.handle_message(msg),
            ChatEvent::Disconnected => self.handle_disconnected(),
        }
    }
}
