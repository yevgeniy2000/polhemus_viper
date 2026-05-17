use crate::internal::command_action::CommandActionType;
use crate::internal::command_frame_body::{CommandConfiguration, CommandFrameBody};
use crate::internal::command_types::CommandType;
use crate::internal::pno_frame_body::PnoFrameMode;

#[allow(dead_code)]
pub struct CommandGetSinglePno{
    pno_frame_mode: PnoFrameMode,
}

#[allow(dead_code)]
impl CommandGetSinglePno {
    pub fn new(pno_frame_mode: PnoFrameMode,) -> Self {
        Self {pno_frame_mode}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let command_frame_body = CommandFrameBody::new(0, CommandType::SinglePno, CommandActionType::Get, 0, self.pno_frame_mode.to_u32(), vec![], CommandConfiguration::default());
        command_frame_body.to_vec()
    }
}