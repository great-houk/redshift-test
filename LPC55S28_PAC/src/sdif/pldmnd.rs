#[doc = "Register `PLDMND` reader"]
pub struct R(crate::R<PLDMND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLDMND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLDMND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLDMND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLDMND` writer"]
pub struct W(crate::W<PLDMND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLDMND_SPEC>;
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
impl From<crate::W<PLDMND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLDMND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - Poll Demand."]
pub type PD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PD` writer - Poll Demand."]
pub type PD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLDMND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Poll Demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pldmnd](index.html) module"]
pub struct PLDMND_SPEC;
impl crate::RegisterSpec for PLDMND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pldmnd::R](R) reader structure"]
impl crate::Readable for PLDMND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pldmnd::W](W) writer structure"]
impl crate::Writable for PLDMND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLDMND to value 0"]
impl crate::Resettable for PLDMND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
