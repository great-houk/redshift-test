#![no_std]
#![no_main]
use panic_rtt_target as _;
use rtt_target::{self as rtt_t, rdbg as dbg, rprintln, rtt_init_default};
use LPC55S28_PAC as pac;

pub mod setup;

fn run() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let SYSCON = peripherals.SYSCON;
    let INPUTMUX = peripherals.INPUTMUX;
    let ANACTRL = peripherals.ANACTRL;

    // Setup default system values
    setup::setup_main_clock_96mhz(&SYSCON);

    // Enable GPIO block
    SYSCON
        .ahbclkctrl_ahbclkctrl0()
        .write(|w| w.gpio1().enable());
    let gpio = peripherals.GPIO;
    // Set output
    gpio.dir[1].write(|w| w.dirp().variant(1 << 9));
    let mut on = true;

    // Setup analog control
    SYSCON
        .ahbclkctrl_ahbclkctrl2()
        .write(|w| w.analog_ctrl().enable().freqme().enable());
    SYSCON.ahbclkctrl_ahbclkctrl0().write(|w| w.mux().enable());
    INPUTMUX.freqmeas_ref.write(|w| w.clkin().variant(4)); // 96 Mhz reference
    INPUTMUX.freqmeas_target.write(|w| w.clkin().variant(4)); // External
    ANACTRL
        .freq_me_ctrl
        .write(|w| w.capval_scale().variant(11).prog().set_bit());
    while ANACTRL.freq_me_ctrl.read().prog().bit_is_set() {}
    let freq = (ANACTRL.freq_me_ctrl.read().capval_scale().bits() * 96_000_000) / ((1 << 11) - 1);
    dbg!(ANACTRL.freq_me_ctrl.read().capval_scale().bits());
    dbg!(freq);

    loop {
        // Delay a bit
        cortex_m::asm::delay(12_000_000);

        // Flip USER LED
        if on {
            gpio.b[1].b_[9].write(|w| w.pbyte().set_bit());
        } else {
            gpio.b[1].b_[9].write(|w| w.pbyte().clear_bit());
        }
        on = !on;

        // Print
        dbg!("Hello, world!");
    }
}
