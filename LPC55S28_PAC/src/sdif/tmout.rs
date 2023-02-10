#[doc = "Register `TMOUT` reader"]
pub struct R(crate::R<TMOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMOUT` writer"]
pub struct W(crate::W<TMOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMOUT_SPEC>;
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
impl From<crate::W<TMOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE_TIMEOUT` reader - Response time-out value."]
pub type RESPONSE_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESPONSE_TIMEOUT` writer - Response time-out value."]
pub type RESPONSE_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMOUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_TIMEOUT` reader - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
pub type DATA_TIMEOUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA_TIMEOUT` writer - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
pub type DATA_TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMOUT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUT_R {
        RESPONSE_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DATA_TIMEOUT_R {
        DATA_TIMEOUT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    #[must_use]
    pub fn response_timeout(&mut self) -> RESPONSE_TIMEOUT_W<0> {
        RESPONSE_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout(&mut self) -> DATA_TIMEOUT_W<8> {
        DATA_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-out register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmout](index.html) module"]
pub struct TMOUT_SPEC;
impl crate::RegisterSpec for TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmout::R](R) reader structure"]
impl crate::Readable for TMOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmout::W](W) writer structure"]
impl crate::Writable for TMOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMOUT to value 0xffff_ff40"]
impl crate::Resettable for TMOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff40;
}
