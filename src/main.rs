mod devices;
mod models;
mod services;

use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;
use crate::models::house::House;
use crate::models::room::Room;
use crate::services::device_info::DeviceInfo;

fn main() {
    let mut house = House::new("Домик у реки");

    // Создаем первую комнату - Гостиная
    let mut living_room = Room::new("Гостиная");
    let socket1 = Socket::new("Розетка 1", 100);
    let socket2 = Socket::new("Розетка 2", 150);
    let thermometer1 = Thermometer::new("Термометр 1", 22.5);
    let thermometer2 = Thermometer::new("Термометр 2", 23.0);

    living_room.add_device(socket1.get_name());
    living_room.add_device(socket2.get_name());
    living_room.add_device(thermometer1.get_name());
    living_room.add_device(thermometer2.get_name());

    house.add_room(living_room);

    // Создаем вторую комнату - Спальня
    let mut bedroom = Room::new("Спальня");
    let socket3 = Socket::new("Розетка 3", 120);
    let thermometer3 = Thermometer::new("Термометр 3", 21.0);

    bedroom.add_device(socket3.get_name());
    bedroom.add_device(thermometer3.get_name());

    house.add_room(bedroom);

    // Создаем третью комнату - Кухня
    let mut kitchen = Room::new("Кухня");
    let socket4 = Socket::new("Розетка 4", 130);
    let thermometer4 = Thermometer::new("Термометр 4", 24.0);

    kitchen.add_device(socket4.get_name());
    kitchen.add_device(thermometer4.get_name());

    house.add_room(kitchen);

    // Создаем информацию об устройствах
    let device_info = DeviceInfo::new(
        vec![&socket1, &socket2, &socket3, &socket4],
        vec![&thermometer1, &thermometer2, &thermometer3, &thermometer4],
    );

    // Получаем список комнат и устройств
    let rooms = house.get_rooms();
    for (room_name, room) in rooms {
        println!("Комната: {}", room_name);
        for device in room.get_devices() {
            println!("\t- Устройство: {}", device);
        }
    }
    println!();

    // Генерируем отчет о состоянии дома
    let report = house.create_report(&device_info);
    println!("{}", report);
}
