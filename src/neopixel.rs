use smart_leds::RGB8;
use usb_device::class_prelude::*;
use usb_device::control::*;
use usb_device::Result;

pub struct NeoPixelClass {
    iface: InterfaceNumber,
    color: [u8; 3],
}

impl NeoPixelClass {
    pub fn new<B: UsbBus>(alloc: &UsbBusAllocator<B>) -> Self {
        NeoPixelClass {
            iface: alloc.interface(),
            color: [0, 0, 0],
        }
    }

    pub fn color(&self) -> RGB8 {
        self.color.into()
    }
}

impl<B: UsbBus> UsbClass<B> for NeoPixelClass {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(self.iface, 0xff, 0x00, 0x00)?;
        Ok(())
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();
        if !(req.request_type == RequestType::Vendor && req.recipient == Recipient::Device) {
            return;
        }

        match req.request {
            0x01 if req.length >= 3 => xfer.accept_with(&self.color).ok(),
            _ => xfer.reject().ok(),
        };
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();
        if !(req.request_type == RequestType::Vendor && req.recipient == Recipient::Device) {
            return;
        }

        match req.request {
            0x01 if req.length == 3 => {
                self.color.copy_from_slice(xfer.data());
                xfer.accept().ok()
            }
            _ => xfer.reject().ok(),
        };
    }
}
