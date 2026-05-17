use std::fmt;
use crate::measurement::viper::{OriUnit, PosUnit};
use crate::internal::command_action::CommandActionType;
use crate::internal::command_frame_body::{CommandConfiguration, CommandFrameBody};
use crate::internal::command_types::CommandType;

impl PosUnit {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            PosUnit::Inches => 0u32.to_le_bytes(),
            PosUnit::Feet => 1u32.to_le_bytes(),
            PosUnit::Centimeters => 2u32.to_le_bytes(),
            PosUnit::Meters => 3u32.to_le_bytes()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> { if bytes.len() < 4 { return None; } match bytes[0..4] {
        [0x00, 0x00, 0x00, 0x00] => Some(PosUnit::Inches),
        [0x01, 0x00, 0x00, 0x00] => Some(PosUnit::Feet),
        [0x02, 0x00, 0x00, 0x00] => Some(PosUnit::Centimeters),
        [0x03, 0x00, 0x00, 0x00] => Some(PosUnit::Meters),
        _ => None, }
    }
}

impl OriUnit {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            OriUnit::EulerDegree => 0u32.to_le_bytes(),
            OriUnit::EulerRadian => 1u32.to_le_bytes(),
            OriUnit::Quaternion => 2u32.to_le_bytes(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> { if bytes.len() < 4 { return None; } match bytes[0..4] {
        [0x00, 0x00, 0x00, 0x00] => Some(OriUnit::EulerDegree),
        [0x01, 0x00, 0x00, 0x00] => Some(OriUnit::EulerRadian),
        [0x02, 0x00, 0x00, 0x00] => Some(OriUnit::Quaternion),
        _ => None, }
    }
}

#[derive(Debug)]
pub struct CommandUnitsPayload {
    pub(crate) pos_unit: PosUnit,
    pub(crate) ori_unit: OriUnit,
}

impl CommandUnitsPayload {
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&self.pos_unit.to_bytes());
        bytes[4..8].copy_from_slice(&self.ori_unit.to_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 { return None; }
        Some(Self{
            pos_unit: PosUnit::from_bytes(&bytes[0..4]).unwrap(),
            ori_unit: OriUnit::from_bytes(&bytes[4..8]).unwrap()})
    }
}

impl fmt::Display for CommandUnitsPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitPayload: {{Pos: {}, Ori: {}}}", self.pos_unit, self.ori_unit)
    }
}
pub struct CommandSetUnits {
    pub(crate) payload: CommandUnitsPayload,
}

impl CommandSetUnits {
    pub fn new(payload: CommandUnitsPayload) -> Self {
        Self { payload }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let command_frame_body = CommandFrameBody::new(0, CommandType::Units, CommandActionType::Set, 0, 0, Vec::from(self.payload.to_bytes()), CommandConfiguration::default());
        command_frame_body.to_vec()
    }
}

#[allow(dead_code)]
pub struct CommandGetUnits;
#[allow(dead_code)]
impl CommandGetUnits {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let command_frame_body = CommandFrameBody::new(0, CommandType::Units, CommandActionType::Get, 0, 0, vec![], CommandConfiguration::default());
        command_frame_body.to_vec()
    }
}