#[doc = "Register `SETVALID0` writer"]
pub struct W(crate::W<SETVALID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETVALID0_SPEC>;
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
impl From<crate::W<SETVALID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETVALID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SV` writer - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
pub type SV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETVALID0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
    #[inline(always)]
    #[must_use]
    pub fn sv(&mut self) -> SV_W<0> {
        SV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set ValidPending control bits for all DMA channels.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setvalid0](index.html) module"]
pub struct SETVALID0_SPEC;
impl crate::RegisterSpec for SETVALID0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [setvalid0::W](W) writer structure"]
impl crate::Writable for SETVALID0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETVALID0 to value 0"]
impl crate::Resettable for SETVALID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
