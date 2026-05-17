use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
    Hemisphere,
    Filter,
    TipOffset,
    Increment,
    Boresight,
    SensorWhoAmI,
    FrameRate,
    Units,
    SrcRotation,
    SinglePno,
    ContinuousPno,
    WhoAmI,
    SrcWhoAmI,
}

impl CommandType {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            CommandType::Hemisphere => 0u32.to_le_bytes(),
            CommandType::Filter => 1u32.to_le_bytes(),
            CommandType::TipOffset => 2u32.to_le_bytes(),
            CommandType::Increment => 3u32.to_le_bytes(),
            CommandType::Boresight => 4u32.to_le_bytes(),
            CommandType::SensorWhoAmI => 5u32.to_le_bytes(),
            CommandType::FrameRate => 6u32.to_le_bytes(),
            CommandType::Units => 7u32.to_le_bytes(),
            CommandType::SrcRotation => 8u32.to_le_bytes(),
            CommandType::SinglePno => 18u32.to_le_bytes(),
            CommandType::ContinuousPno => 19u32.to_le_bytes(),
            CommandType::WhoAmI => 20u32.to_le_bytes(),
            CommandType::SrcWhoAmI => 32u32.to_le_bytes(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> { if bytes.len() < 4 { return None; } match bytes[0..4] {
        [0x00, 0x00, 0x00, 0x00] => Some(CommandType::Hemisphere),
        [0x01, 0x00, 0x00, 0x00] => Some(CommandType::Filter),
        [0x02, 0x00, 0x00, 0x00] => Some(CommandType::TipOffset),
        [0x03, 0x00, 0x00, 0x00] => Some(CommandType::Increment),
        [0x04, 0x00, 0x00, 0x00] => Some(CommandType::Boresight),
        [0x05, 0x00, 0x00, 0x00] => Some(CommandType::SensorWhoAmI),
        [0x06, 0x00, 0x00, 0x00] => Some(CommandType::FrameRate),
        [0x07, 0x00, 0x00, 0x00] => Some(CommandType::Units),
        [0x08, 0x00, 0x00, 0x00] => Some(CommandType::SrcRotation),
        [0x12, 0x00, 0x00, 0x00] => Some(CommandType::SinglePno),
        [0x13, 0x00, 0x00, 0x00] => Some(CommandType::ContinuousPno),
        [0x14, 0x00, 0x00, 0x00] => Some(CommandType::WhoAmI),
        [0x24, 0x00, 0x00, 0x00] => Some(CommandType::SrcWhoAmI),
        _ => None, }
    }
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::Hemisphere => write!(f, "CMD_HEMISPHERE"),
            CommandType::Filter => write!(f, "CMD_FILTER"),
            CommandType::TipOffset => write!(f, "CMD_TIP_OFFSET"),
            CommandType::Increment => write!(f, "CMD_INCREMENT"),
            CommandType::Boresight => write!(f, "CMD_BORESIGHT"),
            CommandType::SensorWhoAmI => write!(f, "CMD_SENSOR_WHOAMI"),
            CommandType::FrameRate => write!(f, "CMD_FRAMERATE"),
            CommandType::Units => write!(f, "CMD_UNITS"),
            CommandType::SrcRotation => write!(f, "CMD_SRC_ROTATION"),
            CommandType::SinglePno => write!(f, "CMD_SINGLE_PNO"),
            CommandType::ContinuousPno => write!(f, "CMD_CONTINUOUS_PNO"),
            CommandType::WhoAmI => write!(f, "CMD_WHOAMI"),
            CommandType::SrcWhoAmI => write!(f, "CMD_SRC_WHOAMI"),
        }
    }
}