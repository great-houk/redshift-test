#[doc = "Register `INTB0` reader"]
pub struct R(crate::R<INTB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTB0` writer"]
pub struct W(crate::W<INTB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTB0_SPEC>;
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
impl From<crate::W<INTB0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IB` reader - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
pub type IB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IB` writer - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
pub type IB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTB0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn ib(&self) -> IB_R {
        IB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    #[must_use]
    pub fn ib(&mut self) -> IB_W<0> {
        IB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt B status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intb0](index.html) module"]
pub struct INTB0_SPEC;
impl crate::RegisterSpec for INTB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intb0::R](R) reader structure"]
impl crate::Readable for INTB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intb0::W](W) writer structure"]
impl crate::Writable for INTB0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTB0 to value 0"]
impl crate::Resettable for INTB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
