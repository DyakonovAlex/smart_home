use smart_home::devices::error::{SOCKET_CREATION_ERROR, THERMOMETER_CREATION_ERROR};
use smart_home::devices::{socket::Socket, thermometer::Thermometer};
use smart_home::models::{house::House, room::Room};
use smart_home::services::device_info::DeviceInfo;

fn main() {
    let mut house = House::new("Домик у реки");

    // Создаем первую комнату - Гостиная
    let mut living_room = Room::new("Гостиная");
    let socket1 = Socket::new("Розетка 1", 3520).expect(SOCKET_CREATION_ERROR);
    let socket2 = Socket::new("Розетка 2", 2200).expect(SOCKET_CREATION_ERROR);
    let thermometer1 = Thermometer::new("Термометр 1", 22.5).expect(THERMOMETER_CREATION_ERROR);
    let thermometer2 = Thermometer::new("Термометр 2", 23.0).expect(THERMOMETER_CREATION_ERROR);

    living_room.add_device(socket1.get_name());
    living_room.add_device(socket2.get_name());
    living_room.add_device(thermometer1.get_name());
    living_room.add_device(thermometer2.get_name());

    house.add_room(living_room);

    // Создаем вторую комнату - Спальня
    let mut bedroom = Room::new("Спальня");
    let socket3 = Socket::new("Розетка 3", 2200).expect(SOCKET_CREATION_ERROR);
    let thermometer3 = Thermometer::new("Термометр 3", 21.0).expect(THERMOMETER_CREATION_ERROR);

    bedroom.add_device(socket3.get_name());
    bedroom.add_device(thermometer3.get_name());

    house.add_room(bedroom);

    // Создаем третью комнату - Кухня
    let mut kitchen = Room::new("Кухня");
    let socket4 = Socket::new("Розетка 4", 3500).expect(SOCKET_CREATION_ERROR);
    let thermometer4 = Thermometer::new("Термометр 4", 24.0).expect(THERMOMETER_CREATION_ERROR);

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
