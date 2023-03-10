#[doc = "Register `RNR` reader"]
pub struct R(crate::R<RNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNR` writer"]
pub struct W(crate::W<RNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNR_SPEC>;
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
impl From<crate::W<RNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION` reader - Region number."]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - Region number."]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Region Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnr](index.html) module"]
pub struct RNR_SPEC;
impl crate::RegisterSpec for RNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnr::R](R) reader structure"]
impl crate::Readable for RNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnr::W](W) writer structure"]
impl crate::Writable for RNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNR to value 0"]
impl crate::Resettable for RNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
