#![no_std]
#![no_main]

use bsp::hal;
use clearcore as bsp;

use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use core::fmt::Write as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::usb::UsbBus;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cortex_m::peripheral::NVIC;

#[entry]
fn main() -> ! {
    // RTT uses the SWD debug connection; it does not use the target's USB.
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::devices::usb_allocator(
            pins.usb_dm,
            pins.usb_dp,
            peripherals.usb,
            &mut clocks,
            &mut peripherals.mclk,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut n = 0u32;
    loop {
        rprintln!("hello world {}", n);
        usb_println(format_args!("hello world {}", n));
        n = n.wrapping_add(1);
        delay.delay_ms(1_000u16);
        if n == 16 {
            panic!("WA WA WEE WAH");
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

/// Formats a line into a stack buffer and sends it out the USB serial port.
fn usb_println(args: core::fmt::Arguments) {
    let mut line = LineBuffer {
        buf: [0u8; 128],
        len: 0,
    };
    // Truncation on overflow is fine for a demo; CRLF plays nicest with
    // terminal emulators.
    let _ = write!(line, "{}\r\n", args);
    usb_write(&line.buf[..line.len]);
}

/// Best-effort write to the USB serial port. Bytes are silently dropped if
/// no host is connected (or its buffer stays full), so the loop keeps the
/// same timing as the RTT-only example.
fn usb_write(mut bytes: &[u8]) {
    while !bytes.is_empty() {
        // The USB interrupts also touch USB_SERIAL, so take a critical
        // section for each chunk.
        let written = cortex_m::interrupt::free(|_| unsafe {
            USB_SERIAL
                .as_mut()
                .and_then(|serial| serial.write(bytes).ok())
        });
        match written {
            Some(count) if count > 0 => bytes = &bytes[count..],
            _ => break,
        }
    }
}

struct LineBuffer {
    buf: [u8; 128],
    len: usize,
}

impl core::fmt::Write for LineBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let n = bytes.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
        self.len += n;
        Ok(())
    }
}

fn poll_usb() {
    unsafe {
        if let (Some(usb_dev), Some(serial)) = (USB_BUS.as_mut(), USB_SERIAL.as_mut()) {
            usb_dev.poll(&mut [serial]);

            // Drain (and discard) anything the host sends so its buffers
            // don't back up.
            let mut buf = [0u8; 64];
            let _ = serial.read(&mut buf);
        }
    };
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}