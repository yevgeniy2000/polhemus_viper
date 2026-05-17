use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CommandActionType {
    Set,
    Get,
    Reset,
    Ack,
    Nak,
}

impl CommandActionType {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            CommandActionType::Set => [0x00, 0x00, 0x00, 0x00],
            CommandActionType::Get => [0x01, 0x00, 0x00, 0x00],
            CommandActionType::Reset => [0x02, 0x00, 0x00, 0x00],
            CommandActionType::Ack => [0x03, 0x00, 0x00, 0x00],
            CommandActionType::Nak => [0x04, 0x00, 0x00, 0x00],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> { if bytes.len() < 4 { return None; } match bytes[0..4] {
        [0x00, 0x00, 0x00, 0x00] => Some(CommandActionType::Set),
        [0x01, 0x00, 0x00, 0x00] => Some(CommandActionType::Get),
        [0x02, 0x00, 0x00, 0x00] => Some(CommandActionType::Reset),
        [0x03, 0x00, 0x00, 0x00] => Some(CommandActionType::Ack),
        [0x04, 0x00, 0x00, 0x00] => Some(CommandActionType::Nak),
        _ => None, }
    }
}

impl fmt::Display for CommandActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandActionType::Set => write!(f, "CMD_ACTION_SET"),
            CommandActionType::Get => write!(f, "CMD_ACTION_GET"),
            CommandActionType::Reset => write!(f, "CMD_ACTION_RESET"),
            CommandActionType::Ack => write!(f, "CMD_ACTION_ACK"),
            CommandActionType::Nak => write!(f, "CMD_ACTION_NAK"),
        }
    }
}