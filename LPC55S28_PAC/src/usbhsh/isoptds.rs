#[doc = "Register `ISOPTDS` reader"]
pub struct R(crate::R<ISOPTDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOPTDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOPTDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOPTDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOPTDS` writer"]
pub struct W(crate::W<ISOPTDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOPTDS_SPEC>;
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
impl From<crate::W<ISOPTDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOPTDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_SKIP` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type ISO_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISO_SKIP` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type ISO_SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISOPTDS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_skip(&self) -> ISO_SKIP_R {
        ISO_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    #[must_use]
    pub fn iso_skip(&mut self) -> ISO_SKIP_W<0> {
        ISO_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptds](index.html) module"]
pub struct ISOPTDS_SPEC;
impl crate::RegisterSpec for ISOPTDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoptds::R](R) reader structure"]
impl crate::Readable for ISOPTDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isoptds::W](W) writer structure"]
impl crate::Writable for ISOPTDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISOPTDS to value 0"]
impl crate::Resettable for ISOPTDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
