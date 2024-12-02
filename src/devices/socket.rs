use crate::devices::error::DeviceError;

#[derive(Debug, Clone)]
pub struct Socket {
    name: String,
    power: u32, // Мощность потребления в ваттах
    is_on: bool,
}

impl Socket {
    pub const MAX_POWER: u32 = 5000;

    fn validate_power(power: u32) -> Result<(), DeviceError> {
        if power > Self::MAX_POWER {
            return Err(DeviceError::InvalidSocketPower(power));
        }
        Ok(())
    }

    /// Creates a new Socket with the specified name and power consumption
    ///
    /// # Arguments
    /// * `name` - The name of the socket
    /// * `power` - Power consumption in watts
    ///
    /// # Returns
    /// * `Result<Socket, DeviceError>` - A new socket or an error if power is invalid
    pub fn new(name: &str, power: u32) -> Result<Self, DeviceError> {
        Self::validate_power(power)?;

        Ok(Self {
            name: name.to_string(),
            power,
            is_on: false,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn get_power(&self) -> u32 {
        self.power
    }

    pub fn set_power(&mut self, power: u32) -> Result<(), DeviceError> {
        Self::validate_power(power)?;

        self.power = power;

        Ok(())
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn description(&self) -> String {
        format!(
            "Умная розетка: {}, Потребляемая мощность: {}, Вкл: {}",
            self.name, self.power, self.is_on
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::devices::error::{SOCKET_CREATION_ERROR, SOCKET_POWER_ERROR};

    use super::*;

    #[test]
    fn test_socket_new_valid() {
        let socket = Socket::new("Socket", 3520).expect(SOCKET_CREATION_ERROR);
        assert_eq!(socket.get_name(), "Socket");
        assert_eq!(socket.get_power(), 3520);
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_turn_on() {
        let mut socket = Socket::new("Socket", 3520).expect(SOCKET_CREATION_ERROR);
        socket.turn_on();
        assert!(socket.is_on());
    }

    #[test]
    fn test_socket_turn_off() {
        let mut socket = Socket::new("Socket", 3520).expect(SOCKET_CREATION_ERROR);
        socket.turn_on();
        socket.turn_off();
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_set_power_valid() {
        let mut socket = Socket::new("Socket", 0).expect(SOCKET_CREATION_ERROR);
        socket.set_power(3520).expect(SOCKET_POWER_ERROR);
        assert_eq!(socket.get_power(), 3520);
    }

    #[test]
    fn test_socket_description() {
        let socket = Socket::new("Socket", 3520).expect(SOCKET_CREATION_ERROR);
        let expected_description = "Умная розетка: Socket, Потребляемая мощность: 3520, Вкл: false";
        assert_eq!(socket.description(), expected_description);
    }

    #[test]
    fn test_socket_new_invalid() {
        let result = Socket::new("Socket", 6000);
        assert!(result.is_err());
    }

    #[test]
    fn test_socket_set_power_invalid() {
        let mut socket = Socket::new("Socket", 0).expect(SOCKET_CREATION_ERROR);
        let result = socket.set_power(6000);
        assert!(result.is_err());
    }
}
