use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::spi::{self, SpiDeviceDriver, SPI2};
use esp_idf_hal::units::FromValueType;
use esp_idf_sys as _;
use max7219::MAX7219;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherial = Peripherals::take().unwrap();

    let spi = peripherial.spi2;
    let sclk = peripherial.pins.gpio1;
    let sdo = peripherial.pins.gpio4;
    let sdi = peripherial.pins.gpio0;
    let cs = peripherial.pins.gpio5;

    let config = spi::config::Config::new()
        .baudrate(1.MHz().into())
        .data_mode(embedded_hal::spi::MODE_0);

    let spi_driver = spi::SpiDriver::new::<SPI2>(spi, sclk, sdo, Some(sdi), spi::Dma::Disabled)?;

    let device = SpiDeviceDriver::new(&spi_driver, Some(cs), &config)?;

    let mut max = MAX7219::from_spi(1, device).map_err(|err| anyhow::anyhow!("{:?}", err))?;

    max.power_on().unwrap();

    loop {
        max.clear_display(0).unwrap();

        max.write_str(0, b"        ", 0b10101010).unwrap();
        FreeRtos::delay_ms(1000);

        max.write_str(0, b"        ", 0b01010101).unwrap();
        FreeRtos::delay_ms(1000);
    }
}
