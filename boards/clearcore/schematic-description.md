ATSAME53N19A

### PA00
Connected to `OUT00` which: when pulled low, drives `IO0` low. When `IO0` is in digital output mode, it is pulled up to 24v through a 10kΩ resistor
### PA01
Connected to `OUT01` which: when pulled low, drives `IO1` low. `IO1` is pulled up to 24v through a 10kΩ resistor
### PA02
Connected to `AOUT00` which drives the 4-20mA Analog output on `OUT00` when is in analog mode. Schematic says "with a reference of 2.5V, the required 11-bit value required for a given current is: `DATA0 = mA * 84.664` Full range is 0.0 - 24.1mA with a 2.5V reference." `DATA0` is a register on the DAC peripheral (though its 12 bits?) Whether `IO0` is set to analog out or digital in/out is controlled by `Cfg00_DIO_AOUTn`.
### PA03
Connected to `IN02n` which directly connects to `IO2` through a 169kΩ resistor. All digital IO is active low and pulled up to 24v
### PA04
Connected to `MicroSD_SS` which is pulled up to 3.3v and then goes through a SN74AHCT125 which despite being a 5v part, is powered on 3.3v, and then finally to the `CD/DAT3/SPI_CS` pin of the SD card.
### PA05
Connected to `IN00n_{AOUT00n}` which acts as the digital input for `IO0`
### PA06
Connected to `OUT02` which behaves the same as `OUT01` but for `IO2` instead of `IO1`
### PA07
Connected to `OUT03`. Same behavior as `OUT01` and `OUT02`
### PA08
Connected to `Com1_TX_{MOSI}` which connects to TX/MOSI on the `COM1` connector. When `CfgCom1_Polarity` is pulled high, it's output is inverted. `Com1_TX_{MOSI}` is always pulled low.
### PA09
Connected to `Com1_RX_{SCK}` which has 2 modes. `CfgCom1_UART_SPIn` decides which it is. In 'RX mode', the RX/SCK pin on the `COM1` connector is connected to `Com1_RX_{SCK}` through an XOR that can invert the incoming signal (`CfgCom1_Polarity`). In 'SCK mode', it is connected through a buffer to RX/SCK on `COM1`.
### PA10
Connected to `Com1_RTS_{SS}`. Connects to the RTS/SS pin on the `COM1` connector. It is also inverted by `CfgCom1_Polarity` before reacing the `COM1` connector, and is pulled low before the gate.
### PA11
Connected to `Com1_CTS_{MISO}`. This is the ouput of the XOR on CTS/MISO of the `COM1` connector, meaning it can be inverted depending on `CfgCom1_Polarity`.
### PA12
Connected to `PHY_RXD1`. This is connected to the `KSZ8081RNACA` ethernet chip on the `RXD1` pin.
### PA13
Connected to `PHY_RXD0`. This is connected to the `KSZ8081RNACA` ethernet chip on the `RXD0` pin.
### PA14
Connected to `PHY_TXCLK`. This is connected to the `KSZ8081RNACA` ethernet chip on the `REF_CLK` pin.
### PA15
Connected to `PHY_RXER`. This is connected to the `KSZ8081RNACA` ethernet chip on the `RXER` pin, pulled low through a 2KΩ resistor.
### PA16
Connected to `Mtr3_An_{Sdrvr3_PWMA}` which is pulled up to 5v through a 5KΩ resistor and inverted before going to motor output A1
### PA17
Connected to `PHY_TXEN`. This is connected to the `KSZ8081RNACA` ethernet chip on the `TXEN` pin.
### PA18
Connected to `PHY_TXD0`. This is connected to the `KSZ8081RNACA` ethernet chip on the `TXD0` pin.
### PA19
Connected to `PHY_TXD1`. This is connected to the `KSZ8081RNACA` ethernet chip on the `TXD1` pin.
### PA20
`Mtr0_HLFB_{SCRx}`
### PA21
`Mtr1_An`
### PA22
`Mtr2_An_{Sdrvr2_PWMA}`
### PA23
`Mtr0_An_{SCTx}`
### PA24
Connects to `USB_N` through some filtering.
### PA25
Connects to `USB_P` through some filtering.
### PA27
Pulled to 3.3v through a 5KΩ resistor before connecting to `Mtr_CLK_01`. This signal provides the clock for step and direction of motors 0 and 1, while other pins decide the duty cycle by masking the output.
### PA30
Connects to `SWCLK` on the debug connector
### PA31
Connects to `SWDIO` on the debug connector
### PB00
Connected to `OutFault_04or05`
From the schem: "The fault output pulls low if ANY of the 4 output channels is shorted, or in an overtemp orovervoltage condition. When a channel is shorted, all otherchannels remain operating until a reset or power cycle. Any fault can be cleared by a reset or
power cycle also."
This comes from the driver for the `IO4` and `IO5` pins, and is pulled to 3.3v via a 5KΩ resistor.
### PB01
Connected to `SR_ENn`. This is the enable pin for all 4 shift registers in the chain. Pulled to 3.3v via a 5KΩ resistor, and enables all shift register outputs when pulled low
### PB02
Connected to `SR_LOAD`, or `RCLK`/`SCTP` of each shift register in the chain.
### PB03
Connected to `OUT05_{ENABLE05}`. Pulled low via a 5KΩ resistor. Connects to the enable pin for the primary output of connector `IO5`. `IO4` and `IO5` are actually H-bridge outputs, unlike most `IOn` connectors which have a +24v, GND, and signal pin (NPN output), `IO4` and `IO5` can drive the signal to 24v or GND (or floating when `OUT05_{ENABLE05}` is low). They also can do the same on what is normally the +24V pin. This `IO5+` (or `IO4+`) pin is also on the H-bridge driver but enable is always on, and the polarity of `IO5+` is driven by `Polarity05_{PWM05A}`
### PB04
Connected to `5VOB_MON`. This signal connects to the 5VOB bus with a 5KΩ/5KΩ resistor voltage divider. It can be used to monitor the voltage of the bus.
### PB05
Connects to `IN11n_{AIN11}` which is the analog input pin for `A11`. It has a 10KΩ/20KΩ resistor voltage divider, so the expected 0-10v input is read from 0 to 3.3v. This configuration can also change. When an input's associated `CfgXX_AIN_DINn` line is set high, the voltage divider described above is active. When the `CfgXX_AIN_DINn` is low, the voltage divider is not used, and `A11` (or any other `A` connector) is treated as a digital input with a 5v pullup via 5KΩ.
### PB06
Connects to `IN10n_{AIN10}` which behaves the same as `IN11n_{AIN11}` but for `A10`.
### PB07
Connects to `IN09n_{AIN09}` which behaves the same as `IN11n_{AIN11}` but for `A09.
### PB08
Connects to `MicroSD_MOSI` which connects through a buffer to the `CMD/SPI_DI` pin of the SD card slot
### PB09
Connects to `MicroSD_SCK` which connects through a buffer to the `CLK/SPI_SCLK` pin of the SD card slot
### PB10
Connects to `MicroSD_MISO` which comes from `DAT0/SPI_DO` on the SD card before being pulled up to 3.3v via 20KΩ and then passed through the same buffer.
### PB11
`Mtr1_HLFB`
### PB12
Connects to `Polarity05_{PWM05A}`. When high, `IO5+` is driven to 24v. This is the default, unless `IO5` is to be used as an H-bridge.
### PB13
Connects to `Polarity05S_{PWM05B}` which is pulled up to 3.3v via 5KΩ. This goes to the input for the `IO5-` H-bridge channel. Unless `IO5` is to be used as an H-bridge, this can be treated like the primary output for `IO5`
### PB14
Connects to `Polarity04_{PWM04A}`. `IO4` is identical to `IO5`, see `Polarity05_{PWM05A}`
### PB15
Connects to `Polarity04S_{PWM04B}`. `IO4` is identical to `IO5`, see `Polarity05S_{PWM05B}`
### PB16
Connects to `OUT04_{ENABLE04}`. `IO4` is identical to `IO5`, see `OUT05_{ENABLE05}`
### PB17
Connects to `IN01n`. This is the digital input for `IO1`. It can be used simultaneously with `OUT01`, but it will just be reading the same net we are forcibly pulling low. When `OUT01` is floating or high, `IO1` is pulled up to +24v via 10KΩ. `IN01n` is protected by a 169KΩ resistor so when `IO1` is not connected to anything, it should read 1.34v.
### PB18
Connects to `Com0_RTS_{SS}`. `COM0` is identical to `COM1` so see above.
### PB19
Connects to `Com0_CTS_{MISO}`. `COM0` is identical to `COM1` so see above.
### PB20
Connects to `Com0_RX_{SCK}`. `COM0` is identical to `COM1` so see above.
### PB21
Connects to `Com0_TX_{MOSI}`. `COM0` is identical to `COM1` so see above.
### PB22
Connects to `SYS_CLK` which is the output of a 25Mhz crystal. `SYS_CLK` also connects to `XI` on the ethernet PHY.
### PB23
`Mtr_CLK_23`
### PB24
Connects to `XBee_Rx_IN`. `DOUT/DIO13` of the XBEE connector is pulled up to 3.3v and then goes through a buffer before becoming `XBee_Rx_IN`.
### PB25
Connects to `XBee_Tx_OUT` before getting pulled up to 3.3v via 5KΩ and going through a buffer to reach `DIN/CONFIG/DIO14` on the XBEE slot.
### PB30
Connects to `SWO/TDO` on the debug connector
### PB31
`Mtr3_HLFB_{Sdrvr3_Trig}`
### PC00
`Sdrvr3_iMON`
### PC01
`Sdrvr2_iMON`
### PC02
Connects to `Vsupply_MON_{IO_4and5_RST}`. From the schematic:
"This is set to trip at about 36.2V and will reset with a hysteresis of 0.4V. `OverV_DISABLEn` only disables the motor driver IC, and isn't read by the MCU. However, the MCU can also disable the motor driver IC by briefly setting `Vsupply-MON_{IO_4and5_RST}` as an output and pulling it high."

For context, `OverV_DISABLEn` trips when the voltage is too high and disables the H-bridge driver for `IO4`/`IO5`. You can force this behavior as described above, or just read this as an analog signal. Voltage divider as follows: `Vsupply` (ostensibly 24v) -> 47KΩ -> {This net} -> 2KΩ -> `GND`, making 24.00v read as 0.9796v on this pin.
### PC03
`IN12n_{AIN12}`, see `IN11n_{AIN11}`.
### PC05
Connects to `SR_CLK`, which is the `SRCLK` pin on all 4 shift registers in the chain.
### PC06
Connects to `SR_DATA_RET` which is the `Q7'` or 'carry bit' of the last shift register. Unclear why this is needed, maybe to verify no desyncs occurred?
### PC07
Connects to `SR_DATA` which feeds the first shift register.
### PC10
`Mtr3_B_{Sdrvr3_PWMB}`
### PC11
Connects to `PHY_MDC` which directly connects to the `MDC` pin of the ethernet PHY
### PC12
Connects to `PHY_MDIO`, which is pulled up to 3.3v via 2KΩ and then connected to `MDIO` on the ethernet PHY.
### PC13
`Mtr2_B_{Sdrvr2_PWMB}`
### PC14
`Mtr0_B`
### PC15
`Mtr1_B`
### PC16
Connects to `IN06n_{QuadA}` which is the digital input for the `DI6` connector. It can also be used for a quadrature encoder, but there is no specific hardware support, just optimized routing and a pullup res that only goes to 5v.
### PC17
`IN07n_{QuadB}`, see above. Identical behavior
### PC18
`IN08n_{QuadI}`, see above. Identical behavior
### PC19
Connects to `IN05n` which serves as the digital input pin for `IO5`. Once again, this board is a little strange, while the user facing `IO5`(-) signal can be driven high or low or left floating (pulled up to 24v via 10KΩ), a different pin (this one) read the signal as if it was digital. When reading, it is implied that the output is only using the resistor pull up, although it is always safe to read.
### PC20
Connects to `PHY_RXDV` which is pulled to ground via 2KΩ and then connected to `CRS_DV/PHYAD` on the ethernet PHY.
### PC21
`IN03n` is the digital input for `IO3`
### PC24
Connects to `XBee_RTS_OUT` which is pulled up to 3.3v via 5KΩ and then connected to `RTS/DIO6` on the XBEE slot. This signal also goes through the buffer.
### PC25
Connects to `XBee_CTS_IN` which originates at the `CTS/DIO7` pin of the XBEE slot, before being pulled up to 3.3v via 20KΩ and going through the buffer.
### PC26
`Mtr2_HLFB_{Sdrvr2_Trig}`
### PC27
`IN04n` is like `IN05n` but for `IO4`.
### PC28
Connects to `PHY_INT` which is pulled up to 3.3v via 2KΩ and connected to `INTRP` on the ethernet PHY.


## Shift register outputs
#### SR0-Q0
`Cfg09_AIN_DINn` From the schematic:
"These go to the Analog_Digital_In page. Set high to configure the associated input as analog (divider enabled), set low for digital (pull-up enabled)." Explained further at `IN11n_{AIN11}`
#### SR0-Q1
`Cfg10_AIN_DINn` - See above
#### SR0-Q2
`Cfg11_AIN_DINn` - See above
#### SR0-Q3
`Cfg12_AIN_DINn` - See above
#### SR0-Q4
`LED09n` - `A09` indicator. Active low.
#### SR0-Q5
`LED10n` - `A10` indicator. Active low.
#### SR0-Q6
`LED11n` - `A11` indicator. Active low.
#### SR0-Q7
`LED12n` - `A12` indicator. Active low.
#### SR1-Q0
`LED08n` - `DI8` indicator. Active low.
#### SR1-Q1
`LED07n` - `DI7` indicator. Active low.
#### SR1-Q2
`LED06n` - `DI6` indicator. Active low.
#### SR1-Q3
`Cfg00_DIO_AOUTn` From schem: "This goes to the Analog_Out page. Set high to use `IO0` as a HP output or a digital input, set low to use as a 4-20mA analog output."
#### SR1-Q4
`LED_Com1` - Led indicator on `COM1`. Set high to turn on the LED.
#### SR1-Q5
`LED_Com0` - See above
#### SR1-Q6
`CfgCom0_UART_SPIn` - See `CfgCom1_UART_SPIn` below
#### SR1-Q7
`CfgCom1_UART_SPIn` - From schem: "These go to the Serial_Interfaces page. Set high to configure the associated COM port as UART, set low for SPI." More details under `Com1_RX_{SCK}`.
#### SR2-Q0
`LED_User` - The red 'user controlled' LED. Active high.
#### SR2-Q1
`LED_Underglow` - Leds are on when this signal is high, but it's also pulled up, so must be driven low to have any effect.
#### SR2-Q2
`CfgCom0_Polarity` - See `CfgCom1_Polarity` below
#### SR2-Q3
`CfgCom1_Polarity` - Inverts `COM1` pins. See any `COM1` pin definition to see how.
#### SR2-Q4
`Mtr0_ENable`
#### SR2-Q5
`Mtr1_ENable`
#### SR2-Q6
`Mtr2_ENable`
#### SR2-Q7
`Mtr3_ENable`
#### SR3-Q0
`LED00n` - The LED for the `IO0` connector. Active low.
#### SR3-Q1
`LED01n` - See above
#### SR3-Q2
`LED02n` - See above
#### SR3-Q3
`LED03n` - See above
#### SR3-Q4
`LED04n` - The LED for the `IO4` connector. __Active high__.
#### SR3-Q5
`LED05n` - The LED for the `IO5` connector. __Active high__.
#### SR3-Q6
`CfgM2_Mtr_SDRVRn`
#### SR3-Q7
`CfgM3_Mtr_SDRVRn`


## Sercom allocation

#### COM0
PB18-21
Sercom 7
#### COM1
PA08-11
Sercom 0
#### XBEE
PB24,PB25,PC24,PC25,
Sercom 2
#### SD card
PB08-10, PA04
Sercom 4
