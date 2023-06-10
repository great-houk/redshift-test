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
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usb_device::test_class::TestClass;

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
        .system_frequency(96.MHz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .expect("Clock configuration failed");

    let mut _delay_timer = Timer::new(
        hal.ctimer
            .0
            .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
    );

    let usb_peripheral = hal.usbfs.enabled_as_device(
        &mut anactrl,
        &mut pmc,
        &mut syscon,
        clocks.support_usbfs_token().unwrap(),
    );

    let usb_bus = UsbBus::new(usb_peripheral, usb0_vbus_pin);

    const VID: u16 = 0x16c0;
    const PID: u16 = 0x05dc;
    const MANUFACTURER: &'static str = "TestClass Manufacturer";
    const PRODUCT: &'static str = "virkkunen.net usb-device TestClass";
    const SERIAL_NUMBER: &'static str = "TestClass Serial";

    let mut test = TestClass::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(VID, PID))
        .manufacturer(MANUFACTURER)
        .product(PRODUCT)
        .serial_number(SERIAL_NUMBER)
        .max_packet_size_0(64)
        .build();

    loop {
        if usb_dev.poll(&mut [&mut test]) {
            test.poll();
        }
    }
}
