use crate::{BitFlags, CurrentDivisionRatio, Error, IntegrationTime, Max44009, Register};
use embedded_hal::blocking::i2c;

impl<I2C, E> Max44009<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Reads whether an interrupt has happened.
    pub fn has_interrupt_happened(&mut self) -> Result<bool, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[Register::INT_STATUS], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0] != 0))
    }

    /// Read the lux intensity.
    pub fn read_lux(&mut self) -> Result<f32, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::LUX_HIGH], &mut data)
            .map_err(Error::I2C)
            .and(Ok(convert_to_lux(data[0], data[1])))
    }

    /// Read the integration time.
    pub fn read_integration_time(&mut self) -> Result<IntegrationTime, Error<E>> {
        let mut config = [0];
        self.i2c
            .write_read(self.address, &[Register::CONFIGURATION], &mut config)
            .map_err(Error::I2C)?;
        match config[0] & 0b0000_0111 {
            0 => Ok(IntegrationTime::_800ms),
            1 => Ok(IntegrationTime::_400ms),
            2 => Ok(IntegrationTime::_200ms),
            3 => Ok(IntegrationTime::_100ms),
            4 => Ok(IntegrationTime::_50ms),
            5 => Ok(IntegrationTime::_25ms),
            6 => Ok(IntegrationTime::_12_5ms),
            7 => Ok(IntegrationTime::_6_25ms),
            _ => panic!("Programming error!"),
        }
    }

    /// Read the current division ratio.
    pub fn read_current_division_ratio(&mut self) -> Result<CurrentDivisionRatio, Error<E>> {
        let mut config = [0];
        self.i2c
            .write_read(self.address, &[Register::CONFIGURATION], &mut config)
            .map_err(Error::I2C)?;
        if (config[0] & BitFlags::CDR) == 0 {
            Ok(CurrentDivisionRatio::One)
        } else {
            Ok(CurrentDivisionRatio::OneEighth)
        }
    }
}

fn convert_to_lux(msb: u8, lsb: u8) -> f32 {
    let mantissa = (msb & 0x0F) << 4 | (lsb & 0x0F);
    let exp = (msb & 0xF0) >> 4;
    (((1_u32) << exp) * u32::from(mantissa)) as f32 * 0.045
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(a: f32, b: f32, epsilon: f32) {
        assert!((a - b).abs() < epsilon);
    }

    #[test]
    fn can_convert_to_lux() {
        assert_near(0.045, convert_to_lux(0b0000_0000, 0b0000_0001), 0.001);
        assert_near(0.72, convert_to_lux(0b0000_0001, 0b0000_0000), 0.001);
        assert_near(1.53, convert_to_lux(0b0001_0001, 0b0000_0001), 0.001);
        assert_near(188_006.0, convert_to_lux(0b1110_1111, 0b0000_1111), 0.5);
        assert_near(187_269.0, convert_to_lux(0b1110_1111, 0b0000_1110), 0.5);
        assert_near(176_947.0, convert_to_lux(0b1110_1111, 0b0000_0000), 0.5);
        assert_near(165_151.0, convert_to_lux(0b1110_1110, 0b0000_0000), 0.5);
    }
}
