use crate::internal::command_action::CommandActionType;
use crate::internal::command_frame_body::{CommandConfiguration, CommandFrameBody};
use crate::internal::command_types::CommandType;
use crate::internal::pno_frame_body::PnoFrameMode;

pub struct CommandStartContinuousPno {
    pno_frame_mode: PnoFrameMode,
    cont_next_frame_number: u32,
}

pub struct CommandStopContinuousPno;

impl CommandStartContinuousPno {
    pub fn new(pno_frame_mode: PnoFrameMode,cont_fc_config: u32,) -> Self {
        Self {pno_frame_mode, cont_next_frame_number: cont_fc_config }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let command_frame_body = CommandFrameBody::new(0, CommandType::ContinuousPno, CommandActionType::Set, 0, self.pno_frame_mode.to_u32(), self.cont_next_frame_number.to_le_bytes().to_vec(), CommandConfiguration::default());
        command_frame_body.to_vec()
    }
}

impl CommandStopContinuousPno {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let command_frame_body = CommandFrameBody::new(0, CommandType::ContinuousPno, CommandActionType::Reset, 0, 0, vec![], CommandConfiguration::default());
        command_frame_body.to_vec()
    }
}