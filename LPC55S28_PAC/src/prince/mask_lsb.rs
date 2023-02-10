#[doc = "Register `MASK_LSB` writer"]
pub struct W(crate::W<MASK_LSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_LSB_SPEC>;
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
impl From<crate::W<MASK_LSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_LSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKVAL` writer - Value of the 32 Least Significant Bits of the 64-bit data mask."]
pub type MASKVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK_LSB_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    #[must_use]
    pub fn maskval(&mut self) -> MASKVAL_W<0> {
        MASKVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Mask register, 32 Least Significant Bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_lsb](index.html) module"]
pub struct MASK_LSB_SPEC;
impl crate::RegisterSpec for MASK_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mask_lsb::W](W) writer structure"]
impl crate::Writable for MASK_LSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK_LSB to value 0"]
impl crate::Resettable for MASK_LSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
