#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{spi, time::Hertz};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

pub mod adxl345 {
    use embassy_stm32::{Peripheral, gpio, spi, time::Hertz};

    pub const READ: u8 = 1 << 7;
    pub const MULTI: u8 = 1 << 6;

    pub mod register {
        pub const DEVID: u8 = 0x00;
        pub const POWER_CTL: u8 = 0x2D;
        pub const DATA_FORMAT: u8 = 0x31;
        pub const DATAX0: u8 = 0x32;
        pub const DATAX1: u8 = 0x33;
        pub const DATAY0: u8 = 0x34;
        pub const DATAY1: u8 = 0x35;
        pub const DATAZ0: u8 = 0x36;
        pub const DATAZ1: u8 = 0x37;
    }

    pub mod power_ctl {
        pub const LINK: u8 = 0b00_1000_00;
        pub const AUTO_SLEEP: u8 = 0b00_0100_00;
        pub const MEASURE: u8 = 0b00_0010_00;
        pub const SLEEP: u8 = 0b00_0001_00;
        pub const WAKEUP_1HZ: u8 = 0b00_0000_11;
        pub const WAKEUP_4HZ: u8 = 0b00_0000_01;
        pub const WAKEUP_2HZ: u8 = 0b00_0000_10;
        pub const WAKEUP_8HZ: u8 = 0b00_0000_00;
    }

    pub mod data_format {
        pub const SELF_TEST: u8 = 0b100_000_00;
        pub const SPI: u8 = 0b010_000_00;
        pub const INT_INVERT: u8 = 0b001_000_00;
        pub const FULL_RES: u8 = 0b000_010_00;
        pub const JUSTIFY: u8 = 0b000_001_00;
        pub const RANGE_2G: u8 = 0b000_000_00;
        pub const RANGE_4G: u8 = 0b000_000_01;
        pub const RANGE_8G: u8 = 0b000_000_10;
        pub const RANGE_16G: u8 = 0b000_000_11;
    }

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

        pub async fn read_single(&mut self, address: u8) -> Result<u8, spi::Error> {
            let [data] = self.read(address).await?;
            Ok(data)
        }

        pub async fn write_single(&mut self, address: u8, data: u8) -> Result<(), spi::Error> {
            self.write(&mut [address, data]).await?;
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

    let id = unwrap!(accel.read_single(adxl345::register::DEVID).await);
    info!("Device ID: {:X}", id);

    Timer::after(Duration::from_micros(1)).await;

    unwrap!(
        accel
            .write_single(adxl345::register::POWER_CTL, {
                use adxl345::power_ctl::*;
                MEASURE | SLEEP | WAKEUP_8HZ
            })
            .await
    );

    Timer::after(Duration::from_micros(1)).await;

    unwrap!(
        accel
            .write_single(adxl345::register::DATA_FORMAT, {
                use adxl345::data_format::*;
                FULL_RES | RANGE_2G
            })
            .await
    );

    Timer::after(Duration::from_micros(1)).await;

    for _ in 0u32.. {
        let data = unwrap!(accel.read::<6>(adxl345::register::DATAX0).await);
        let x = i16::from_le_bytes(data[0..2].try_into().unwrap()) as isize;
        let y = i16::from_le_bytes(data[2..4].try_into().unwrap()) as isize;
        let z = i16::from_le_bytes(data[4..6].try_into().unwrap()) as isize;

        // with FULL_RES, the acceleration is in 4milli-g steps
        // multiply by four for a nice scale
        let total = isize::isqrt(x * x + y * y + z * z) * 4;
        info!(
            "Acceleration: ({:04}, {:04}, {:04}) = {}.{:03}g",
            x,
            y,
            z,
            total / 1000,
            total % 1000
        );

        Timer::after(Duration::from_millis(200)).await;
    }
}
