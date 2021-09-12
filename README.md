# Rust MAX44009/MAX44007 Ambient Light Sensor Driver

[![crates.io](https://img.shields.io/crates/v/max44009.svg)](https://crates.io/crates/max44009)
[![Docs](https://docs.rs/max44009/badge.svg)](https://docs.rs/max44009)
[![Build Status](https://github.com/eldruin/max44009-rs/workflows/Build/badge.svg)](https://github.com/eldruin/max44009-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/max44009-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/max44009-rs?branch=master)

This is a platform agnostic Rust driver for the MAX44009 and MAX44007 ambient
light sensors, using the [`embedded-hal`] traits.

This driver allows you to:
- Read lux measurement.
- Set the measurement mode.
- Set the configuration mode.
- Set the integration time.
- Set the current division ratio.
- Read the integration time.
- Read the current division ratio.
- Enable/disable interrupt generation.
- Check if an interrupt has happened.

## The devices
The MAX44009 and MAX44007 ambient light sensor feature an I2C digital output
that is ideal for a number of portable applications such as
smartphones, notebooks, and industrial sensors.
At less than 1μA operating current, the MAX44009 is the lowest power ambient
light sensor in the industry and features an ultra-wide 22-bit
dynamic range from 0.045 lux to 188,000 lux.
Low-light operation allows easy operation in dark-glass
applications.
The on-chip photodiode's spectral response is optimized to mimic
the human eye's perception of ambient light and incorporates
IR and UV blocking capability. The adaptive gain block
automatically selects the correct lux range to optimize the
counts/lux.

Datasheets: [MAX44007], [MAX44009]

[MAX44007]: https://datasheets.maximintegrated.com/en/ds/MAX44007.pdf
[MAX44009]: https://datasheets.maximintegrated.com/en/ds/MAX44009.pdf

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use max44009::{Max44009, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Max44009::new(dev, SlaveAddr::default());
    let lux = sensor.read_lux().unwrap();
    println!("lux: {}", lux);
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/max44009-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
