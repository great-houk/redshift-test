#[doc = "Register `FIFOWR48H` writer"]
pub struct W(crate::W<FIFOWR48H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR48H_SPEC>;
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
impl From<crate::W<FIFOWR48H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWR48H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR48H_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:23 - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr48h](index.html) module"]
pub struct FIFOWR48H_SPEC;
impl crate::RegisterSpec for FIFOWR48H_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fifowr48h::W](W) writer structure"]
impl crate::Writable for FIFOWR48H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOWR48H to value 0"]
impl crate::Resettable for FIFOWR48H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
