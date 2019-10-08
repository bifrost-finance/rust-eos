use crate::{NumBytes, Read, Write, ReadError, WriteError};
use crate::{
    HandshakeMessage, ChainSizeMessage, GoAwayMessage, TimeMessage,
    NoticeMessage, RequestMessage, SyncRequestMessage, Block
};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    HandshakeMessage(HandshakeMessage),
    ChainSizeMessage(ChainSizeMessage),
    GoAwayMessage(GoAwayMessage),
    TimeMessage(TimeMessage),
    NoticeMessage(NoticeMessage),
    RequestMessage(RequestMessage),
    SyncRequestMessage(SyncRequestMessage),
    SignedBlock(Block),
//    PackedTransaction(PackedTransaction),
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let msg_str = match self {
            Message::HandshakeMessage(msg) => format!("{}", msg),
            Message::ChainSizeMessage(msg) => format!("{}", msg),
            Message::GoAwayMessage(msg) => format!("{}", msg),
            Message::TimeMessage(msg) => format!("{}", msg),
            Message::NoticeMessage(msg) => format!("{}", msg),
            Message::RequestMessage(msg) => format!("{}", msg),
            Message::SyncRequestMessage(msg) => format!("{}", msg),
            Message::SignedBlock(msg) =>  format!("SignedBlock"),
        };
        write!(f, "{}", msg_str)
    }
}

#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct MessageHeader {
    pub size: u32,
    pub msg_type: MessageType,
}

impl core::fmt::Display for MessageHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "size: {}, msg_type: {}", self.size, self.msg_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageType {
    HandshakeMessage,
    ChainSizeMessage,
    GoAwayMessage,
    TimeMessage,
    NoticeMessage,
    RequestMessage,
    SyncRequestMessage,
    SignedBlock,
    PackedTransaction,
    None,
}

impl core::fmt::Display for MessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let msg_str = match self {
            MessageType::HandshakeMessage => "handshake message",
            MessageType::ChainSizeMessage => "chain size message",
            MessageType::GoAwayMessage => "go away message",
            MessageType::TimeMessage => "time message",
            MessageType::NoticeMessage => "notice message",
            MessageType::RequestMessage => "request message",
            MessageType::SyncRequestMessage => "sync request message",
            MessageType::SignedBlock => "signed block",
            MessageType::PackedTransaction => "packed transaction",
            MessageType::None => "none",
        };
        write!(f, "{}", msg_str)
    }
}

impl Default for MessageType {
    fn default() -> Self {
        Self::None
    }
}

impl NumBytes for MessageType {
    fn num_bytes(&self) -> usize {
        1
    }
}

impl From<u8> for MessageType {
    fn from(mode: u8) -> Self {
        match mode {
            0 => MessageType::HandshakeMessage,
            1 => MessageType::ChainSizeMessage,
            2 => MessageType::GoAwayMessage,
            3 => MessageType::TimeMessage,
            4 => MessageType::NoticeMessage,
            5 => MessageType::RequestMessage,
            6 => MessageType::SyncRequestMessage,
            7 => MessageType::SignedBlock,
            8 => MessageType::PackedTransaction,
            _ => MessageType::None,
        }
    }
}

impl From<MessageType> for u8 {
    fn from(mode: MessageType) -> Self {
        match mode {
            MessageType::HandshakeMessage   => 0,
            MessageType::ChainSizeMessage   => 1,
            MessageType::GoAwayMessage      => 2,
            MessageType::TimeMessage        => 3,
            MessageType::NoticeMessage      => 4,
            MessageType::RequestMessage     => 5,
            MessageType::SyncRequestMessage => 6,
            MessageType::SignedBlock        => 7,
            MessageType::PackedTransaction  => 8,
            MessageType::None              => 255,
        }
    }
}

impl Read for MessageType {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|res| MessageType::from(res))
    }
}

impl Write for MessageType {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        u8::from(self.clone()).write(bytes, pos)
    }
}
