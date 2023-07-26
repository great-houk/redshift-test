use core::marker::PhantomData;
use embedded_hal::spi::FullDuplex;
use lpc55_hal::{
    drivers::spi::{Mode, Phase, Polarity},
    time::Hertz,
    typestates::pin::{
        flexcomm::{ChipSelect, Spi, SpiPins},
        PinId,
    },
};

#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,
}

pub type Result<T> = nb::Result<T, Error>;

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

impl<SCK, MOSI, MISO, CS, SPI, PINS> FullDuplex<u8> for SpiMaster<SCK, MOSI, MISO, CS, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    CS: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, CS, SPI>,
{
    type Error = Error;

    fn read(&mut self) -> Result<u8> {
        if self.spi.fifostat.read().rxnotempty().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(self.spi.fiford.read().rxdata().bits() as u8)
        }
    }

    fn send(&mut self, words: u8) -> Result<()> {
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
            if self.spi.fifostat.read().txnotfull().bit_is_clear() {
                return Err(SpiError::Overrun);
            }

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
}
