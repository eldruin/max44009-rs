extern crate max44009;
extern crate embedded_hal_mock as hal;
use max44009::{ MeasurementMode, ConfigurationMode, IntegrationTime,
                CurrentDivisionRatio, Error };

mod common;
use common::{ setup, check_sent_data, Register };

fn assert_operation_not_available_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::OperationNotAvailable) => (),
        _ => panic!("Did not return Error::OperationNotAvailable."),
    }
}

#[test]
fn can_enable_interrupt() {
    let mut dev = setup(&[0]);
    dev.enable_interrupt().unwrap();
    check_sent_data(dev, &[Register::INT_ENABLE, 1]);
}

#[test]
fn can_disable_interrupt() {
    let mut dev = setup(&[0]);
    dev.disable_interrupt().unwrap();
    check_sent_data(dev, &[Register::INT_ENABLE, 0]);
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
fn can_set_automatic_mode() {
    let mut dev = setup(&[0]);
    dev.set_configuration_mode(ConfigurationMode::Automatic).unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 0b0000_0000]);
}

#[test]
fn can_set_manual_mode() {
    let mut dev = setup(&[0]);
    dev.set_configuration_mode(ConfigurationMode::Manual).unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 0b0100_0000]);
}

#[test]
fn cannot_set_current_division_ratio_in_automatic_mode() {
    let mut dev = setup(&[0]);
    assert_operation_not_available_error(dev.set_current_division_ratio(CurrentDivisionRatio::One));
}

#[test]
fn cannot_set_integration_time_in_automatic_mode() {
    let mut dev = setup(&[0]);
    assert_operation_not_available_error(dev.set_integration_time(IntegrationTime::_100ms));
}

macro_rules! set_param_test {
    ($test_name:ident, $method:ident, $enum:ident::$variant:ident, $expected:expr) => {
        #[test]
        fn $test_name() {
            let mut dev = setup(&[0]);
            dev.set_configuration_mode(ConfigurationMode::Manual).unwrap();
            dev.$method($enum::$variant).unwrap();
            check_sent_data(dev, &[Register::CONFIGURATION, 0b0100_0000 | $expected]);
        }
    };
}

set_param_test!(can_set_cdr_one,        set_current_division_ratio, CurrentDivisionRatio::One,        0);
set_param_test!(can_set_cdr_one_eighth, set_current_division_ratio, CurrentDivisionRatio::OneEighth,  8);

set_param_test!(can_set_it_800ms,  set_integration_time, IntegrationTime::_800ms,  0);
set_param_test!(can_set_it_400ms,  set_integration_time, IntegrationTime::_400ms,  1);
set_param_test!(can_set_it_200ms,  set_integration_time, IntegrationTime::_200ms,  2);
set_param_test!(can_set_it_100ms,  set_integration_time, IntegrationTime::_100ms,  3);
set_param_test!(can_set_it_50ms,   set_integration_time, IntegrationTime::_50ms,   4);
set_param_test!(can_set_it_25ms,   set_integration_time, IntegrationTime::_25ms,   5);
set_param_test!(can_set_it_12_5ms, set_integration_time, IntegrationTime::_12_5ms, 6);
set_param_test!(can_set_it_6_25ms, set_integration_time, IntegrationTime::_6_25ms, 7);
