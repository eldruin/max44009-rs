extern crate max44009;
extern crate embedded_hal_mock as hal;
use max44009::{ Max44009, SlaveAddr, MeasurementMode };

const DEVICE_BASE_ADDRESS: u8 = 0b100_1010;

struct Register;

impl Register {
    const CONFIGURATION     : u8 = 0x02;
    const LUX_HIGH          : u8 = 0x03;
}

fn setup<'a>(data: &'a[u8]) -> Max44009<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Max44009::new(dev, SlaveAddr::default())
}

fn check_sent_data(sensor: Max44009<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}

#[test]
fn can_set_measurement_mode_once_every_800ms() {
    let mut dev = setup(&[0]);
    dev.set_measurement_mode(MeasurementMode::OnceEvery800ms).unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 0]);
}

#[test]
fn can_set_measurement_mode_continuous() {
    let mut dev = setup(&[0]);
    dev.set_measurement_mode(MeasurementMode::Continuous).unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 0b1000_0000]);
}

#[test]
fn can_read_lux() {
    let mut dev = setup(&[0, 1]);
    let lux = dev.read_lux().unwrap();
    assert_eq!(0.045, lux);
    check_sent_data(dev, &[Register::LUX_HIGH]);
}
