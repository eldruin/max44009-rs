extern crate embedded_hal_mock as hal;
extern crate max44009;
use max44009::{CurrentDivisionRatio as CDR, IntegrationTime as IT};

mod common;
use common::{check_sent_data, setup, Register};

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
    assert!((lux - 0.045).abs() < 0.001);
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
    };
}

read_param_test!(can_read_cdr_one, read_current_division_ratio, 0, CDR::One);
read_param_test!(
    can_read_cdr_one_eighth,
    read_current_division_ratio,
    8,
    CDR::OneEighth
);

read_param_test!(can_read_it_800ms, read_integration_time, 0, IT::_800ms);
read_param_test!(can_read_it_400ms, read_integration_time, 1, IT::_400ms);
read_param_test!(can_read_it_200ms, read_integration_time, 2, IT::_200ms);
read_param_test!(can_read_it_100ms, read_integration_time, 3, IT::_100ms);
read_param_test!(can_read_it_50ms, read_integration_time, 4, IT::_50ms);
read_param_test!(can_read_it_25ms, read_integration_time, 5, IT::_25ms);
read_param_test!(can_read_it_12_5ms, read_integration_time, 6, IT::_12_5ms);
read_param_test!(can_read_it_6_25ms, read_integration_time, 7, IT::_6_25ms);
