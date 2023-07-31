#![no_std]
#![no_main]
use core::fmt::Write;
use cortex_m_rt as _;
use delay::Delay;
use embedded_hal::{digital::v2::InputPin, timer::CountDown};
use embedded_hal_alpha::delay::DelayUs;
use embedded_hal_bus::spi::ExclusiveDevice;
use gpio::Pin;
use hal::UsbBus;
use hal::{drivers::Timer, time::*, traits::wg::spi::MODE_3, Pins};
use heapless::String;
use lpc55_hal as hal;
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
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod delay;
mod gpio;
mod setup;
mod spi;

fn run() -> ! {
    let hal = hal::new();

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
        let speed: Hertz = 500u32.kHz().try_into().unwrap();
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

    // Check sensor status
    let mut delay: Delay<_> = Timer::new(
        hal.ctimer
            .3
            .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
    )
    .into();
    sensor.write(Register::Observation, 0x00).unwrap();
    delay.delay_ms(600);
    let res = sensor.read(Register::Observation).unwrap();
    if res == 0xB7 || res == 0xBF {
        println!("Sensor Initialized Successfully!");
    } else {
        panic!("Sensor failed to start correctly...");
    }

    let usb_hs = {
        let mut delay_timer_usb = Timer::new(
            hal.ctimer
                .2
                .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap()),
        );
        hal.usbhs.enabled_as_device(
            &mut anactrl,
            &mut pmc,
            &mut syscon,
            &mut delay_timer_usb,
            clocks.support_usbhs_token().unwrap(),
        )
    };

    let usb_bus = UsbBus::new(usb_hs, pins.pio0_22.into_usb0_vbus_pin(&mut iocon));

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
        if motion_pin.is_low().unwrap() {
            let motion = sensor.motion_read().unwrap();
            println!("{:?}", motion);
            const LEN: usize = 6 + 2 + 6 + 1;
            let mut output: String<LEN> = String::new();
            writeln!(output, "{}, {}", motion.delta_x, motion.delta_y).unwrap();
            let _ = serial.write(output.as_bytes());
        }

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
