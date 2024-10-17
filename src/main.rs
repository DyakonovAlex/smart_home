mod devices;

fn main() {
    println!("Умный дом запущен!");

    let mut smart_socket = devices::socket::Socket::new(1, "Кухня 1", 100);
    let mut thermometer = devices::thermometer::Thermometer::new(1, "Кухня 1", 25.0);

    println!("{}", smart_socket.description());
    println!("{}", thermometer.description());

    smart_socket.turn_on();
    println!("{}", smart_socket.description());

    thermometer.set_temp(26.0);
    println!("{}", thermometer.description());
}
