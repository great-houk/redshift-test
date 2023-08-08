#![no_std]
#![no_main]
use core::fmt::{UpperHex, Write};
use cortex_m_rt as _;
use delay::Delay;
use embedded_hal::{digital::v2::InputPin, timer::CountDown};
use embedded_hal_alpha::{delay::DelayUs, digital::OutputPin, spi::SpiDevice};
use embedded_hal_bus::spi::ExclusiveDevice;
use gpio::Pin;
use hal::{drivers::Timer, time::*, traits::wg::spi::MODE_3, Pins};
use heapless::String;
use lpc55_hal as hal;
use lpc55_usbhs::{UsbHS, UsbHSBus};
use nb::block;
use panic_rtt_target as _;
use paw3399::{
    registers::{self as regs, RegisterRead, RegisterWrite},
    MotionRead, Paw3399,
};
#[allow(unused)]
use rtt_target::{rdbg as dbg, rprint as print, rprintln as println};
use spi::SpiMaster;
use usb_device::{
    device::{UsbDeviceBuilder, UsbVidPid},
    prelude::UsbDeviceState,
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

    let usb_bus = UsbHSBus::new(usb_hs);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xcc1d))
        .manufacturer("Tyler")
        .product("Test Serial! 🌈")
        .serial_number("2023-07-05")
        .device_release(0xBEEF)
        // Must be 64 bytes for HighSpeed
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64)
        .build();

    let mut prev = String::new();
    let mut input = String::new();
    let mut output = String::new();
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        if usb_dev.state() != UsbDeviceState::Configured {
            continue;
        }

        let mut buf = [0; 128];
        match serial.read(&mut buf) {
            Ok(count) => {
                for &byte in &buf[..count] {
                    if byte == '\n' as u8 {
                        let _ = serial.write(b"\n");
                        // Write command output
                        let _ = serial
                            .write(match_command(&input, &mut output, &mut sensor).as_bytes());
                        // Clear stuff, start new line
                        prev.clone_from(&input);
                        input.clear();
                        let _ = serial.write(b"\n> ");
                    } else if byte != 27 {
                        if let Err(()) =
                            input.push_str(core::str::from_utf8(&buf[..count]).unwrap())
                        {
                            todo!();
                        }
                        let _ = serial.write(&[byte]);
                    } else {
                        input.clone_from(&prev);
                        let _ = serial.write(input.as_bytes());
                        break;
                    }
                }
            }
            Err(UsbError::WouldBlock) => {}
            Err(_) => {
                dbg!("Unknown Serial Error");
            }
        }
    }
}

fn match_command<'a, S: SpiDevice, D: DelayUs, O: OutputPin>(
    string: &String<256>,
    output: &'a mut String<512>,
    sensor: &mut Paw3399<S, D, O>,
) -> &'a String<512> {
    output.clear();

    fn read_reg<Data: UpperHex, S: SpiDevice, D: DelayUs, O: OutputPin>(
        output: &mut String<512>,
        sensor: &mut Paw3399<S, D, O>,
        register: impl RegisterRead<Data>,
    ) -> Result<(), core::fmt::Error> {
        match sensor.read(register) {
            Ok(v) => write!(output, "0x{v:X}"),
            Err(err) => write!(output, "Error: {err:?}"),
        }
    }

    fn write_reg<Data, S: SpiDevice, D: DelayUs, O: OutputPin>(
        output: &mut String<512>,
        sensor: &mut Paw3399<S, D, O>,
        register: impl RegisterWrite<Data>,
        data: Data,
    ) -> Result<(), core::fmt::Error> {
        match sensor.write(register, data) {
            Ok(()) => write!(output, "Success!"),
            Err(err) => write!(output, "Error: {err:?}"),
        }
    }

    if string.contains('=') {
        // Write Command
        let (name, value) = string.split_once('=').unwrap();
        let name = name.trim();
        let value = value.trim().trim_start_matches("0x");
        let _ = match name {
            "Motion" => write_reg(
                output,
                sensor,
                regs::Motion,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Observation" => write_reg(
                output,
                sensor,
                regs::Observation,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "MotionBurst" => write_reg(
                output,
                sensor,
                regs::MotionBurst,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "PowerUpReset" => write_reg(
                output,
                sensor,
                regs::PowerUpReset,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Shutdown" => write_reg(
                output,
                sensor,
                regs::Shutdown,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Performance" => write_reg(
                output,
                sensor,
                regs::Performance,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "SetResolution" => write_reg(
                output,
                sensor,
                regs::SetResolution,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "ResolutionX" => write_reg(
                output,
                sensor,
                regs::ResolutionX,
                u16::from_str_radix(value, 16).unwrap(),
            ),
            "ResolutionY" => write_reg(
                output,
                sensor,
                regs::ResolutionY,
                u16::from_str_radix(value, 16).unwrap(),
            ),
            "AngleSnap" => write_reg(
                output,
                sensor,
                regs::AngleSnap,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "RippleControl" => write_reg(
                output,
                sensor,
                regs::RippleControl,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "AxisControl" => write_reg(
                output,
                sensor,
                regs::AxisControl,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "MotionCtrl" => write_reg(
                output,
                sensor,
                regs::MotionCtrl,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "RunDownshift" => write_reg(
                output,
                sensor,
                regs::RunDownshift,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Rest1Rate" => write_reg(
                output,
                sensor,
                regs::Rest1Rate,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Rest1Downshift" => write_reg(
                output,
                sensor,
                regs::Rest1Downshift,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Rest2Rate" => write_reg(
                output,
                sensor,
                regs::Rest2Rate,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Rest2Downshift" => write_reg(
                output,
                sensor,
                regs::Rest2Downshift,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "Rest3Rate" => write_reg(
                output,
                sensor,
                regs::Rest3Rate,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "RunDownshiftMult" => write_reg(
                output,
                sensor,
                regs::RunDownshiftMult,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "RestDownshiftMult" => write_reg(
                output,
                sensor,
                regs::RestDownshiftMult,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "AngleTune1" => write_reg(
                output,
                sensor,
                regs::AngleTune1,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "AngleTune2" => write_reg(
                output,
                sensor,
                regs::AngleTune2,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            "LiftConfig" => write_reg(
                output,
                sensor,
                regs::LiftConfig,
                u8::from_str_radix(value, 16).unwrap(),
            ),
            _ => write!(output, "Command Not Found: {string}"),
        };
    } else {
        // Read command
        let _ = match string.trim() {
            "ProductId" => read_reg(output, sensor, regs::ProductId),
            "RevisionId" => read_reg(output, sensor, regs::RevisionId),
            "Motion" => read_reg(output, sensor, regs::Motion),
            "DeltaX" => read_reg(output, sensor, regs::DeltaX),
            "DeltaY" => read_reg(output, sensor, regs::DeltaY),
            "SQUAL" => read_reg(output, sensor, regs::SQUAL),
            "RawDataSum" => read_reg(output, sensor, regs::RawDataSum),
            "MaximumRawData" => read_reg(output, sensor, regs::MaximumRawData),
            "MinimumRawData" => read_reg(output, sensor, regs::MinimumRawData),
            "Shutter" => read_reg(output, sensor, regs::Shutter),
            "Observation" => read_reg(output, sensor, regs::Observation),
            "MotionBurst" => read_reg(output, sensor, regs::MotionBurst),
            "Performance" => read_reg(output, sensor, regs::Performance),
            "ResolutionX" => read_reg(output, sensor, regs::ResolutionX),
            "ResolutionY" => read_reg(output, sensor, regs::ResolutionY),
            "AngleSnap" => read_reg(output, sensor, regs::AngleSnap),
            "RawDataOutput" => read_reg(output, sensor, regs::RawDataOutput),
            "RawDataStatus" => read_reg(output, sensor, regs::RawDataStatus),
            "RippleControl" => read_reg(output, sensor, regs::RippleControl),
            "AxisControl" => read_reg(output, sensor, regs::AxisControl),
            "MotionCtrl" => read_reg(output, sensor, regs::MotionCtrl),
            "InvProductId" => read_reg(output, sensor, regs::InvProductId),
            "RunDownshift" => read_reg(output, sensor, regs::RunDownshift),
            "Rest1Rate" => read_reg(output, sensor, regs::Rest1Rate),
            "Rest1Downshift" => read_reg(output, sensor, regs::Rest1Downshift),
            "Rest2Rate" => read_reg(output, sensor, regs::Rest2Rate),
            "Rest2Downshift" => read_reg(output, sensor, regs::Rest2Downshift),
            "Rest3Rate" => read_reg(output, sensor, regs::Rest3Rate),
            "RunDownshiftMult" => read_reg(output, sensor, regs::RunDownshiftMult),
            "RestDownshiftMult" => read_reg(output, sensor, regs::RestDownshiftMult),
            "AngleTune1" => read_reg(output, sensor, regs::AngleTune1),
            "AngleTune2" => read_reg(output, sensor, regs::AngleTune2),
            "LiftConfig" => read_reg(output, sensor, regs::LiftConfig),
            "m" => {
                let data = sensor.motion_read().unwrap();
                write!(output, "{data:?}")
            }
            "help" => write!(output, "{HELP}"),
            "echo" => write!(output, "ECHO echo ᵉᶜʰᵒ"),
            _ => write!(output, "Command Not Found: {string}"),
        };
    }
    output
}

const HELP: &'static str = "Read: {register}\nWrite: {register}=0x....\nRegisters:
\tProductId
\tRevisionId
\tMotion
\tDeltaX
\tDeltaY
\tSQUAL
\tRawDataSum
\tMaximumRawData
\tMinimumRawData
\tShutter
\tObservation
\tMotionBurst
\tPowerUpReset
\tShutdown
\tPerformance
\tSetResolution
\tResolutionX
\tResolutionY
\tAngleSnap
\tRawDataOutput
\tRawDataStatus
\tRippleControl
\tAxisControl
\tMotionCtrl
\tInvProductId
\tRunDownshift
\tRest1Rate
\tRest1Downshift
\tRest2Rate
\tRest2Downshift
\tRest3Rate
\tRunDownshiftMult
\tRestDownshiftMult
\tAngleTune1
\tAngleTune2
\tLiftConfig";
