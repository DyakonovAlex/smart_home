pub struct SmartSocket {
    _id: u32,
    name: String,
    power_consumption: u32,
    is_on: bool,
}

impl SmartSocket {
    pub fn new(id: u32, name: &str, power_consumption: u32) -> Self {
        Self {
            _id: id,
            name: name.to_string(),
            power_consumption,
            is_on: false,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn _turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn _get_power(&self) -> u32 {
        self.power_consumption
    }

    pub fn _set_power(&mut self, power_consumption: u32) {
        self.power_consumption = power_consumption;
    }

    pub fn _is_on(&self) -> bool {
        self.is_on
    }

    pub fn description(&self) -> String {
        format!(
            "SmartSocket: {}, Power Consumption: {}, On: {}",
            self.name, self.power_consumption, self.is_on
        )
    }
}
