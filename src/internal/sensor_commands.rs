use std::io;
use crate::internal::command_continuous_pno::{CommandStartContinuousPno, CommandStopContinuousPno};
use crate::internal::command_single_pno::CommandGetSinglePno;
use crate::internal::command_units::{CommandGetUnits, CommandSetUnits, CommandUnitsPayload};
use crate::internal::frame::{send_frame, FrameType};
use crate::internal::pno_frame_body::PnoFrameMode;
use serialport::SerialPort;

pub fn send_set_units_command(port: &mut Box<dyn SerialPort>, units : CommandUnitsPayload) -> io::Result<()> {
    let body = CommandSetUnits::new(units);

    //println!("body: {:02X?}", body.to_bytes());

    send_frame(port, FrameType::Command, &body.to_bytes())
}

pub(crate) fn send_get_units_command(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    let body = CommandGetUnits::new();

    //println!("body: {:02X?}", body.to_bytes());

    send_frame(port, FrameType::Command, &body.to_bytes())
}

pub(crate) fn send_get_single_pno_command(port: &mut Box<dyn SerialPort>, pno_frame_mode: PnoFrameMode) -> io::Result<()> {
    let body = CommandGetSinglePno::new(pno_frame_mode);

    send_frame(port, FrameType::Command, &body.to_bytes())
}

pub(crate) fn send_start_continuous_command(port: &mut Box<dyn SerialPort>, pno_frame_mode: PnoFrameMode, cont_next_frame_number: u32) -> io::Result<()> {
    let body = CommandStartContinuousPno::new(pno_frame_mode, cont_next_frame_number);

    send_frame(port, FrameType::Command, &body.to_bytes())
}

pub(crate) fn send_stop_continuous_command(port: &mut Box<dyn SerialPort>) -> io::Result<()> {
    let body = CommandStopContinuousPno::new();

    send_frame(port, FrameType::Command, &body.to_bytes())
}