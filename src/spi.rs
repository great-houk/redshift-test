use core::{convert::Infallible, marker::PhantomData};
use embedded_hal::{
    digital::OutputPin,
    spi::{ErrorKind as SpiError, ErrorType, SpiBus},
};
use lpc55_hal::{
    drivers::spi::{Mode, Phase, Polarity},
    time::Hertz,
    typestates::pin::{
        flexcomm::{ChipSelect, Spi, SpiPins},
        PinId,
    },
};

type Result<T> = core::result::Result<T, SpiError>;

/// SPI peripheral operating in master mode
pub struct SpiMaster<SCK, MOSI, MISO, CS, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    CS: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, CS, SPI>,
{
    spi: SPI,
    pins: PINS,
    _sck: PhantomData<SCK>,
    _mosi: PhantomData<MOSI>,
    _miso: PhantomData<MISO>,
    _cs: PhantomData<CS>,
    cs: ChipSelect,
}

impl<SCK, MOSI, MISO, CS, SPI, PINS> SpiMaster<SCK, MOSI, MISO, CS, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    CS: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, CS, SPI>,
{
    pub fn new<Speed: Into<Hertz>>(spi: SPI, pins: PINS, speed: Speed, mode: Mode) -> Self {
        let speed: Hertz = speed.into();
        let speed: u32 = speed.0;

        while spi.stat.read().mstidle().bit_is_clear() {
            continue;
        }

        spi.fifocfg
            .modify(|_, w| w.enabletx().disabled().enablerx().disabled());
        spi.cfg.modify(|_, w| {
            w.enable()
                .disabled()
                .master()
                .master_mode()
                .lsbf()
                .standard() // MSB first
                .cpha()
                .bit(mode.phase == Phase::CaptureOnSecondTransition)
                .cpol()
                .bit(mode.polarity == Polarity::IdleHigh)
                .loop_()
                .disabled()
        });

        let div: u32 = 12_000_000 / speed - 1;
        debug_assert!(div <= 0xFFFF);
        spi.div
            .modify(|_, w| unsafe { w.divval().bits(div as u16) });

        // spi.raw.fifowr.write(|w| w
        //     .rxignore().ignore() // otherwise transmit halts if FIFORD buffer is full
        // );
        // spi.raw.fifotrig.modify(|_, w| w
        //     .enabletx().enabled()
        //     .enablerx().enabled()
        // );
        spi.fifocfg
            .modify(|_, w| w.enabletx().enabled().enablerx().enabled());
        spi.cfg.modify(|_, w| w.enable().enabled());
        // match pins.3.CS {
        //     0...3 => {},
        //     _ => { panic!() },
        // }

        Self {
            spi,
            pins,
            _sck: PhantomData,
            _mosi: PhantomData,
            _miso: PhantomData,
            _cs: PhantomData,
            // _cs_pin: PhantomData,
            cs: PINS::CS,
        }
    }

    pub fn release(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }
}

impl<SCK, MOSI, MISO, CS, SPI, PINS> ErrorType for SpiMaster<SCK, MOSI, MISO, CS, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    CS: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, CS, SPI>,
{
    type Error = SpiError;
}

impl<SCK, MOSI, MISO, CS, SPI, PINS> SpiBus for SpiMaster<SCK, MOSI, MISO, CS, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    CS: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, CS, SPI>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<()> {
        // Clear buffers
        self.spi
            .fifocfg
            .modify(|_, w| w.emptyrx().set_bit().emptytx().set_bit());

        let len = words.len();
        let mut i = 0;
        while i < len {
            let mut read_index = i;

            // Write as much as possible
            while self.spi.fifostat.read().rxfull().bit_is_clear()
                && self.spi.fifostat.read().txnotfull().bit_is_set()
                && i < len
            {
                use ChipSelect::*;

                const ASSERTED: bool = false;
                const NOT: bool = true;
                let (cs0, cs1, cs2, cs3) = match self.cs {
                    Chip0 => (ASSERTED, NOT, NOT, NOT),
                    Chip1 => (NOT, ASSERTED, NOT, NOT),
                    Chip2 => (NOT, NOT, ASSERTED, NOT),
                    Chip3 => (NOT, NOT, NOT, ASSERTED),
                    NoChips => (NOT, NOT, NOT, NOT),
                };

                self.spi.fifowr.write(|w| unsafe {
                    w
                        // control
                        .len()
                        .bits(0x8 - 1) // 8 bits
                        // Chip Select
                        .txssel0_n()
                        .bit(cs0)
                        .txssel1_n()
                        .bit(cs1)
                        .txssel2_n()
                        .bit(cs2)
                        .txssel3_n()
                        .bit(cs3)
                        // data
                        .txdata()
                        .bits(0)
                });

                i += 1;
            }

            // Read as much as possible
            while self.spi.fifostat.read().rxnotempty().bit_is_set() && read_index < i {
                words[read_index] = self.spi.fiford.read().rxdata().bits() as u8;
                read_index += 1;
            }
        }

        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<()> {
        // Clear buffers
        self.spi
            .fifocfg
            .modify(|_, w| w.emptyrx().set_bit().emptytx().set_bit());

        // NB: UM says "Do not read-modify-write the register."
        // - writing 0 to upper-half word means: keep previous control settings

        // NB: we set 8 bits in constructor
        // We could probably repeat this here
        use ChipSelect::*;

        const ASSERTED: bool = false;
        const NOT: bool = true;
        let (cs0, cs1, cs2, cs3) = match self.cs {
            Chip0 => (ASSERTED, NOT, NOT, NOT),
            Chip1 => (NOT, ASSERTED, NOT, NOT),
            Chip2 => (NOT, NOT, ASSERTED, NOT),
            Chip3 => (NOT, NOT, NOT, ASSERTED),
            NoChips => (NOT, NOT, NOT, NOT),
        };

        let mut iter = words.iter();
        while let Some(&byte) = iter.next() {
            // Wait for buffer to go down...
            while self.spi.fifostat.read().txnotfull().bit_is_clear() {}

            self.spi.fifowr.write(|w| unsafe {
                w
                    // control
                    .len()
                    .bits(0x8 - 1) // 8 bits
                    .rxignore()
                    .set_bit()
                    // Chip Select
                    .txssel0_n()
                    .bit(cs0)
                    .txssel1_n()
                    .bit(cs1)
                    .txssel2_n()
                    .bit(cs2)
                    .txssel3_n()
                    .bit(cs3)
                    // data
                    .txdata()
                    .bits(byte as u16)
            });
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<()> {
        // Clear buffers
        self.spi
            .fifocfg
            .modify(|_, w| w.emptyrx().set_bit().emptytx().set_bit());

        let len = read.len().max(write.len());
        let mut i = 0;
        while i < len {
            let mut read_index = i;

            // Write as much as possible
            while self.spi.fifostat.read().rxfull().bit_is_clear()
                && self.spi.fifostat.read().txnotfull().bit_is_set()
                && i < len
            {
                use ChipSelect::*;

                const ASSERTED: bool = false;
                const NOT: bool = true;
                let (cs0, cs1, cs2, cs3) = match self.cs {
                    Chip0 => (ASSERTED, NOT, NOT, NOT),
                    Chip1 => (NOT, ASSERTED, NOT, NOT),
                    Chip2 => (NOT, NOT, ASSERTED, NOT),
                    Chip3 => (NOT, NOT, NOT, ASSERTED),
                    NoChips => (NOT, NOT, NOT, NOT),
                };

                let byte;
                if i < write.len() {
                    byte = write[i];
                } else {
                    byte = 0x00;
                }

                self.spi.fifowr.write(|w| unsafe {
                    w
                        // control
                        .len()
                        .bits(0x8 - 1) // 8 bits
                        // Chip Select
                        .txssel0_n()
                        .bit(cs0)
                        .txssel1_n()
                        .bit(cs1)
                        .txssel2_n()
                        .bit(cs2)
                        .txssel3_n()
                        .bit(cs3)
                        // data
                        .txdata()
                        .bits(byte as u16)
                });

                i += 1;
            }

            // Read as much as possible
            while self.spi.fifostat.read().rxnotempty().bit_is_set() && read_index < i {
                read[read_index] = self.spi.fiford.read().rxdata().bits() as u8;
                read_index += 1;
            }
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<()> {
        // Clear buffers
        self.spi
            .fifocfg
            .modify(|_, w| w.emptyrx().set_bit().emptytx().set_bit());

        let len = words.len();
        let mut i = 0;
        while i < len {
            let mut read_index = i;

            // Write as much as possible
            while self.spi.fifostat.read().rxfull().bit_is_clear()
                && self.spi.fifostat.read().txnotfull().bit_is_set()
                && i < len
            {
                use ChipSelect::*;

                const ASSERTED: bool = false;
                const NOT: bool = true;
                let (cs0, cs1, cs2, cs3) = match self.cs {
                    Chip0 => (ASSERTED, NOT, NOT, NOT),
                    Chip1 => (NOT, ASSERTED, NOT, NOT),
                    Chip2 => (NOT, NOT, ASSERTED, NOT),
                    Chip3 => (NOT, NOT, NOT, ASSERTED),
                    NoChips => (NOT, NOT, NOT, NOT),
                };

                self.spi.fifowr.write(|w| unsafe {
                    w
                        // control
                        .len()
                        .bits(0x8 - 1) // 8 bits
                        // Chip Select
                        .txssel0_n()
                        .bit(cs0)
                        .txssel1_n()
                        .bit(cs1)
                        .txssel2_n()
                        .bit(cs2)
                        .txssel3_n()
                        .bit(cs3)
                        // data
                        .txdata()
                        .bits(words[i] as u16)
                });

                i += 1;
            }

            // Read as much as possible
            while self.spi.fifostat.read().rxnotempty().bit_is_set() && read_index < i {
                words[read_index] = self.spi.fiford.read().rxdata().bits() as u8;
                read_index += 1;
            }
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        while self.spi.fifostat.read().txempty().bit_is_clear() {}
        Ok(())
    }
}

/// Needed because the hardware manages the CS pin, but the software still want's to think it's doing something
pub struct FakeCS;

impl embedded_hal::digital::ErrorType for FakeCS {
    type Error = Infallible;
}

impl OutputPin for FakeCS {
    fn set_low(&mut self) -> core::result::Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> core::result::Result<(), Self::Error> {
        Ok(())
    }
}
