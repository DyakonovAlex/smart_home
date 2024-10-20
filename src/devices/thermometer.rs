pub struct Thermometer {
    name: String,
    temperature: f64,
}

impl Thermometer {
    pub fn new(name: &str, temperature: f64) -> Self {
        Self {
            name: name.to_string(),
            temperature,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn _get_temp(&self) -> f64 {
        self.temperature
    }

    pub fn _set_temp(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn description(&self) -> String {
        format!(
            "Термометр: {}, Температура: {}",
            self.name, self.temperature
        )
    }
}
