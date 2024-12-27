#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio,
    spi,
    time::Hertz,
    Peripheral,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

struct Adxl345<'d, T: spi::Instance, Rx: spi::RxDma<T>, Tx: spi::TxDma<T>, NCS: gpio::Pin> {
    ncs: gpio::Output<'d, NCS>,
    spi: spi::Spi<'d, T, Tx, Rx>,
}

impl<'d, T: spi::Instance, Rx: spi::RxDma<T>, Tx: spi::TxDma<T>, NCS: gpio::Pin>
Adxl345<'d, T, Rx, Tx, NCS>
{
    pub fn new(
        spi: impl Peripheral<P = T> + 'd,
        sck: impl Peripheral<P = impl spi::SckPin<T>> + 'd,
        mosi: impl Peripheral<P = impl spi::MosiPin<T>> + 'd,
        miso: impl Peripheral<P = impl spi::MisoPin<T>> + 'd,
        rxdma: impl Peripheral<P = Rx> + 'd,
        txdma: impl Peripheral<P = Tx> + 'd,
        ncs: impl Peripheral<P = NCS> + 'd,
    ) -> Self {
        let ncs = gpio::Output::new(ncs, gpio::Level::High, gpio::Speed::Low);
        let mut config = spi::Config::default();
        config.mode = spi::MODE_3;
        config.frequency = Hertz(1_000_000);
        let spi = spi::Spi::new(spi, sck, mosi, miso, txdma, rxdma, config);

        Self { ncs, spi }
    }

    pub async fn transfer(&mut self, data: &mut [u8]) -> Result<(), spi::Error> {
        self.ncs.set_low();
        self.spi.transfer_in_place(data).await?;
        self.ncs.set_high();

        Ok(())
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut config = spi::Config::default();
    config.mode = spi::MODE_3;
    config.frequency = Hertz(1_000_000);

    // let ncs = gpio::Output::new(p.PD6, gpio::Level::High, gpio::Speed::Low);
    // let spi = Spi::new(p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA1_CH3, p.DMA1_CH2, config);
    let mut accel = Adxl345::new(p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA1_CH2, p.DMA1_CH3, p.PD6);

    const READ: u8 = 1 << 7;
    const MULTI: u8 = 1 << 6;

    let mut buf = [0u8; 7];

    buf[0] = 0 | READ;
    unwrap!(accel.transfer(&mut buf[..2]).await);
    info!("Device ID: {:X}", buf[1]);

    Timer::after(Duration::from_micros(1)).await;

    buf[0] = 0x2D;
    buf[1] = 0x08;
    unwrap!(accel.transfer(&mut buf[..2]).await);

    Timer::after(Duration::from_micros(1)).await;

    for _ in 0u32.. {
        buf[0] = 0x32 | READ | MULTI;
        unwrap!(accel.transfer(&mut buf[..7]).await);
        let x = i16::from_le_bytes(buf[1..3].try_into().unwrap());
        let y = i16::from_le_bytes(buf[3..5].try_into().unwrap());
        let z = i16::from_le_bytes(buf[5..7].try_into().unwrap());
        info!("Acceleration: ({}, {}, {})", x, y, z);

        Timer::after(Duration::from_millis(200)).await;
    }
}