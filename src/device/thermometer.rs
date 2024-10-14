pub struct Thermometer {
    _id: u32,
    name: String,
    temperature: f64,
}

impl Thermometer {
    pub fn new(id: u32, name: &str, temperature: f64) -> Self {
        Self {
            _id: id,
            name: name.to_string(),
            temperature,
        }
    }

    pub fn _get_temp(&self) -> f64 {
        self.temperature
    }

    pub fn set_temp(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn description(&self) -> String {
        format!(
            "Thermometer: {}, Temperature: {}",
            self.name, self.temperature
        )
    }
}
