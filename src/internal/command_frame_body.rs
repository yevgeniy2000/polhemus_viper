use crate::internal::command_action::CommandActionType;
use crate::internal::command_types::CommandType;
use crate::internal::command_units::CommandUnitsPayload;
use crate::internal::pno_frame_body::{PnoFrameBody, PnoFrameMode};
use std::fmt;
use std::fmt::Formatter;
use crate::measurement::viper::{OriUnit, PosUnit};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CommandConfiguration {
    pub pos_unit: PosUnit,
    pub ori_unit: OriUnit,
    pub pno_frame_mode: PnoFrameMode,
}

impl CommandConfiguration {
    pub fn new(pos_unit: PosUnit, ori_unit: OriUnit, pno_frame_mode: PnoFrameMode ) -> Self {
        Self { pos_unit, ori_unit, pno_frame_mode}
    }
    pub fn default() -> Self {
        Self::new(PosUnit::Meters, OriUnit::EulerDegree, PnoFrameMode::Standard)
    }
}

#[derive(Debug)]
pub struct CommandFrameBody {
    pub seuid: u32,
    pub cmd: CommandType,
    pub action: CommandActionType,
    pub arg1: u32,
    pub arg2: u32,
    pub command_payload: Vec<u8>,

    /// This is not sent in the message but is used to determine how to interpret the payload.
    command_configuration: CommandConfiguration
}

impl CommandFrameBody {
    pub fn new(seuid: u32, cmd: CommandType, action: CommandActionType, arg1: u32, arg2: u32, command_payload: Vec<u8>, command_configuration: CommandConfiguration) -> Self {
        Self { seuid, cmd, action, arg1, arg2, command_payload, command_configuration }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut body = Vec::with_capacity(20 + self.command_payload.len());
        body.extend_from_slice(&self.seuid.to_le_bytes());
        body.extend_from_slice(&self.cmd.to_bytes());
        body.extend_from_slice(&self.action.to_bytes());
        body.extend_from_slice(&self.arg1.to_le_bytes());
        body.extend_from_slice(&self.arg2.to_le_bytes());
        body.extend_from_slice(&self.command_payload);
        body
    }

    pub fn from_vec(body: &[u8], command_configuration: CommandConfiguration) -> Option<Self> {
        if body.len() < 20 {
            return None;
        }
        let seuid = u32::from_le_bytes(body[0..4].try_into().unwrap());
        let cmd = CommandType::from_bytes(&body[4..8]).unwrap();
        let action = CommandActionType::from_bytes(&body[8..12]).unwrap();
        let arg1 = u32::from_le_bytes(body[12..16].try_into().unwrap());
        let arg2 = u32::from_le_bytes(body[16..20].try_into().unwrap());
        let command_payload = body[20..].to_vec();
        Some(Self::new(seuid, cmd, action, arg1, arg2, command_payload, command_configuration))
    }

}


impl fmt::Display for CommandFrameBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.cmd {
            CommandType::Units => {
                let units = CommandUnitsPayload::from_bytes(&self.command_payload);
                if let Some(u) = units {
                    write!(f, "CommandUnitsFrameBody {{ action: {}, units: {:?} }}", self.action, &u)
                }else{
                    write!(f, "CommandUnitsFrameBody {{ action: {} }}", self.action)
                }
            },
            CommandType::SinglePno => {
                let pno_frame_body = PnoFrameBody::from_bytes(PnoFrameMode::from_u32(self.arg2).unwrap_or(PnoFrameMode::Standard), self.command_configuration.pos_unit, self.command_configuration.ori_unit, &self.command_payload);
                if let Some(b) = pno_frame_body {
                    let pno_frame_type = PnoFrameMode::from_u32(self.arg2);

                    write!(f, "CommandSinglePno {{ action: {}, pno_frame_type: ", self.action)?;

                    match pno_frame_type {
                        None => {
                            write!(f, "Unknown")?;
                        }
                        Some(pno_frame_type) => {
                            write!(f, "{}", pno_frame_type)?;
                        }
                    }
                    write!(f, ", pno_frame_body: {} }}", &b)
                }else{
                    write!(f, "CommandSinglePno {{ action: {} }}", self.action)
                }
            },
            CommandType::ContinuousPno => {
                let pno_frame_type = PnoFrameMode::from_u32(self.arg2);
                write!(f, "CommandContinuousPno {{ action: {}, pno_frame_type: ", self.action)?;

                match pno_frame_type {
                    None => {
                        write!(f, "Unknown")?;
                    }
                    Some(pno_frame_type) => {
                        write!(f, "{}", pno_frame_type)?;
                    }
                }
                if self.command_payload.len() == 4{
                    let cont_next_frame_number = u32::from_le_bytes(self.command_payload[..4].try_into().unwrap());
                    write!(f, ", cont_next_frame_number: {}", cont_next_frame_number)?;
                }
                write!(f, " }}")
            }
            _ => {
                write!(f, "CommandFrameBody {{ seuid: {}, cmd: {}, action: {}, arg1: {}, arg2: {}, command_payload: {:?} }}", self.seuid, self.cmd, self.action, self.arg1, self.arg2, self.command_payload)
            }
        }


    }
}