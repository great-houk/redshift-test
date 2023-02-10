#[doc = "Register `DMA0_ITRIG_ENA` reader"]
pub struct R(crate::R<DMA0_ITRIG_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA0_ITRIG_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA0_ITRIG_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA0_ITRIG_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA0_ITRIG_ENA` writer"]
pub struct W(crate::W<DMA0_ITRIG_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0_ITRIG_ENA_SPEC>;
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
impl From<crate::W<DMA0_ITRIG_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA0_ITRIG_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITRIG_ENA` reader - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ITRIG_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ITRIG_ENA` writer - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ITRIG_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA0_ITRIG_ENA_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ITRIG_ENA_R {
        ITRIG_ENA_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn itrig_ena(&mut self) -> ITRIG_ENA_W<0> {
        ITRIG_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DMA0 triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena](index.html) module"]
pub struct DMA0_ITRIG_ENA_SPEC;
impl crate::RegisterSpec for DMA0_ITRIG_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma0_itrig_ena::R](R) reader structure"]
impl crate::Readable for DMA0_ITRIG_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena::W](W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA0_ITRIG_ENA to value 0x003f_ffff"]
impl crate::Resettable for DMA0_ITRIG_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
