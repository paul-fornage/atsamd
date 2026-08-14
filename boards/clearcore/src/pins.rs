//! ClearCore Pin Definitions
//!
//! MCU: ATSAME53N19A
//!
//! Motor pins not considered yet.

use super::hal;

hal::bsp_pins!(
    PA00 {
        /// PA00: Digital output 0 (`OUT00`). When pulled low, drives `IO0`
        /// low. When `IO0` is in digital output mode, it is pulled up to 24v
        /// through a 10kΩ resistor. Only meaningful when `IO0` is in digital
        /// mode (`Cfg00_DIO_AOUTn` set high on the shift register chain).
        name: out00,
        aliases: {
            PushPullOutput: Out00
        }
    }
    PA01 {
        /// PA01: Digital output 1 (`OUT01`). When pulled low, drives `IO1`
        /// low. `IO1` is pulled up to 24v through a 10kΩ resistor.
        name: out01,
        aliases: {
            PushPullOutput: Out01
        }
    }
    PA02 {
        /// PA02: Analog output 0 (`AOUT00`), DAC `VOUT[0]`. Drives the 4-20mA
        /// analog output on `IO0` when it is in analog mode
        /// (`Cfg00_DIO_AOUTn` set low). From the schematic: "with a reference
        /// of 2.5V, the required 11-bit value required for a given current is:
        /// `DATA0 = mA * 84.664`. Full range is 0.0 - 24.1mA with a 2.5V
        /// reference." (`DATA0` is the 12-bit DAC data register; the usable
        /// range here simply tops out below full scale.)
        name: aout00,
        aliases: {
            AlternateB: Aout00
        }
    }
    PA03 {
        /// PA03: Digital input 2 (`IN02n`). Directly connects to `IO2` through
        /// a 169kΩ resistor. All digital IO is active-low and pulled up to
        /// 24v.
        name: in02,
        aliases: {
            FloatingInput: In02
        }
    }
    PA04 {
        /// PA04: Micro SD chip select (`MicroSD_SS`). Pulled up to 3.3v, then
        /// buffered through a SN74AHCT125 (a 5v part run at 3.3v) to the
        /// `CD/DAT3/SPI_CS` pin of the SD card. Not a SERCOM4 pad, so CS must
        /// be software controlled.
        name: micro_sd_ss,
        aliases: {
            PushPullOutput: MicroSdCs
        }
    }
    PA05 {
        /// PA05: Digital input 0 (`IN00n_{AOUT00n}`). Digital input for `IO0`,
        /// active-low, pulled up to 24v via 10kΩ, protected by 169kΩ.
        ///
        /// NOTE: PA05 is also ADC0 `AIN[5]` / DAC `VOUT[1]`; the `{AOUT00n}`
        /// suffix suggests it can monitor `IO0` while in analog output mode,
        /// hence the analog alias. Unverified.
        name: in00,
        aliases: {
            FloatingInput: In00
            AlternateB: Ain00
        }
    }
    PA06 {
        /// PA06: Digital output 2 (`OUT02`). When pulled low, drives `IO2`
        /// low. `IO2` is pulled up to 24v through a 10kΩ resistor.
        name: out02,
        aliases: {
            PushPullOutput: Out02
        }
    }
    PA07 {
        /// PA07: Digital output 3 (`OUT03`). When pulled low, drives `IO3`
        /// low. `IO3` is pulled up to 24v through a 10kΩ resistor.
        name: out03,
        aliases: {
            PushPullOutput: Out03
        }
    }
    PA08 {
        /// PA08: `Com1_TX_{MOSI}`, SERCOM0 PAD\[0\]. TX/MOSI on the `COM1`
        /// connector. Output is inverted when `CfgCom1_Polarity` is high.
        /// Always pulled low before the gate.
        name: com1_tx_mosi,
        aliases: {
            AlternateC: Com1Tx
            AlternateC: Com1Mosi
        }
    }
    PA09 {
        /// PA09: `Com1_RX_{SCK}`, SERCOM0 PAD\[1\]. Two modes, selected by
        /// `CfgCom1_UART_SPIn`: in 'RX mode', RX/SCK on `COM1` reaches this
        /// pin through an XOR that can invert it (`CfgCom1_Polarity`); in
        /// 'SCK mode', this pin drives RX/SCK on `COM1` through a buffer.
        name: com1_rx_sck,
        aliases: {
            AlternateC: Com1Rx
            AlternateC: Com1Sck
        }
    }
    PA10 {
        /// PA10: `Com1_RTS_{SS}`, SERCOM0 PAD\[2\]. Drives the RTS/SS pin on
        /// the `COM1` connector, inverted by `CfgCom1_Polarity` before
        /// reaching the connector. Pulled low before the gate.
        name: com1_rts_ss,
        aliases: {
            AlternateC: Com1Rts
            AlternateC: Com1Ss
        }
    }
    PA11 {
        /// PA11: `Com1_CTS_{MISO}`, SERCOM0 PAD\[3\]. Output of the XOR on
        /// CTS/MISO of the `COM1` connector, so it can be inverted by
        /// `CfgCom1_Polarity`.
        name: com1_cts_miso,
        aliases: {
            AlternateC: Com1Cts
            AlternateC: Com1Miso
        }
    }
    PA12 {
        /// PA12: `PHY_RXD1`. GMAC `GRX1`, connected to the KSZ8081RNACA
        /// ethernet PHY `RXD1` pin.
        name: phy_rxd1,
        aliases: {
            AlternateL: EthRxd1
        }
    }
    PA13 {
        /// PA13: `PHY_RXD0`. GMAC `GRX0`, connected to the KSZ8081RNACA
        /// ethernet PHY `RXD0` pin.
        name: phy_rxd0,
        aliases: {
            AlternateL: EthRxd0
        }
    }
    PA14 {
        /// PA14: `PHY_TXCLK`. GMAC `GTXCK`, connected to the KSZ8081RNACA
        /// ethernet PHY `REF_CLK` pin (RMII reference clock).
        name: phy_txclk,
        aliases: {
            AlternateL: EthRefClk
        }
    }
    PA15 {
        /// PA15: `PHY_RXER`. GMAC `GRXER`, connected to the KSZ8081RNACA
        /// ethernet PHY `RXER` pin, pulled low through a 2kΩ resistor.
        name: phy_rxer,
        aliases: {
            AlternateL: EthRxer
        }
    }
    PA16 {
        /// PA16: `Mtr3_An_{Sdrvr3_PWMA}`. Pulled up to 5v through a 5kΩ
        /// resistor and inverted before going to motor output A1.
        name: mtr3_a,
    }
    PA17 {
        /// PA17: `PHY_TXEN`. GMAC `GTXEN`, connected to the KSZ8081RNACA
        /// ethernet PHY `TXEN` pin.
        name: phy_txen,
        aliases: {
            AlternateL: EthTxen
        }
    }
    PA18 {
        /// PA18: `PHY_TXD0`. GMAC `GTX0`, connected to the KSZ8081RNACA
        /// ethernet PHY `TXD0` pin.
        name: phy_txd0,
        aliases: {
            AlternateL: EthTxd0
        }
    }
    PA19 {
        /// PA19: `PHY_TXD1`. GMAC `GTX1`, connected to the KSZ8081RNACA
        /// ethernet PHY `TXD1` pin.
        name: phy_txd1,
        aliases: {
            AlternateL: EthTxd1
        }
    }
    PA20 {
        /// PA20: `Mtr0_HLFB_{SCRx}`. Motor 0 high-level feedback.
        name: mtr0_hlfb,
    }
    PA21 {
        /// PA21: `Mtr1_An`. Motor 1 output A.
        name: mtr1_a,
    }
    PA22 {
        /// PA22: `Mtr2_An_{Sdrvr2_PWMA}`. Motor 2 output A.
        name: mtr2_a,
    }
    PA23 {
        /// PA23: `Mtr0_An_{SCTx}`. Motor 0 output A.
        name: mtr0_a,
    }
    PA24 {
        /// PA24: USB Data-. Connects to `USB_N` through some filtering.
        name: usb_dm,
        aliases: {
            AlternateH: UsbDm
        }
    }
    PA25 {
        /// PA25: USB Data+. Connects to `USB_P` through some filtering.
        name: usb_dp,
        aliases: {
            AlternateH: UsbDp
        }
    }
    PA27 {
        /// PA27: `Mtr_CLK_01`. Pulled to 3.3v through a 5kΩ resistor. Provides
        /// the clock for step and direction of motors 0 and 1, while other
        /// pins decide the duty cycle by masking the output.
        name: mtr_clk_01,
    }
    PA30 {
        /// PA30: `SWCLK` on the debug connector.
        name: swclk,
        aliases: {
            AlternateH: Swclk
        }
    }
    PA31 {
        /// PA31: `SWDIO` on the debug connector.
        name: swdio,
        aliases: {
            AlternateH: Swdio
        }
    }
    PB00 {
        /// PB00: `OutFault_04or05`, from the `IO4`/`IO5` H-bridge driver. From
        /// the schematic: "The fault output pulls low if ANY of the 4 output
        /// channels is shorted, or in an overtemp or overvoltage condition.
        /// When a channel is shorted, all other channels remain operating
        /// until a reset or power cycle. Any fault can be cleared by a reset
        /// or power cycle also."
        name: out_fault_04or05,
        aliases: {
            FloatingInput: OutFault
        }
    }
    PB01 {
        /// PB01: `SR_ENn`. Output-enable for all 4 shift registers in the
        /// chain. Pulled to 3.3v via a 5kΩ resistor; enables all shift
        /// register outputs when pulled low.
        name: sr_en,
        aliases: {
            PushPullOutput: SrEn
        }
    }
    PB02 {
        /// PB02: `SR_LOAD`. The `RCLK`/`SCTP` (storage register clock) pin of
        /// each shift register in the chain. Pulse high to latch the shifted
        /// bits onto the outputs.
        name: sr_load,
        aliases: {
            PushPullOutput: SrLoad
        }
    }
    PB03 {
        /// PB03: `OUT05_{ENABLE05}`. Pulled low via a 5kΩ resistor. Enable pin
        /// for the primary output of connector `IO5`. `IO4` and `IO5` are
        /// H-bridge outputs: unlike most `IOn` connectors (+24v, GND, and an
        /// NPN signal pin), they can drive the signal pin to 24v or GND (or
        /// float it when this pin is low). The same is possible on what is
        /// normally the +24v pin (`IO5+`), whose driver enable is always on
        /// and whose polarity is driven by `Polarity05_{PWM05A}`.
        name: out05,
        aliases: {
            PushPullOutput: Out05
        }
    }
    PB04 {
        /// PB04: `5VOB_MON`, ADC1 `AIN[6]`. Connects to the 5VOB bus through a
        /// 5kΩ/5kΩ resistor voltage divider; can be used to monitor the bus
        /// voltage (reads at half the bus voltage).
        name: mon_5vob,
        aliases: {
            AlternateB: Mon5Vob
        }
    }
    PB05 {
        /// PB05: `IN11n_{AIN11}`, ADC1 `AIN[7]`. Analog input pin for `A11`.
        /// Has a 10kΩ/20kΩ voltage divider so the expected 0-10v input reads
        /// as 0-3.3v. When `Cfg11_AIN_DINn` is high, the divider is active;
        /// when low, the divider is disconnected and `A11` is a digital input
        /// with a 5v pull-up via 5kΩ.
        name: in11,
        aliases: {
            AlternateB: Ain11
            FloatingInput: In11
        }
    }
    PB06 {
        /// PB06: `IN10n_{AIN10}`, ADC1 `AIN[8]`. Behaves the same as
        /// `IN11n_{AIN11}` but for `A10` (see PB05).
        name: in10,
        aliases: {
            AlternateB: Ain10
            FloatingInput: In10
        }
    }
    PB07 {
        /// PB07: `IN09n_{AIN09}`, ADC1 `AIN[9]`. Behaves the same as
        /// `IN11n_{AIN11}` but for `A09` (see PB05).
        name: in09,
        aliases: {
            AlternateB: Ain09
            FloatingInput: In09
        }
    }
    PB08 {
        /// PB08: `MicroSD_MOSI`, SERCOM4 PAD\[0\]. Connects through a buffer
        /// to the `CMD/SPI_DI` pin of the SD card slot.
        name: micro_sd_mosi,
        aliases: {
            AlternateD: MicroSdMosi
        }
    }
    PB09 {
        /// PB09: `MicroSD_SCK`, SERCOM4 PAD\[1\]. Connects through a buffer to
        /// the `CLK/SPI_SCLK` pin of the SD card slot.
        name: micro_sd_sck,
        aliases: {
            AlternateD: MicroSdSck
        }
    }
    PB10 {
        /// PB10: `MicroSD_MISO`, SERCOM4 PAD\[2\] (alternate D on this pin).
        /// Comes from `DAT0/SPI_DO` on the SD card, pulled up to 3.3v via
        /// 20kΩ, then through the same buffer.
        name: micro_sd_miso,
        aliases: {
            AlternateD: MicroSdMiso
        }
    }
    PB11 {
        /// PB11: `Mtr1_HLFB`. Motor 1 high-level feedback.
        name: mtr1_hlfb,
    }
    PB12 {
        /// PB12: `Polarity05_{PWM05A}`. When high, `IO5+` is driven to 24v.
        /// This is the default, unless `IO5` is to be used as an H-bridge.
        ///
        /// NOTE: The `{PWM05A}` designator implies a timer output; PB12 is
        /// TC4/WO\[0\] (E) and, I believe, TCC3/WO\[0\] (F). The `AlternateF`
        /// alias is a best guess — verify against the datasheet mux table.
        name: polarity05,
        aliases: {
            PushPullOutput: Polarity05
            AlternateF: Pwm05A
        }
    }
    PB13 {
        /// PB13: `Polarity05S_{PWM05B}`. Pulled up to 3.3v via 5kΩ. Input for
        /// the `IO5-` H-bridge channel. Unless `IO5` is used as an H-bridge,
        /// this can be treated as the primary output for `IO5`.
        ///
        /// NOTE: `AlternateF` (TCC3/WO\[1\]) is a best guess, see PB12.
        name: polarity05s,
        aliases: {
            PushPullOutput: Polarity05S
            AlternateF: Pwm05B
        }
    }
    PB14 {
        /// PB14: `Polarity04_{PWM04A}`. `IO4` is identical to `IO5`, see
        /// `Polarity05_{PWM05A}` (PB12).
        ///
        /// NOTE: `AlternateF` (TCC4/WO\[0\]) is a best guess, see PB12.
        name: polarity04,
        aliases: {
            PushPullOutput: Polarity04
            AlternateF: Pwm04A
        }
    }
    PB15 {
        /// PB15: `Polarity04S_{PWM04B}`. `IO4` is identical to `IO5`, see
        /// `Polarity05S_{PWM05B}` (PB13).
        ///
        /// NOTE: `AlternateF` (TCC4/WO\[1\]) is a best guess, see PB12.
        name: polarity04s,
        aliases: {
            PushPullOutput: Polarity04S
            AlternateF: Pwm04B
        }
    }
    PB16 {
        /// PB16: `OUT04_{ENABLE04}`. `IO4` is identical to `IO5`, see
        /// `OUT05_{ENABLE05}` (PB03).
        name: out04,
        aliases: {
            PushPullOutput: Out04
        }
    }
    PB17 {
        /// PB17: `IN01n`. Digital input for `IO1`. Can be used simultaneously
        /// with `OUT01`, but it will just read the same net being forcibly
        /// pulled low. When `OUT01` is floating or high, `IO1` is pulled up to
        /// +24v via 10kΩ. Protected by a 169kΩ resistor, so when `IO1` is not
        /// connected to anything it should read 1.34v.
        name: in01,
        aliases: {
            FloatingInput: In01
        }
    }
    PB18 {
        /// PB18: `Com0_RTS_{SS}`, SERCOM7 PAD\[2\]. `COM0` is identical to
        /// `COM1`; see `Com1_RTS_{SS}` (PA10).
        name: com0_rts,
        aliases: {
            AlternateD: Com0Rts
            AlternateD: Com0Ss
        }
    }
    PB19 {
        /// PB19: `Com0_CTS_{MISO}`, SERCOM7 PAD\[3\]. `COM0` is identical to
        /// `COM1`; see `Com1_CTS_{MISO}` (PA11).
        name: com0_cts,
        aliases: {
            AlternateD: Com0Cts
            AlternateD: Com0Miso
        }
    }
    PB20 {
        /// PB20: `Com0_RX_{SCK}`, SERCOM7 PAD\[1\]. `COM0` is identical to
        /// `COM1`; see `Com1_RX_{SCK}` (PA09).
        name: com0_rx,
        aliases: {
            AlternateD: Com0Rx
            AlternateD: Com0Sck
        }
    }
    PB21 {
        /// PB21: `Com0_TX_{MOSI}`, SERCOM7 PAD\[0\]. `COM0` is identical to
        /// `COM1`; see `Com1_TX_{MOSI}` (PA08).
        name: com0_tx,
        aliases: {
            AlternateD: Com0Tx
            AlternateD: Com0Mosi
        }
    }
    PB22 {
        /// PB22: `SYS_CLK`, XOSC1 `XIN1`. Output of a 25MHz crystal
        /// oscillator; `SYS_CLK` also feeds `XI` on the ethernet PHY. Since
        /// this is a driven clock (not a crystal), configure XOSC1 in
        /// external clock mode and leave PB23 free.
        name: sys_clk,
        aliases: {
            FloatingDisabled: SysClk
        }
    }
    PB23 {
        /// PB23: `Mtr_CLK_23`. Clock for step and direction of motors 2 and 3;
        /// see `Mtr_CLK_01` (PA27).
        name: mtr_clk_23,
    }
    PB24 {
        /// PB24: `XBee_Rx_IN`, SERCOM2 PAD\[1\]. `DOUT/DIO13` of the XBEE
        /// connector is pulled up to 3.3v and buffered before becoming this
        /// signal.
        name: xbee_rx,
        aliases: {
            AlternateD: XbeeRx
        }
    }
    PB25 {
        /// PB25: `XBee_Tx_OUT`, SERCOM2 PAD\[0\]. Pulled up to 3.3v via 5kΩ
        /// and buffered before reaching `DIN/CONFIG/DIO14` on the XBEE slot.
        name: xbee_tx,
        aliases: {
            AlternateD: XbeeTx
        }
    }
    PB30 {
        /// PB30: `SWO/TDO` on the debug connector.
        name: swo,
        aliases: {
            AlternateH: Swo
        }
    }
    PB31 {
        /// PB31: `Mtr3_HLFB_{Sdrvr3_Trig}`. Motor 3 high-level feedback.
        name: mtr3_hlfb,
    }
    PC00 {
        /// PC00: `Sdrvr3_iMON`. Motor/step driver 3 current monitor.
        name: sdrvr3_imon,
    }
    PC01 {
        /// PC01: `Sdrvr2_iMON`. Motor/step driver 2 current monitor.
        name: sdrvr2_imon,
    }
    PC02 {
        /// PC02: `Vsupply_MON_{IO_4and5_RST}`, ADC1 `AIN[4]`. From the
        /// schematic: "This is set to trip at about 36.2V and will reset with
        /// a hysteresis of 0.4V. `OverV_DISABLEn` only disables the motor
        /// driver IC, and isn't read by the MCU. However, the MCU can also
        /// disable the motor driver IC by briefly setting
        /// `Vsupply-MON_{IO_4and5_RST}` as an output and pulling it high."
        ///
        /// `OverV_DISABLEn` trips when the voltage is too high and disables
        /// the H-bridge driver for `IO4`/`IO5`. Force that behavior as
        /// described above, or just read this as an analog signal. Voltage
        /// divider: `Vsupply` (ostensibly 24v) -> 47kΩ -> this net -> 2kΩ ->
        /// GND, so 24.00v reads as 0.9796v.
        name: vsupply_mon,
        aliases: {
            AlternateB: VsupplyMon
            PushPullOutput: Io45Rst
        }
    }
    PC03 {
        /// PC03: `IN12n_{AIN12}`, ADC1 `AIN[5]`. Behaves the same as
        /// `IN11n_{AIN11}` but for `A12` (see PB05).
        name: in12,
        aliases: {
            AlternateB: Ain12
            FloatingInput: In12
        }
    }
    PC05 {
        /// PC05: `SR_CLK`, SERCOM6 PAD\[1\]. The `SRCLK` pin on all 4 shift
        /// registers in the chain. With `SR_DATA` (PC07, PAD\[3\], DOPO=2) and
        /// `SR_DATA_RET` (PC06, PAD\[2\], DIPO=2) this forms a valid SERCOM6
        /// SPI master configuration.
        name: sr_clk,
        aliases: {
            AlternateC: SrClk
        }
    }
    PC06 {
        /// PC06: `SR_DATA_RET`, SERCOM6 PAD\[2\]. The `Q7'` ('carry bit') of
        /// the last shift register. Read back 32 clocks later, it should
        /// match what was shifted out — likely intended to verify no desyncs
        /// occurred.
        name: sr_data_ret,
        aliases: {
            AlternateC: SrDataRet
        }
    }
    PC07 {
        /// PC07: `SR_DATA`, SERCOM6 PAD\[3\]. Feeds the first shift register
        /// in the chain (SPI DO with DOPO=2).
        name: sr_data,
        aliases: {
            AlternateC: SrData
        }
    }
    PC10 {
        /// PC10: `Mtr3_B_{Sdrvr3_PWMB}`. Motor 3 output B.
        name: mtr3_b,
    }
    PC11 {
        /// PC11: `PHY_MDC`. GMAC `GMDC`, directly connected to the `MDC` pin
        /// of the ethernet PHY.
        name: phy_mdc,
        aliases: {
            AlternateL: EthMdc
        }
    }
    PC12 {
        /// PC12: `PHY_MDIO`. GMAC `GMDIO`, pulled up to 3.3v via 2kΩ and
        /// connected to `MDIO` on the ethernet PHY.
        name: phy_mdio,
        aliases: {
            AlternateL: EthMdio
        }
    }
    PC13 {
        /// PC13: `Mtr2_B_{Sdrvr2_PWMB}`. Motor 2 output B.
        name: mtr2_b,
    }
    PC14 {
        /// PC14: `Mtr0_B`. Motor 0 output B.
        name: mtr0_b,
    }
    PC15 {
        /// PC15: `Mtr1_B`. Motor 1 output B.
        name: mtr1_b,
    }
    PC16 {
        /// PC16: `IN06n_{QuadA}`, PDEC `QDI[0]`. Digital input for the `DI6`
        /// connector. Can also be used for a quadrature encoder via the
        /// MCU's position decoder peripheral; the board provides no specific
        /// hardware support, just optimized routing and a pull-up that only
        /// goes to 5v.
        name: in06,
        aliases: {
            FloatingInput: In06
            AlternateG: QuadA
        }
    }
    PC17 {
        /// PC17: `IN07n_{QuadB}`, PDEC `QDI[1]`. Digital input for `DI7`.
        /// Identical behavior to `IN06n_{QuadA}` (PC16).
        name: in07,
        aliases: {
            FloatingInput: In07
            AlternateG: QuadB
        }
    }
    PC18 {
        /// PC18: `IN08n_{QuadI}`, PDEC `QDI[2]`. Digital input for `DI8`.
        /// Identical behavior to `IN06n_{QuadA}` (PC16).
        name: in08,
        aliases: {
            FloatingInput: In08
            AlternateG: QuadIndex
        }
    }
    PC19 {
        /// PC19: `IN05n`. Digital input for `IO5`. While the user-facing
        /// `IO5`(-) signal can be driven high or low or left floating (pulled
        /// up to 24v via 10kΩ), this separate pin reads the signal as if it
        /// were digital. When reading, it is implied that the output is only
        /// using the resistor pull-up, although it is always safe to read.
        name: in05,
        aliases: {
            FloatingInput: In05
        }
    }
    PC20 {
        /// PC20: `PHY_RXDV`. GMAC `GRXDV`, pulled to ground via 2kΩ and
        /// connected to `CRS_DV/PHYAD` on the ethernet PHY.
        name: phy_rxdv,
        aliases: {
            AlternateL: EthCrsDv
        }
    }
    PC21 {
        /// PC21: `IN03n`. Digital input for `IO3`.
        name: in03,
        aliases: {
            FloatingInput: In03
        }
    }
    PC24 {
        /// PC24: `XBee_RTS_OUT`, SERCOM2 PAD\[2\]. Pulled up to 3.3v via 5kΩ,
        /// buffered, then connected to `RTS/DIO6` on the XBEE slot.
        name: xbee_rts,
        aliases: {
            AlternateD: XbeeRts
        }
    }
    PC25 {
        /// PC25: `XBee_CTS_IN`, SERCOM2 PAD\[3\]. Originates at `CTS/DIO7` on
        /// the XBEE slot, pulled up to 3.3v via 20kΩ, then buffered.
        name: xbee_cts,
        aliases: {
            AlternateD: XbeeCts
        }
    }
    PC26 {
        /// PC26: `Mtr2_HLFB_{Sdrvr2_Trig}`. Motor 2 high-level feedback.
        name: mtr2_hlfb,
    }
    PC27 {
        /// PC27: `IN04n`. Digital input for `IO4`; identical to `IN05n`
        /// (PC19) but for `IO4`.
        name: in04,
        aliases: {
            FloatingInput: In04
        }
    }
    PC28 {
        /// PC28: `PHY_INT`. Pulled up to 3.3v via 2kΩ and connected to
        /// `INTRP` on the ethernet PHY (open-drain, active-low).
        ///
        /// NOTE: An EIC interrupt alias is provided; verify PC28's EXTINT
        /// channel doesn't conflict with anything else you're using.
        name: phy_int,
        aliases: {
            FloatingInput: EthInt
            FloatingInterrupt: EthIntIrq
        }
    }
);
