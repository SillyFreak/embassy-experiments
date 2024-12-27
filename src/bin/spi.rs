#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    spi,
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

pub mod adxl345 {
    use embassy_stm32::{
        gpio,
        spi,
        time::Hertz,
        Peripheral,
    };

    pub const READ: u8 = 1 << 7;
    pub const MULTI: u8 = 1 << 6;

    pub struct Adxl345<'d, T: spi::Instance, Rx: spi::RxDma<T>, Tx: spi::TxDma<T>, NCS: gpio::Pin> {
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

        pub async fn read<const N: usize>(&mut self, address: u8) -> Result<[u8; N], spi::Error> {
            let mut address = address | READ;
            if N != 1 {
                address |= MULTI;
            }

            self.ncs.set_low();
            self.spi.write(&[address]).await?;
            let mut buf = [0u8; N];
            self.spi.read(&mut buf).await?;
            self.ncs.set_high();

            Ok(buf)
        }

        pub async fn write(&mut self, address_data: &mut [u8]) -> Result<(), spi::Error> {
            if address_data.len() != 2 {
                address_data[0] |= MULTI;
            }

            self.ncs.set_low();
            self.spi.write(address_data).await?;
            self.ncs.set_high();

            Ok(())
        }
    }
}

use adxl345::Adxl345;

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

    let [id] = unwrap!(accel.read(0x00).await);
    info!("Device ID: {:X}", id);

    Timer::after(Duration::from_micros(1)).await;

    unwrap!(accel.write(&mut [0x2D, 0x08]).await);

    Timer::after(Duration::from_micros(1)).await;

    for _ in 0u32.. {
        let data = unwrap!(accel.read::<6>(0x32).await);
        let x = i16::from_le_bytes(data[0..2].try_into().unwrap());
        let y = i16::from_le_bytes(data[2..4].try_into().unwrap());
        let z = i16::from_le_bytes(data[4..6].try_into().unwrap());
        info!("Acceleration: ({}, {}, {})", x, y, z);

        Timer::after(Duration::from_millis(200)).await;
    }
}