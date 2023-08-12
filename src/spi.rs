use core::marker::PhantomData;
use embedded_hal_alpha::spi::{ErrorKind as SpiError, ErrorType, SpiBus};
use lpc55_hal::{
    drivers::spi::{Mode, Phase, Polarity},
    time::Hertz,
    typestates::pin::{
        flexcomm::{Spi, SpiMisoPin, SpiMosiPin, SpiSckPin},
        PinId,
    },
};

type Result<T> = core::result::Result<T, SpiError>;

pub trait SpiPins<PIO1: PinId, PIO2: PinId, PIO3: PinId, SPI: Spi> {}

impl<PIO1, PIO2, PIO3, SPI, SCK, MISO, MOSI> SpiPins<PIO1, PIO2, PIO3, SPI> for (SCK, MOSI, MISO)
where
    PIO1: PinId,
    PIO2: PinId,
    PIO3: PinId,
    SPI: Spi,
    SCK: SpiSckPin<PIO1, SPI>,
    MOSI: SpiMosiPin<PIO2, SPI>,
    MISO: SpiMisoPin<PIO3, SPI>,
{
}

/// SPI peripheral operating in master mode
pub struct SpiMaster<SCK, MOSI, MISO, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, SPI>,
{
    spi: SPI,
    _pins: PINS,
    _sck: PhantomData<SCK>,
    _mosi: PhantomData<MOSI>,
    _miso: PhantomData<MISO>,
}

impl<SCK, MOSI, MISO, SPI, PINS> SpiMaster<SCK, MOSI, MISO, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, SPI>,
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
            _pins: pins,
            _sck: PhantomData,
            _mosi: PhantomData,
            _miso: PhantomData,
        }
    }

    pub fn release(self) -> (SPI, PINS) {
        (self.spi, self._pins)
    }

    fn trans(
        &self,
        mut read: Option<&mut [u8]>,
        write: Option<&[u8]>,
        in_place: bool,
    ) -> Result<()> {
        if read.is_none() && write.is_none() {
            return Err(SpiError::Other);
        }
        if in_place {
            assert!(read.is_some());
        }

        // // Clear buffers
        // self.spi
        //     .fifocfg
        //     .modify(|_, w| w.emptyrx().set_bit().emptytx().set_bit());

        let len = write.unwrap_or_else(|| read.as_ref().unwrap()).len();
        let mut i = 0;
        while i < len {
            let mut read_index = i;
            const SIZE: usize = 8;

            // Write as much as possible
            while self.spi.fifostat.read().rxfull().bit_is_clear()
                && self.spi.fifostat.read().txnotfull().bit_is_set()
                && i < len
                && i - read_index < SIZE
            {
                let mut byte = 0x00;
                if let Some(write) = write {
                    if i < write.len() {
                        byte = write[i];
                    }
                }
                if in_place {
                    byte = read.as_ref().unwrap()[i];
                }

                self.spi.fifowr.write(|w| unsafe {
                    w
                        // control
                        .len()
                        .bits(0x8 - 1) // 8 bits
                        // Chip Select
                        .txssel0_n()
                        .not_asserted()
                        .txssel1_n()
                        .not_asserted()
                        .txssel2_n()
                        .not_asserted()
                        .txssel3_n()
                        .not_asserted()
                        // data
                        .txdata()
                        .bits(byte as u16)
                });

                i += 1;
            }

            // Read as much as possible
            while read_index < i {
                // Wait for data
                while self.spi.fifostat.read().rxnotempty().bit_is_clear() {}
                // Read data
                let temp = self.spi.fiford.read().rxdata().bits() as u8;
                if let Some(read) = read.as_mut() {
                    read[read_index] = temp;
                }
                read_index += 1;
            }
        }

        Ok(())
    }
}

impl<SCK, MOSI, MISO, SPI, PINS> ErrorType for SpiMaster<SCK, MOSI, MISO, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, SPI>,
{
    type Error = SpiError;
}

impl<SCK, MOSI, MISO, SPI, PINS> SpiBus for SpiMaster<SCK, MOSI, MISO, SPI, PINS>
where
    SCK: PinId,
    MOSI: PinId,
    MISO: PinId,
    SPI: Spi,
    PINS: SpiPins<SCK, MOSI, MISO, SPI>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<()> {
        self.trans(Some(words), None, false)
    }

    fn write(&mut self, words: &[u8]) -> Result<()> {
        self.trans(None, Some(words), false)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<()> {
        self.trans(Some(read), Some(write), false)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<()> {
        self.trans(Some(words), None, true)
    }

    fn flush(&mut self) -> Result<()> {
        while self.spi.fifostat.read().txempty().bit_is_clear() {}
        Ok(())
    }
}
