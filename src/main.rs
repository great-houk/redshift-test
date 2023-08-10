#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use cortex_m_rt as _;
use panic_rtt_target as _;

mod delay;
mod gpio;
mod setup;
mod spi;

#[rtic::app(device = lpc55_hal::raw, dispatchers = [FLEXCOMM0])]
mod app {
    use super::delay::Delay;
    use super::gpio::Pin;
    use super::spi::SpiMaster;
    use core::fmt::Write;
    use cortex_m_rt as _;
    use embedded_hal::{digital::v2::InputPin, timer::CountDown};
    use embedded_hal_alpha::delay::DelayUs;
    use embedded_hal_bus::spi::ExclusiveDevice;
    use hal::{drivers::Timer, time::*, traits::wg::spi::MODE_3, Pins};
    use heapless::String;
    use lpc55_hal as hal;
    use lpc55_usbhs::{UsbHS, UsbHSBus};
    use nb::block;
    use panic_rtt_target as _;
    use paw3399::{registers as regs, MotionRead, Paw3399};
    #[allow(unused)]
    use rtt_target::{rdbg as dbg, rprint as print, rprintln as println, rtt_init_default};
    use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{MouseReport, SerializedDescriptor},
        hid_class::HIDClass,
    };

    #[shared]
    struct Shared {
        device_ready: bool,
        hid: HIDClass<'static, UsbHSBus>,
    }

    #[local]
    struct Local {
        device: UsbDevice<'static, UsbHSBus>,
    }

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

        let spi_device = {
            let spi = hal
                .flexcomm
                .6
                .enabled_as_spi(&mut syscon, &clocks.support_flexcomm_token().unwrap());
            let sck = pins.pio1_12.into_spi6_sck_pin(&mut iocon);
            let mosi = pins.pio1_13.into_spi6_mosi_pin(&mut iocon);
            let miso = pins.pio1_16.into_spi6_miso_pin(&mut iocon);
            let speed: Hertz = 10.MHz().try_into().unwrap();
            let spi = SpiMaster::new(spi, (sck, mosi, miso), speed, MODE_3);

            let cs: Pin<_, _> = pins
                .pio0_15
                .into_gpio_pin(&mut iocon, &mut gpio)
                .into_output_high()
                .into();
            let delay_timer_spi: Delay<_> = Timer::new(
                hal.ctimer
                    .0
                    .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
            )
            .into();
            ExclusiveDevice::new(spi, cs, delay_timer_spi)
        };

        let mut sensor = {
            let delay_timer_sensor: Delay<_> = Timer::new(
                hal.ctimer
                    .1
                    .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
            )
            .into();
            let reset: Pin<_, _> = pins
                .pio1_8
                .into_gpio_pin(&mut iocon, &mut gpio)
                .into_output_low()
                .into();
            // SAFTEY: SPI is in mode 3
            unsafe { Paw3399::new(spi_device, delay_timer_sensor, reset) }.unwrap()
        };

        let motion_pin = pins
            .pio0_19
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_input();

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

        let hid = HIDClass::new(&bus, MouseReport::desc(), 1);

        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x1209, 0xcc1d))
            .manufacturer("Tyler")
            .product("Test Serial! 🌈")
            .serial_number("2023-07-05")
            .device_release(0xBEEF)
            // Must be 64 bytes for HighSpeed
            .max_packet_size_0(64)
            .build();

        (
            Shared {
                device_ready: false,
                hid,
            },
            Local { device },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = USB1, priority = 1, local = [device], shared = [hid, device_ready])]
    fn usb(cx: usb::Context) {}

    #[task(shared = [device_ready], priority = 1)]
    async fn send_report(cx: send_report::Context) {
        println!("Echo!");
    }
}
