use core::convert::Infallible;

use lpc55_hal::{peripherals::ctimer::Ctimer, time::Microseconds, typestates::init_state};

pub struct Timer<TIMER>
where
    TIMER: Ctimer<init_state::Enabled>,
{
    timer: TIMER,
}

impl<TIMER> Timer<TIMER>
where
    TIMER: Ctimer<init_state::Enabled>,
{
    pub fn new(timer: TIMER) -> Self {
        Self { timer: timer }
    }

    pub fn release(self) -> TIMER {
        self.timer
    }

    pub fn start(&mut self, count: impl Into<Microseconds>, auto_loop: bool) {
        // Match should reset and stop timer, and generate interrupt.
        self.timer
            .mcr
            .modify(|_, w| w.mr0i().set_bit().mr0r().set_bit().mr0s().bit(!auto_loop));

        // Set match to target time.  Ctimer fixed input 1MHz.
        self.timer.mr[0].write(|w| unsafe { w.bits(count.into().0) });

        // No divsion necessary.
        self.timer.pr.write(|w| unsafe { w.bits(0) });

        // clear interrupt
        self.timer.ir.modify(|_, w| w.mr0int().set_bit());

        // Start timer
        self.timer
            .tcr
            .write(|w| w.crst().clear_bit().cen().set_bit());
    }

    #[inline(always)]
    pub fn clear_int(&mut self) {
        self.timer.ir.write(|w| w.mr0int().set_bit());
    }

    pub fn wait(&mut self) -> nb::Result<(), Infallible> {
        if self.timer.ir.read().mr0int().bit_is_set() {
            self.timer
                .tcr
                .write(|w| w.crst().set_bit().cen().clear_bit());
            return Ok(());
        }

        Err(nb::Error::WouldBlock)
    }

    pub fn cancel(&mut self) -> Result<(), Infallible> {
        self.timer
            .tcr
            .write(|w| w.crst().set_bit().cen().clear_bit());
        self.timer.ir.write(|w| w.mr0int().set_bit());
        Ok(())
    }
}
