#[doc = "Register `DMA1_REQ_ENA` reader"]
pub struct R(crate::R<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_REQ_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA1_REQ_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA1_REQ_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1_REQ_ENA` writer"]
pub struct W(crate::W<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_REQ_ENA_SPEC>;
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
impl From<crate::W<DMA1_REQ_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA1_REQ_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ_ENA` reader - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REQ_ENA` writer - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA1_REQ_ENA_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> REQ_ENA_R {
        REQ_ENA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn req_ena(&mut self) -> REQ_ENA_W<0> {
        REQ_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DMA1 requests\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena](index.html) module"]
pub struct DMA1_REQ_ENA_SPEC;
impl crate::RegisterSpec for DMA1_REQ_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1_req_ena::R](R) reader structure"]
impl crate::Readable for DMA1_REQ_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1_req_ena::W](W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA1_REQ_ENA to value 0x03ff"]
impl crate::Resettable for DMA1_REQ_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
