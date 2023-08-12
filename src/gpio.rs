use core::convert::Infallible;

use embedded_hal::digital::v2::{InputPin as OldI, OutputPin as OldO};
use embedded_hal_alpha::digital::{ErrorType, InputPin, OutputPin};
use lpc55_hal::{
    typestates::pin::{
        gpio::direction::{Input, Output},
        state::{Gpio, PinState},
        PinId,
    },
    Pin as HalPin,
};

pub struct Pin<T: PinId, S: PinState> {
    pin: HalPin<T, S>,
}

impl<T: PinId, S: PinState> From<HalPin<T, S>> for Pin<T, S> {
    fn from(pin: HalPin<T, S>) -> Self {
        Pin { pin }
    }
}

impl<T: PinId, S: PinState> ErrorType for Pin<T, S> {
    type Error = Infallible;
}

impl<T: PinId> InputPin for Pin<T, Gpio<Input>> {
    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}

impl<T: PinId> OutputPin for Pin<T, Gpio<Output>> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}
