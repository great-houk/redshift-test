#[doc = "Register `SETTRIG0` writer"]
pub struct W(crate::W<SETTRIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETTRIG0_SPEC>;
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
impl From<crate::W<SETTRIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETTRIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG` writer - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
pub type TRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETTRIG0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<0> {
        TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Trigger control bits for all DMA channels.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [settrig0](index.html) module"]
pub struct SETTRIG0_SPEC;
impl crate::RegisterSpec for SETTRIG0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [settrig0::W](W) writer structure"]
impl crate::Writable for SETTRIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETTRIG0 to value 0"]
impl crate::Resettable for SETTRIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
