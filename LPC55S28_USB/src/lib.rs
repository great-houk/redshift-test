#![no_std]
#![allow(non_snake_case)]
use core::ops::Deref;

use LPC55S28_PAC as pac;

pub mod bus;

struct USB {
    inner: pac::USBHSD,
}

unsafe impl Sync for USB {}
impl Deref for USB {
    type Target = pac::USBHSD;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
