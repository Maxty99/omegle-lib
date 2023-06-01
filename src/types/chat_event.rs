use std::fmt;

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use vec1::Vec1;

use crate::status::OmegleStatus;

pub enum ChatEvent {
    // Status Events
    Waiting,
    Connected,
    StatusInfo(OmegleStatus),
    Count(u64),

    // Notifications
    CommonLikes(Vec1<String>),
    ServerMessage(String),
    // Not sure what this is even used for but gotta include it so it doesn't error
    IdentDigests(String),

    // Errors
    Error(String),
    ConnectionDied,
    Banned,

    // Chat events
    StartedTyping,
    StoppedTyping,
    Message(String),
    Disconnected,
}

impl<'de> Deserialize<'de> for ChatEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ChatEventVisitor)
    }
}

struct ChatEventVisitor;

impl<'de> Visitor<'de> for ChatEventVisitor {
    type Value = ChatEvent;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A list with a chat event")
    }

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let chat_event_identifier = seq
            .next_element::<&str>()?
            .ok_or(Error::custom("Empty event array"))?;

        let chat_event = match chat_event_identifier {
            "waiting" => ChatEvent::Waiting,
            "connected" => ChatEvent::Connected,
            "statusInfo" => {
                let status = seq.next_element::<OmegleStatus>()?.ok_or(Error::custom(
                    "expected status info to be followed by status object",
                ))?;
                ChatEvent::StatusInfo(status)
            }
            "count" => {
                let count = seq
                    .next_element::<u64>()?
                    .ok_or(Error::custom("expected count to be followed by a number"))?;
                ChatEvent::Count(count)
            }
            "commonLikes" => {
                let likes = seq.next_element::<Vec1<String>>()?.ok_or(Error::custom(
                    "expected commonLikes to be followed by a non-empty list of strings",
                ))?;
                ChatEvent::CommonLikes(likes)
            }
            "serverMessage" => {
                let server_msg = seq.next_element::<String>()?.ok_or(Error::custom(
                    "expected serverMessage to be followed by a string",
                ))?;
                ChatEvent::ServerMessage(server_msg)
            }
            "identDigests" => {
                let digests = seq.next_element::<String>()?.ok_or(Error::custom(
                    "expected identDigests to be followed by a string",
                ))?;
                ChatEvent::IdentDigests(digests)
            }
            "error" => {
                let error_msg = seq
                    .next_element::<String>()?
                    .ok_or(Error::custom("expected error to be followed by a string"))?;
                ChatEvent::Error(error_msg)
            }
            "connectionDied" => ChatEvent::ConnectionDied,
            "antinudeBanned" => ChatEvent::Banned,
            "typing" => ChatEvent::StartedTyping,
            "stoppedTyping" => ChatEvent::StoppedTyping,
            "gotMessage" => {
                let msg = seq.next_element::<String>()?.ok_or(Error::custom(
                    "expected gotMessage to be followed by a string",
                ))?;
                ChatEvent::Message(msg)
            }
            "strangerDisconnected" => ChatEvent::Disconnected,
            _ => Err(Error::unknown_variant(
                chat_event_identifier,
                &[
                    "waiting",
                    "connected",
                    "statusInfo",
                    "count",
                    "commonLikes",
                    "serverMessage",
                    "identDigests",
                    "error",
                    "connectionDied",
                    "antinudeBanned",
                    "typing",
                    "stoppedTyping",
                    "gotMessage",
                    "strangerDisconnected",
                ],
            ))?,
        };

        Ok(chat_event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_single_event() {
        let event_text = "[[\"typing\"]]";
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(if let Ok(val) = events {
            matches!(val.first(), ChatEvent::StartedTyping)
        } else {
            false
        })
    }

    #[test]
    fn can_deserialize_multi_event() {
        let event_text = "[[\"connected\"], [\"commonLikes\", \
                          [\"books\"]], [\"identDigests\", \
                          \"33eddfe1387518a2233a77cbbfce6a58,\
                          f816c928bf6598357d20977bfffe2052,\
                          9e533b0b6f3397b194c193e717703ed3\
                          ,d5526b7d44ed36f62d05a939132ce755\"]]";
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(if let Ok(val) = events {
            let mut event_iter = val.iter();
            matches!(event_iter.next(), Some(ChatEvent::Connected))
                && matches!(event_iter.next(), Some(ChatEvent::CommonLikes(_)))
                && matches!(event_iter.next(), Some(ChatEvent::IdentDigests(_)))
                && matches!(event_iter.next(), None)
        } else {
            false
        })
    }

    #[test]
    fn can_deserialize_status_info_event() {
        let event_text = r#"[["statusInfo", {"count": 45148, "antinudeservers": 
                            ["waw3.omegle.com", "waw1.omegle.com", "waw4.omegle.com", 
                            "waw2.omegle.com"], "spyQueueTime": 80.12030000686646, 
                            "rtmfp": "rtmfp://p2p.rtmfp.net", "antinudepercent": 1.0, 
                            "spyeeQueueTime": 229.33259999752045, "timestamp": 1685331229.225212, 
                            "servers": ["front26", "front20", "front2", "front1", 
                            "front45", "front39", "front28", "front15", "front46", 
                            "front48", "front40", "front47", "front23", "front27"]}]]"#;
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(if let Ok(val) = events {
            matches!(val.first(), ChatEvent::StatusInfo(_))
        } else {
            false
        })
    }

    #[test]
    fn can_not_deserialize_with_one_invalid_event() {
        let event_text = "[[\"connected\"], [\"commonLikes\", \
                          []], [\"identDigests\", \
                          \"33eddfe1387518a2233a77cbbfce6a58,\
                          f816c928bf6598357d20977bfffe2052,\
                          9e533b0b6f3397b194c193e717703ed3\
                          ,d5526b7d44ed36f62d05a939132ce755\"]]";
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(events.is_err())
    }

    #[test]
    fn can_not_deserialize_empty_events() {
        let event_text = "[[]]";
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(events.is_err())
    }

    #[test]
    fn can_not_deserialize_unexpected_event() {
        let event_text = "[[\"test\"]]";
        let events = serde_json::from_str::<Vec1<ChatEvent>>(&event_text);
        assert!(events.is_err())
    }
}
