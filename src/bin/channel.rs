#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use futures::join;

use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::*,
    channel,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

type RawMutex = ThreadModeRawMutex;
type Channel = channel::Channel<RawMutex, &'static str, 3>;
type Sender<'a> = channel::Sender<'a, RawMutex, &'static str, 3>;
type Receiver<'a> = channel::Receiver<'a, RawMutex, &'static str, 3>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let channel: Channel = Channel::new();

    join!(
        sender(channel.sender(), Duration::from_millis(500)),
        receiver(channel.receiver(), Duration::from_millis(300)),
    );
}

async fn sender(channel: Sender<'_>, interval: Duration) {
    loop {
        for msg in &["a", "b"] {
            info!("send {}", msg);
            channel.send(msg).await;
            Timer::after(interval).await;
        }
    }
}

async fn receiver(channel: Receiver<'_>, interval: Duration) {
    loop {
        let msg = channel.receive().await;
        info!("recv {}", msg);
        Timer::after(interval).await;
    }
}
