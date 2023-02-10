#[doc = "Register `ENABLESET0` reader"]
pub struct R(crate::R<ENABLESET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLESET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLESET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLESET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLESET0` writer"]
pub struct W(crate::W<ENABLESET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLESET0_SPEC>;
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
impl From<crate::W<ENABLESET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLESET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub type ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENA` writer - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub type ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENABLESET0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<0> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable read and Set for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableset0](index.html) module"]
pub struct ENABLESET0_SPEC;
impl crate::RegisterSpec for ENABLESET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableset0::R](R) reader structure"]
impl crate::Readable for ENABLESET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableset0::W](W) writer structure"]
impl crate::Writable for ENABLESET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLESET0 to value 0"]
impl crate::Resettable for ENABLESET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
