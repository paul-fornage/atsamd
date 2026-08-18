#![no_std]
#![no_main]

use bsp::hal;
use clearcore as bsp;

use hal::clock::v2::{
    clock_system_at_reset,
    dpll::Dpll,
    gclk::{Gclk, GclkDiv8},
    osculp32k::OscUlp1k,
    pclk::Pclk,
    rtcosc::RtcOsc,
    xosc::Xosc,
};
use hal::ehal::digital::StatefulOutputPin;
use hal::prelude::*;
use hal::rtc::rtic::rtc_clock;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

hal::rtc_monotonic!(Mono, rtc_clock::Clock1k);

#[rtic::app(device = hal::pac, dispatchers = [FREQM])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        out01: bsp::Out01,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut device = ctx.device;
        let mut core: rtic::export::Peripherals = ctx.core;

        rtt_init_print!();

        let pins = bsp::Pins::new(device.port);
        let (_buses, clocks, tokens) = clock_system_at_reset(
            device.oscctrl,
            device.osc32kctrl,
            device.gclk,
            device.mclk,
            &mut device.nvmctrl,
        );


        // ClearCore supplies a driven 25 MHz clock on PB22/XOSC1-XIN1.
        let xosc1 = Xosc::from_clock(tokens.xosc1, pins.sys_clk, 25.MHz())
            .on_demand(false)
            .enable();
        while !xosc1.is_ready() {}


        // XOSC1 -> GCLK5: 25 MHz / 25 = 1 MHz.
        let (gclk5, _xosc1) = Gclk::from_source(tokens.gclks.gclk5, xosc1);
        let gclk5 = gclk5.div(GclkDiv8::Div(25)).enable();

        // GCLK5 feeds both DPLLs. DPLL0 provides the 96 MHz USB source;
        // DPLL1 provides the 120 MHz CPU source.
        let (dpll0_pclk, gclk5) = Pclk::enable(tokens.pclks.dpll0, gclk5);
        let (dpll1_pclk, _gclk5) = Pclk::enable(tokens.pclks.dpll1, gclk5);
        let dpll0 = Dpll::from_pclk(tokens.dpll0, dpll0_pclk)
            .loop_div(96, 0)
            .on_demand(false)
            .enable();
        let dpll1 = Dpll::from_pclk(tokens.dpll1, dpll1_pclk)
            .loop_div(120, 0)
            .on_demand(false)
            .enable();
        while !dpll0.is_ready() || !dpll1.is_ready() {}

        // DPLL0 -> GCLK4: 96 MHz / 2 = 48 MHz. Configure USB's peripheral
        // channel to use this generator.
        let (gclk4, _dpll0) = Gclk::from_source(tokens.gclks.gclk4, dpll0);
        let gclk4 = gclk4.div(GclkDiv8::Div(2)).enable();
        let (_usb_clock, _gclk4) = Pclk::enable(tokens.pclks.usb, gclk4);
        // DPLL1 -> GCLK0: 120 MHz / 1 = 120 MHz for the CPU.
        let (_gclk0, _dfll, _dpll1) = clocks.gclk0.swap_sources(clocks.dfll, dpll1);
        // Make the intended CPU divider explicit after switching GCLK0.
        let (_, _, _, mclk) = unsafe { clocks.pac.steal() };
        mclk.cpudiv().write(|w| w.div().div1());

        // Enable the 1 kHz clock from the internal 32 kHz source
        let (osculp1k, _) = OscUlp1k::enable(tokens.osculp32k.osculp1k, clocks.osculp32k_base);

        // Enable the RTC clock with the 1 kHz source.
        // Note that currently the proof of this (the `RtcOsc` instance) is not
        // required to start the monotonic.
        let _ = RtcOsc::enable(tokens.rtcosc, osculp1k);

        // Start the monotonic
        Mono::start(device.rtc);

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        blink_led::spawn().unwrap();

        (
            Shared {},
            Local {
                out01: bsp::pin_alias!(pins.out01).into(),
            },
        )
    }

    /// This function is spawned and never returns.
    #[task(priority = 1, local=[out01])]
    async fn blink_led(ctx: blink_led::Context) {
        loop {
            StatefulOutputPin::toggle(ctx.local.out01).unwrap();
            rprintln!(
                "LED {}!",
                if ctx.local.out01.is_set_high().unwrap() {
                    "OFF"
                } else {
                    "ON"
                }
            );
            Mono::delay(1000u64.millis()).await;
        }
    }
}
