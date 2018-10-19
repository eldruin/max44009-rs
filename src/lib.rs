//! This is a platform agnostic Rust driver for the MAX44009 ambient
//! light sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read lux measurement.
//! - Set the measurement mode.
//!
//! ## The device
//! The MAX44009 ambient light sensor features an I2C digital output
//! that is ideal for a number of portable applications such as
//! smartphones, notebooks, and industrial sensors.
//! At less than 1μA operating current, it is the lowest power ambient
//! light sensor in the industry and features an ultra-wide 22-bit
//! dynamic range from 0.045 lux to 188,000 lux.
//! Low-light operation allows easy operation in dark-glass
//! applications.
//! The on-chip photodiode's spectral response is optimized to mimic
//! the human eye's perception of ambient light and incorporates
//! IR and UV blocking capability. The adaptive gain block
//! automatically selects the correct lux range to optimize the
//! counts/lux.
//!
//! Datasheet:
//! - [MAX44009](https://datasheets.maximintegrated.com/en/ds/MAX44009.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Read lux
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate max44009;
//!
//! use hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Max44009::new(dev, address);
//! let lux = sensor.read_lux().unwrap();
//! # }
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate max44009;
//!
//! use hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let a0 = true;
//! let address = SlaveAddr::Alternative(a0);
//! let mut sensor = Max44009::new(dev, address);
//! # }
//! ```
//!
//! ### Set the measurement mode to continuous
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate max44009;
//!
//! use hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr, MeasurementMode };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.set_measurement_mode(MeasurementMode::Continuous).unwrap();
//! # }
//! ```
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error.
    I2C(E),
}

/// Measurement mode
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementMode {
    /// Once every 800ms mode (default).
    ///
    /// Measures lux intensity every 800ms regardless of the integration time.
    /// Sensor operates on lowest possible supply current.
    OnceEvery800ms,
    /// Continuous mode.
    ///
    /// Continuously measures lux intensity. As soon as a reading finishes,
    /// the next one begins. The actual cadence depends on the integration
    /// time selected.
    Continuous
}

/// Possible slave addresses
#[derive(Debug, Clone, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for A0
    Alternative(bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a0) => default | a0 as u8
        }
    }
}

const DEVICE_BASE_ADDRESS: u8 = 0b100_1010;

struct Register;

impl Register {
    const CONFIGURATION     : u8 = 0x02;
    const LUX_HIGH          : u8 = 0x03;
}

struct BitFlags;

impl BitFlags {
    const CONTINUOUS : u8 = 0b1000_0000;
}

/// MAX44009 ambient light sensor driver.
#[derive(Debug, Default)]
pub struct Max44009<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: u8,
}

impl<I2C, E> Max44009<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the Max44009 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Max44009 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: 0
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Set the measurement mode.
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            MeasurementMode::OnceEvery800ms => self.write_config(config & !BitFlags::CONTINUOUS),
            MeasurementMode::Continuous     => self.write_config(config |  BitFlags::CONTINUOUS)
        }
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIGURATION, config])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

impl<I2C, E> Max44009<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Read the lux intensity.
    pub fn read_lux(&mut self) -> Result<f32, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::LUX_HIGH], &mut data)
            .map_err(Error::I2C)
            .and(Ok(convert_to_lux(data[0], data[1])))
    }
}

fn convert_to_lux(msb: u8, lsb: u8) -> f32 {
    let mantissa = (msb & 0x0F) << 4 | (lsb & 0x0F);
    let exp = (msb & 0xF0) >> 4;
    (((1 as u32) << exp) * mantissa as u32) as f32 * 0.045
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_1010, SlaveAddr::Alternative(false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1011, SlaveAddr::Alternative(true ).addr(DEVICE_BASE_ADDRESS));
    }

    fn assert_near(a: f32, b: f32, epsilon: f32) {
        assert!((a - b).abs() < epsilon);
    }

    #[test]
    fn can_convert_to_lux() {
        assert_eq!(      0.045, convert_to_lux(0b0000_0000, 0b0000_0001));
        assert_eq!(      0.72,  convert_to_lux(0b0000_0001, 0b0000_0000));
        assert_near(     1.53,  convert_to_lux(0b0001_0001, 0b0000_0001), 0.001);
        assert_near(188006.0,   convert_to_lux(0b1110_1111, 0b0000_1111), 0.5);
        assert_near(187269.0,   convert_to_lux(0b1110_1111, 0b0000_1110), 0.5);
        assert_near(176947.0,   convert_to_lux(0b1110_1111, 0b0000_0000), 0.5);
        assert_near(165151.0,   convert_to_lux(0b1110_1110, 0b0000_0000), 0.5);
    }
}