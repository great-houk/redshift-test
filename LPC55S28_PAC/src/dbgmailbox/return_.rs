#[doc = "Register `RETURN` reader"]
pub struct R(crate::R<RETURN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETURN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETURN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETURN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETURN` writer"]
pub struct W(crate::W<RETURN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETURN_SPEC>;
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
impl From<crate::W<RETURN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETURN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RET` reader - The Return value from ROM."]
pub type RET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RET` writer - The Return value from ROM."]
pub type RET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RETURN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The Return value from ROM."]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Return value from ROM."]
    #[inline(always)]
    #[must_use]
    pub fn ret(&mut self) -> RET_W<0> {
        RET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Return value from ROM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [return_](index.html) module"]
pub struct RETURN_SPEC;
impl crate::RegisterSpec for RETURN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [return_::R](R) reader structure"]
impl crate::Readable for RETURN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [return_::W](W) writer structure"]
impl crate::Writable for RETURN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETURN to value 0"]
impl crate::Resettable for RETURN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
