use embedded_hal_mock::i2c::Transaction as I2cTrans;
use max44009::{
    ConfigurationMode as CM, CurrentDivisionRatio as CDR, Error, IntegrationTime as IT,
    MeasurementMode as MM,
};
mod common;
use crate::common::{destroy, new, Register, DEV_BASE_ADDR};

fn assert_operation_not_available_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::OperationNotAvailable) => (),
        _ => panic!("Did not return Error::OperationNotAvailable."),
    }
}

#[test]
fn can_enable_interrupt() {
    let mut dev = new(&[I2cTrans::write(
        DEV_BASE_ADDR,
        vec![Register::INT_ENABLE, 1],
    )]);
    dev.enable_interrupt().unwrap();
    destroy(dev);
}

#[test]
fn can_disable_interrupt() {
    let mut dev = new(&[I2cTrans::write(
        DEV_BASE_ADDR,
        vec![Register::INT_ENABLE, 0],
    )]);
    dev.disable_interrupt().unwrap();
    destroy(dev);
}

macro_rules! set_mode_test {
    ($test_name:ident, $method:ident, $enum:ident::$variant:ident, $expected:expr) => {
        #[test]
        fn $test_name() {
            let mut dev = new(&[I2cTrans::write(
                DEV_BASE_ADDR,
                vec![Register::CONFIGURATION, $expected],
            )]);
            dev.$method($enum::$variant).unwrap();
            destroy(dev);
        }
    };
}

set_mode_test!(
    can_set_measurement_mode_once_every_800ms,
    set_measurement_mode,
    MM::OnceEvery800ms,
    0
);
set_mode_test!(
    can_set_measurement_mode_continuous,
    set_measurement_mode,
    MM::Continuous,
    0b1000_0000
);

set_mode_test!(
    can_set_automatic_mode,
    set_configuration_mode,
    CM::Automatic,
    0b0000_0000
);
set_mode_test!(
    can_set_manual_mode,
    set_configuration_mode,
    CM::Manual,
    0b0100_0000
);

#[test]
fn cannot_set_current_division_ratio_in_automatic_mode() {
    let mut dev = new(&[]);
    assert_operation_not_available_error(dev.set_current_division_ratio(CDR::One));
    destroy(dev);
}

#[test]
fn cannot_set_integration_time_in_automatic_mode() {
    let mut dev = new(&[]);
    assert_operation_not_available_error(dev.set_integration_time(IT::_100ms));
    destroy(dev);
}

macro_rules! set_param_test {
    ($test_name:ident, $method:ident, $enum:ident::$variant:ident, $expected:expr) => {
        #[test]
        fn $test_name() {
            let mut dev = new(&[
                I2cTrans::write(DEV_BASE_ADDR, vec![Register::CONFIGURATION, 0b0100_0000]),
                I2cTrans::write(
                    DEV_BASE_ADDR,
                    vec![Register::CONFIGURATION, 0b0100_0000 | $expected],
                ),
            ]);
            dev.set_configuration_mode(CM::Manual).unwrap();
            dev.$method($enum::$variant).unwrap();
            destroy(dev);
        }
    };
}

set_param_test!(can_set_cdr_one, set_current_division_ratio, CDR::One, 0);
set_param_test!(
    can_set_cdr_one_eighth,
    set_current_division_ratio,
    CDR::OneEighth,
    8
);

set_param_test!(can_set_it_800ms, set_integration_time, IT::_800ms, 0);
set_param_test!(can_set_it_400ms, set_integration_time, IT::_400ms, 1);
set_param_test!(can_set_it_200ms, set_integration_time, IT::_200ms, 2);
set_param_test!(can_set_it_100ms, set_integration_time, IT::_100ms, 3);
set_param_test!(can_set_it_50ms, set_integration_time, IT::_50ms, 4);
set_param_test!(can_set_it_25ms, set_integration_time, IT::_25ms, 5);
set_param_test!(can_set_it_12_5ms, set_integration_time, IT::_12_5ms, 6);
set_param_test!(can_set_it_6_25ms, set_integration_time, IT::_6_25ms, 7);
