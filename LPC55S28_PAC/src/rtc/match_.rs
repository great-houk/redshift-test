#[doc = "Register `MATCH` reader"]
pub struct R(crate::R<MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH` writer"]
pub struct W(crate::W<MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATCH_SPEC>;
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
impl From<crate::W<MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATVAL` reader - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
pub type MATVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MATVAL` writer - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
pub type MATVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATCH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub fn matval(&self) -> MATVAL_R {
        MATVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn matval(&mut self) -> MATVAL_W<0> {
        MATVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](index.html) module"]
pub struct MATCH_SPEC;
impl crate::RegisterSpec for MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [match_::R](R) reader structure"]
impl crate::Readable for MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [match_::W](W) writer structure"]
impl crate::Writable for MATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATCH to value 0xffff_ffff"]
impl crate::Resettable for MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
