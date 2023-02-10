#[doc = "Register `CODEINPUT` writer"]
pub struct W(crate::W<CODEINPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEINPUT_SPEC>;
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
impl From<crate::W<CODEINPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEINPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODEIN` writer - AC/KC input data"]
pub type CODEIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CODEINPUT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - AC/KC input data"]
    #[inline(always)]
    #[must_use]
    pub fn codein(&mut self) -> CODEIN_W<0> {
        CODEIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Code Input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codeinput](index.html) module"]
pub struct CODEINPUT_SPEC;
impl crate::RegisterSpec for CODEINPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [codeinput::W](W) writer structure"]
impl crate::Writable for CODEINPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CODEINPUT to value 0"]
impl crate::Resettable for CODEINPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
