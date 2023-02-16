use std::{time::Duration, thread};

use esp_idf_hal::{
    adc::{config::Config, AdcChannelDriver, AdcDriver, Atten2p5dB},
    prelude::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true)).unwrap();

    let mut adc_pin: AdcChannelDriver<esp_idf_hal::gpio::Gpio0, Atten2p5dB<_>> =
        AdcChannelDriver::new(peripherals.pins.gpio0).unwrap();

    loop {
        println!("adc: {}", adc.read(&mut adc_pin).unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}
