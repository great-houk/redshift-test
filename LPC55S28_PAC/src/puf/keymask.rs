#[doc = "Register `KEYMASK[%s]` writer"]
pub struct W(crate::W<KEYMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYMASK_SPEC>;
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
impl From<crate::W<KEYMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYMASK` writer - no description available"]
pub type KEYMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYMASK_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn keymask(&mut self) -> KEYMASK_W<0> {
        KEYMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Only reset in case of full IC reset\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keymask](index.html) module"]
pub struct KEYMASK_SPEC;
impl crate::RegisterSpec for KEYMASK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keymask::W](W) writer structure"]
impl crate::Writable for KEYMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYMASK[%s]
to value 0"]
impl crate::Resettable for KEYMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
