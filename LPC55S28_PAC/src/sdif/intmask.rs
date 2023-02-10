#[doc = "Register `INTMASK` reader"]
pub struct R(crate::R<INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMASK` writer"]
pub struct W(crate::W<INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMASK_SPEC>;
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
impl From<crate::W<INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDET` reader - Card detect."]
pub type CDET_R = crate::BitReader<bool>;
#[doc = "Field `CDET` writer - Card detect."]
pub type CDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RE` reader - Response error."]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Response error."]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `CDONE` reader - Command done."]
pub type CDONE_R = crate::BitReader<bool>;
#[doc = "Field `CDONE` writer - Command done."]
pub type CDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DTO` reader - Data transfer over."]
pub type DTO_R = crate::BitReader<bool>;
#[doc = "Field `DTO` writer - Data transfer over."]
pub type DTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `TXDR` reader - Transmit FIFO data request."]
pub type TXDR_R = crate::BitReader<bool>;
#[doc = "Field `TXDR` writer - Transmit FIFO data request."]
pub type TXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RXDR` reader - Receive FIFO data request."]
pub type RXDR_R = crate::BitReader<bool>;
#[doc = "Field `RXDR` writer - Receive FIFO data request."]
pub type RXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RCRC` reader - Response CRC error."]
pub type RCRC_R = crate::BitReader<bool>;
#[doc = "Field `RCRC` writer - Response CRC error."]
pub type RCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DCRC` reader - Data CRC error."]
pub type DCRC_R = crate::BitReader<bool>;
#[doc = "Field `DCRC` writer - Data CRC error."]
pub type DCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RTO` reader - Response time-out."]
pub type RTO_R = crate::BitReader<bool>;
#[doc = "Field `RTO` writer - Response time-out."]
pub type RTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DRTO` reader - Data read time-out."]
pub type DRTO_R = crate::BitReader<bool>;
#[doc = "Field `DRTO` writer - Data read time-out."]
pub type DRTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `HTO` reader - Data starvation-by-host time-out (HTO)."]
pub type HTO_R = crate::BitReader<bool>;
#[doc = "Field `HTO` writer - Data starvation-by-host time-out (HTO)."]
pub type HTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `FRUN` reader - FIFO underrun/overrun error."]
pub type FRUN_R = crate::BitReader<bool>;
#[doc = "Field `FRUN` writer - FIFO underrun/overrun error."]
pub type FRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `HLE` reader - Hardware locked write error."]
pub type HLE_R = crate::BitReader<bool>;
#[doc = "Field `HLE` writer - Hardware locked write error."]
pub type HLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `SBE` reader - Start-bit error."]
pub type SBE_R = crate::BitReader<bool>;
#[doc = "Field `SBE` writer - Start-bit error."]
pub type SBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `ACD` reader - Auto command done."]
pub type ACD_R = crate::BitReader<bool>;
#[doc = "Field `ACD` writer - Auto command done."]
pub type ACD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `EBE` reader - End-bit error (read)/Write no CRC."]
pub type EBE_R = crate::BitReader<bool>;
#[doc = "Field `EBE` writer - End-bit error (read)/Write no CRC."]
pub type EBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `SDIO_INT_MASK` reader - Mask SDIO interrupt."]
pub type SDIO_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_INT_MASK` writer - Mask SDIO interrupt."]
pub type SDIO_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&self) -> CDET_R {
        CDET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&self) -> CDONE_R {
        CDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&self) -> TXDR_R {
        TXDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&self) -> DCRC_R {
        DCRC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&self) -> DRTO_R {
        DRTO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&self) -> HTO_R {
        HTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&self) -> FRUN_R {
        FRUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&self) -> HLE_R {
        HLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub fn ebe(&self) -> EBE_R {
        EBE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    #[must_use]
    pub fn cdet(&mut self) -> CDET_W<0> {
        CDET_W::new(self)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<1> {
        RE_W::new(self)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    #[must_use]
    pub fn cdone(&mut self) -> CDONE_W<2> {
        CDONE_W::new(self)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    #[must_use]
    pub fn dto(&mut self) -> DTO_W<3> {
        DTO_W::new(self)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<4> {
        TXDR_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    #[must_use]
    pub fn rxdr(&mut self) -> RXDR_W<5> {
        RXDR_W::new(self)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    #[must_use]
    pub fn rcrc(&mut self) -> RCRC_W<6> {
        RCRC_W::new(self)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    #[must_use]
    pub fn dcrc(&mut self) -> DCRC_W<7> {
        DCRC_W::new(self)
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<8> {
        RTO_W::new(self)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    #[must_use]
    pub fn drto(&mut self) -> DRTO_W<9> {
        DRTO_W::new(self)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    #[must_use]
    pub fn hto(&mut self) -> HTO_W<10> {
        HTO_W::new(self)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    #[must_use]
    pub fn frun(&mut self) -> FRUN_W<11> {
        FRUN_W::new(self)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    #[must_use]
    pub fn hle(&mut self) -> HLE_W<12> {
        HLE_W::new(self)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    #[must_use]
    pub fn sbe(&mut self) -> SBE_W<13> {
        SBE_W::new(self)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    #[must_use]
    pub fn acd(&mut self) -> ACD_W<14> {
        ACD_W::new(self)
    }
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    #[must_use]
    pub fn ebe(&mut self) -> EBE_W<15> {
        EBE_W::new(self)
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<16> {
        SDIO_INT_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmask](index.html) module"]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmask::R](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmask::W](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
