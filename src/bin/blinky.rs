#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use futures::join;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let base = 500;
    join!(
        blink(p.PE9, "PE9", Duration::from_millis(base << 0)), // LD3
        blink(p.PE10, "PE10", Duration::from_millis(base << 1)), // LD5
        blink(p.PE11, "PE11", Duration::from_millis(base << 2)), // LD7
        blink(p.PE12, "PE12", Duration::from_millis(base << 3)), // LD9
        blink(p.PE13, "PE13", Duration::from_millis(base << 4)), // LD10
        blink(p.PE14, "PE14", Duration::from_millis(base << 5)), // LD8
        blink(p.PE15, "PE15", Duration::from_millis(base << 6)), // LD6
        blink(p.PE8, "PE8", Duration::from_millis(base << 6)), // LD4
    );
}

async fn blink(pin: impl Pin, _name: &'static str, interval: Duration) {
    let mut led = Output::new(pin, Level::Low, Speed::Low);

    loop {
        // info!("{} low", name);
        led.set_low();
        Timer::after(interval).await;
        // info!("{} high", name);
        led.set_high();
        Timer::after(interval).await;
    }
}
