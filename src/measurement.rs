pub mod viper {
    use std::cmp::PartialEq;
    use crate::internal::pno_frame_body::PnoData;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Measurement {
        pub timestamp: i64,
        pub values: Vec<f32>,

        pub pos_unit: PosUnit,
        pub ori_unit: OriUnit,
    }


    impl Measurement {
        pub(crate) fn from_pno_data(timestamp : i64, pno_data: &PnoData) -> Measurement{
            let (values,pos_unit,ori_unit) = match pno_data {
                PnoData::Standard(pno_data) => {
                    (vec![pno_data.position.x, pno_data.position.y, pno_data.position.z, pno_data.orientation.f0, pno_data.orientation.f1, pno_data.orientation.f2, pno_data.orientation.f3], pno_data.position.pos_unit, pno_data.orientation.ori_unit)
                }
                PnoData::OrientationAndAcceleration(pno_data) => {
                    (match pno_data.orientation.ori_unit {
                        OriUnit::EulerDegree | OriUnit::EulerRadian => {
                            vec![pno_data.position.x, pno_data.position.y, pno_data.position.z,
                                 pno_data.orientation.f0, pno_data.orientation.f1, pno_data.orientation.f2,
                            pno_data.orientation.a0, pno_data.orientation.a1, pno_data.orientation.a2, pno_data.orientation.a3]
                        }
                        OriUnit::Quaternion => {
                            vec![pno_data.position.x, pno_data.position.y, pno_data.position.z,
                                 pno_data.orientation.f0, pno_data.orientation.f1, pno_data.orientation.f2, pno_data.orientation.f3,
                                 pno_data.orientation.a0, pno_data.orientation.a1, pno_data.orientation.a2, pno_data.orientation.a3]
                        }
                    }, pno_data.position.pos_unit, pno_data.orientation.ori_unit)
                }
            };

            Self::new(timestamp, values, pos_unit, ori_unit)
        }

        pub fn timestamp_secs(&self) -> f64 {
            self.timestamp as f64 / 1000.0
        }

        pub fn new(timestamp: i64, values: Vec<f32>, pos_unit: PosUnit, ori_unit: OriUnit) -> Self {
            if values.len() != 7 {
                panic!("Invalid number of values in measurement: {}", values.len());
            }

            let q0 = values[3];
            let q1 = values[4];
            let q2 = values[5];
            let q3 = values[6];

            if (q0*q0 + q1*q1 + q2*q2 + q3*q3).sqrt() - 1.0 > 0.01 {
                panic!("Invalid quaternion in measurement: {:#?}", values);
            }

            Self { timestamp, values, pos_unit, ori_unit }
        }
    }


    impl fmt::Display for Measurement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{timestamp: {}, values: ", self.timestamp)?;
            if self.values.len() > 2{
                let u = match self.pos_unit {
                    PosUnit::Meters => "m",
                    PosUnit::Centimeters => "cm",
                    PosUnit::Inches => "in",
                    PosUnit::Feet => "ft",
                };
                write!(f, "{{position: (x: {} {}, y: {} {}, z: {} {})", self.values[0], u, self.values[1], u, self.values[2], u)?;
                if self.values.len() > 5 && self.ori_unit != OriUnit::Quaternion || self.values.len() > 6 && self.ori_unit == OriUnit::Quaternion{
                    write!(f, ", orientation: (")?;
                    match self.ori_unit {
                        OriUnit::EulerDegree => {
                            write!(f, " yaw: {}, pitch: {}, roll: {}", self.values[3], self.values[4], self.values[5])?;
                        },
                        OriUnit::EulerRadian => {
                            write!(f, " yaw: {}°, pitch: {}°, roll: {}°", self.values[3], self.values[4], self.values[5])?;
                        },
                        OriUnit::Quaternion => {
                            write!(f, " w: {}, x: {}, y: {}, z: {}", self.values[3], self.values[4], self.values[5], self.values[6])?;
                        }
                    }
                    write!(f, ")")?;
                }else{
                    write!(f, "{:?}", &self.values[3..])?;
                }
                write!(f, "}}")?;
            }else{
                write!(f, "{:?}", self.values)?;
            }
            write!(f, "}}")
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PosUnit {
        Inches,
        Feet,
        Centimeters,
        Meters,
    }

    impl fmt::Display for PosUnit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PosUnit::Inches => write!(f, "POS_INCH"),
                PosUnit::Feet => write!(f, "POS_FOOT"),
                PosUnit::Centimeters => write!(f, "POS_CM"),
                PosUnit::Meters => write!(f, "POS_METER"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OriUnit {
        EulerDegree,
        EulerRadian,
        Quaternion,
    }

    impl fmt::Display for OriUnit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                OriUnit::EulerDegree => write!(f, "ORI_EULER_DEGREE "),
                OriUnit::EulerRadian => write!(f, "ORI_EULER_RADIAN "),
                OriUnit::Quaternion => write!(f, "ORI_QUATERNION "),
            }
        }
    }
}