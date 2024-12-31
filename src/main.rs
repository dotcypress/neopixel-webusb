#![no_std]
#![no_main]

mod neopixel;

extern crate panic_halt;
extern crate rp2040_hal as hal;

use crate::hal::pio::PIOExt;
use cortex_m::singleton;
use embedded_hal::digital::v2::PinState;
use hal::gpio::*;
use hal::pac;
use hal::usb::UsbBus;
use hal::Clock;
use neopixel::*;
use rp2040_hal::gpio::bank0::Gpio16;
use rp2040_hal::pio::SM0;
use smart_leds::SmartLedsWrite;
use usb_device::{class_prelude::*, prelude::*};
use usbd_webusb::*;

pub const VID: u16 = 0x1209;
pub const PID: u16 = 0xb420;
pub const MANUFACTURER: &str = "vitaly.codes";
pub const PRODUCT: &str = "NeoPixel Controller";
pub const SERIAL_NUMBER: &str = "_neopixel_";
pub const XTAL_FREQ_HZ: u32 = 12_000_000_u32;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub type NeopixelDriver =
    ws2812_pio::Ws2812Direct<pac::PIO1, SM0, Pin<Gpio16, FunctionPio1, PullDown>>;

#[rtic::app(device = pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        driver: NeopixelDriver,
        usb_dev: UsbDevice<'static, hal::usb::UsbBus>,
        neopixel: NeoPixelClass,
        wusb: WebUsb<hal::usb::UsbBus>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut resets = ctx.device.RESETS;
        let mut watchdog = hal::Watchdog::new(ctx.device.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        unsafe {
            let pll = &*pac::PLL_SYS::ptr();
            pll.prim.modify(|_, w| w.postdiv1().bits(5));
            pll.prim.modify(|_, w| w.postdiv2().bits(3));
        };

        let sio = hal::Sio::new(ctx.device.SIO);
        let pins = Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let (mut neo_pio, sm0, _, _, _) = ctx.device.PIO1.split(&mut resets);
        let driver = ws2812_pio::Ws2812Direct::new(
            pins.gpio16.into_function(),
            &mut neo_pio,
            sm0,
            clocks.peripheral_clock.freq(),
        );

        let usb_regs = ctx.device.USBCTRL_REGS;
        let usb_dpram = ctx.device.USBCTRL_DPRAM;
        let usb_bus = UsbBus::new(usb_regs, usb_dpram, clocks.usb_clock, true, &mut resets);
        let usb_bus: &'static UsbBusAllocator<UsbBus> =
            singleton!(: UsbBusAllocator<UsbBus> = UsbBusAllocator::new(usb_bus)).unwrap();

        let wusb = WebUsb::new(usb_bus, url_scheme::HTTP, "neopixel-webusb.vercel.app");
        let neopixel = NeoPixelClass::new(usb_bus);
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(VID, PID))
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(SERIAL_NUMBER)
            .build();

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
        };

        // Enable NeoPixel power switch
        pins.gpio11.into_push_pull_output_in_state(PinState::High);

        (
            Shared {},
            Local {
                driver,
                usb_dev,
                neopixel,
                wusb,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBCTRL_IRQ, local = [neopixel, wusb, usb_dev, driver])]
    fn usb_irq(ctx: usb_irq::Context) {
        let usb_irq::LocalResources {
            neopixel,
            wusb,
            usb_dev,
            driver,
        } = ctx.local;

        if usb_dev.poll(&mut [wusb, neopixel]) {
            driver.write([neopixel.color()].iter().cloned()).ok();
        }
    }
}
