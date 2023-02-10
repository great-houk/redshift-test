#![no_std]
#![no_main]
use core::fmt::Write;
use cortex_m_semihosting::hio;
use panic_semihosting as _;
use LPC55S28_PAC as _;

// This function will be called by the application
fn print() -> Result<(), core::fmt::Error> {
    let mut stdout = hio::hstdout().map_err(|_| core::fmt::Error)?;
    let language = "Rust";
    let ranking = 1;

    writeln!(stdout, "{} on embedded is #{}!", language, ranking)?;

    Ok(())
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = LPC55S28_PAC::Peripherals::take().unwrap();
    let SYSCON = peripherals.SYSCON;
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

        // Print stuff
        print().unwrap();
    }
}
