extern crate max44009;
extern crate embedded_hal_mock as hal;
use max44009::{ Max44009, SlaveAddr, MeasurementMode, ConfigurationMode,
                IntegrationTime, CurrentDivisionRatio, Error };

const DEVICE_BASE_ADDRESS: u8 = 0b100_1010;

struct Register;

impl Register {
    const INT_STATUS        : u8 = 0x00;
    const INT_ENABLE        : u8 = 0x01;
    const CONFIGURATION     : u8 = 0x02;
    const LUX_HIGH          : u8 = 0x03;
    //const UPPER_THRESH_HIGH : u8 = 0x05;
    //const LOWER_THRESH_HIGH : u8 = 0x06;
    //const THRESH_TIMER      : u8 = 0x07;
}

fn setup<'a>(data: &'a[u8]) -> Max44009<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Max44009::new(dev, SlaveAddr::default())
}

fn check_sent_data(sensor: Max44009<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}

fn assert_operation_not_available_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::OperationNotAvailable) => (),
        _ => panic!("Did not return Error::OperationNotAvailable."),
    }
}

#[test]
fn can_read_interrupt_did_not_happened() {
    let mut dev = setup(&[0]);
    let interrupt_happened = dev.has_interrupt_happened().unwrap();
    assert!(!interrupt_happened);
    check_sent_data(dev, &[Register::INT_STATUS]);
}

#[test]
fn can_read_interrupt_happened() {
    let mut dev = setup(&[1]);
    let interrupt_happened = dev.has_interrupt_happened().unwrap();
    assert!(interrupt_happened);
    check_sent_data(dev, &[Register::INT_STATUS]);
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
fn can_read_lux() {
    let mut dev = setup(&[0, 1]);
    let lux = dev.read_lux().unwrap();
    assert_eq!(0.045, lux);
    check_sent_data(dev, &[Register::LUX_HIGH]);
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

macro_rules! read_param_test {
    ($test_name:ident, $method:ident, $input_data:expr, $enum:ident::$expected_variant:ident) => {
        #[test]
        fn $test_name() {
            let mut dev = setup(&[$input_data]);
            let it = dev.$method().unwrap();
            assert_eq!($enum::$expected_variant, it);
            check_sent_data(dev, &[Register::CONFIGURATION]);
        }
    }
}

read_param_test!(can_read_cdr_one,        read_current_division_ratio, 0, CurrentDivisionRatio::One      );
read_param_test!(can_read_cdr_one_eighth, read_current_division_ratio, 8, CurrentDivisionRatio::OneEighth);

read_param_test!(can_read_it_800ms,  read_integration_time, 0, IntegrationTime::_800ms);
read_param_test!(can_read_it_400ms,  read_integration_time, 1, IntegrationTime::_400ms);
read_param_test!(can_read_it_200ms,  read_integration_time, 2, IntegrationTime::_200ms);
read_param_test!(can_read_it_100ms,  read_integration_time, 3, IntegrationTime::_100ms);
read_param_test!(can_read_it_50ms,   read_integration_time, 4, IntegrationTime::_50ms);
read_param_test!(can_read_it_25ms,   read_integration_time, 5, IntegrationTime::_25ms);
read_param_test!(can_read_it_12_5ms, read_integration_time, 6, IntegrationTime::_12_5ms);
read_param_test!(can_read_it_6_25ms, read_integration_time, 7, IntegrationTime::_6_25ms);

