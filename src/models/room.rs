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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_new() {
        let room = Room::new("Room");
        assert_eq!(room.get_name(), "Room");
        assert!(room.get_devices().is_empty());
    }

    #[test]
    fn test_room_add_device() {
        let mut room = Room::new("Room");
        room.add_device("Device");
        assert_eq!(room.get_devices().len(), 1);
        assert!(room.get_devices().contains("Device"));
    }
}
