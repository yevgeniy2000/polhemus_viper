pub mod viper {
    use crate::internal::command_action::CommandActionType;
    use crate::internal::command_frame_body::{CommandConfiguration, CommandFrameBody};
    use crate::internal::command_units::CommandUnitsPayload;
    use crate::internal::frame::{read_frame, FrameType};
    use crate::internal::pno_frame_body::{PnoFrameBody, PnoFrameMode};
    use crate::internal::sensor_commands::{send_set_units_command, send_start_continuous_command, send_stop_continuous_command};
    use serialport::SerialPort;
    use std::io;
    use std::time::Duration;
    use crate::measurement::viper::{Measurement, OriUnit, PosUnit};

    #[derive(Debug)]
    pub struct Sensor {
        command_configuration: CommandConfiguration,
        port_name: String,
        baud_rate: u32,
        port: Option<Box<dyn SerialPort>>,
        running: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
        pos_unit: PosUnit,
        ori_unit: OriUnit,
    }

    impl Sensor {
        fn execute_port(&mut self, command_executor: &dyn Fn(&mut Box<dyn SerialPort>) -> io::Result<()>) -> io::Result<CommandFrameBody> {
            match self.port.as_mut() {
                None => {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Sensor is not configured!"))?
                }
                Some(mut port) => {
                    command_executor(&mut port)?;
                    loop {
                        let (_, frame) = read_frame(&mut port, self.command_configuration)?;
                        match frame.frame_type {
                            FrameType::Command => {
                                let command_frame_body = CommandFrameBody::from_vec(&frame.frame_body, self.command_configuration);
                                match command_frame_body {
                                    None => {
                                        Err(io::Error::new(io::ErrorKind::InvalidData, "Sensor replied with unreadable method!"))?
                                    }
                                    Some(command_frame_body) => {
                                        println!("Response: {}", command_frame_body);
                                        return Ok(command_frame_body);
                                    }
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
            }
        }

        fn check_ack_response(response: &CommandFrameBody) -> io::Result<()> {
            match response.action {
                CommandActionType::Ack => { Ok(()) }
                CommandActionType::Nak => {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Sensor did not acknowledge!"))
                }
                _ => {
                    Err(io::Error::new(io::ErrorKind::InvalidData, format!("Sensor sent unexpected action: {}", response.action)))
                }
            }
        }

        // ... existing code ...
        fn set_units(&mut self, pos_unit: PosUnit, ori_unit: OriUnit) -> io::Result<()> {
            let response = self.execute_port(&|port| {
                send_set_units_command(port, CommandUnitsPayload {
                    pos_unit,
                    ori_unit,
                })?;
                Ok(())
            })?;

            Self::check_ack_response(&response)?;

            self.command_configuration.pos_unit = pos_unit;
            self.command_configuration.ori_unit = ori_unit;
            Ok(())
        }

        fn start_reading(&mut self, pno_frame_mode: PnoFrameMode, cont_next_frame_number: u32) -> io::Result<()> {
            let response = self.execute_port(&|port| {
                send_start_continuous_command(port, pno_frame_mode, cont_next_frame_number)?;
                Ok(())
            })?;

            Self::check_ack_response(&response)?;

            Ok(())
        }

        fn stop_reading(&mut self) -> io::Result<()> {
            let response = self.execute_port(&|port| {
                send_stop_continuous_command(port)?;
                Ok(())
            })?;

            Self::check_ack_response(&response)?;

            Ok(())
        }

        fn configure_commands(&mut self) -> io::Result<()> {
            self.stop_reading()?;
            self.set_units(self.pos_unit, self.ori_unit)?;
            Ok(())
        }

        pub fn start_sensor_reading(&mut self, on_data: std::sync::Arc<dyn Fn(Measurement) + Send + Sync>
        ) -> io::Result<std::thread::JoinHandle<()>> {
            self.start_reading(PnoFrameMode::Standard, 0)?;
            if let Some(mut port) = self.port.take() {
                let config = self.command_configuration;

                use std::sync::Arc;
                use std::sync::atomic::{AtomicBool, Ordering};
                let running = Arc::new(AtomicBool::new(true));
                let r = running.clone();

                self.running = Some(running.clone());

                ctrlc::set_handler(move || {
                    println!("\nTermination signal received...");
                    r.store(false, Ordering::SeqCst);
                }).ok(); // Ignore error if handler is already set

                let handle = std::thread::spawn(move || {
                    struct SensorGuard<'a>(&'a mut Box<dyn SerialPort>);
                    impl Drop for SensorGuard<'_> {
                        fn drop(&mut self) {
                            println!("SensorGuard: Stopping continuous reading...");
                            let _ = send_stop_continuous_command(self.0);
                        }
                    }

                    let guard = SensorGuard(&mut port);
                    // let mut packet_count = 0;
                    while running.load(Ordering::SeqCst) {
                        match read_frame(guard.0, config) {
                            Ok((time_millis, frame)) => {
                                if let FrameType::Pno = frame.frame_type {
                                    // packet_count += 1;
                                    /*if packet_count % 100 == 0 {
                                        println!("PNO data received: {} packets...", packet_count);
                                    }*/
                                    match PnoFrameBody::from_bytes(config.pno_frame_mode, config.pos_unit, config.ori_unit, &frame.frame_body) {
                                        Some(pno_frame_body) => {
                                            // println!("{}", pno_frame_body);
                                            for data in pno_frame_body.command_payload {
                                                //FIXME
                                                let measurement = Measurement::from_pno_data(time_millis, &data);
                                                on_data(measurement);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Thread terminated: {}", e);
                                break;
                            }
                        }
                    }
                    println!("Sensor stopped.");
                });
                Ok(handle)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot use port"))
            }
        }
        pub fn configure(&mut self) -> io::Result<()> {
            if self.port.is_some() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Sensor is already configured!"));
            }

            self.port = Some({
                serialport::new(self.port_name.clone(), self.baud_rate)
                    .timeout(Duration::from_millis(5000))
                    .open()?
            });
            self.configure_commands()?;
            Ok(())
        }

        pub fn new(port_name: &str, baud_rate: u32) -> Self {
            Self {
                command_configuration: CommandConfiguration::default(),
                port_name: port_name.parse().unwrap(),
                baud_rate,
                port: None,
                running: None,
                pos_unit: PosUnit::Centimeters,
                ori_unit: OriUnit::Quaternion,
            }
        }

        pub fn with_units(mut self, pos_unit: PosUnit, ori_unit: OriUnit) -> Self {
            self.pos_unit = pos_unit;
            self.ori_unit = ori_unit;
            self
        }
    }

    impl Drop for Sensor {
        fn drop(&mut self) {
            if let Some(running) = &self.running {
                running.store(false, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }
}