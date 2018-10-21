extern crate max44009;
extern crate embedded_hal_mock as hal;
use max44009::{ Max44009, SlaveAddr };

const DEVICE_BASE_ADDRESS: u8 = 0b100_1010;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const INT_STATUS        : u8 = 0x00;
    pub const INT_ENABLE        : u8 = 0x01;
    pub const CONFIGURATION     : u8 = 0x02;
    pub const LUX_HIGH          : u8 = 0x03;
}

pub fn setup<'a>(data: &'a[u8]) -> Max44009<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Max44009::new(dev, SlaveAddr::default())
}

pub fn check_sent_data(sensor: Max44009<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}
