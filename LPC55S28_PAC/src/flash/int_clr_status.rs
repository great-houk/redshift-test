#[doc = "Register `INT_CLR_STATUS` writer"]
pub struct W(crate::W<INT_CLR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_STATUS_SPEC>;
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
impl From<crate::W<INT_CLR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_STATUS_SPEC, bool, O>;
#[doc = "Field `ERR` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_STATUS_SPEC, bool, O>;
#[doc = "Field `DONE` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_STATUS_SPEC, bool, O>;
#[doc = "Field `ECC_ERR` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type ECC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_STATUS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn fail(&mut self) -> FAIL_W<0> {
        FAIL_W::new(self)
    }
    #[doc = "Bit 1 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 2 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<2> {
        DONE_W::new(self)
    }
    #[doc = "Bit 3 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<3> {
        ECC_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear interrupt status bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_status](index.html) module"]
pub struct INT_CLR_STATUS_SPEC;
impl crate::RegisterSpec for INT_CLR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_status::W](W) writer structure"]
impl crate::Writable for INT_CLR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR_STATUS to value 0"]
impl crate::Resettable for INT_CLR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
