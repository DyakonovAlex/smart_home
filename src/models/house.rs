use crate::devices::device_info_provider::DeviceInfoProvider;
use crate::models::room::Room;
use std::collections::HashMap;

pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.get_name().to_string(), room);
    }

    pub fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }

    pub fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    pub fn create_report(&self, info: &impl DeviceInfoProvider) -> String {
        let mut report = format!("Отчет по дому {}\n", self.name);
        for (room_name, room) in &self.rooms {
            report.push_str(&format!("\tКомната: {}\n", room_name));
            for device_name in room.get_devices() {
                let device_info = info.get_device_info(room_name.clone(), device_name.clone());
                match device_info {
                    Ok(info) => report.push_str(&format!("\t\t- {}\n", info)),
                    Err(e) => report.push_str(&format!("\t\t- Error: {:?}\n", e)),
                }
            }
            report.push('\n');
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::errors::DeviceError;

    use super::*;

    struct MockDeviceInfoProvider;

    impl DeviceInfoProvider for MockDeviceInfoProvider {
        fn get_device_info(
            &self,
            _room_name: String,
            device_name: String,
        ) -> Result<String, DeviceError> {
            Ok(format!("Информация о устройстве: {}", device_name))
        }
    }

    #[test]
    fn test_house_new() {
        let house = House::new("House");
        assert_eq!(house.get_name(), "House");
        assert!(house.get_rooms().is_empty());
    }

    #[test]
    fn test_house_add_room() {
        let mut house = House::new("House");
        let room = Room::new("Room");
        house.add_room(room);
        assert_eq!(house.get_rooms().len(), 1);
        assert!(house.get_rooms().contains_key("Room"));
    }

    #[test]
    fn test_house_create_report() {
        let mut house = House::new("House");
        let mut room = Room::new("Room");
        room.add_device("Socket");
        house.add_room(room);

        let mock_info_provider = MockDeviceInfoProvider;
        let report = house.create_report(&mock_info_provider);

        let expected_report =
            "Отчет по дому House\n\tКомната: Room\n\t\t- Информация о устройстве: Socket\n\n";
        assert_eq!(report, expected_report);
    }

    #[test]
    fn test_house_get_room_found() {
        let mut house = House::new("House");
        let room = Room::new("Room");
        house.add_room(room);

        let result = house.get_room("Room");
        assert!(result.is_some());
        assert_eq!(result.unwrap().get_name(), "Room");
    }

    #[test]
    fn test_house_get_room_not_found() {
        let house = House::new("House");
        let result = house.get_room("Room");
        assert!(result.is_none());
    }

    #[test]
    fn test_house_remove_room() {
        let mut house = House::new("House");
        let room1 = Room::new("Living Room");
        let room2 = Room::new("Bedroom");

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().len(), 2);

        house.remove_room("Living Room");

        assert_eq!(house.get_rooms().len(), 1);
        assert!(house.get_room("Living Room").is_none());
        assert!(house.get_room("Bedroom").is_some());
    }
}
