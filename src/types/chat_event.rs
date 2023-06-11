use std::fmt;

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use vec1::Vec1;

use crate::status::OmegleStatus;

#[derive(Debug, PartialEq)]
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
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use vec1::vec1;

    use crate::status::OmegleStatus;
    use crate::types::chat_server::ChatServer;
    use crate::types::check_server::CheckServer;

    use super::*;

    #[test]
    fn can_deserialize_single_event() {
        // Response: [["typing"]]
        let expected_val = vec1![ChatEvent::StartedTyping];
        assert_de_tokens(
            &expected_val,
            &[
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(1) },
                Token::BorrowedStr("typing"),
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        )
    }

    #[test]
    fn can_deserialize_multi_event() {
        // Response:     "[["connected"], ["commonLikes",
        //                ["books"]], ["identDigests",
        //                "33eddfe1387518a2233a77cbbfce6a58,
        //                f816c928bf6598357d20977bfffe2052,
        //                9e533b0b6f3397b194c193e717703ed3
        //                ,d5526b7d44ed36f62d05a939132ce755"]]"
        let expected_val = vec1![
            ChatEvent::Connected,
            ChatEvent::CommonLikes(vec1![String::from("books")]),
            ChatEvent::IdentDigests(String::from("33eddfe1387518a2233a77cbbfce6a58,f816c928bf6598357d20977bfffe2052,9e533b0b6f3397b194c193e717703ed3,d5526b7d44ed36f62d05a939132ce755"))
        ];
        assert_de_tokens(
            &expected_val,
            &[
                Token::Seq { len: Some(3) },

                Token::Seq { len: Some(1) },
                Token::BorrowedStr("connected"),
                Token::SeqEnd,

                Token::Seq { len: Some(2) },
                Token::BorrowedStr("commonLikes"),
                Token::Seq { len: Some(1) },
                Token::BorrowedStr("books"),
                Token::SeqEnd,
                Token::SeqEnd,

                Token::Seq { len: Some(2) },
                Token::BorrowedStr("identDigests"),
                Token::BorrowedStr("33eddfe1387518a2233a77cbbfce6a58,f816c928bf6598357d20977bfffe2052,9e533b0b6f3397b194c193e717703ed3,d5526b7d44ed36f62d05a939132ce755"),
                Token::SeqEnd,

                Token::SeqEnd,
            ],
        )
    }

    #[test]
    fn can_deserialize_status_info_event() {
        // Response:        [["statusInfo", {"count": 45148, "antinudeservers":
        //                  ["waw3.omegle.com", "waw1.omegle.com", "waw4.omegle.com",
        //                  "waw2.omegle.com"], "spyQueueTime": 80.12030000686646,
        //                  "rtmfp": "rtmfp://p2p.rtmfp.net", "antinudepercent": 1.0,
        //                  "spyeeQueueTime": 229.33259999752045, "timestamp": 1685331229.225212,
        //                  "servers": ["front26", "front20", "front2", "front1",
        //                  "front45", "front39", "front28", "front15", "front46",
        //                  "front48", "front40", "front47", "front23", "front27"]}]]
        let expected_val = vec1![ChatEvent::StatusInfo(OmegleStatus {
            count: 45148,
            servers: vec1![
                ChatServer { id_number: 26 },
                ChatServer { id_number: 20 },
                ChatServer { id_number: 2 },
                ChatServer { id_number: 1 },
                ChatServer { id_number: 45 },
                ChatServer { id_number: 39 },
                ChatServer { id_number: 28 },
                ChatServer { id_number: 15 },
                ChatServer { id_number: 46 },
                ChatServer { id_number: 48 },
                ChatServer { id_number: 40 },
                ChatServer { id_number: 47 },
                ChatServer { id_number: 23 },
                ChatServer { id_number: 27 },
            ],
            antinudeservers: vec1![
                CheckServer(3),
                CheckServer(1),
                CheckServer(4),
                CheckServer(2)
            ]
        })];
        assert_de_tokens(
            &expected_val,
            &[
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(2) },
                Token::BorrowedStr("statusInfo"),
                Token::Map { len: Some(8) },
                Token::BorrowedStr("count"),
                Token::I64(45148),
                Token::BorrowedStr("antinudeservers"),
                Token::Seq { len: Some(4) },
                Token::BorrowedStr("waw3.omegle.com"),
                Token::BorrowedStr("waw1.omegle.com"),
                Token::BorrowedStr("waw4.omegle.com"),
                Token::BorrowedStr("waw2.omegle.com"),
                Token::SeqEnd,
                Token::BorrowedStr("spyQueueTime"),
                Token::F64(80.12030000686646),
                Token::BorrowedStr("rtmfp"),
                Token::BorrowedStr("rtmfp://p2p.rtmfp.net"),
                Token::BorrowedStr("antinudepercent"),
                Token::F32(1.0),
                Token::BorrowedStr("spyeeQueueTime"),
                Token::F64(229.33259999752045),
                Token::BorrowedStr("timestamp"),
                Token::F64(1685331229.225212),
                Token::BorrowedStr("servers"),
                Token::Seq { len: Some(14) },
                Token::BorrowedStr("front26"),
                Token::BorrowedStr("front20"),
                Token::BorrowedStr("front2"),
                Token::BorrowedStr("front1"),
                Token::BorrowedStr("front45"),
                Token::BorrowedStr("front39"),
                Token::BorrowedStr("front28"),
                Token::BorrowedStr("front15"),
                Token::BorrowedStr("front46"),
                Token::BorrowedStr("front48"),
                Token::BorrowedStr("front40"),
                Token::BorrowedStr("front47"),
                Token::BorrowedStr("front23"),
                Token::BorrowedStr("front27"),
                Token::SeqEnd,
                Token::MapEnd,
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        )
    }

    #[test]
    fn can_not_deserialize_with_one_invalid_event() {
        // Response:     "[["connected"], ["commonLikes"],
        //                [\"identDigests\",
        //                "33eddfe1387518a2233a77cbbfce6a58,
        //                f816c928bf6598357d20977bfffe2052,
        //                9e533b0b6f3397b194c193e717703ed3
        //                ,d5526b7d44ed36f62d05a939132ce755"]]";

        assert_de_tokens_error::<Vec1<ChatEvent>>(
            &[
                Token::Seq { len: Some(3) },
                Token::Seq { len: Some(1) },
                Token::BorrowedStr("connected"),
                Token::SeqEnd,
                Token::Seq { len: Some(2) },
                Token::BorrowedStr("commonLikes"),
                Token::SeqEnd,
                // The rest are irrelevant but still want them here for completness/reference
                // Token::Seq { len: Some(2) },
                // Token::BorrowedStr("identDigests"),
                // Token::BorrowedStr("33eddfe1387518a2233a77cbbfce6a58,f816c928bf6598357d20977bfffe2052,9e533b0b6f3397b194c193e717703ed3,d5526b7d44ed36f62d05a939132ce755"),
                // Token::SeqEnd,

                // Token::SeqEnd,
            ],
            "expected commonLikes to be followed by a non-empty list of strings",
        )
    }

    #[test]
    fn can_not_deserialize_empty_events() {
        // Response "[[]]";

        assert_de_tokens_error::<Vec1<ChatEvent>>(
            &[
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                // The rest are irrelevant but still want them here for completness/reference
                // Token::SeqEnd,
            ],
            "Empty event array",
        )
    }

    #[test]
    fn can_not_deserialize_unexpected_event() {
        // Response: "[["test"]]";

        assert_de_tokens_error::<Vec1<ChatEvent>>(
            &[
                Token::Seq { len: Some(1) },

                Token::Seq { len: Some(1) },
                Token::BorrowedStr("test"),
                Token::SeqEnd,

                // The rest are irrelevant but still want them here for completness/reference
                // Token::SeqEnd,
            ],
            "unknown variant `test`, expected one of `waiting`, `connected`, `statusInfo`, `count`, `commonLikes`, `serverMessage`, `identDigests`, `error`, `connectionDied`, `antinudeBanned`, `typing`, `stoppedTyping`, `gotMessage`, `strangerDisconnected`"
        )
    }
}
