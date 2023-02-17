use esp_idf_sys as _;
use sandbox::wifi;


use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{EspWifi, WifiWait};
use std::net::Ipv4Addr;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use std::env;

const SSID: &str = env!("SSID");
const PSK: &str = env!("PSK");

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let modem = peripherals.modem;
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let _wifi = wifi(SSID, PSK, modem, sysloop, nvs).map_err(|err| {
        eprintln!("{:?}", err);
        anyhow::anyhow!("{:?}", err)
    });

    println!("{}", SSID);

    loop {
        FreeRtos::delay_ms(5000);
        println!("waiting");
    }
}
