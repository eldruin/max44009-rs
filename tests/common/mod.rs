extern crate embedded_hal_mock as hal;
extern crate max44009;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use max44009::{Max44009, SlaveAddr};

pub const DEV_BASE_ADDR: u8 = 0b100_1010;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const INT_STATUS: u8 = 0x00;
    pub const INT_ENABLE: u8 = 0x01;
    pub const CONFIGURATION: u8 = 0x02;
    pub const LUX_HIGH: u8 = 0x03;
}

pub fn new(transactions: &[I2cTrans]) -> Max44009<I2cMock> {
    Max44009::new(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy(dev: Max44009<I2cMock>) {
    dev.destroy().done();
}
