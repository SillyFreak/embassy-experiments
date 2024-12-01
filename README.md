# Embassy Experiments

This repo contains example programs for the [STM32F3 Discovery board](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) - which uses the [STM32F303VC MCU](https://www.st.com/en/microcontrollers-microprocessors/stm32f303vc.html) - utilizing the [Embassy](https://embassy.dev/) embedded Rust framework.

## Prerequisites

To run these programs, you'll need a nightly rust toolchain for the thumbv7em-none-eabihf target:

```sh
rustup install nightly
rustup target add thumbv7em-none-eabihf
```

In addition, `cargo run` will try to flash the discovery board using [probe-rs](https://probe.rs/docs/getting-started/installation/), so you'll need that as well. Alternatively, you can only `cargo build` the programs and flash them via other means, or adapt the `runner` option in `.cargo/config.toml`.

Also, you will naturally need an STM32F3 Discovery board.

## Usage

Several programs are contained in `src/bin/`. For example, to run the `hello.rs` program, execute this command:

```sh
cargo run --release --bin hello
```

The programs are based on and inspired by Embassy's [stm32f3](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f3/src/bin) and [stm32f4](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f4/src/bin) examples.

These programs work:

- **hello**: prints "Hello World!" to the debugging interface, that's it.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/hello.rs)
- **blinky**: blinks the eight LEDs of the discovery board at different frequencies, by multiplexing eight independent tasks.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/blinky.rs)
- **button**: waits for button presses and releases using interrupts and controls an LED accordingly.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f3/src/bin/button_exti.rs)
- **pwm**: controls an LED's brightness by varying its duty cycle.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/pwm.rs)
- **adc**: reads a voltage from one of the ADC capable pins periodically. Raw ADC values (0-4095) are shown, no voltage calibration is performed.
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/adc.rs)

  This example has been fixed by [anothersteven2](https://github.com/anotherstevest2/embassy-experiments). See their repo for a more complete example with voltage calibration and temperature reading.
- **channel**: uses a Channel to communicate between two async tasks. If the receiver interval is larger than the sender interval, this will demonstrate backpressure.
- **signal**: uses a Signal to communicate between two async tasks. If the receiver interval is larger than the sender interval, this will demonstrate overwriting without backpressure.

These don't:

- **uart**: when TX and RX (PE0, PE1) are connected, should echo the UART output. However, no data is received and printed (I have not yet explicitly tested if the data is sent correctly).
  [See this example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/usart_dma.rs)
