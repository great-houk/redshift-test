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