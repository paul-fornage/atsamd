//! ClearCore board devices
//!
//! Higher-level abstractions built on top of the pin definitions in
//! [`pins`](super::pins):
//!
//! - Type aliases and constructors for the SERCOM peripherals (COM0, COM1,
//!   XBEE, SD card, shift register chain).
//! - A driver for the 4x shift-register chain that carries all board
//!   configuration lines, LEDs, and motor enables.
//! - Conversion helpers for the various scaled analog signals and the
//!   4-20mA DAC output.
//!
//! ## Pin-mux notes
//!
//! - The SERCOM pad assignments below follow the supplied SAME53N19A
//!   multiplexing table: COM1 is SERCOM0 on mux C, COM0 is SERCOM7 on mux D,
//!   XBEE is SERCOM2 on mux D, the SD signals are SERCOM4 on mux D, and the
//!   shift-register signals are SERCOM6 on mux C.
//! - The shift registers are assumed to be 74HC595-style (`SRCLK`/`RCLK`/
//!   `Q7'`/`OEn` match the note names `SR_CLK`/`SR_LOAD`/`SR_DATA_RET`/
//!   `SR_ENn`). With SR_DATA feeding SR0 and `Q7'` of SR3 returning as
//!   `SR_DATA_RET`, the *first* byte shifted out ends up in SR3, and the
//!   first bit of each byte ends up in Q7. Hence: bytes are sent
//!   [SR3, SR2, SR1, SR0], MSB first (SPI MODE_0, MSB-first — the SERCOM
//!   default).

use super::hal;
use super::pins::*;

use hal::clock::GenericClockController;
use hal::ehal::spi::SpiBus;
use hal::pac;
use hal::sercom::uart::{self, BaudMode, Oversampling};
use hal::sercom::{spi, IoSet1, IoSet2, IoSet3, Sercom0, Sercom2, Sercom4, Sercom6, Sercom7};
use hal::time::Hertz;

// ----------------------------------------------------------------------------
// COM1 (SERCOM0, PA08-PA11, alternate C)
// ----------------------------------------------------------------------------

/// UART pads for `COM1` (RX = PA09/PAD1, TX = PA08/PAD0).
///
/// RTS (PA10/PAD2) and CTS (PA11/PAD3) are available; add them to the pads if
/// you want hardware flow control.
pub type Com1UartPads = uart::Pads<Sercom0, Com1Rx, Com1Tx>;
/// UART for the `COM1` connector.
///
/// Remember to set `CfgCom1_UART_SPIn` **high** on the shift register chain
/// ([`ShiftRegisters::set_com_uart`]), and note that `CfgCom1_Polarity`
/// inverts every `COM1` line before it reaches the connector.
pub type Com1Uart = uart::Uart<uart::Config<Com1UartPads>, uart::Duplex>;

/// SPI pads for `COM1` (DI = PA11/PAD3, DO = PA08/PAD0, SCK = PA09/PAD1).
///
/// SS (PA10) is left out; drive it as a software-controlled [`Com1Ss`]
/// reconfigured as a push-pull output if needed.
pub type Com1SpiPads = spi::Pads<Sercom0, Com1Miso, Com1Mosi, Com1Sck>;
/// SPI master for the `COM1` connector (`CfgCom1_UART_SPIn` low).
pub type Com1Spi = spi::Spi<spi::Config<Com1SpiPads>, spi::Duplex>;

/// Convenience constructor for [`Com1Uart`], clocked from GCLK0.
pub fn com1_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::Sercom0,
    mclk: &mut pac::Mclk,
    rx: impl Into<Com1Rx>,
    tx: impl Into<Com1Tx>,
) -> Com1Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom0, pads, clock.freq())
        .baud(baud.into(), BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// Convenience constructor for [`Com1Spi`], clocked from GCLK0.
pub fn com1_spi(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::Sercom0,
    mclk: &mut pac::Mclk,
    sck: impl Into<Com1Sck>,
    mosi: impl Into<Com1Mosi>,
    miso: impl Into<Com1Miso>,
) -> Com1Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let pads = spi::Pads::default()
        .data_in(miso.into())
        .data_out(mosi.into())
        .sclk(sck.into());
    spi::Config::new(mclk, sercom0, pads, clock.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable()
}

// ----------------------------------------------------------------------------
// COM0 (SERCOM7, PB18-PB21, alternate D)
// ----------------------------------------------------------------------------

/// UART pads for `COM0` (RX = PB20/PAD1, TX = PB21/PAD0). RTS/CTS on
/// PB18/PB19 are available for hardware flow control.
pub type Com0UartPads = uart::Pads<Sercom7, Com0Rx, Com0Tx>;
/// UART for the `COM0` connector. See [`Com1Uart`] for the `Cfg*` caveats.
pub type Com0Uart = uart::Uart<uart::Config<Com0UartPads>, uart::Duplex>;

/// SPI pads for `COM0` (DI = PB19/PAD3, DO = PB21/PAD0, SCK = PB20/PAD1).
pub type Com0SpiPads = spi::Pads<Sercom7, Com0Miso, Com0Mosi, Com0Sck>;
/// SPI master for the `COM0` connector (`CfgCom0_UART_SPIn` low).
pub type Com0Spi = spi::Spi<spi::Config<Com0SpiPads>, spi::Duplex>;

/// Convenience constructor for [`Com0Uart`], clocked from GCLK0.
pub fn com0_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom7: pac::Sercom7,
    mclk: &mut pac::Mclk,
    rx: impl Into<Com0Rx>,
    tx: impl Into<Com0Tx>,
) -> Com0Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom7_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom7, pads, clock.freq())
        .baud(baud.into(), BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// Convenience constructor for [`Com0Spi`], clocked from GCLK0.
pub fn com0_spi(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom7: pac::Sercom7,
    mclk: &mut pac::Mclk,
    sck: impl Into<Com0Sck>,
    mosi: impl Into<Com0Mosi>,
    miso: impl Into<Com0Miso>,
) -> Com0Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom7_core(&gclk0).unwrap();
    let pads = spi::Pads::default()
        .data_in(miso.into())
        .data_out(mosi.into())
        .sclk(sck.into());
    spi::Config::new(mclk, sercom7, pads, clock.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable()
}

// ----------------------------------------------------------------------------
// XBEE (SERCOM2, PB24/PB25 + PC24/PC25, alternate D)
// ----------------------------------------------------------------------------

/// UART pads for the XBEE socket (RX = PB24/PAD1, TX = PB25/PAD0).
/// RTS (PC24/PAD2) and CTS (PC25/PAD3) are available for flow control,
/// which XBee modules generally want at higher baud rates.
pub type XbeeUartPads = uart::Pads<Sercom2, XbeeRx, XbeeTx>;
/// UART for the XBEE socket. All lines are 3.3v and buffered on-board.
pub type XbeeUart = uart::Uart<uart::Config<XbeeUartPads>, uart::Duplex>;

/// Convenience constructor for [`XbeeUart`], clocked from GCLK0.
pub fn xbee_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom2: pac::Sercom2,
    mclk: &mut pac::Mclk,
    rx: impl Into<XbeeRx>,
    tx: impl Into<XbeeTx>,
) -> XbeeUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom2, pads, clock.freq())
        .baud(baud.into(), BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

// ----------------------------------------------------------------------------
// SD card (SERCOM4 mux D, PB08-PB10; CS on PA04 is software controlled)
// ----------------------------------------------------------------------------

/// SPI pads for the micro SD slot
/// (DI = PB10/PAD2, DO = PB08/PAD0, SCK = PB09/PAD1).
pub type SdSpiPads = spi::Pads<Sercom4, MicroSdMiso, MicroSdMosi, MicroSdSck>;
/// SPI master for the micro SD slot.
///
/// PA04 (`MicroSD_SS`) is not a SERCOM4 pad, so chip select must be handled
/// in software with [`MicroSdCs`] — e.g. wrap this bus and the CS pin in an
/// `embedded_hal_bus::spi::ExclusiveDevice` for use with `embedded-sdmmc`.
pub type SdSpi = spi::Spi<spi::Config<SdSpiPads>, spi::Duplex>;

/// Convenience constructor for [`SdSpi`], clocked from GCLK0.
///
/// Start at ≤400kHz for card initialization, then raise the baud rate.
pub fn sd_spi(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom4: pac::Sercom4,
    mclk: &mut pac::Mclk,
    sck: impl Into<MicroSdSck>,
    mosi: impl Into<MicroSdMosi>,
    miso: impl Into<MicroSdMiso>,
) -> SdSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let pads = spi::Pads::default()
        .data_in(miso.into())
        .data_out(mosi.into())
        .sclk(sck.into());
    spi::Config::new(mclk, sercom4, pads, clock.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable()
}

// ----------------------------------------------------------------------------
// Shift register chain (SERCOM6, PC05-PC07 + PB01/PB02)
// ----------------------------------------------------------------------------

/// SPI pads for the shift register chain
/// (DI = `SR_DATA_RET` PC06/PAD2, DO = `SR_DATA` PC07/PAD3,
/// SCK = `SR_CLK` PC05/PAD1).
pub type SrSpiPads = spi::PadsFromIds<Sercom6, IoSet1, SrDataRet, SrData, SrClk>;
/// SPI master driving the shift register chain.
pub type SrSpi = spi::Spi<spi::Config<SrSpiPads>, spi::Duplex>;

/// One of the 32 shift register outputs.
///
/// The discriminant is the bit position in [`ShiftRegisters`]' internal
/// state: `SRn-Qm` is bit `n * 8 + m`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrOutput {
    /// SR0-Q0: high = `A09` analog (divider enabled), low = digital (5v pull-up).
    Cfg09AinDin = 0,
    /// SR0-Q1: as [`Self::Cfg09AinDin`], for `A10`.
    Cfg10AinDin = 1,
    /// SR0-Q2: as [`Self::Cfg09AinDin`], for `A11`.
    Cfg11AinDin = 2,
    /// SR0-Q3: as [`Self::Cfg09AinDin`], for `A12`.
    Cfg12AinDin = 3,
    /// SR0-Q4: `A09` indicator LED. Active **low**.
    Led09n = 4,
    /// SR0-Q5: `A10` indicator LED. Active **low**.
    Led10n = 5,
    /// SR0-Q6: `A11` indicator LED. Active **low**.
    Led11n = 6,
    /// SR0-Q7: `A12` indicator LED. Active **low**.
    Led12n = 7,
    /// SR1-Q0: `DI8` indicator LED. Active **low**.
    Led08n = 8,
    /// SR1-Q1: `DI7` indicator LED. Active **low**.
    Led07n = 9,
    /// SR1-Q2: `DI6` indicator LED. Active **low**.
    Led06n = 10,
    /// SR1-Q3: high = `IO0` is a HP digital output / digital input,
    /// low = 4-20mA analog output (DAC `AOUT00`).
    Cfg00DioAout = 11,
    /// SR1-Q4: `COM1` indicator LED. Active high.
    LedCom1 = 12,
    /// SR1-Q5: `COM0` indicator LED. Active high.
    LedCom0 = 13,
    /// SR1-Q6: high = `COM0` is UART, low = SPI.
    CfgCom0UartSpi = 14,
    /// SR1-Q7: high = `COM1` is UART, low = SPI.
    CfgCom1UartSpi = 15,
    /// SR2-Q0: red 'user controlled' LED. Active high.
    LedUser = 16,
    /// SR2-Q1: underglow LEDs. On when high (net is also pulled up, so it
    /// must be actively driven low to turn the LEDs off).
    LedUnderglow = 17,
    /// SR2-Q2: inverts all `COM0` connector signals when high.
    CfgCom0Polarity = 18,
    /// SR2-Q3: inverts all `COM1` connector signals when high.
    CfgCom1Polarity = 19,
    /// SR2-Q4: motor 0 enable.
    Mtr0Enable = 20,
    /// SR2-Q5: motor 1 enable.
    Mtr1Enable = 21,
    /// SR2-Q6: motor 2 enable.
    Mtr2Enable = 22,
    /// SR2-Q7: motor 3 enable.
    Mtr3Enable = 23,
    /// SR3-Q0: `IO0` indicator LED. Active **low**.
    Led00n = 24,
    /// SR3-Q1: `IO1` indicator LED. Active **low**.
    Led01n = 25,
    /// SR3-Q2: `IO2` indicator LED. Active **low**.
    Led02n = 26,
    /// SR3-Q3: `IO3` indicator LED. Active **low**.
    Led03n = 27,
    /// SR3-Q4: `IO4` indicator LED. Active **high**.
    Led04 = 28,
    /// SR3-Q5: `IO5` indicator LED. Active **high**.
    Led05 = 29,
    /// SR3-Q6: motor connector 2 mode: motor (ClearPath) vs. step driver.
    /// Exact polarity unverified; the trailing `n` suggests low = step driver.
    CfgM2MtrSdrvr = 30,
    /// SR3-Q7: as [`Self::CfgM2MtrSdrvr`], for motor connector 3.
    CfgM3MtrSdrvr = 31,
}

impl SrOutput {
    /// Bit mask of this output within the 32-bit chain state.
    pub const fn mask(self) -> u32 {
        1 << (self as u8)
    }
}

/// LEDs reachable through the shift register chain, with polarity abstracted
/// away (see [`ShiftRegisters::set_led`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Led {
    Io0,
    Io1,
    Io2,
    Io3,
    Io4,
    Io5,
    Di6,
    Di7,
    Di8,
    A09,
    A10,
    A11,
    A12,
    Com0,
    Com1,
    User,
    Underglow,
}

impl Led {
    /// Returns the underlying shift register output and whether it is
    /// active-low.
    pub const fn output(self) -> (SrOutput, bool) {
        match self {
            Led::Io0 => (SrOutput::Led00n, true),
            Led::Io1 => (SrOutput::Led01n, true),
            Led::Io2 => (SrOutput::Led02n, true),
            Led::Io3 => (SrOutput::Led03n, true),
            Led::Io4 => (SrOutput::Led04, false),
            Led::Io5 => (SrOutput::Led05, false),
            Led::Di6 => (SrOutput::Led06n, true),
            Led::Di7 => (SrOutput::Led07n, true),
            Led::Di8 => (SrOutput::Led08n, true),
            Led::A09 => (SrOutput::Led09n, true),
            Led::A10 => (SrOutput::Led10n, true),
            Led::A11 => (SrOutput::Led11n, true),
            Led::A12 => (SrOutput::Led12n, true),
            Led::Com0 => (SrOutput::LedCom0, false),
            Led::Com1 => (SrOutput::LedCom1, false),
            Led::User => (SrOutput::LedUser, false),
            Led::Underglow => (SrOutput::LedUnderglow, false),
        }
    }
}

/// One of the two `COM` connectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComPort {
    Com0,
    Com1,
}

/// One of the four `A` (analog-capable) input connectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalogIn {
    A09,
    A10,
    A11,
    A12,
}

/// Errors from the shift register chain.
#[derive(Debug)]
pub enum SrError {
    /// SPI bus error.
    Spi(spi::Error),
    /// The bits read back on `SR_DATA_RET` did not match what was shifted
    /// out on the previous transfer — the chain has probably desynced.
    ReadbackMismatch { expected: u32, got: u32 },
}

impl From<spi::Error> for SrError {
    fn from(e: spi::Error) -> Self {
        SrError::Spi(e)
    }
}

/// Driver for the chain of four shift registers carrying board configuration
/// lines, LEDs, and motor enables.
///
/// All `set_*` methods only modify a buffered copy of the state; call
/// [`flush`](Self::flush) (or [`flush_verified`](Self::flush_verified)) to
/// shift it out and latch it onto the outputs.
pub struct ShiftRegisters {
    spi: SrSpi,
    load: SrLoad,
    enable: SrEn,
    state: u32,
}

impl ShiftRegisters {
    /// Power-on-safe state: all LEDs off, all motors disabled, `A09`-`A12`
    /// digital, `IO0` digital, both COM ports UART with normal polarity, and
    /// motor connectors 2/3 in `Mtr` mode.
    pub const SAFE_DEFAULT: u32 = SrOutput::Led09n.mask()
        | SrOutput::Led10n.mask()
        | SrOutput::Led11n.mask()
        | SrOutput::Led12n.mask()
        | SrOutput::Led08n.mask()
        | SrOutput::Led07n.mask()
        | SrOutput::Led06n.mask()
        | SrOutput::Cfg00DioAout.mask()
        | SrOutput::CfgCom0UartSpi.mask()
        | SrOutput::CfgCom1UartSpi.mask()
        | SrOutput::Led00n.mask()
        | SrOutput::Led01n.mask()
        | SrOutput::Led02n.mask()
        | SrOutput::Led03n.mask()
        | SrOutput::CfgM2MtrSdrvr.mask()
        | SrOutput::CfgM3MtrSdrvr.mask();

    /// Takes ownership of the SPI bus (see [`shift_registers`]), the
    /// `SR_LOAD` latch pin (PB02), and the `SR_ENn` output-enable pin (PB01).
    ///
    /// Shifts out [`Self::SAFE_DEFAULT`] and latches it, but leaves the
    /// outputs **disabled** (`SR_ENn` high); call
    /// [`enable_outputs`](Self::enable_outputs) once you are happy with the
    /// state.
    pub fn new(spi: SrSpi, mut load: SrLoad, mut enable: SrEn) -> Result<Self, SrError> {
        enable.set_high();
        load.set_low();
        let mut sr = Self {
            spi,
            load,
            enable,
            state: Self::SAFE_DEFAULT,
        };
        sr.flush()?;
        Ok(sr)
    }

    /// Drives `SR_ENn` low, enabling all shift register outputs.
    pub fn enable_outputs(&mut self) {
        self.enable.set_low();
    }

    /// Drives `SR_ENn` high, tri-stating all shift register outputs.
    pub fn disable_outputs(&mut self) {
        self.enable.set_high();
    }

    /// The buffered (not necessarily latched) chain state.
    pub fn state(&self) -> u32 {
        self.state
    }

    /// Sets the raw logic level of one output in the buffered state.
    pub fn set_level(&mut self, out: SrOutput, high: bool) {
        if high {
            self.state |= out.mask();
        } else {
            self.state &= !out.mask();
        }
    }

    /// Reads the raw logic level of one output from the buffered state.
    pub fn level(&self, out: SrOutput) -> bool {
        self.state & out.mask() != 0
    }

    /// Turns an LED on or off, accounting for its polarity.
    pub fn set_led(&mut self, led: Led, on: bool) {
        let (out, active_low) = led.output();
        self.set_level(out, on ^ active_low);
    }

    /// Configures an `A` connector: `analog = true` enables the 10kΩ/20kΩ
    /// divider (0-10v reads as 0-3.3v); `analog = false` makes it a digital
    /// input with a 5v pull-up via 5kΩ.
    pub fn set_ain_mode(&mut self, input: AnalogIn, analog: bool) {
        let out = match input {
            AnalogIn::A09 => SrOutput::Cfg09AinDin,
            AnalogIn::A10 => SrOutput::Cfg10AinDin,
            AnalogIn::A11 => SrOutput::Cfg11AinDin,
            AnalogIn::A12 => SrOutput::Cfg12AinDin,
        };
        self.set_level(out, analog);
    }

    /// Configures `IO0`: `analog = true` routes the DAC (`AOUT00`) to the
    /// 4-20mA output; `analog = false` uses it as a HP digital output /
    /// digital input.
    pub fn set_io0_analog(&mut self, analog: bool) {
        self.set_level(SrOutput::Cfg00DioAout, !analog);
    }

    /// Configures a COM port as UART (`true`) or SPI (`false`).
    pub fn set_com_uart(&mut self, port: ComPort, uart: bool) {
        let out = match port {
            ComPort::Com0 => SrOutput::CfgCom0UartSpi,
            ComPort::Com1 => SrOutput::CfgCom1UartSpi,
        };
        self.set_level(out, uart);
    }

    /// Sets a COM port's polarity inversion (for e.g. inverted-TTL serial).
    pub fn set_com_inverted(&mut self, port: ComPort, inverted: bool) {
        let out = match port {
            ComPort::Com0 => SrOutput::CfgCom0Polarity,
            ComPort::Com1 => SrOutput::CfgCom1Polarity,
        };
        self.set_level(out, inverted);
    }

    /// Enables or disables motor connector `n` (0-3).
    ///
    /// # Panics
    /// Panics if `n > 3`.
    pub fn set_motor_enable(&mut self, n: u8, enabled: bool) {
        let out = match n {
            0 => SrOutput::Mtr0Enable,
            1 => SrOutput::Mtr1Enable,
            2 => SrOutput::Mtr2Enable,
            3 => SrOutput::Mtr3Enable,
            _ => panic!("no such motor"),
        };
        self.set_level(out, enabled);
    }

    /// Shifts out the buffered state and pulses `SR_LOAD` to latch it onto
    /// the outputs. Returns the *previous* chain contents, read back through
    /// `SR_DATA_RET`.
    pub fn flush(&mut self) -> Result<u32, SrError> {
        // First byte out lands in the last register (SR3); MSB of each byte
        // lands in Q7.
        let mut buf = self.state.to_be_bytes();
        self.spi.transfer_in_place(&mut buf)?;
        self.spi.flush()?;
        self.load.set_high();
        // 74HC595 t_w(RCLK) is tens of ns; two GPIO writes at 120MHz are
        // comfortably slower than that.
        self.load.set_low();
        Ok(u32::from_be_bytes(buf))
    }

    /// Like [`flush`](Self::flush), but shifts the state out twice and checks
    /// that the second pass reads back exactly what the first pass wrote,
    /// verifying the chain hasn't desynced. The outputs are latched on both
    /// passes (with identical data, so this is invisible externally).
    pub fn flush_verified(&mut self) -> Result<(), SrError> {
        self.flush()?;
        let got = self.flush()?;
        if got != self.state {
            return Err(SrError::ReadbackMismatch {
                expected: self.state,
                got,
            });
        }
        Ok(())
    }

    /// Releases the underlying resources. `SR_ENn` is driven high first.
    pub fn free(mut self) -> (SrSpi, SrLoad, SrEn) {
        self.disable_outputs();
        (self.spi, self.load, self.enable)
    }
}

/// Builds the SERCOM6 SPI master and wraps it, `SR_LOAD`, and `SR_ENn` into a
/// [`ShiftRegisters`] driver. 4MHz is a comfortable baud rate for a
/// 74HC595-class chain at 3.3v.
pub fn shift_registers(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom6: pac::Sercom6,
    mclk: &mut pac::Mclk,
    sck: impl Into<SrClk>,
    data: impl Into<SrData>,
    data_ret: impl Into<SrDataRet>,
    load: impl Into<SrLoad>,
    enable: impl Into<SrEn>,
) -> Result<ShiftRegisters, SrError> {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom6_core(&gclk0).unwrap();
    let pads = spi::Pads::default()
        .data_in(data_ret.into())
        .data_out(data.into())
        .sclk(sck.into());
    let spi = spi::Config::new(mclk, sercom6, pads, clock.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable();
    ShiftRegisters::new(spi, load.into(), enable.into())
}

// ----------------------------------------------------------------------------
// Analog scaling helpers
// ----------------------------------------------------------------------------

/// DAC counts per mA on `AOUT00` with a 2.5v reference (from the schematic:
/// `DATA0 = mA * 84.664`).
pub const AOUT00_COUNTS_PER_MA: f32 = 84.664;

/// Maximum usable `AOUT00` code. The schematic calls this an "11-bit value"
/// (full range 0.0 - 24.1mA), even though the DAC data register is 12 bits.
pub const AOUT00_MAX_COUNTS: u16 = 0x7FF;

/// Converts a desired `IO0` analog output current (mA) to a DAC `DATA0`
/// value, clamped to `0..=`[`AOUT00_MAX_COUNTS`]. Note the connector's
/// nominal range is 4-20mA.
pub fn aout00_counts_for_ma(ma: f32) -> u16 {
    let counts = ma * AOUT00_COUNTS_PER_MA;
    if counts <= 0.0 {
        0
    } else if counts >= AOUT00_MAX_COUNTS as f32 {
        AOUT00_MAX_COUNTS
    } else {
        counts as u16
    }
}

/// Converts a pin voltage on an `A` connector input (PB05-PB07, PC03, with
/// the divider enabled via `CfgXX_AIN_DINn`) to the connector voltage.
/// 10kΩ top / 20kΩ bottom: `Vin = Vpin * 1.5` (0-10v ↦ 0-3.3v).
pub fn analog_in_volts(v_pin: f32) -> f32 {
    v_pin * 1.5
}

/// Converts the `Vsupply_MON` (PC02) pin voltage to the supply voltage.
/// 47kΩ / 2kΩ divider: `Vsupply = Vpin * 24.5` (24.00v reads as 0.9796v).
pub fn vsupply_volts(v_pin: f32) -> f32 {
    v_pin * (49.0 / 2.0)
}

/// Converts the `5VOB_MON` (PB04) pin voltage to the 5VOB bus voltage.
/// 5kΩ / 5kΩ divider: `V5ob = Vpin * 2`.
pub fn v5ob_volts(v_pin: f32) -> f32 {
    v_pin * 2.0
}
