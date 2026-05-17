use crate::internal::command_frame_body::{CommandConfiguration, CommandFrameBody};
use crate::internal::crc16::crc16;
use crate::internal::pno_frame_body::PnoFrameBody;
use serialport::SerialPort;
use std::io::Write;
use std::{fmt, io};
use chrono::Utc;

pub struct Frame {
    pub(crate) frame_type: FrameType,
    pub(crate) frame_body: Vec<u8>,
    pub(crate) command_configuration: CommandConfiguration,
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.frame_type {
            FrameType::Command => {
                let command_frame_body = CommandFrameBody::from_vec(&self.frame_body, self.command_configuration);
                if let Some(command_frame_body) = command_frame_body {
                    write!(f, "{}", command_frame_body)
                }else {
                    write!(f, "None")
                }
            },
            FrameType::Pno => {
                let pno_frame_body = PnoFrameBody::from_bytes(self.command_configuration.pno_frame_mode, self.command_configuration.pos_unit, self.command_configuration.ori_unit, &self.frame_body);

                if let Some(pno_frame_body) = pno_frame_body {
                    write!(f, "{}", pno_frame_body)
                }else {
                    write!(f, "Unrecognized PNO ({:02X?})", &self.frame_body)
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Command,
    Pno,
}

impl FrameType {
    pub fn header(&self) -> [u8; 4] {
        match self {
            FrameType::Command => [0x56, 0x50, 0x52, 0x43], // "VPRC"
            FrameType::Pno     => [0x56, 0x50, 0x52, 0x50], // "VPRP"
        }
    }

    pub fn from_header(bytes: &[u8]) -> Option<Self> { if bytes.len() < 4 { return None; } match bytes[0..4] {
        [0x56, 0x50, 0x52, 0x43] => Some(FrameType::Command), // "VPRC"
        [0x56, 0x50, 0x52, 0x50] => Some(FrameType::Pno), // "VPRP"
        _ => None, }
    }
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameType::Command => write!(f, "COMMAND"),
            FrameType::Pno => write!(f, "PNO"),
        }
    }
}

/// Sends a command in Viper binary format (VPRV)
pub fn send_frame(port: &mut Box<dyn SerialPort>, frame_type: FrameType, frame_body: &[u8]) -> io::Result<()> {
    let mut frame: Vec<u8> = Vec::new();

    frame.extend_from_slice(&frame_type.header());

    // 2. SIZE (4 Bytes, Little Endian): Bytes after this field (Body + 4 Bytes CRC)
    let size_value: u32 = (frame_body.len() as u32) + 4;
    frame.extend_from_slice(&size_value.to_le_bytes());

    // 3. FRAME BODY (Variable Length)
    frame.extend_from_slice(frame_body);

    // 4. CRC-16 (4 Bytes, Little Endian): Calculated over Preamble, Size and Body
    let crc_val = crc16(&frame);
    let crc_field = crc_val as u32;
    frame.extend_from_slice(&crc_field.to_le_bytes());

    // Send the packet
    port.write_all(&frame)?;
    port.flush()?;

    //println!("VPRC frame sent (Body Len: {}): {:02X?}", frame_body.len(), frame);
    Ok(())
}

pub fn read_frame(port: &mut Box<dyn SerialPort>, command_configuration: CommandConfiguration) -> io::Result<(i64, Frame)> {
    let time_millis = Utc::now().timestamp_millis();

    let mut header = [0u8; 8];

    // Synchronization: Search for header "VPR"
    let mut sync_buf = [0u8; 1];
    let mut sync_match = 0;
    let sync_pattern = b"VPR";
    let mut skipped_bytes = 0;

    while sync_match < 3 {
        match port.read_exact(&mut sync_buf) {
            Ok(_) => {
                if sync_buf[0] == sync_pattern[sync_match] {
                    sync_match += 1;
                } else {
                    if sync_buf[0] == sync_pattern[0] {
                        sync_match = 1;
                    } else {
                        sync_match = 0;
                    }
                    skipped_bytes += 1;
                }
            }
            Err(e) => {
                if skipped_bytes > 0 {
                    eprintln!("Synchronization failed after {} skipped bytes: {}", skipped_bytes, e);
                }
                return Err(e);
            }
        }
        if skipped_bytes > 10000 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many bytes skipped during synchronization"));
        }
    }

    if skipped_bytes > 0 {
        // println!("Synchronization: {} bytes skipped", skipped_bytes);
    }

    // "VPR" found, now read 4th byte
    header[0..3].copy_from_slice(sync_pattern);
    port.read_exact(&mut header[3..4])?;

    let frame_type = match FrameType::from_header(&header[0..4]){
        None => {
            eprintln!("Unknown frame type after synchronization: {:?}", &header[0..4]);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown frame type"));
        }
        Some(t) => {t}
    };

    port.read_exact(&mut header[4..8])?;
    let frame_size = u32::from_le_bytes(header[4..8].try_into().unwrap());


    if frame_size < 4 {
        println!("Frame size is smaller than 4 bytes!");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Frame size is smaller than 4 bytes!"));
    }

    let body_len = (frame_size - 4) as usize;

    let mut frame_body = vec![0u8; (body_len) as usize];
    port.read_exact(frame_body.as_mut())?;
    //println!("Body: {:02X?}", frame_body);

    let mut crc_field = [0u8; 4];
    port.read_exact(crc_field.as_mut())?;
    //println!("CRC: {:02X?}", crc_field);

    let mut full_data_for_crc = Vec::with_capacity(8 + body_len);
    full_data_for_crc.extend_from_slice(&header);
    full_data_for_crc.extend_from_slice(&frame_body);

    let crc_val = u32::from_le_bytes(crc_field.try_into().unwrap()) as u16;

    if crc16(&full_data_for_crc) != crc_val {
        println!("CRC error! (Calculated: {:04X}, Received: {:04X})", crc16(&full_data_for_crc), crc_val);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "CRC error!"));
    }

    Ok((time_millis, Frame {frame_type, frame_body, command_configuration}))

}
