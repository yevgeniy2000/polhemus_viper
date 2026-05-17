mod sensor;
mod measurement;
mod internal;

use crate::measurement::viper::{Measurement, OriUnit, PosUnit};
use crate::sensor::viper::Sensor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // --- Configuration ---
    let port_name = "COM6";
    let baud_rate = 115_200;

    let mut sensor = Sensor::new(port_name, baud_rate)
        .with_units(PosUnit::Centimeters, OriUnit::Quaternion);

    sensor.configure()?;

    let callback = std::sync::Arc::new(|measurement: Measurement| {
        println!("Measurement received: {}", measurement);
    });

    let join_handle = sensor.start_sensor_reading(callback)?;

    join_handle.join().unwrap();

    Ok(())
}