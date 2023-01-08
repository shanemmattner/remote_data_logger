use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use std::thread;
use std::time::Duration;
use sx1509;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let p = Peripherals::take().unwrap();
    // I2C
    let sda = p.pins.gpio1.into_input_output().unwrap();
    let scl = p.pins.gpio10.into_output().unwrap();
    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    let mut i2c1 =
        i2c::Master::<i2c::I2C0, _, _>::new(p.i2c0, i2c::MasterPins { sda, scl }, config).unwrap();

    // GPIO expander
    let mut expander = sx1509::Sx1509::new(&mut i2c1, sx1509::DEFAULT_ADDRESS);
    expander.borrow(&mut i2c1).software_reset().unwrap();
    expander.borrow(&mut i2c1).set_bank_a_direction(0).unwrap();
    expander
        .borrow(&mut i2c1)
        .set_bank_b_direction(0xFF)
        .unwrap();
    expander.borrow(&mut i2c1).set_bank_a_pullup(0xFF).unwrap();

    // MAIN LOOP
    loop {
        thread::sleep(Duration::from_millis(100));

        let buff = expander.borrow(&mut i2c1).get_bank_a_data().unwrap();

        println!("buff: {}", buff);
        thread::sleep(Duration::from_millis(100));
    }
}
