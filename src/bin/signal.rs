#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use futures::join;

use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::*,
    signal::Signal,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

type RawMutex = ThreadModeRawMutex;

static SIGNAL: Signal<RawMutex, &'static str> = Signal::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    join!(
        sender(Duration::from_millis(500)),
        receiver(Duration::from_millis(300)),
    );
}

async fn sender(interval: Duration) {
    loop {
        for msg in &["a", "b"] {
            info!("send {}", msg);
            SIGNAL.signal(msg);
            Timer::after(interval).await;
        }
    }
}

async fn receiver(interval: Duration) {
    loop {
        let msg = SIGNAL.wait().await;
        info!("recv {}", msg);
        Timer::after(interval).await;
    }
}
