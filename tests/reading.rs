extern crate embedded_hal_mock as hal;
extern crate max44009;
use hal::i2c::Transaction as I2cTrans;
use max44009::{CurrentDivisionRatio as CDR, IntegrationTime as IT};

mod common;
use common::{destroy, new, Register, DEV_BASE_ADDR};

#[test]
fn can_read_interrupt_did_not_happened() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_BASE_ADDR,
        vec![Register::INT_STATUS],
        vec![0],
    )]);
    let interrupt_happened = dev.has_interrupt_happened().unwrap();
    assert!(!interrupt_happened);
    destroy(dev);
}

#[test]
fn can_read_interrupt_happened() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_BASE_ADDR,
        vec![Register::INT_STATUS],
        vec![1],
    )]);
    let interrupt_happened = dev.has_interrupt_happened().unwrap();
    assert!(interrupt_happened);
    destroy(dev);
}

#[test]
fn can_read_lux() {
    let mut dev = new(&[I2cTrans::write_read(
        DEV_BASE_ADDR,
        vec![Register::LUX_HIGH],
        vec![0, 1],
    )]);
    let lux = dev.read_lux().unwrap();
    assert!((lux - 0.045).abs() < 0.001);
    destroy(dev);
}

macro_rules! read_param_test {
    ($test_name:ident, $method:ident, $input_data:expr, $enum:ident::$expected_variant:ident) => {
        #[test]
        fn $test_name() {
            let mut dev = new(&[I2cTrans::write_read(
                DEV_BASE_ADDR,
                vec![Register::CONFIGURATION],
                vec![$input_data],
            )]);
            let it = dev.$method().unwrap();
            assert_eq!($enum::$expected_variant, it);
            destroy(dev);
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
