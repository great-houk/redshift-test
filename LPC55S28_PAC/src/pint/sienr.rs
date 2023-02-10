#[doc = "Register `SIENR` writer"]
pub struct W(crate::W<SIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIENR_SPEC>;
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
impl From<crate::W<SIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENRL` writer - Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SETENRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIENR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn setenrl(&mut self) -> SETENRL_W<0> {
        SETENRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt level or rising edge interrupt set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienr](index.html) module"]
pub struct SIENR_SPEC;
impl crate::RegisterSpec for SIENR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sienr::W](W) writer structure"]
impl crate::Writable for SIENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIENR to value 0"]
impl crate::Resettable for SIENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
