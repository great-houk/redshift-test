#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use cortex_m_rt as _;
use panic_rtt_target as _;

#[rtic::app(device = lpc55_hal::raw, dispatchers = [FLEXCOMM0])]
mod app {
    use core::fmt::Write;
    use heapless::String;
    use lpc55_hal::{
        self as hal,
        drivers::Timer,
        time::{DurationExtensions, RateExtensions},
        Pins,
    };
    use lpc55_usbhs::{UsbHS, UsbHSBus};
    use rtt_target::{rdbg as dbg, rprint as print, rprintln as println, rtt_init_default};
    use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
    use usbd_serial::{SerialPort, USB_CLASS_CDC};

    #[shared]
    struct Shared {
        serial: SerialPort<'static, UsbHSBus>,
        device: UsbDevice<'static, UsbHSBus>,
    }

    #[local]
    struct Local {}

    #[init(local = [usb_bus: Option<UsbBusAllocator<UsbHSBus>> = None])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let rtt = rtt_init_default!();
        rtt_target::set_print_channel(rtt.up.0);
        // Saftey Delay, so it's possible to halt before user code runs
        cortex_m::asm::delay(10_000_000);
        println!("Start up finished!");

        let hal = hal::from((cx.device, cx.core.into()));

        let mut anactrl = hal.anactrl;
        let mut syscon = hal.syscon;
        let mut pmc = hal.pmc;
        let mut iocon = hal.iocon.enabled(&mut syscon);
        let mut gpio = hal.gpio.enabled(&mut syscon);
        let pins = Pins::take().unwrap();

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
                &mut anactrl,
                &mut delay_timer_usb,
            )
        };

        let bus = cx.local.usb_bus.insert(UsbHSBus::new(usb_hs));

        let serial = SerialPort::new(bus);

        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x1209, 0xcc1d))
            .manufacturer("Tyler")
            .product("Test Serial! 🌈")
            .serial_number("2023-07-05")
            .device_release(0xBEEF)
            // Must be 64 bytes for HighSpeed
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        (Shared { serial, device }, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = USB1, priority = 1, local = [i: u8 = 0], shared = [serial, device])]
    fn usb(cx: usb::Context) {
        let int: u32 = unsafe { core::ptr::read_volatile(0x4009_4020 as _) };
        let cmd: u32 = unsafe { core::ptr::read_volatile(0x4009_4000 as _) };
        println!("{} USB Int 0x{:X} 0x{:X}", cx.local.i, int, cmd);
        *cx.local.i = u8::wrapping_add(*cx.local.i, 1);

        let device = cx.shared.device;
        let serial = cx.shared.serial;
        (device, serial).lock(|device, serial| {
            if device.poll(&mut [serial]) && device.state() == UsbDeviceState::Configured {
                println!("Inner");
                let _ = echo_serial::spawn();
            }
        });
    }

    #[task(local = [buf: [u8; 256] = [0; 256], string_buf: String<256> = String::new()], shared = [serial, device], priority = 1)]
    async fn echo_serial(mut cx: echo_serial::Context) {
        println!("Echo!");

        cx.shared
            .serial
            .lock(|serial| match serial.read(cx.local.buf) {
                Ok(count) => {
                    let _ = serial.write(&cx.local.buf[..count]);
                }
                Err(UsbError::WouldBlock) => {}
                Err(err) => {
                    let buf = cx.local.string_buf;
                    buf.clear();
                    write!(buf, "Usb Error: {err:?}").unwrap();
                    let _ = serial.write(buf.as_bytes());
                }
            });
    }
}
