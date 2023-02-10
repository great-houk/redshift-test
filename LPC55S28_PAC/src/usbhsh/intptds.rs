#[doc = "Register `INTPTDS` reader"]
pub struct R(crate::R<INTPTDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPTDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPTDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPTDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPTDS` writer"]
pub struct W(crate::W<INTPTDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPTDS_SPEC>;
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
impl From<crate::W<INTPTDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPTDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type INT_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type INT_SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTPTDS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&self) -> INT_SKIP_R {
        INT_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    #[must_use]
    pub fn int_skip(&mut self) -> INT_SKIP_W<0> {
        INT_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptds](index.html) module"]
pub struct INTPTDS_SPEC;
impl crate::RegisterSpec for INTPTDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intptds::R](R) reader structure"]
impl crate::Readable for INTPTDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intptds::W](W) writer structure"]
impl crate::Writable for INTPTDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTPTDS to value 0"]
impl crate::Resettable for INTPTDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
