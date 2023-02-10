#[doc = "Register `DATAPAYLOAD` reader"]
pub struct R(crate::R<DATAPAYLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAPAYLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAPAYLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAPAYLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAPAYLOAD` writer"]
pub struct W(crate::W<DATAPAYLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAPAYLOAD_SPEC>;
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
impl From<crate::W<DATAPAYLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAPAYLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_BASE` reader - Base address to be used by the hardware to find the start of the data payload section."]
pub type DAT_BASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAT_BASE` writer - Base address to be used by the hardware to find the start of the data payload section."]
pub type DAT_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATAPAYLOAD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&self) -> DAT_BASE_R {
        DAT_BASE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    #[must_use]
    pub fn dat_base(&mut self) -> DAT_BASE_W<16> {
        DAT_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address that indicates the start of the data payload buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datapayload](index.html) module"]
pub struct DATAPAYLOAD_SPEC;
impl crate::RegisterSpec for DATAPAYLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datapayload::R](R) reader structure"]
impl crate::Readable for DATAPAYLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datapayload::W](W) writer structure"]
impl crate::Writable for DATAPAYLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAPAYLOAD to value 0"]
impl crate::Resettable for DATAPAYLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
