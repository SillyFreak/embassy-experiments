#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{self, Adc};
use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals;
use embassy_stm32::time::Hertz;
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    ADC1_2 => adc::InterruptHandler<peripherals::ADC1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init({
        // adapted from https://github.com/anotherstevest2/embassy-experiments/blob/main/src/bin/adc.rs#L21-L47
        // Copyright anotherstevest2, provided under MIT or Apache license

        use embassy_stm32::Config;
        use embassy_stm32::rcc::*;

        // default is not sufficient for clocking the adc so we manually configure it
        let mut config = Config::default();
        config.rcc.hse = Some(Hertz::mhz(8));
        config.rcc.bypass_hse = true;
        config.rcc.sysclk = Some(Hertz::mhz(48));
        config.rcc.hclk = Some(Hertz::mhz(48));
        config.rcc.pclk1 = Some(Hertz::mhz(24));
        config.rcc.pclk2 = Some(Hertz::mhz(48));
        config.rcc.pll48 = true;
        // The following (commented out) does not work due to the ADC hanging waiting for the adcal bit to clear
        // The reason the calibration never completes is that the clock initialization which occurs during
        // the embassy_stm32::init (see: embassy-stm32-0.1.0/src/rcc/f3.rs:237) attempts to update the CKMODE bits (ADC clock mode) in the ADC peripheral
        // to configure the AdcClockSource and this operation silently fails as the ADC peripheral has not yet been
        // enabled (which occurs during Adc::new()).  The only way to hack it is within Adc::new() as this
        // is where both the ADC peripheral is enabled *and* where the Adc Cal takes place. One work-around (used here) is to use the PLL
        // clock for AdcClockSource which uses the reset/default value in CKMODE and therefore doesn't require updating
        // One possible non-breaking fix is to modify the rcc/src/f3.rc to ignore the rcc.adc field (since only one value works - doing so
        // won't break anything that wasn't already broken) and add a method to adc to set the clock source which will only be runable
        // after adc.new has been run, and so we can assure that it is already enabled.  We may have to re-cal etc. (I haven't looked into that)
        // but we can if required.

        // config.rcc.adc = Some(AdcClockSource::BusDiv1);  // HCLK Synchronous Mode 48MHz -> 20.83333 ns )

        config.rcc.adc = Some(AdcClockSource::Pll(Adcpres::DIV1)); // PLL Asynchronous Mode 48MHz -> 20.83333 ns
        config.rcc.adc34 = None;
        config
    });
    info!("Hello World!");

    info!("create ADC...");
    let mut adc = Adc::new(p.ADC1, Irqs, &mut Delay);
    // >= 2.2 us per 6.3.22 in STMicrosystems doc DS9118 Rev 14
    adc.set_sample_time(adc.sample_time_for_us(6));
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

        // let t = adc.read(&mut temp).await;
        // info!("T: {} °C", convert_to_celcius(t));

        Timer::after(Duration::from_millis(100)).await;
    }
}
