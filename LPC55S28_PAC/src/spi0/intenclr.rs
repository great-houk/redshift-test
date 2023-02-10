#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSAEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SSAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SSDEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SSDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `MSTIDLE` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type MSTIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 4 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn ssaen(&mut self) -> SSAEN_W<4> {
        SSAEN_W::new(self)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn ssden(&mut self) -> SSDEN_W<5> {
        SSDEN_W::new(self)
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn mstidle(&mut self) -> MSTIDLE_W<8> {
        MSTIDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
