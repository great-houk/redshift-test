#[doc = "Register `DIRCLR0` writer"]
pub struct W(crate::W<DIRCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCLR0_SPEC>;
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
impl From<crate::W<DIRCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRCLRP` writer - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
pub type DIRCLRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIRCLR0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp(&mut self) -> DIRCLRP_W<0> {
        DIRCLRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear pin direction bits for port\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr0](index.html) module"]
pub struct DIRCLR0_SPEC;
impl crate::RegisterSpec for DIRCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dirclr0::W](W) writer structure"]
impl crate::Writable for DIRCLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCLR0 to value 0"]
impl crate::Resettable for DIRCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
