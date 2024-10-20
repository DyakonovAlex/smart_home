use std::collections::HashSet;

pub struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashSet::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.insert(device_name.to_string());
    }

    pub fn get_devices(&self) -> &HashSet<String> {
        &self.devices
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
