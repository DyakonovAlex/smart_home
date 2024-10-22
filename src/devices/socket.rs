pub struct Socket {
    name: String,
    power_consumption: u32,
    is_on: bool,
}

impl Socket {
    pub fn new(name: &str, power_consumption: u32) -> Self {
        Self {
            name: name.to_string(),
            power_consumption,
            is_on: false,
        }
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
        self.power_consumption
    }

    pub fn set_power(&mut self, power_consumption: u32) {
        self.power_consumption = power_consumption;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn description(&self) -> String {
        format!(
            "Умная розетка: {}, Потребляемая мощность: {}, Вкл: {}",
            self.name, self.power_consumption, self.is_on
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_new() {
        let socket = Socket::new("Socket", 220);
        assert_eq!(socket.get_name(), "Socket");
        assert_eq!(socket.get_power(), 220);
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_turn_on() {
        let mut socket = Socket::new("Socket", 220);
        socket.turn_on();
        assert!(socket.is_on());
    }

    #[test]
    fn test_socket_turn_off() {
        let mut socket = Socket::new("Socket", 220);
        socket.turn_on();
        socket.turn_off();
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_set_power() {
        let mut socket = Socket::new("Socket", 110);
        socket.set_power(220);
        assert_eq!(socket.get_power(), 220);
    }

    #[test]
    fn test_socket_description() {
        let socket = Socket::new("Socket", 220);
        let expected_description = "Умная розетка: Socket, Потребляемая мощность: 220, Вкл: false";
        assert_eq!(socket.description(), expected_description);
    }
}
