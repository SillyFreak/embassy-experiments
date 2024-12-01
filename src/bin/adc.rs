#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{self, Adc};
use embassy_stm32::bind_interrupts;
// use embassy_stm32::gpio::low_level::Pin;
use embassy_stm32::peripherals;
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    ADC1_2 => adc::InterruptHandler<peripherals::ADC1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    info!("create ADC...");
    let mut adc = Adc::new(p.ADC1, Irqs, &mut Delay);
    info!("done");

    let mut pin = p.PC1;
    // pin.set_as_analog();
    // let mut vref = adc.enable_vref(&mut Delay);
    // let mut temp = adc.enable_temperature();

    // let vref_sample = adc.read(&mut vref).await;

    // let convert_to_millivolts = |sample| {
    //     // From http://www.st.com/resource/en/datasheet/DM00071990.pdf
    //     // 6.3.24 Reference voltage
    //     const VREFINT_MV: u32 = 1210; // mV

    //     (u32::from(sample) * VREFINT_MV / u32::from(vref_sample)) as u16
    // };

    // let convert_to_celcius = |sample| {
    //     // From http://www.st.com/resource/en/datasheet/DM00071990.pdf
    //     // 6.3.22 Temperature sensor characteristics
    //     const V25: i32 = 760; // mV
    //     const AVG_SLOPE: f32 = 2.5; // mV/C

    //     let sample_mv = convert_to_millivolts(sample) as i32;

    //     (sample_mv - V25) as f32 / AVG_SLOPE + 25.0
    // };

    // info!("Vref: {}", vref_sample);
    // const MAX_ADC_SAMPLE: u16 = (1 << 12) - 1;
    // info!("VCCA: {} mV", convert_to_millivolts(MAX_ADC_SAMPLE));

    loop {
        // Read pin
        let v = adc.read(&mut pin).await;
        info!("PC1: {}", v);

        Timer::after(Duration::from_millis(100)).await;
    }
}
