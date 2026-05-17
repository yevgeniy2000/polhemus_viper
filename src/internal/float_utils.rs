const FFACTOR_EULER_DEGREE: f32 = 180.0;
const FFACTOR_EULER_RAD: f32 = std::f32::consts::PI;
const FFACTOR_QTRN: f32 = 1.0;
const FFACTOR_ACCEL: f32 = 16.0;

#[inline]
pub fn fract_to_float(fract: i16, factor: f32) -> f32 {
    (fract as f32) / 32768.0 * factor
}

#[inline]
pub fn degree_fract_to_float(i_deg: i16) -> f32 {
    fract_to_float(i_deg, FFACTOR_EULER_DEGREE)
}

#[inline]
pub fn radian_fract_to_float(i_rad: i16) -> f32 {
    fract_to_float(i_rad, FFACTOR_EULER_RAD)
}

#[inline]
pub fn quaternion_fract_to_float(i_quat: i16) -> f32 {
    fract_to_float(i_quat, FFACTOR_QTRN)
}

#[inline]
pub fn acceleration_fract_to_float(i_acc: i16) -> f32 {
    fract_to_float(i_acc, FFACTOR_ACCEL)
}