use prompt::get_index;
use thiserror::Error;
use utils::get_devices;

use crate::utils::get_battery_info;
pub mod prompt;
pub mod utils;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ADB tools not found")]
    NoAdbPresent,
    #[error("No Android devices connected")]
    NoDevicesFound,
    #[error("Failed to parse ADB devices response: {0}")]
    FailedToParseAdbDeviceResponse(String),
    #[error("ADB device type isn't supported: {0}")]
    AdbDeviceTypeNotSupported(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceState {
    Connected,
    Unauthorised,
}

impl TryFrom<&str> for DeviceState {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "device" => Ok(Self::Connected),
            "unauthorised" | "unauthorized" => Ok(Self::Unauthorised),
            a => Err(Error::AdbDeviceTypeNotSupported(a.to_string())),
        }
    }
}

fn main() {
    match get_devices() {
        Ok(devices) => {
            let devices = devices
                .iter()
                .filter(|a| a.1 == DeviceState::Connected)
                .enumerate()
                .collect::<Vec<(usize, &(String, DeviceState))>>();
            for (index, d) in &devices {
                println!("[{index}] {:#?}", d.0);
            }
            let index = get_index();
            if let Some((_, device)) = devices.get(index) {
                match get_battery_info(&device.0) {
                    Ok(battery_info) => {
                        for bi in battery_info {
                            println!("{:<25}{}", bi.0, bi.1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Something went wrong while reading battery info! {e}");
                    }
                }
            } else {
                eprintln!("A device with that index doesn't exist!");
            }
        }
        Err(e) => {
            eprintln!("Unable to fetch devices! {e}");
        }
    }
}
