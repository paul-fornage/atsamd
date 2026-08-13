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

    let mut n = 0u32;
    loop {
        rprintln!("hello world {}", n);
        n = n.wrapping_add(1);
        delay.delay_ms(1_000u16);
        if n == 16 {
            panic!("WA WA WEE WAH");
        }
    }
}
