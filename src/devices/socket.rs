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

    pub fn _turn_on(&mut self) {
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
            "Умная розетка: {}, Потребляемая мощность: {}, Вкл: {}",
            self.name, self.power_consumption, self.is_on
        )
    }
}
