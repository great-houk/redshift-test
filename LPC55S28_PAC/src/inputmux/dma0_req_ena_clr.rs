#[doc = "Register `DMA0_REQ_ENA_CLR` writer"]
pub struct W(crate::W<DMA0_REQ_ENA_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0_REQ_ENA_CLR_SPEC>;
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
impl From<crate::W<DMA0_REQ_ENA_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA0_REQ_ENA_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
pub type CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA0_REQ_ENA_CLR_SPEC, u32, u32, 23, O>;
impl W {
    #[doc = "Bits 0:22 - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_req_ena_clr](index.html) module"]
pub struct DMA0_REQ_ENA_CLR_SPEC;
impl crate::RegisterSpec for DMA0_REQ_ENA_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma0_req_ena_clr::W](W) writer structure"]
impl crate::Writable for DMA0_REQ_ENA_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA0_REQ_ENA_CLR to value 0"]
impl crate::Resettable for DMA0_REQ_ENA_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
