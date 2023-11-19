# neopixel-webusb: WebUSB NeoPixel Controller

## Installation

1. Download latest firmware build from [Releases page](https://github.com/dotcypress/neopixel-webusb/releases)
2. Hold the BOOTSEL button while connecting your board to the computer
3. Copy firmware file downloaded earlier to the RPi-RP2 disk
4. Go to https://neopixel-webusb.vercel.app
5. Enjoy

## Building firmware

1. Install rustup by following the instructions at https://rustup.rs
2. Install Cortex-M0, M0+, and M1 (ARMv6-M architecture) target: `rustup target add thumbv6m-none-eabi`
3. Install LLVM tools: `rustup component add llvm-tools-preview`
4. Install cargo-binutils: `cargo install cargo-binutils` (Note: on some Linux distros (e.g. Ubuntu) you may need to install the packages build-essential, gcc-arm-none-eabi, libssl-dev and pkg-config prior to installing cargo-binutils)
5. Install elf2uf2: `cargo install elf2uf2-rs`
6. Clone this repo: `git clone git@github.com:dotcypress/neopixel-webusb.git && cd neopixel-webusb`
7. Hold the BOOTSEL button while connecting your board to the computer
8. Flash microcontroller: `cargo run --release`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
