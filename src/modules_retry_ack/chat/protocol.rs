use std::net::SocketAddr;

use chrono::prelude::*;
use hydroflow::DemuxEnum;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, DemuxEnum, Hash)]
pub enum Message {
    ConnectRequest,
    ConnectResponse,
    ChatMsg {
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
    RetryAck {
        // message id
        msg_id: usize,
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
    ChatMsg {
        addr: SocketAddr,
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
    RetryAck {
        addr: SocketAddr,
        /// message id
        msg_id: usize,
    },
}
impl MessageWithAddr {
    pub fn from_message(message: Message, addr: SocketAddr) -> Self {
        match message {
            Message::ConnectRequest => Self::ConnectRequest { addr },
            Message::ConnectResponse => Self::ConnectResponse { addr },
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
            Message::RetryAck { msg_id } => Self::RetryAck { addr, msg_id },
        }
    }
}

/// A sequence-numbered message
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct RetryMsg {
    /// message id
    pub msg_id: usize,
    /// payload
    pub payload: Message,
}
