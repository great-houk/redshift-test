#[doc = "Register `LUT_TRUTH[%s]` reader"]
pub struct R(crate::R<LUT_TRUTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_TRUTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_TRUTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_TRUTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT_TRUTH[%s]` writer"]
pub struct W(crate::W<LUT_TRUTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_TRUTH_SPEC>;
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
impl From<crate::W<LUT_TRUTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_TRUTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUTn_TRUTH` reader - Specifies the Truth Table contents for LUT0.."]
pub type LUTN_TRUTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LUTn_TRUTH` writer - Specifies the Truth Table contents for LUT0.."]
pub type LUTN_TRUTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUT_TRUTH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    pub fn lutn_truth(&self) -> LUTN_TRUTH_R {
        LUTN_TRUTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    #[must_use]
    pub fn lutn_truth(&mut self) -> LUTN_TRUTH_W<0> {
        LUTN_TRUTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specifies the Truth Table contents for LUTLUTn\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_truth](index.html) module"]
pub struct LUT_TRUTH_SPEC;
impl crate::RegisterSpec for LUT_TRUTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut_truth::R](R) reader structure"]
impl crate::Readable for LUT_TRUTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut_truth::W](W) writer structure"]
impl crate::Writable for LUT_TRUTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT_TRUTH[%s]
to value 0"]
impl crate::Resettable for LUT_TRUTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
