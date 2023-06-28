#![no_std]
#![no_main]
use cortex_m_rt as _;
use hal::{
    drivers::{pins, Timer, UsbBus},
    time::RateExtensions,
};
use lpc55_hal as hal;
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

    let mut iocon = hal.iocon.enabled(&mut syscon);
    let usb0_vbus_pin = pins::Pio0_22::take()
        .unwrap()
        .into_usb0_vbus_pin(&mut iocon);
    iocon.disabled(&mut syscon); // perfectionist ;)

    let clocks = hal::ClockRequirements::default()
        .system_frequency(150.MHz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .expect("Clock configuration failed");

    let mut delay_timer = Timer::new(
        hal.ctimer
            .0
            .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
    );

    let usb_peripheral = hal.usbhs.enabled_as_device(
        &mut anactrl,
        &mut pmc,
        &mut syscon,
        &mut delay_timer,
        clocks.support_usbhs_token().unwrap(),
    );

    let usb_bus = UsbBus::new(usb_peripheral, usb0_vbus_pin);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xcc1d))
        .manufacturer("nickray")
        .product("Demo Demo Demo")
        .serial_number("2019-10-10")
        .device_release(0x0123)
        // Must be 64 bytes for HighSpeed
        .max_packet_size_0(64)
        // .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf[..]) {
            Ok(count) => {
                // count bytes were read to &buf[..count]
            }
            Err(UsbError::WouldBlock) => {} // No data received
            Err(err) => {}                  // An error occurred
        };

        match serial.write(&[0x3a, 0x29]) {
            Ok(count) => {
                // count bytes were written
            }
            Err(UsbError::WouldBlock) => {} // No data could be written (buffers full)
            Err(err) => {}                  // An error occurred
        };
    }
}
