#[doc = "Register `EVTIMERH` reader"]
pub struct R(crate::R<EVTIMERH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTIMERH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTIMERH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTIMERH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTIMERH` writer"]
pub struct W(crate::W<EVTIMERH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTIMERH_SPEC>;
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
impl From<crate::W<EVTIMERH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTIMERH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - A read reflects the current value of the upper 10 bits of the 42-bits EVTIMER. Note there is only one EVTIMER, readable from all domains."]
pub type EVTIMER_COUNT_VALUE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - A read reflects the current value of the upper 10 bits of the 42-bits EVTIMER. Note there is only one EVTIMER, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUE_R {
        EVTIMER_COUNT_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVTIMER High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtimerh](index.html) module"]
pub struct EVTIMERH_SPEC;
impl crate::RegisterSpec for EVTIMERH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtimerh::R](R) reader structure"]
impl crate::Readable for EVTIMERH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtimerh::W](W) writer structure"]
impl crate::Writable for EVTIMERH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTIMERH to value 0"]
impl crate::Resettable for EVTIMERH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
