#[doc = "Register `STOPA` reader"]
pub struct R(crate::R<STOPA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STOPA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STOPA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STOPA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STOPA` writer"]
pub struct W(crate::W<STOPA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STOPA_SPEC>;
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
impl From<crate::W<STOPA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STOPA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPA` reader - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
pub type STOPA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STOPA` writer - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
pub type STOPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STOPA_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    #[must_use]
    pub fn stopa(&mut self) -> STOPA_W<0> {
        STOPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "end address for next flash command, if command operates on address ranges\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stopa](index.html) module"]
pub struct STOPA_SPEC;
impl crate::RegisterSpec for STOPA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stopa::R](R) reader structure"]
impl crate::Readable for STOPA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stopa::W](W) writer structure"]
impl crate::Writable for STOPA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STOPA to value 0"]
impl crate::Resettable for STOPA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
