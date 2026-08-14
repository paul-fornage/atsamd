//! ClearCore board devices
//! Just helpers and stuff

use super::hal;
use super::pins::*;

#[cfg(feature = "usb")]
use atsamd_hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusAllocator;

use hal::clock::GenericClockController;
use hal::pac;
use hal::sercom::uart::{self, BaudMode, Oversampling};
use hal::sercom::{spi, Sercom0, Sercom2, Sercom4, Sercom6, Sercom7};
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
pub type SrSpiPads = spi::Pads<Sercom6, SrDataRet, SrData, SrClk>;
/// SPI master driving the shift register chain.
pub type SrSpi = spi::Spi<spi::Config<SrSpiPads>, spi::Duplex>;

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
) -> SrSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom6_core(&gclk0).unwrap();
    let pads = spi::Pads::default()
        .data_in(data_ret.into())
        .data_out(data.into())
        .sclk(sck.into());
    spi::Config::new(mclk, sercom6, pads, clock.freq())
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    mclk: &mut pac::Mclk,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::Srcselect, pchctrl::Genselect};

    clocks.configure_gclk_divider_and_source(Genselect::Gclk2, 1, Srcselect::Dfll, false);
    let usb_gclk = clocks.get_gclk(Genselect::Gclk2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}

