use crate::devices::errors::DeviceError;

pub struct Thermometer {
    name: String,
    temperature: f64,
}

impl Thermometer {
    fn validate_temperature(temperature: f64) -> Result<(), DeviceError> {
        if !(-273.15..=100.0).contains(&temperature) {
            return Err(DeviceError::InvalidThermometerTemperature(temperature));
        }
        Ok(())
    }

    pub fn new(name: &str, temperature: f64) -> Result<Self, DeviceError> {
        Self::validate_temperature(temperature)?;

        Ok(Self {
            name: name.to_string(),
            temperature,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_temp(&self) -> f64 {
        self.temperature
    }

    pub fn set_temp(&mut self, temperature: f64) -> Result<(), DeviceError> {
        Self::validate_temperature(temperature)?;

        self.temperature = temperature;

        Ok(())
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
    use crate::devices::errors::{THERMOMETER_CREATION_ERROR, THERMOMETER_TEMP_ERROR};

    #[test]
    fn test_thermometer_new_valid() {
        let thermometer = Thermometer::new("Thermometer", 25.5).expect(THERMOMETER_CREATION_ERROR);
        assert_eq!(thermometer.get_name(), "Thermometer");
        assert_eq!(thermometer.get_temp(), 25.5);
    }

    #[test]
    fn test_thermometer_set_temperature_valid() {
        let mut thermometer =
            Thermometer::new("Thermometer", 0.0).expect(THERMOMETER_CREATION_ERROR);
        thermometer.set_temp(25.5).expect(THERMOMETER_TEMP_ERROR);
        assert_eq!(thermometer.get_temp(), 25.5);
    }

    #[test]
    fn test_thermometer_description() {
        let thermometer = Thermometer::new("Thermometer", 25.5).expect(THERMOMETER_CREATION_ERROR);
        let expected_description = "Термометр: Thermometer, Температура: 25.5";
        assert_eq!(thermometer.description(), expected_description);
    }

    #[test]
    fn test_thermometer_new_invalid() {
        let result = Thermometer::new("Thermometer", -300.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_thermometer_set_temperature_invalid() {
        let mut thermometer =
            Thermometer::new("Thermometer", 25.0).expect(THERMOMETER_CREATION_ERROR);
        let result = thermometer.set_temp(300.0);
        assert!(result.is_err());
    }
}
