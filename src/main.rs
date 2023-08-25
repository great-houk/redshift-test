#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use cortex_m_rt as _;
use panic_rtt_target as _;

mod delay;
mod gpio;
mod spi;
mod timer;

#[rtic::app(device = lpc55_hal::raw)]
mod app {
    use super::timer::Timer as LoopTimer;
    use atomic::Atomic;
    use core::{hint::black_box, sync::atomic::Ordering::Relaxed};
    use cortex_m_rt as _;
    use lpc55_hal::{
        drivers::Timer, peripherals::ctimer::Ctimer0, raw::Interrupt, time::*, Enabled,
    };
    use lpc55_usbhs::{UsbHS, UsbHSBus};
    use panic_rtt_target as _;
    #[allow(unused)]
    use rtt_target::{rdbg as dbg, rprint as print, rprintln as println, rtt_init_default};
    use usb_device::{
        class_prelude::{UsbBus, UsbBusAllocator},
        prelude::*,
    };
    use usbd_hid::{
        descriptor::{MouseReport, SerializedDescriptor},
        hid_class::HIDClass,
    };

    #[shared]
    struct Shared {
        hid: HIDClass<'static, UsbHSBus>,
        usb_state: Atomic<Wrap>,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Wrap(pub UsbDeviceState);
    unsafe impl bytemuck::NoUninit for Wrap {}

    #[local]
    struct Local {
        device: UsbDevice<'static, UsbHSBus>,
        wiggle_timer: LoopTimer<Ctimer0<Enabled>>,
    }

    #[inline(never)]
    fn breakpt() {
        black_box(cortex_m::asm::nop());
    }

    #[init(local = [usb_bus: Option<UsbBusAllocator<UsbHSBus>> = None])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let rtt = rtt_init_default!();
        rtt_target::set_print_channel(rtt.up.0);
        // Saftey Delay, so it's possible to halt before user code runs
        cortex_m::asm::delay(10_000_000);
        breakpt();
        println!("Start up finished!");

        let hal = lpc55_hal::from((cx.device, cx.core.into()));

        let mut anactrl = hal.anactrl;
        let mut syscon = hal.syscon;
        let mut pmc = hal.pmc;

        let clocks = lpc55_hal::ClockRequirements::default()
            .system_frequency(150.MHz())
            .configure(&mut anactrl, &mut pmc, &mut syscon)
            .expect("Clock configuration failed");

        let mut wiggle_timer = LoopTimer::new(
            hal.ctimer
                .0
                .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
        );
        wiggle_timer.start(250u32.microseconds(), true);

        let usb_hs = {
            let mut delay_timer_usb = Timer::new(
                hal.ctimer
                    .3
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

        let hid = HIDClass::new(bus, MouseReport::desc(), 1);

        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x1209, 0xcc1d))
            .manufacturer("Tyler")
            .product("Test Mouse! 🌈")
            .serial_number("2023-07-05")
            .device_release(0xBEEF)
            .supports_remote_wakeup(true)
            // Must be 64 bytes for HighSpeed
            .max_packet_size_0(64)
            .build();

        (
            Shared {
                hid,
                usb_state: Atomic::new(Wrap(UsbDeviceState::Default)),
            },
            Local {
                device,
                wiggle_timer,
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = USB1, priority = 3, local = [device], shared = [hid, &usb_state])]
    fn usb(mut cx: usb::Context) {
        // Poll USB and update state
        if cx.shared.hid.lock(|hid| cx.local.device.poll(&mut [hid])) {
            cx.shared
                .usb_state
                .store(Wrap(cx.local.device.state()), Relaxed);
        }

        // Remote wake-up
        if cx.local.device.state() == UsbDeviceState::Suspend
            && cx.shared.usb_state.load(Relaxed).0 == UsbDeviceState::Default
        {
            cx.local.device.bus().resume();
        }
    }

    #[task(binds = CTIMER0, local = [x: i8 = 1, wiggle_timer], shared = [hid, &usb_state])]
    fn wiggle(mut cx: wiggle::Context) {
        cx.local.wiggle_timer.clear_int();

        if cx.shared.usb_state.load(Relaxed).0 == UsbDeviceState::Configured {
            match cx.shared.hid.lock(|hid| {
                hid.push_input(&MouseReport {
                    buttons: 0,
                    x: *cx.local.x,
                    y: 0,
                    wheel: 0,
                    pan: 0,
                })
            }) {
                Ok(_) => {
                    *cx.local.x *= -1;
                }
                Err(_) => {}
            }
        } else if cx.shared.usb_state.load(Relaxed).0 == UsbDeviceState::Suspend {
            // Remote wakeup
            cx.shared
                .usb_state
                .store(Wrap(UsbDeviceState::Default), Relaxed);
            rtic::pend(Interrupt::USB1);
        }
    }
}
