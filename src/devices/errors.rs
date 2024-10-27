pub const THERMOMETER_CREATION_ERROR: &str = "Failed to create thermometer";
pub const THERMOMETER_TEMP_ERROR: &str = "Invalid thermometer temperature";

pub const SOCKET_CREATION_ERROR: &str = "Failed to create socket";
pub const SOCKET_POWER_ERROR: &str = "Invalid socket power";

#[derive(Debug)]
pub enum DeviceError {
    NotFound(String),
    InvalidSocketPower(u32),
    InvalidThermometerTemperature(f64),
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::NotFound(device_name) => {
                write!(f, "Устройство {} не найдено.", device_name)
            }
            DeviceError::InvalidSocketPower(power) => {
                write!(f, "Некорректная мощность розетки: {}", power)
            }
            DeviceError::InvalidThermometerTemperature(temperature) => {
                write!(f, "Некорректная температура термометра: {}", temperature)
            }
        }
    }
}

impl std::error::Error for DeviceError {}
