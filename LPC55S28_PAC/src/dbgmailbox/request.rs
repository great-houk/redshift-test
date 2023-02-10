#[doc = "Register `REQUEST` reader"]
pub struct R(crate::R<REQUEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQUEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQUEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQUEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQUEST` writer"]
pub struct W(crate::W<REQUEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQUEST_SPEC>;
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
impl From<crate::W<REQUEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQUEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - Request Value"]
pub type REQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REQ` writer - Request Value"]
pub type REQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REQUEST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Request Value"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Request Value"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC seed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [request](index.html) module"]
pub struct REQUEST_SPEC;
impl crate::RegisterSpec for REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [request::R](R) reader structure"]
impl crate::Readable for REQUEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [request::W](W) writer structure"]
impl crate::Writable for REQUEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQUEST to value 0xffff"]
impl crate::Resettable for REQUEST_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
