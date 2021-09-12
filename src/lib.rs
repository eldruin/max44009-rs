//! This is a platform agnostic Rust driver for the MAX44009 and MAX44007 ambient
//! light sensors (ALS), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read lux measurement.
//! - Set the measurement mode.
//! - Set the configuration mode.
//! - Set the integration time.
//! - Set the current division ratio.
//! - Read the integration time.
//! - Read the current division ratio.
//! - Enable/disable interrupt generation.
//! - Check if an interrupt has happened.
//!
//! ## The devices
//! The MAX44009 and MAX44007 ambient light sensors feature an I2C digital output
//! that is ideal for a number of portable applications such as
//! smartphones, notebooks, and industrial sensors.
//! At less than 1μA operating current, the MAX44009 is the lowest power ambient
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
//! Datasheets: [MAX44007], [MAX44009]
//!
//! [MAX44007]: https://datasheets.maximintegrated.com/en/ds/MAX44007.pdf
//! [MAX44009]: https://datasheets.maximintegrated.com/en/ds/MAX44009.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read lux
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Max44009::new(dev, address);
//! let lux = sensor.read_lux().unwrap();
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let a0 = true;
//! let address = SlaveAddr::Alternative(a0);
//! let mut sensor = Max44009::new(dev, address);
//! ```
//!
//! ### Enable interruptions and see if one has happened
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.enable_interrupt().unwrap();
//! if sensor.has_interrupt_happened().unwrap() {
//!     println!("Interrupt happened.");
//! }
//! ```
//!
//! ### Set the measurement mode to continuous
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr, MeasurementMode };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.set_measurement_mode(MeasurementMode::Continuous).unwrap();
//! ```
//!
//! ### Read the parameters selected in automatic mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! let it  = sensor.read_integration_time().unwrap();
//! let cdr = sensor.read_current_division_ratio().unwrap();
//! ```
//!
//! ### Configure manually
//! - Set configuration mode to manual.
//! - Set current division ratio to 1/8.
//! - Set integration time to 100ms.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr, ConfigurationMode,
//!                 CurrentDivisionRatio, IntegrationTime };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.set_configuration_mode(ConfigurationMode::Manual).unwrap();
//! sensor.set_current_division_ratio(CurrentDivisionRatio::OneEighth).unwrap();
//! sensor.set_integration_time(IntegrationTime::_100ms).unwrap();
//! ```
//!

#![doc(html_root_url = "https://docs.rs/max44009/0.1.0")]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal::blocking::i2c;

const DEVICE_BASE_ADDRESS: u8 = 0b100_1010;

struct Register;

impl Register {
    const INT_STATUS: u8 = 0x00;
    const INT_ENABLE: u8 = 0x01;
    const CONFIGURATION: u8 = 0x02;
    const LUX_HIGH: u8 = 0x03;
}

struct BitFlags;

impl BitFlags {
    const CONTINUOUS: u8 = 0b1000_0000;
    const MANUAL: u8 = 0b0100_0000;
    const CDR: u8 = 0b0000_1000;
}

/// MAX44009 ambient light sensor driver.
#[derive(Debug)]
pub struct Max44009<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: u8,
}

mod configuration;
mod reading;
mod types;
pub use crate::types::{
    ConfigurationMode, CurrentDivisionRatio, Error, IntegrationTime, MeasurementMode, SlaveAddr,
};

impl<I2C, E> Max44009<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the Max44009 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Max44009 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: 0,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
