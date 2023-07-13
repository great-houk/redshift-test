#![no_std]
#![no_main]
use cortex_m_rt as _;
use hal::{drivers::Timer, time::RateExtensions};
use lpc55_hal as hal;
use lpc55_usbhs::{UsbHS, UsbHSBus};
use panic_rtt_target as _;
#[allow(unused)]
use rtt_target::{rdbg as dbg, rprintln as println};
use usb_device::{
    device::{UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod setup;

fn run() -> ! {
    let hal = hal::new();

    let mut anactrl = hal.anactrl;
    let mut syscon = hal.syscon;
    let mut pmc = hal.pmc;

    let clocks = hal::ClockRequirements::default()
        .system_frequency(150.MHz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .expect("Clock configuration failed");

    let mut delay_timer = Timer::new(
        hal.ctimer
            .0
            .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
    );

    let usb = UsbHS::new(hal.usbhs, &mut syscon, &mut pmc, &anactrl, &mut delay_timer);

    let usb_bus = UsbHSBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xcc1d))
        .manufacturer("Tyler")
        .product("Test Serial! 🌈")
        .serial_number("2023-07-05")
        .device_release(0xBEEF)
        // Must be 64 bytes for HighSpeed
        .max_packet_size_0(64)
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 512];

        match serial.read(&mut buf[..]) {
            Ok(count) => {
                let _ = serial.write(&buf[..count]);
            }
            Err(UsbError::WouldBlock) => {} // No data received
            Err(err) => {
                dbg!(err);
            } // An error occurred
        };
    }
}
