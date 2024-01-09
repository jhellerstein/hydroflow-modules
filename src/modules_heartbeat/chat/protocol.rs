use std::net::SocketAddr;

use chrono::prelude::*;
use hydroflow::DemuxEnum;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, DemuxEnum)]
pub enum Message {
    ConnectRequest,
    ConnectResponse,
    HeartbeatMsg,
    HeartbeatResponse,
    HeartbeatDisconnect,
    ChatMsg {
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
}

#[derive(Clone, Debug, DemuxEnum)]
pub enum MessageWithAddr {
    ConnectRequest {
        addr: SocketAddr,
    },
    ConnectResponse {
        addr: SocketAddr,
    },
    HeartbeatMsg {
        addr: SocketAddr,
    },
    HeartbeatResponse {
        addr: SocketAddr,
    },
    HeartbeatDisconnect {
        addr: SocketAddr,
    },
    ChatMsg {
        addr: SocketAddr,
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
}
impl MessageWithAddr {
    pub fn from_message(message: Message, addr: SocketAddr) -> Self {
        match message {
            Message::ConnectRequest => Self::ConnectRequest { addr },
            Message::ConnectResponse => Self::ConnectResponse { addr },
            Message::HeartbeatMsg => Self::HeartbeatMsg { addr },
            Message::HeartbeatResponse => Self::HeartbeatResponse { addr },
            Message::HeartbeatDisconnect => Self::HeartbeatDisconnect { addr },
            Message::ChatMsg {
                nickname,
                message,
                ts,
            } => Self::ChatMsg {
                addr,
                nickname,
                message,
                ts,
            },
        }
    }
}
