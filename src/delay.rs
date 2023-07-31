use embedded_hal::timer::CountDown;
use embedded_hal_alpha::delay::DelayUs;
use lpc55_hal::{drivers::Timer, peripherals::ctimer::Ctimer, time::DurationExtensions, Enabled};
use nb::block;

pub struct Delay<C: Ctimer<Enabled>> {
    timer: Timer<C>,
}

impl<C: Ctimer<Enabled>> From<Timer<C>> for Delay<C> {
    fn from(timer: Timer<C>) -> Self {
        Self { timer }
    }
}

impl<C: Ctimer<Enabled>> DelayUs for Delay<C> {
    fn delay_us(&mut self, us: u32) {
        self.timer.start(us.microseconds());
        block!(self.timer.wait()).unwrap();
    }
}
