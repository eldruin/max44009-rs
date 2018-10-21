//! This is a platform agnostic Rust driver for the MAX44009 ambient
//! light sensor, based on the [`embedded-hal`] traits.
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
//! ### Enable interruptions and see if one has happened
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
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.enable_interrupt().unwrap();
//! if sensor.has_interrupt_happened().unwrap() {
//!     println!("Interrupt happened.")
//! }
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
//! ### Read the parameters selected in automatic mode
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
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! let it  = sensor.read_integration_time().unwrap();
//! let cdr = sensor.read_current_division_ratio().unwrap();
//! # }
//! ```
//!
//! ### Configure manually
//! - Set configuration mode to manual.
//! - Set current division ratio to 1/8.
//! - Set integration time to 100ms.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate max44009;
//!
//! use hal::I2cdev;
//! use max44009::{ Max44009, SlaveAddr, ConfigurationMode,
//!                 CurrentDivisionRatio, IntegrationTime };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Max44009::new(dev, SlaveAddr::default());
//! sensor.set_configuration_mode(ConfigurationMode::Manual).unwrap();
//! sensor.set_current_division_ratio(CurrentDivisionRatio::OneEighth).unwrap();
//! sensor.set_integration_time(IntegrationTime::_100ms).unwrap();
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
    /// A manual-configuration-mode-only was attempted while in automatic
    /// configuration mode.
    OperationNotAvailable
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

/// Configuration mode
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigurationMode {
    /// Automatic mode (default).
    ///
    /// On-chip algorithm selects the integration time (100ms - 800ms) and
    /// the current division ratio
    Automatic,
    /// Manual mode.
    ///
    /// The user can select the integration time and the current division
    /// ratio manually.
    Manual
}

/// Integration time
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationTime {
    /// 6.25ms. (Only in manual mode)
    _6_25ms,
    /// 12.5ms. (Only in manual mode)
    _12_5ms,
    /// 25ms. (Only in manual mode)
    _25ms,
    /// 50ms. (Only in manual mode)
    _50ms,
    /// 100ms. (Preferred mode for high-brightness applications)
    _100ms,
    /// 200ms
    _200ms,
    /// 400ms
    _400ms,
    /// 800ms. (Preferred mode for boosting low-light sensitivity)
    _800ms
}

/// Current division ratio
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentDivisionRatio {
    /// No current division (default).
    ///
    /// All the photodiode current goes to the ADC.
    One,
    /// 1/8 current division ratio.
    ///
    /// Only 1/8 of the photodiode current goes to the ADC. This mode is used in
    /// high-brightness situations.
    OneEighth
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
    const INT_STATUS        : u8 = 0x00;
    const INT_ENABLE        : u8 = 0x01;
    const CONFIGURATION     : u8 = 0x02;
    const LUX_HIGH          : u8 = 0x03;
}

struct BitFlags;

impl BitFlags {
    const CONTINUOUS : u8 = 0b1000_0000;
    const MANUAL     : u8 = 0b0100_0000;
    const CDR        : u8 = 0b0000_1000;
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

mod configuration;
mod reading;

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
}