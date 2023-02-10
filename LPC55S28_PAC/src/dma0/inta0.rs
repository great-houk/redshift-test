#[doc = "Register `INTA0` reader"]
pub struct R(crate::R<INTA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTA0` writer"]
pub struct W(crate::W<INTA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTA0_SPEC>;
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
impl From<crate::W<INTA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IA` reader - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub type IA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IA` writer - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub type IA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTA0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn ia(&self) -> IA_R {
        IA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    #[must_use]
    pub fn ia(&mut self) -> IA_W<0> {
        IA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt A status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inta0](index.html) module"]
pub struct INTA0_SPEC;
impl crate::RegisterSpec for INTA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inta0::R](R) reader structure"]
impl crate::Readable for INTA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inta0::W](W) writer structure"]
impl crate::Writable for INTA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTA0 to value 0"]
impl crate::Resettable for INTA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
