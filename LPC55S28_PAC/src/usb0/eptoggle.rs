#[doc = "Register `EPTOGGLE` reader"]
pub struct R(crate::R<EPTOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPTOGGLE` writer"]
pub struct W(crate::W<EPTOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTOGGLE_SPEC>;
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
impl From<crate::W<EPTOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOGGLE` reader - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub type TOGGLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOGGLE` writer - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub type TOGGLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPTOGGLE_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn toggle(&mut self) -> TOGGLE_W<0> {
        TOGGLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint toggle register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptoggle](index.html) module"]
pub struct EPTOGGLE_SPEC;
impl crate::RegisterSpec for EPTOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptoggle::R](R) reader structure"]
impl crate::Readable for EPTOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eptoggle::W](W) writer structure"]
impl crate::Writable for EPTOGGLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPTOGGLE to value 0"]
impl crate::Resettable for EPTOGGLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
