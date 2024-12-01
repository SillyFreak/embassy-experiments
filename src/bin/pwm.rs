#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::{Channel, CountingMode};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let ch1 = PwmPin::new_ch1(p.PE9, OutputType::PushPull);
    let mut pwm = SimplePwm::new(
        p.TIM1,
        Some(ch1),
        None,
        None,
        None,
        Hertz::khz(10),
        CountingMode::EdgeAlignedUp,
    );
    let max = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);

    info!("PWM initialized");
    info!("PWM max duty {}", max);

    loop {
        for percent in [0, 25, 50, 100] {
            info!("{}%", percent);
            let duty = (max as u32 * percent / 100) as u16;
            pwm.set_duty(Channel::Ch1, duty);
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}
