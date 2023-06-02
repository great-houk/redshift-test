use crate::USB;
use usb_device::{
    bus::{PollResult, UsbBus as UB},
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection,
};

pub struct UsbBus {
    peripheral: USB,
}

impl UB for UsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> usb_device::Result<EndpointAddress> {
        todo!()
    }

    fn enable(&mut self) {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }

    fn set_device_address(&self, addr: u8) {
        todo!()
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        todo!()
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        todo!()
    }

    fn suspend(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn poll(&self) -> PollResult {
        todo!()
    }

    fn force_reset(&self) -> usb_device::Result<()> {
        Err(usb_device::UsbError::Unsupported)
    }

    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = false;
}
