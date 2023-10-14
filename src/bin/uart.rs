#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use core::str::from_utf8_unchecked;
use futures::join;

use defmt::*;
use heapless::{String, Vec};
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals;
use embassy_stm32::usart::{self, Config, Uart};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let config = Config::default();
    let mut usart = Uart::new(p.USART1, p.PE1, p.PE0, Irqs, p.DMA1_CH4, p.DMA1_CH5, config).unwrap();

    let (tx, rx) = usart.split();

    let mut send = || async {
        let mut tx = tx;
        let mut s: String<128> = String::new();
    
        for n in 0u32.. {
            s.clear();
            core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();
            unwrap!(tx.write(s.as_bytes()).await);
            info!("wrote DMA");
    
            Timer::after(Duration::from_millis(100)).await;
        }
    };

    let mut recv = || async {
        let mut rx = rx;
        let mut s = [0u8; 128];
    
        info!("waiting");
        loop {
            let len = unwrap!(rx.read_until_idle(&mut s).await);
            let msg = unsafe { from_utf8_unchecked(&s[..len]) };
            info!("read DMA: {}", msg);
        }
    };

    join!(
        send(),
        recv(),
    );
}
