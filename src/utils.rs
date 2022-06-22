use std::collections::BTreeMap;

use crate::{DeviceState, Error};

/// Returns connected ADB devices.
///
/// # Errors
///
/// Will error with [`Error::NoAdbPresent`] if it fails to find ADB in `PATH`, with [`Error::FailedToParseAdbDeviceResponse`] if it fails to parse a line from the `adb devices` response, and with [`Error::AdbDeviceTypeNotSupported`] if it finds an unhandled [`DeviceState`].
pub fn get_devices() -> Result<Vec<(String, DeviceState)>, Error> {
    let response = execute_command("adb", &["devices"])?;
    let mut devices = vec![];
    for l in response.lines().skip(1) {
        if l.is_empty() {
            continue;
        }
        let splot = l.split('\t').collect::<Vec<&str>>();
        if let &[name, status] = &splot[..] {
            devices.push((name.to_string(), DeviceState::try_from(status)?));
        } else {
            return Err(Error::FailedToParseAdbDeviceResponse(l.to_string()));
        }
    }
    Ok(devices)
}

/// Returns formatted hashmap of battery information for a specified device.
///
/// # Errors
///
/// Will error with [`Error::NoAdbPresent`] if it fails to find ADB in `PATH`.
pub fn get_battery_info(device: &str) -> Result<BTreeMap<String, String>, Error> {
    let mut btree_map = BTreeMap::new();

    let response = execute_command("adb", &["-s", device, "shell", "dumpsys", "battery"])?;
    for l in response.lines().skip(1) {
        let splot = l.split(": ").collect::<Vec<&str>>();
        if let &[key, value] = &splot[..] {
            btree_map.insert(key.trim_start().to_string(), value.to_string());
        }
    }

    format_values(&mut btree_map);

    Ok(btree_map)
}

fn format_values(map: &mut BTreeMap<String, String>) {
    let scale = map.get("scale").unwrap_or(&"100".to_string()).clone();
    map.remove("scale");
    for (key, value) in map.iter_mut() {
        match key.as_str() {
            "level" => {
                if scale == "100" {
                    value.push_str(" %");
                } else {
                    let v = value.parse::<f32>().unwrap();
                    let s = scale.parse::<f32>().unwrap();
                    let r = format!("{:.0} %", (v / s) * 100f32);
                    *value = r;
                }
            }
            "temperature" => {
                *value = format!("{:.1} °C", value.parse::<f32>().unwrap() / 10.0);
            }
            "voltage" => {
                *value = format!("{:.3} V", value.parse::<f32>().unwrap() / 1000.0);
            }
            "Charge counter" => {
                *value = format!("{:.0} mAh", value[..4].parse::<u32>().unwrap());
            }
            "Max charging current" => {
                const MULT_MILIA: f32 = 1_000.0;
                const MULT_A: f32 = 1_000_000.0;
                let v = value.parse::<f32>().unwrap();
                if v / MULT_A < 1.0 {
                    *value = format!("{:.0} mA", v / MULT_MILIA);
                } else {
                    *value = format!("{:.3} A", v / MULT_A);
                }
            }
            "Max charging voltage" => {
                let v = value.parse::<f32>().unwrap();
                *value = format!("{:.1} V", v / 1_000_000.0);
            }
            _ => {}
        }
    }
}

fn execute_command(command: &str, args: &[&str]) -> Result<String, Error> {
    let cmd = std::process::Command::new(command).args(args).output();
    if cmd.is_err() {
        return Err(Error::NoAdbPresent);
    }
    let cmd = cmd.unwrap();

    let response = String::from_utf8_lossy(&cmd.stdout);
    Ok(response.to_string())
}
