#![no_std]
#![no_main]
use core::fmt::Write;
use cortex_m_rt as _;
use delay::Delay;
use embedded_hal::{digital::v2::InputPin, timer::CountDown};
use embedded_hal_alpha::delay::DelayUs;
use embedded_hal_bus::spi::ExclusiveDevice;
use gpio::Pin;
use hal::{drivers::Timer, time::*, traits::wg::spi::MODE_3, Pins};
use heapless::String;
use lpc55_hal as hal;
use lpc55_usbhs::{UsbHS, UsbHSBus};
use nb::block;
use panic_rtt_target as _;
use paw3399::{MotionRead, Paw3399, Register};
#[allow(unused)]
use rtt_target::{rdbg as dbg, rprint as print, rprintln as println};
use spi::SpiMaster;
use usb_device::{
    device::{UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_hid::{
    descriptor::{MouseReport, SerializedDescriptor},
    hid_class::HIDClass,
};

mod delay;
mod gpio;
mod setup;
mod spi;

fn run() -> ! {
    let hal = hal::new();

    let mut anactrl = hal.anactrl;
    let mut syscon = hal.syscon;
    let mut pmc = hal.pmc;

    let clocks = hal::ClockRequirements::default()
        .system_frequency(150.MHz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .expect("Clock configuration failed");

    let usb_hs = {
        let mut delay_timer_usb = Timer::new(
            hal.ctimer
                .2
                .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
        );
        UsbHS::new(
            hal.usbhs,
            &mut syscon,
            &mut pmc,
            &anactrl,
            &mut delay_timer_usb,
        )
    };

    let usb_bus = UsbHSBus::new(usb_hs);

    let mut hid = HIDClass::new(&usb_bus, MouseReport::desc(), 1);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xcc1d))
        .manufacturer("Tyler")
        .product("Test Serial! 🌈")
        .serial_number("2023-07-05")
        .device_release(0xBEEF)
        // Must be 64 bytes for HighSpeed
        .max_packet_size_0(64)
        .build();

    let mut wiggle = 2;
    loop {
        let _ = hid.push_input(&MouseReport {
            x: wiggle,
            y: 0,
            buttons: 0,
            wheel: 0,
            pan: 0,
        });

        wiggle *= -1;

        if !usb_dev.poll(&mut [&mut hid]) {
            continue;
        }
    }
}
