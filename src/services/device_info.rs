use crate::devices::{
    device_info_provider::DeviceInfoProvider, socket::Socket, thermometer::Thermometer,
};
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
            if device_name == socket.get_name() {
                return socket.description();
            }
        }
        for thermometer in &self.thermometers {
            if device_name == thermometer.get_name() {
                return thermometer.description();
            }
        }
        format!(
            "Устройство {} в комнате {} не найдено.",
            device_name, room_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_device_info_socket() {
        let socket1 = Socket::new("Living Room Socket", 230);
        let socket2 = Socket::new("Kitchen Socket", 220);
        let sockets = vec![&socket1, &socket2];

        let thermometer1 = Thermometer::new("Living Room Thermometer", 23.0);
        let thermometers = vec![&thermometer1];

        let device_info = DeviceInfo::new(sockets, thermometers);

        let info = device_info
            .get_device_info("Living Room".to_string(), "Living Room Socket".to_string());

        assert_eq!(
            info,
            "Умная розетка: Living Room Socket, Потребляемая мощность: 230, Вкл: false"
        );
    }

    #[test]
    fn test_get_device_info_thermometer() {
        let socket1 = Socket::new("Living Room Socket", 220);
        let sockets = vec![&socket1];

        let thermometer1 = Thermometer::new("Living Room Thermometer", 23.0);
        let thermometer2 = Thermometer::new("Kitchen Thermometer", 25.0);
        let thermometers = vec![&thermometer1, &thermometer2];

        let device_info = DeviceInfo::new(sockets, thermometers);

        let info = device_info.get_device_info(
            "Living Room".to_string(),
            "Living Room Thermometer".to_string(),
        );

        assert_eq!(info, "Термометр: Living Room Thermometer, Температура: 23");
    }

    #[test]
    fn test_get_device_info_not_found() {
        let socket1 = Socket::new("Living Room Socket", 220);
        let sockets = vec![&socket1];

        let thermometer1 = Thermometer::new("Living Room Thermometer", 23.0);
        let thermometers = vec![&thermometer1];

        let device_info = DeviceInfo::new(sockets, thermometers);

        let info =
            device_info.get_device_info("Living Room".to_string(), "Unknown Device".to_string());
        assert_eq!(
            info,
            "Устройство Unknown Device в комнате Living Room не найдено."
        );
    }
}
