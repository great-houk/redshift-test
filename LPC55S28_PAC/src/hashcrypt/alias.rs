#[doc = "Register `ALIAS[%s]` writer"]
pub struct W(crate::W<ALIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALIAS_SPEC>;
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
impl From<crate::W<ALIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALIAS_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alias](index.html) module"]
pub struct ALIAS_SPEC;
impl crate::RegisterSpec for ALIAS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alias::W](W) writer structure"]
impl crate::Writable for ALIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALIAS[%s]
to value 0"]
impl crate::Resettable for ALIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
