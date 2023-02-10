#[doc = "Register `DIRSET0` writer"]
pub struct W(crate::W<DIRSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRSET0_SPEC>;
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
impl From<crate::W<DIRSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRSETP` writer - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
pub type DIRSETP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIRSET0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp(&mut self) -> DIRSETP_W<0> {
        DIRSETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set pin direction bits for port\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset0](index.html) module"]
pub struct DIRSET0_SPEC;
impl crate::RegisterSpec for DIRSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dirset0::W](W) writer structure"]
impl crate::Writable for DIRSET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRSET0 to value 0"]
impl crate::Resettable for DIRSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
