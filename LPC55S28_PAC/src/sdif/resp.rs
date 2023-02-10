#[doc = "Register `RESP[%s]` reader"]
pub struct R(crate::R<RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESP[%s]` writer"]
pub struct W(crate::W<RESP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESP_SPEC>;
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
impl From<crate::W<RESP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE` reader - Bits of response."]
pub type RESPONSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESPONSE` writer - Bits of response."]
pub type RESPONSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    #[must_use]
    pub fn response(&mut self) -> RESPONSE_W<0> {
        RESPONSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](index.html) module"]
pub struct RESP_SPEC;
impl crate::RegisterSpec for RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp::R](R) reader structure"]
impl crate::Readable for RESP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resp::W](W) writer structure"]
impl crate::Writable for RESP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESP[%s]
to value 0"]
impl crate::Resettable for RESP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
