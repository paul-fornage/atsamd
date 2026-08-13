#![no_std]
#![no_main]

use bsp::hal;
use clearcore as bsp;

use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    // RTT uses the SWD debug connection; it does not use the target's USB.
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = bsp::Pins::new(peripherals.port);
    let mut led = bsp::pin_alias!(pins.out01).into_push_pull_output();

    loop {
        delay.delay_ms(1000u16);
        led.set_high().unwrap();
        rprintln!("LED set high");
        delay.delay_ms(1000u16);
        led.set_low().unwrap();
        rprintln!("LED set low");
    }
}
