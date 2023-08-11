#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use cortex_m_rt as _;
use panic_rtt_target as _;

mod delay;
mod gpio;
mod spi;
mod timer;

#[rtic::app(device = lpc55_hal::raw, dispatchers = [FLEXCOMM0])]
mod app {
    use super::delay::Delay;
    use super::gpio::Pin;
    use super::spi::SpiMaster;
    use core::{fmt::Write, sync::atomic::AtomicBool};
    use core::{hint::black_box, sync::atomic::Ordering::Relaxed};
    use cortex_m_rt as _;
    use embedded_hal::{digital::v2::InputPin, timer::CountDown};
    use embedded_hal_alpha::delay::DelayUs;
    use embedded_hal_bus::spi::ExclusiveDevice;
    use hal::{
        drivers::Timer,
        peripherals::pint::{Mode, Slot},
        raw::{utick0::CAP, Interrupt},
        time::*,
        traits::wg::spi::MODE_3,
        Pins,
    };
    use heapless::String;
    use lpc55_hal::{
        self as hal, peripherals::ctimer::Ctimer0, typestates::init_state, Enabled, Pint,
    };
    use lpc55_usbhs::{UsbHS, UsbHSBus};
    use nb::block;
    use panic_rtt_target as _;
    use paw3399::{registers as regs, MotionRead, Paw3399};
    use rtic_sync::{channel::*, make_channel};
    #[allow(unused)]
    use rtt_target::{rdbg as dbg, rprint as print, rprintln as println, rtt_init_default};
    use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{MouseReport, SerializedDescriptor},
        hid_class::HIDClass,
    };

    #[shared]
    struct Shared {
        hid: HIDClass<'static, UsbHSBus>,
    }

    #[local]
    struct Local {
        device: UsbDevice<'static, UsbHSBus>,
        sensor: sensor_type::Sensor,
        pint: Pint<Enabled>,
        pint_sender: Sender<'static, Message, CAPACITY>,
        usb_sender: Sender<'static, Message, CAPACITY>,
        receiver: Receiver<'static, Message, CAPACITY>,
    }
    mod sensor_type {
        use crate::{delay::Delay, gpio::Pin as CPin, spi::SpiMaster};
        use embedded_hal_bus::spi::ExclusiveDevice;
        use lpc55_hal::{
            drivers::pins::*,
            peripherals::{
                ctimer::{Ctimer1, Ctimer2},
                flexcomm::Spi6,
            },
            typestates::pin::{
                function::{FC6_RXD_SDA_MOSI_DATA, FC6_SCK, FC6_TXD_SCL_MISO_WS},
                gpio::direction::Output,
                state::{Gpio, Special},
            },
            Enabled, Pin,
        };
        use paw3399::Paw3399;

        pub type Sensor = Paw3399<
            ExclusiveDevice<
                SpiMaster<
                    Pio1_12,
                    Pio1_13,
                    Pio1_16,
                    Spi6,
                    (
                        Pin<Pio1_12, Special<FC6_SCK>>,
                        Pin<Pio1_13, Special<FC6_RXD_SDA_MOSI_DATA>>,
                        Pin<Pio1_16, Special<FC6_TXD_SCL_MISO_WS>>,
                    ),
                >,
                CPin<Pio0_15, Gpio<Output>>,
                Delay<Ctimer1<Enabled>>,
            >,
            Delay<Ctimer2<Enabled>>,
            CPin<Pio1_8, Gpio<Output>>,
        >;
    }

    const CAPACITY: usize = 8;

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
        black_box(breakpt());
        println!("Start up finished!");

        let hal = hal::from((cx.device, cx.core.into()));

        let mut anactrl = hal.anactrl;
        let mut syscon = hal.syscon;
        let mut pmc = hal.pmc;
        let mut pint = hal.pint.enabled(&mut syscon);
        let mut iocon = hal.iocon.enabled(&mut syscon);
        let mut gpio = hal.gpio.enabled(&mut syscon);
        let mut inputmux = hal.inputmux.enabled(&mut syscon);
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
                    .1
                    .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
            )
            .into();
            ExclusiveDevice::new(spi, cs, delay_timer_spi)
        };

        let sensor = {
            let delay_timer_sensor: Delay<_> = Timer::new(
                hal.ctimer
                    .2
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
        pint.enable_interrupt(&mut inputmux, &motion_pin, Slot::Slot0, Mode::FallingEdge);

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
            .product("Test Serial! 🌈")
            .serial_number("2023-07-05")
            .device_release(0xBEEF)
            // Must be 64 bytes for HighSpeed
            .max_packet_size_0(64)
            .build();

        let (s, r) = make_channel!(Message, CAPACITY);

        (
            Shared { hid },
            Local {
                device,
                sensor,
                pint,
                pint_sender: s.clone(),
                usb_sender: s,
                receiver: r,
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = USB1, priority = 2, local = [device, usb_sender, ready: bool = false], shared = [hid])]
    fn usb(mut cx: usb::Context) {
        if cx.shared.hid.lock(|hid| cx.local.device.poll(&mut [hid])) {
            if cx.local.device.state() == UsbDeviceState::Configured && !*cx.local.ready {
                // Start sensor reading
                println!("Usb Connected!");
                if let Ok(()) = cx.local.usb_sender.try_send(Message::StartReading) {
                    *cx.local.ready = true;
                    rtic::pend(Interrupt::PIN_INT0);
                    sensor::spawn().unwrap();
                }
            }
        }
    }

    #[task(binds = PIN_INT0, local = [pint, pint_sender])]
    fn motion_pin(cx: motion_pin::Context) {
        // Clear interrupt
        cx.local.pint.ist.write(|w| unsafe { w.pstat().bits(1) });
        // Send motion read
        if let Err(err) = cx.local.pint_sender.try_send(Message::MotionRead) {
            dbg!(err);
        }
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        StartReading,
        MotionRead,
    }

    #[task(priority = 1, local = [sensor, receiver, start: bool = false, dx: i16 = 0, dy: i16 = 0], shared = [hid])]
    async fn sensor(mut cx: sensor::Context) {
        while let Ok(msg) = cx.local.receiver.recv().await {
            // Parse message
            match msg {
                Message::StartReading => *cx.local.start = true,
                Message::MotionRead => {
                    if *cx.local.start {
                        if let Ok(r) = cx.local.sensor.motion_read() {
                            let MotionRead {
                                delta_x, delta_y, ..
                            } = dbg!(r);
                            *cx.local.dx += dbg!(delta_x);
                            *cx.local.dy += dbg!(delta_y);
                        }
                    }
                }
            }
            // Send HID report
            if *cx.local.dx != 0 && *cx.local.dy != 0 {
                let (x, y) = (trim_i16(cx.local.dx), trim_i16(cx.local.dy));
                match cx.shared.hid.lock(|hid| {
                    hid.push_input(&MouseReport {
                        buttons: 0,
                        x,
                        y,
                        wheel: 0,
                        pan: 0,
                    })
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        *cx.local.dx += x as i16;
                        *cx.local.dy += y as i16;
                    }
                }
            }
        }
    }

    fn trim_i16(v: &mut i16) -> i8 {
        let r;
        if *v > i8::MAX as i16 {
            r = i8::MAX;
            *v -= i8::MAX as i16;
        } else if *v < i8::MIN as i16 {
            r = i8::MIN;
            *v -= i8::MIN as i16;
        } else {
            r = *v as i8;
            *v = 0;
        }
        r
    }
}
