# polhemus_viper

A Rust library for the Polhemus Viper electromagnetic tracking system.

## Usage

```rust
use polhemus_viper::Sensor;
use polhemus_viper::measurement::{Measurement, PosUnit, OriUnit};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sensor = Sensor::new("COM6", 115_200)
        .with_units(PosUnit::Centimeters, OriUnit::Quaternion);

    sensor.configure()?;

    let callback = Arc::new(|measurement: Measurement| {
        println!("Measurement: {}", measurement);
    });

    let handle = sensor.start_sensor_reading(callback)?;
    handle.join().unwrap();
    
    Ok(())
}
```
