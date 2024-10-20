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

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.get_name().to_string(), room);
    }

    pub fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    pub fn create_report(&self, info: &impl DeviceInfoProvider) -> String {
        let mut report = format!("Отчет по дому {}\n", self.name);
        for (room_name, room) in &self.rooms {
            report.push_str(&format!("\tКомната: {}\n", room_name));
            for device_name in room.get_devices() {
                report.push_str(&format!(
                    "\t\t- {}\n",
                    &info.get_device_info(room_name.clone(), device_name.clone())
                ));
            }
            report.push('\n');
        }
        report
    }
}
