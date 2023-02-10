#[doc = "Register `CMPA_PROG_IN_PROGRESS` reader"]
pub struct R(crate::R<CMPA_PROG_IN_PROGRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPA_PROG_IN_PROGRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPA_PROG_IN_PROGRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPA_PROG_IN_PROGRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPA_PROG_IN_PROGRESS` writer"]
pub struct W(crate::W<CMPA_PROG_IN_PROGRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPA_PROG_IN_PROGRESS_SPEC>;
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
impl From<crate::W<CMPA_PROG_IN_PROGRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPA_PROG_IN_PROGRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPA_PROG_IN_PROGRESS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    #[must_use]
    pub fn field(&mut self) -> FIELD_W<0> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpa_prog_in_progress](index.html) module"]
pub struct CMPA_PROG_IN_PROGRESS_SPEC;
impl crate::RegisterSpec for CMPA_PROG_IN_PROGRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpa_prog_in_progress::R](R) reader structure"]
impl crate::Readable for CMPA_PROG_IN_PROGRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpa_prog_in_progress::W](W) writer structure"]
impl crate::Writable for CMPA_PROG_IN_PROGRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPA_PROG_IN_PROGRESS to value 0"]
impl crate::Resettable for CMPA_PROG_IN_PROGRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
