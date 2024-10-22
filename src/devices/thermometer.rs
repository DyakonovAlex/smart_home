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

    pub fn get_temp(&self) -> f64 {
        self.temperature
    }

    pub fn set_temp(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn description(&self) -> String {
        format!(
            "Термометр: {}, Температура: {}",
            self.name, self.temperature
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer_new() {
        let thermometer = Thermometer::new("Thermometer", 25.5);
        assert_eq!(thermometer.get_name(), "Thermometer");
        assert_eq!(thermometer.get_temp(), 25.5);
    }

    #[test]
    fn test_thermometer_set_temperature() {
        let mut thermometer = Thermometer::new("Thermometer", 0.0);
        thermometer.set_temp(25.5);
        assert_eq!(thermometer.get_temp(), 25.5);
    }

    #[test]
    fn test_thermometer_description() {
        let thermometer = Thermometer::new("Thermometer", 25.5);
        let expected_description = "Термометр: Thermometer, Температура: 25.5";
        assert_eq!(thermometer.description(), expected_description);
    }
}
