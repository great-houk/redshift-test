#[doc = "Register `HCHCCA` reader"]
pub struct R(crate::R<HCHCCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCHCCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCHCCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCHCCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCHCCA` writer"]
pub struct W(crate::W<HCHCCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCHCCA_SPEC>;
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
impl From<crate::W<HCHCCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCHCCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCCA` reader - Base address of the Host Controller Communication Area."]
pub type HCCA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HCCA` writer - Base address of the Host Controller Communication Area."]
pub type HCCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCHCCA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub fn hcca(&self) -> HCCA_R {
        HCCA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    #[must_use]
    pub fn hcca(&mut self) -> HCCA_W<8> {
        HCCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the host controller communication area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hchcca](index.html) module"]
pub struct HCHCCA_SPEC;
impl crate::RegisterSpec for HCHCCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hchcca::R](R) reader structure"]
impl crate::Readable for HCHCCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hchcca::W](W) writer structure"]
impl crate::Writable for HCHCCA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCHCCA to value 0"]
impl crate::Resettable for HCHCCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
