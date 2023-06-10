#![no_std]
#![allow(non_snake_case)]
use core::ops::Deref;

use LPC55S28_PAC as pac;

pub mod bus;

struct USB {
    pub(crate) dev: pac::USBHSD,
    pub(crate) phy: pac::USBPHY,
}

unsafe impl Sync for USB {}
