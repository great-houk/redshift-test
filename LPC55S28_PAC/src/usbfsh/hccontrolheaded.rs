#[doc = "Register `HCCONTROLHEADED` reader"]
pub struct R(crate::R<HCCONTROLHEADED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCONTROLHEADED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCONTROLHEADED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCONTROLHEADED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCONTROLHEADED` writer"]
pub struct W(crate::W<HCCONTROLHEADED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCONTROLHEADED_SPEC>;
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
impl From<crate::W<HCCONTROLHEADED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCONTROLHEADED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHED` reader - HC traverses the Control list starting with the HcControlHeadED pointer."]
pub type CHED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHED` writer - HC traverses the Control list starting with the HcControlHeadED pointer."]
pub type CHED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCCONTROLHEADED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub fn ched(&self) -> CHED_R {
        CHED_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    #[must_use]
    pub fn ched(&mut self) -> CHED_W<4> {
        CHED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrolheaded](index.html) module"]
pub struct HCCONTROLHEADED_SPEC;
impl crate::RegisterSpec for HCCONTROLHEADED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccontrolheaded::R](R) reader structure"]
impl crate::Readable for HCCONTROLHEADED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccontrolheaded::W](W) writer structure"]
impl crate::Writable for HCCONTROLHEADED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCONTROLHEADED to value 0"]
impl crate::Resettable for HCCONTROLHEADED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
