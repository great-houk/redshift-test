#[doc = "Register `PDRUNCFGCLR0` writer"]
pub struct W(crate::W<PDRUNCFGCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFGCLR0_SPEC>;
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
impl From<crate::W<PDRUNCFGCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFGCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDRUNCFGCLR0` writer - Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
pub type PDRUNCFGCLR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDRUNCFGCLR0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    #[must_use]
    pub fn pdruncfgclr0(&mut self) -> PDRUNCFGCLR0_W<0> {
        PDRUNCFGCLR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgclr0](index.html) module"]
pub struct PDRUNCFGCLR0_SPEC;
impl crate::RegisterSpec for PDRUNCFGCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdruncfgclr0::W](W) writer structure"]
impl crate::Writable for PDRUNCFGCLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFGCLR0 to value 0"]
impl crate::Resettable for PDRUNCFGCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
