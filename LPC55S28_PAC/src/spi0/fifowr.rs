#[doc = "Register `FIFOWR` reader"]
pub struct R(crate::R<FIFOWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOWR` writer"]
pub struct W(crate::W<FIFOWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FIFOWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO."]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR_SPEC, u16, u16, 16, O>;
#[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL0_N_AW {
    #[doc = "0: SSEL0 asserted."]
    ASSERTED = 0,
    #[doc = "1: SSEL0 not asserted."]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL0_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL0_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL0_N` writer - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
pub type TXSSEL0_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL0_N_AW, O>;
impl<'a, const O: u8> TXSSEL0_N_W<'a, O> {
    #[doc = "SSEL0 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::ASSERTED)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL1_N_AW {
    #[doc = "0: SSEL1 asserted."]
    ASSERTED = 0,
    #[doc = "1: SSEL1 not asserted."]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL1_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL1_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL1_N` writer - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
pub type TXSSEL1_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL1_N_AW, O>;
impl<'a, const O: u8> TXSSEL1_N_W<'a, O> {
    #[doc = "SSEL1 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::ASSERTED)
    }
    #[doc = "SSEL1 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL2_N_AW {
    #[doc = "0: SSEL2 asserted."]
    ASSERTED = 0,
    #[doc = "1: SSEL2 not asserted."]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL2_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL2_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL2_N` writer - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
pub type TXSSEL2_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL2_N_AW, O>;
impl<'a, const O: u8> TXSSEL2_N_W<'a, O> {
    #[doc = "SSEL2 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::ASSERTED)
    }
    #[doc = "SSEL2 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL3_N_AW {
    #[doc = "0: SSEL3 asserted."]
    ASSERTED = 0,
    #[doc = "1: SSEL3 not asserted."]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL3_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL3_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL3_N` writer - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
pub type TXSSEL3_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL3_N_AW, O>;
impl<'a, const O: u8> TXSSEL3_N_W<'a, O> {
    #[doc = "SSEL3 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::ASSERTED)
    }
    #[doc = "SSEL3 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::NOT_ASSERTED)
    }
}
#[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_AW {
    #[doc = "0: SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NOT_DEASSERTED = 0,
    #[doc = "1: SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    DEASSERTED = 1,
}
impl From<EOT_AW> for bool {
    #[inline(always)]
    fn from(variant: EOT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` writer - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, EOT_AW, O>;
impl<'a, const O: u8> EOT_W<'a, O> {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::NOT_DEASSERTED)
    }
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::DEASSERTED)
    }
}
#[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOF_AW {
    #[doc = "0: Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NOT_EOF = 0,
    #[doc = "1: Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    EOF = 1,
}
impl From<EOF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` writer - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
pub type EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, EOF_AW, O>;
impl<'a, const O: u8> EOF_W<'a, O> {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut W {
        self.variant(EOF_AW::NOT_EOF)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn eof(self) -> &'a mut W {
        self.variant(EOF_AW::EOF)
    }
}
#[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIGNORE_AW {
    #[doc = "0: Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ = 0,
    #[doc = "1: Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE = 1,
}
impl From<RXIGNORE_AW> for bool {
    #[inline(always)]
    fn from(variant: RXIGNORE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIGNORE` writer - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
pub type RXIGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, RXIGNORE_AW, O>;
impl<'a, const O: u8> RXIGNORE_W<'a, O> {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::READ)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::IGNORE)
    }
}
#[doc = "Field `LEN` writer - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bits 0:15 - Transmit data to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Bit 16 - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W<16> {
        TXSSEL0_N_W::new(self)
    }
    #[doc = "Bit 17 - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W<17> {
        TXSSEL1_N_W::new(self)
    }
    #[doc = "Bit 18 - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W<18> {
        TXSSEL2_N_W::new(self)
    }
    #[doc = "Bit 19 - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W<19> {
        TXSSEL3_N_W::new(self)
    }
    #[doc = "Bit 20 - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EOT_W<20> {
        EOT_W::new(self)
    }
    #[doc = "Bit 21 - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EOF_W<21> {
        EOF_W::new(self)
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rxignore(&mut self) -> RXIGNORE_W<22> {
        RXIGNORE_W::new(self)
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<24> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO write data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr](index.html) module"]
pub struct FIFOWR_SPEC;
impl crate::RegisterSpec for FIFOWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifowr::R](R) reader structure"]
impl crate::Readable for FIFOWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifowr::W](W) writer structure"]
impl crate::Writable for FIFOWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOWR to value 0"]
impl crate::Resettable for FIFOWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
