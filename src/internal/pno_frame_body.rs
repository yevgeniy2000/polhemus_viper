use crate::measurement::viper::{OriUnit, PosUnit};
use crate::internal::float_utils::{acceleration_fract_to_float, degree_fract_to_float, quaternion_fract_to_float, radian_fract_to_float};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct PnoFrameBody {
    pub(crate) seuid: u32,
    pub(crate) frame: u32,
    pub(crate) hpinfo: u32,
    pub(crate) command_payload: Vec<PnoData>, //max 16 entries
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct PnoPosition {
    pub(crate) x : f32,
    pub(crate) y : f32,
    pub(crate) z : f32,

    // one may add the unit here
    pub(crate) pos_unit: PosUnit,
}
#[derive(Debug, Clone, Copy)]
pub(crate)  struct PnoStandardOrientation {
    pub(crate) f0 : f32,
    pub(crate) f1 : f32,
    pub(crate) f2 : f32,
    pub(crate) f3 : f32,
    pub(crate) ori_unit: OriUnit,
}

impl Display for PnoStandardOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.ori_unit {
            OriUnit::EulerDegree => {
                write!(f, "EulerDegree {{ yaw: {}°, pitch: {}°, roll: {}° }}", self.f0, self.f1, self.f2)
            }
            OriUnit::EulerRadian => {
                write!(f, "EulerRadian {{ yaw: {}, pitch: {}, roll: {} }}", self.f0, self.f1, self.f2)
            }
            OriUnit::Quaternion => {
                write!(f, "Quaternion {{ w: {}, x: {}, y: {}, z: {} }}", self.f0, self.f1, self.f2, self.f3)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PnoOrientationAndAcceleration {
    pub(crate) f0 : f32,
    pub(crate) f1 : f32,
    pub(crate) f2 : f32,
    pub(crate) f3 : f32,
    pub(crate) a0 : f32,
    pub(crate) a1 : f32,
    pub(crate) a2 : f32,
    pub(crate) a3 : f32,
    pub(crate) ori_unit: OriUnit,
}

impl Display for PnoOrientationAndAcceleration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.ori_unit {
            OriUnit::EulerDegree => {
                write!(f, "(EulerDegree {{ yaw: {}°, pitch: {}°, roll: {}° }}, Acceleration {{x: {}, y: {}, z: {}, xyz: {}}})", self.f0, self.f1, self.f2, self.a0, self.a1, self.a2, self.a3)
            }
            OriUnit::EulerRadian => {
                write!(f, "(EulerRadian {{ yaw: {}, pitch: {}, roll: {} }}, Acceleration {{x: {}, y: {}, z: {}, xyz: {}}})", self.f0, self.f1, self.f2, self.a0, self.a1, self.a2, self.a3)
            }
            OriUnit::Quaternion => {
                write!(f, "(Quaternion {{ w: {}, x: {}, y: {}, z: {} }}, Acceleration {{x: {}, y: {}, z: {}, xyz: {}}})", self.f0, self.f1, self.f2, self.f3, self.a0, self.a1, self.a2, self.a3)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PnoFrameMode{
    Standard,
    Acceleration
}

impl PnoFrameMode {
    pub fn to_u32(self) -> u32 {
        match self {
            PnoFrameMode::Standard => 0,
            PnoFrameMode::Acceleration => 1,
        }
    }

    pub fn from_u32(u32 : u32) -> Option<Self> {
        match u32 {
            0 => Some(PnoFrameMode::Standard),
            1 => Some(PnoFrameMode::Acceleration),
            _ => None
        }
    }
}

impl Display for PnoFrameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PnoFrameMode::Standard => write!(f, "Standard"),
            PnoFrameMode::Acceleration => write!(f, "Acceleration"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PnoSensorData<OrientationType> {
    pub(crate) sf_info : u32, // TODO specify bits: https://ftp.polhemus1.com/hidden/Viper/Software/SDK/VNCP/html/index.htm#rhsearch=sf_info&t=assets%2Fhtml%2Fnativedoxyrh%2Fhtml%2Fstruct_s_f_i_n_f_o.html&rhhlterm=sfinfo
    pub(crate) position : PnoPosition,
    pub(crate) orientation : OrientationType,
}

#[derive(Debug, Clone, Copy)]
pub enum PnoData {
    Standard(PnoSensorData<PnoStandardOrientation>),
    OrientationAndAcceleration(PnoSensorData<PnoOrientationAndAcceleration>),
}

impl PnoData {
    pub fn from_bytes(mode : PnoFrameMode, pos_unit: PosUnit, unit : OriUnit, bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 32 {
            return None;
        }
        let sf_info = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let position = PnoPosition {
            x : f32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            y : f32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            z : f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            pos_unit: pos_unit,
        };
        match mode {
            PnoFrameMode::Standard => {
               let orientation = PnoStandardOrientation {
                   f0 : f32::from_le_bytes(bytes[16..20].try_into().unwrap()),
                   f1 : f32::from_le_bytes(bytes[20..24].try_into().unwrap()),
                   f2 : f32::from_le_bytes(bytes[24..28].try_into().unwrap()),
                   f3 : f32::from_le_bytes(bytes[28..32].try_into().unwrap()),
                   ori_unit: unit,
               };
                Some(Self::Standard(PnoSensorData{sf_info, position, orientation}))
            }
            PnoFrameMode::Acceleration => {
                let convert_position = |x : i16| -> f32 {
                    match unit {
                        OriUnit::EulerDegree => degree_fract_to_float(x),
                        OriUnit::EulerRadian => radian_fract_to_float(x),
                        OriUnit::Quaternion => quaternion_fract_to_float(x),
                    }
                };

                let orientation_acceleration = PnoOrientationAndAcceleration {
                    f0 : convert_position(i16::from_le_bytes(bytes[16..18].try_into().unwrap())),
                    f1 : convert_position(i16::from_le_bytes(bytes[18..20].try_into().unwrap())),
                    f2 : convert_position(i16::from_le_bytes(bytes[20..22].try_into().unwrap())),
                    f3 : convert_position(i16::from_le_bytes(bytes[22..24].try_into().unwrap())),
                    a0: acceleration_fract_to_float(i16::from_le_bytes(bytes[24..26].try_into().unwrap())),
                    a1: acceleration_fract_to_float(i16::from_le_bytes(bytes[26..28].try_into().unwrap())),
                    a2: acceleration_fract_to_float(i16::from_le_bytes(bytes[28..30].try_into().unwrap())),
                    a3: acceleration_fract_to_float(i16::from_le_bytes(bytes[30..32].try_into().unwrap())),
                    ori_unit: unit,
                };
                Some(Self::OrientationAndAcceleration(PnoSensorData{sf_info, position, orientation: orientation_acceleration }))
            }
        }
    }
}

impl Display for PnoData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PnoData::Standard(sensor_data) => {
                write!(f, "{{sf_info: {}, position: {:?}, orientation: {}}}", sensor_data.sf_info, sensor_data.position, sensor_data.orientation)
            }
            PnoData::OrientationAndAcceleration(sensor_data) => {
                write!(f, "{{sf_info: {}, position: {:?}, orientation_and_acceleration: {}}}", sensor_data.sf_info, sensor_data.position, sensor_data.orientation)}
        }
    }
}

impl PnoFrameBody {
    fn new(seuid: u32, frame: u32, hpinfo: u32, command_payload: Vec<PnoData>) -> Self {
        Self { seuid, frame, hpinfo, command_payload }
    }

    pub fn from_bytes(mode : PnoFrameMode, pos_unit: PosUnit, ori_unit: OriUnit, body: &[u8]) -> Option<Self> {
        if body.len() < 16 {
            return None;
        }
        let seuid = u32::from_le_bytes(body[0..4].try_into().unwrap());
        let frame = u32::from_le_bytes(body[4..8].try_into().unwrap());
        let hpinfo = u32::from_le_bytes(body[8..12].try_into().unwrap());
        let sens_count = u32::from_le_bytes(body[12..16].try_into().unwrap());

        let sensor_pno_list = body[16..].to_vec();

        if sensor_pno_list.len() != sens_count as usize * 32 {
            return None;
        }

        let sensor_pnos : Vec<PnoData> = sensor_pno_list.chunks(32).map(|chunk| {
            //chunk[0..4].try_into().unwrap()
            PnoData::from_bytes(mode, pos_unit, ori_unit, chunk)
        }).flatten().collect();

        Some(Self::new(seuid, frame, hpinfo, sensor_pnos))
    }

}


impl Display for PnoFrameBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PnoFrameBody {{ seuid: {}, frame: {}, hpinfo: {}, command_payload: [", self.seuid, self.frame, self.hpinfo)?;
        for (i, data) in self.command_payload.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", data)?;
        }
        write!(f, "] }}")
    }
}