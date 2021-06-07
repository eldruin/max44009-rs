use crate::{
    BitFlags, ConfigurationMode, CurrentDivisionRatio, Error, IntegrationTime, Max44009,
    MeasurementMode, Register,
};
use embedded_hal::blocking::i2c;

impl<I2C, E> Max44009<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Enable interrupt.
    ///
    /// The INT pin will be pulled low if the interrupt condition is triggered.
    pub fn enable_interrupt(&mut self) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::INT_ENABLE, 1])
            .map_err(Error::I2C)
    }

    /// Disable interrupt.
    pub fn disable_interrupt(&mut self) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::INT_ENABLE, 0])
            .map_err(Error::I2C)
    }

    /// Set the measurement mode.
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            MeasurementMode::OnceEvery800ms => self.write_config(config & !BitFlags::CONTINUOUS),
            MeasurementMode::Continuous => self.write_config(config | BitFlags::CONTINUOUS),
        }
    }

    /// Set configuration mode.
    pub fn set_configuration_mode(&mut self, mode: ConfigurationMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            ConfigurationMode::Automatic => self.write_config(config & !BitFlags::MANUAL),
            ConfigurationMode::Manual => self.write_config(config | BitFlags::MANUAL),
        }
    }

    /// Set integration time. (Only in manual configuration mode).
    pub fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        self.assert_is_in_manual_mode()?;
        let config = self.config & 0b1111_1000;
        match it {
            IntegrationTime::_800ms => self.write_config(config),
            IntegrationTime::_400ms => self.write_config(config | 0x01),
            IntegrationTime::_200ms => self.write_config(config | 0x02),
            IntegrationTime::_100ms => self.write_config(config | 0x03),
            IntegrationTime::_50ms => self.write_config(config | 0x04),
            IntegrationTime::_25ms => self.write_config(config | 0x05),
            IntegrationTime::_12_5ms => self.write_config(config | 0x06),
            IntegrationTime::_6_25ms => self.write_config(config | 0x07),
        }
    }

    /// Set current division ratio. (Only in manual configuration mode).
    pub fn set_current_division_ratio(
        &mut self,
        cdr: CurrentDivisionRatio,
    ) -> Result<(), Error<E>> {
        self.assert_is_in_manual_mode()?;
        let config = self.config;
        match cdr {
            CurrentDivisionRatio::One => self.write_config(config & !BitFlags::CDR),
            CurrentDivisionRatio::OneEighth => self.write_config(config | BitFlags::CDR),
        }
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIGURATION, config])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }

    fn assert_is_in_manual_mode(&self) -> Result<(), Error<E>> {
        if (self.config & BitFlags::MANUAL) == 0 {
            return Err(Error::OperationNotAvailable);
        }
        Ok(())
    }
}
