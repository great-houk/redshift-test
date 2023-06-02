#![no_std]
#![no_main]
use panic_rtt_target as _;
use rtt_target::{self as rtt_t, rdbg as dbg, rprintln, rtt_init_default};
use LPC55S28_PAC as pac;

pub mod setup;

fn run() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let SYSCON = peripherals.SYSCON;

    // Setup default system values
    setup::setup_main_clock_96mhz(&SYSCON);

    // Enable GPIO block
    SYSCON
        .ahbclkctrl_ahbclkctrl0()
        .write(|w| w.gpio1().set_bit());
    let gpio = peripherals.GPIO;
    // Set output
    gpio.dir[1].write(|w| w.dirp().variant(1 << 9));
    let mut on = true;

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
