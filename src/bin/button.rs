#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let btn = Input::new(p.PA0, Pull::Down);
    let btn = ExtiInput::new(btn, p.EXTI0);

    button(btn, p.PE9).await;
}

async fn button(mut button: ExtiInput<'static, impl Pin>, pin: impl Pin) {
    let mut led = Output::new(pin, Level::Low, Speed::Low);

    loop {
        button.wait_for_high().await;
        // info!("button high");
        led.set_high();
        button.wait_for_low().await;
        // info!("button low");
        led.set_low();
    }
}
