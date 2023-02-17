use std::{net::Ipv4Addr, time::Duration};

use embedded_svc::wifi::*;
use esp_idf_hal::peripheral;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    netif::{EspNetif, EspNetifWait},
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};

#[allow(unused)]
pub struct Wifi {
    esp_wifi: Box<EspWifi<'static>>,
}

pub fn wifi(
    ssid: &str,
    psk: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> anyhow::Result<Wifi> {
    if ssid.is_empty() {
        anyhow::bail!("SSID empty!");
    }
    let mut auth_method = AuthMethod::WPA2Personal;
    if psk.is_empty() {
        auth_method = AuthMethod::None;
        log::info!("Wifi password is empty");
    }

    let mut wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|ap| ap.ssid == ssid);

    let channel = if let Some(ours) = ours {
        Some(ours.channel)
    } else {
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        auth_method,
        password: psk.into(),
        channel,
        bssid: None,
        ssid: ssid.into(),
    }))?;

    wifi.start()?;

    wifi.connect()?;

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.is_up().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        // anyhow::bail!("Wifi did not connect or didn't receive ip address");
        eprintln!("wifi didn't work");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    log::info!("Wifi info: {:#?}", ip_info);

    Ok(Wifi {
        esp_wifi: Box::new(wifi),
    })
}