use crate::devices::device_info_provider::DeviceInfoProvider;
use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;

pub struct DeviceInfo<'a, 'b> {
    sockets: Vec<&'a Socket>,
    thermometers: Vec<&'b Thermometer>,
}

impl<'a, 'b> DeviceInfo<'a, 'b> {
    pub fn new(sockets: Vec<&'a Socket>, thermometers: Vec<&'b Thermometer>) -> Self {
        Self {
            sockets,
            thermometers,
        }
    }
}

impl DeviceInfoProvider for DeviceInfo<'_, '_> {
    fn get_device_info(&self, room_name: String, device_name: String) -> String {
        for socket in &self.sockets {
            if device_name == *socket.get_name() {
                return socket.description();
            }
        }
        for thermometer in &self.thermometers {
            if device_name == *thermometer.get_name() {
                return thermometer.description();
            }
        }
        format!(
            "Устройство {} в комнате {} не найдено.",
            device_name, room_name
        )
    }
}
