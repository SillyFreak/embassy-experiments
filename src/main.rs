#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());
    info!("Hello World!");
}
