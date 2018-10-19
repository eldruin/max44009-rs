extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate max44009;

use linux_embedded_hal::I2cdev;
use max44009::{ Max44009, SlaveAddr };

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Max44009::new(dev, SlaveAddr::default());
    let lux = sensor.read_lux().unwrap();
    println!("lux: {}", lux);
}
