extern crate max44009;
extern crate embedded_hal_mock as hal;
use max44009::{ IntegrationTime, CurrentDivisionRatio };

mod common;
use common::{ setup, check_sent_data, Register };

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
fn can_read_lux() {
    let mut dev = setup(&[0, 1]);
    let lux = dev.read_lux().unwrap();
    assert_eq!(0.045, lux);
    check_sent_data(dev, &[Register::LUX_HIGH]);
}

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

