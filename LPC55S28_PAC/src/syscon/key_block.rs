#[doc = "Register `KEY_BLOCK` writer"]
pub struct W(crate::W<KEY_BLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_BLOCK_SPEC>;
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
impl From<crate::W<KEY_BLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_BLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_BLOCK` writer - Write a value to block quiddikey/PUF all index."]
pub type KEY_BLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEY_BLOCK_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write a value to block quiddikey/PUF all index."]
    #[inline(always)]
    #[must_use]
    pub fn key_block(&mut self) -> KEY_BLOCK_W<0> {
        KEY_BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "block quiddikey/PUF all index.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_block](index.html) module"]
pub struct KEY_BLOCK_SPEC;
impl crate::RegisterSpec for KEY_BLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key_block::W](W) writer structure"]
impl crate::Writable for KEY_BLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_BLOCK to value 0x3cc3_5aa5"]
impl crate::Resettable for KEY_BLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc3_5aa5;
}
