# ClearCore / ATSAME53N19A

This BSP targets the ATSAME53N19A. Its application flash region deliberately
starts at `0x4000`; the first 16 KiB (`0x0000..0x3fff`) is reserved for an
existing bootloader.

## Build

Install the Rust target once, then build from this directory. Its `.cargo`
configuration supplies the required linker arguments and target triple:

```console
rustup target add thumbv7em-none-eabihf
cd boards/clearcore
cargo run --example hello_world
```

Cargo builds the example, then its configured runner invokes `probe-rs run`.
The program is flashed, run, and its RTT output is displayed on the same SWD
connection. No target USB is involved.

The application ELF is at:

```text
../../target/thumbv7em-none-eabihf/debug/examples/hello_world
```

## Equivalent direct command

If the ELF is already built, the runner command is equivalent to:

```console
probe-rs run --chip ATSAME53N19A --protocol swd --speed 1000 \
  --restore-unwritten \
  ../../target/thumbv7em-none-eabihf/debug/examples/hello_world
```

The included `hello_world` example prints `hello world 0`, `hello world 1`,
and so on once a second. `--speed 1000` is a conservative initial SWD speed;
raise it after confirming the connection is reliable. If more than one probe
is connected, also pass `--probe VID:PID[:SERIAL]`.

`probe-rs attach` is different: it attaches to RTT output from an application
that is already running. It does not program the ELF.

To program without running or keeping the RTT terminal open, use:

```console
probe-rs download --chip ATSAME53N19A --protocol swd --speed 1000 \
  --verify --restore-unwritten \
  ../../target/thumbv7em-none-eabihf/debug/examples/hello_world
```

Do **not** add `--chip-erase`: it erases all flash, including the bootloader.
The ELF's linked addresses begin at `0x4000`, and `--restore-unwritten` also
preserves any bytes in a flash erase sector that the application does not
overwrite.

## Debugging and panics

For the interactive probe-rs debugger, give it the same ELF so source and
symbols are available:

```console
probe-rs debug --chip ATSAME53N19A --protocol swd --speed 1000 \
  --exe ../../target/thumbv7em-none-eabihf/debug/examples/hello_world
```

The example uses RTT for normal `rprintln!` messages and `panic-probe` for
panics. With the ELF supplied, probe-rs can stop at a panic/hard fault and
show its source location and stack trace. Keep development builds (`cargo
build`, not a stripped binary) so the DWARF debug information is present.

## Bootloader hand-off

After reset, the Cortex-M starts at address `0`, so your bootloader must
chain-load the application vector table at `0x4000`: load the application's
initial MSP, set `SCB->VTOR` to `0x4000`, then branch to the application's
reset handler. Setting VTOR is essential for interrupts, hard faults, and
panic debugging to use the application's vector table.
