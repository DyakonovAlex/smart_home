use crate::devices::errors::DeviceError;

pub trait DeviceInfoProvider {
    fn get_device_info(
        &self,
        room_name: String,
        device_name: String,
    ) -> Result<String, DeviceError>;
}
