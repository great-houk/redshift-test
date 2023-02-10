#[doc = "Register `MATCH_H` reader"]
pub struct R(crate::R<MATCH_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATCH_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATCH_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATCH_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH_H` writer"]
pub struct W(crate::W<MATCH_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATCH_H_SPEC>;
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
impl From<crate::W<MATCH_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATCH_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH_VALUE` reader - The value written (upper 10 bits) to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
pub type MATCH_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH_VALUE` writer - The value written (upper 10 bits) to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
pub type MATCH_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATCH_H_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - The value written (upper 10 bits) to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
    #[inline(always)]
    pub fn match_value(&self) -> MATCH_VALUE_R {
        MATCH_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The value written (upper 10 bits) to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn match_value(&mut self) -> MATCH_VALUE_W<0> {
        MATCH_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_h](index.html) module"]
pub struct MATCH_H_SPEC;
impl crate::RegisterSpec for MATCH_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [match_h::R](R) reader structure"]
impl crate::Readable for MATCH_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [match_h::W](W) writer structure"]
impl crate::Writable for MATCH_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATCH_H to value 0"]
impl crate::Resettable for MATCH_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
