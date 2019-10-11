use crate::{NumBytes, Read, ReadError, Write, WriteError};
use crate::{
    Block, ChainSizeMessage, GoAwayMessage, HandshakeMessage,
    NoticeMessage, RequestMessage, SyncRequestMessage, TimeMessage,
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

impl NumBytes for Message {
    fn num_bytes(&self) -> usize {
        match self {
            Message::HandshakeMessage(msg) => msg.num_bytes(),
            Message::ChainSizeMessage(msg) => msg.num_bytes(),
            Message::GoAwayMessage(msg) => msg.num_bytes(),
            Message::TimeMessage(msg) => msg.num_bytes(),
            Message::NoticeMessage(msg) => msg.num_bytes(),
            Message::RequestMessage(msg) => msg.num_bytes(),
            Message::SyncRequestMessage(msg) => msg.num_bytes(),
            Message::SignedBlock(msg) => msg.num_bytes(),
        }
    }
}

impl Write for Message {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        match self {
            Message::HandshakeMessage(msg) => msg.write(bytes, pos),
            Message::ChainSizeMessage(msg) => msg.write(bytes, pos),
            Message::GoAwayMessage(msg) => msg.write(bytes, pos),
            Message::TimeMessage(msg) => msg.write(bytes, pos),
            Message::NoticeMessage(msg) => msg.write(bytes, pos),
            Message::RequestMessage(msg) => msg.write(bytes, pos),
            Message::SyncRequestMessage(msg) => msg.write(bytes, pos),
            Message::SignedBlock(msg) => msg.write(bytes, pos),
        }
    }
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
            Message::SignedBlock(msg) => format!("{}", msg),
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

impl MessageHeader {
    fn new(msg_size: usize, msg_type: MessageType) -> Self {
        Self {
            size: (msg_size + msg_type.num_bytes()) as u32,
            msg_type,
        }
    }
}

impl core::fmt::Display for MessageHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "size: {}, msg_type: {}", self.size, self.msg_type)
    }
}

#[derive(Clone, Debug, NumBytes, Write)]
#[eosio_core_root_path = "crate"]
pub struct RawMessage {
    pub header: MessageHeader,
    pub msg: Message,
}

impl RawMessage {
    pub fn new(msg: Message) -> Self {
        Self {
            msg: msg.clone(),
            header: MessageHeader::new(msg.num_bytes(), msg.into()),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; self.num_bytes()];
        self.write(buffer.as_mut_slice(), &mut 0).unwrap();
        buffer
    }
}

impl Read for RawMessage {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let header = MessageHeader::read(bytes, pos)?;

        let msg = match header.msg_type {
            MessageType::HandshakeMessage => {
                let msg = HandshakeMessage::read(bytes, pos)?;
                Message::HandshakeMessage(msg)
            }
            MessageType::ChainSizeMessage => {
                let msg = ChainSizeMessage::read(bytes, pos)?;
                Message::ChainSizeMessage(msg)
            }
            MessageType::GoAwayMessage => {
                let msg = GoAwayMessage::read(bytes, pos)?;
                Message::GoAwayMessage(msg)
            }
            MessageType::TimeMessage => {
                let msg = TimeMessage::read(bytes, pos)?;
                Message::TimeMessage(msg)
            }
            MessageType::NoticeMessage => {
                let msg = NoticeMessage::read(bytes, pos)?;
                Message::NoticeMessage(msg)
            }
            MessageType::RequestMessage => {
                let msg = RequestMessage::read(bytes, pos)?;
                Message::RequestMessage(msg)
            }
            MessageType::SyncRequestMessage => {
                let msg = SyncRequestMessage::read(bytes, pos)?;
                Message::SyncRequestMessage(msg)
            }
            MessageType::SignedBlock => {
                let msg = Block::read(bytes, pos)?;
                Message::SignedBlock(msg)
            }
            _ => return Err(ReadError::NotSupportMessageType),
        };

        Ok(Self {
            header,
            msg,
        })
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
            MessageType::HandshakeMessage => 0,
            MessageType::ChainSizeMessage => 1,
            MessageType::GoAwayMessage => 2,
            MessageType::TimeMessage => 3,
            MessageType::NoticeMessage => 4,
            MessageType::RequestMessage => 5,
            MessageType::SyncRequestMessage => 6,
            MessageType::SignedBlock => 7,
            MessageType::PackedTransaction => 8,
            MessageType::None => 255,
        }
    }
}

impl From<Message> for MessageType {
    fn from(msg: Message) -> Self {
        match msg {
            Message::HandshakeMessage(_) => MessageType::HandshakeMessage,
            Message::ChainSizeMessage(_) => MessageType::ChainSizeMessage,
            Message::GoAwayMessage(_) => MessageType::GoAwayMessage,
            Message::TimeMessage(_) => MessageType::TimeMessage,
            Message::NoticeMessage(_) => MessageType::NoticeMessage,
            Message::RequestMessage(_) => MessageType::RequestMessage,
            Message::SyncRequestMessage(_) => MessageType::SyncRequestMessage,
            Message::SignedBlock(_) => MessageType::SignedBlock,
        }
    }
}

impl From<HandshakeMessage> for MessageType {
    fn from(_msg: HandshakeMessage) -> Self {
        MessageType::HandshakeMessage
    }
}

impl From<ChainSizeMessage> for MessageType {
    fn from(_msg: ChainSizeMessage) -> Self {
        MessageType::ChainSizeMessage
    }
}

impl From<GoAwayMessage> for MessageType {
    fn from(_msg: GoAwayMessage) -> Self {
        MessageType::GoAwayMessage
    }
}

impl From<TimeMessage> for MessageType {
    fn from(_msg: TimeMessage) -> Self {
        MessageType::TimeMessage
    }
}

impl From<NoticeMessage> for MessageType {
    fn from(_msg: NoticeMessage) -> Self {
        MessageType::NoticeMessage
    }
}

impl From<RequestMessage> for MessageType {
    fn from(_msg: RequestMessage) -> Self {
        MessageType::RequestMessage
    }
}

impl From<SyncRequestMessage> for MessageType {
    fn from(_msg: SyncRequestMessage) -> Self {
        MessageType::SyncRequestMessage
    }
}

impl From<Block> for MessageType {
    fn from(_msg: Block) -> Self {
        MessageType::SignedBlock
    }
}

//impl From<PackedTransaction> for MessageType {
//    fn from(_msg: PackedTransaction) -> Self {
//        MessageType::PackedTransaction
//    }
//}

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
