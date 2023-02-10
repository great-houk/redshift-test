#[doc = "Register `CAPCLR` writer"]
pub struct W(crate::W<CAPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCLR_SPEC>;
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
impl From<crate::W<CAPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCLR0` writer - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
pub type CAPCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, bool, O>;
#[doc = "Field `CAPCLR1` writer - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
pub type CAPCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, bool, O>;
#[doc = "Field `CAPCLR2` writer - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
pub type CAPCLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, bool, O>;
#[doc = "Field `CAPCLR3` writer - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
pub type CAPCLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr0(&mut self) -> CAPCLR0_W<0> {
        CAPCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr1(&mut self) -> CAPCLR1_W<1> {
        CAPCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr2(&mut self) -> CAPCLR2_W<2> {
        CAPCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr3(&mut self) -> CAPCLR3_W<3> {
        CAPCLR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capclr](index.html) module"]
pub struct CAPCLR_SPEC;
impl crate::RegisterSpec for CAPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [capclr::W](W) writer structure"]
impl crate::Writable for CAPCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPCLR to value 0"]
impl crate::Resettable for CAPCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
