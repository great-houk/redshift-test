#[doc = "Register `KEYINPUT` writer"]
pub struct W(crate::W<KEYINPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYINPUT_SPEC>;
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
impl From<crate::W<KEYINPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYINPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYIN` writer - Key input data"]
pub type KEYIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYINPUT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Key input data"]
    #[inline(always)]
    #[must_use]
    pub fn keyin(&mut self) -> KEYIN_W<0> {
        KEYIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Key Input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyinput](index.html) module"]
pub struct KEYINPUT_SPEC;
impl crate::RegisterSpec for KEYINPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keyinput::W](W) writer structure"]
impl crate::Writable for KEYINPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYINPUT to value 0"]
impl crate::Resettable for KEYINPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
