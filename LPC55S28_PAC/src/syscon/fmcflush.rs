#[doc = "Register `FMCFLUSH` writer"]
pub struct W(crate::W<FMCFLUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMCFLUSH_SPEC>;
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
impl From<crate::W<FMCFLUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMCFLUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flush control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSH_AW {
    #[doc = "0: No action is performed."]
    NO_FLUSH = 0,
    #[doc = "1: Flush the FMC buffer contents."]
    FLUSH = 1,
}
impl From<FLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Flush control"]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMCFLUSH_SPEC, FLUSH_AW, O>;
impl<'a, const O: u8> FLUSH_W<'a, O> {
    #[doc = "No action is performed."]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FLUSH_AW::NO_FLUSH)
    }
    #[doc = "Flush the FMC buffer contents."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FLUSH_AW::FLUSH)
    }
}
impl W {
    #[doc = "Bit 0 - Flush control"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<0> {
        FLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMCflush control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmcflush](index.html) module"]
pub struct FMCFLUSH_SPEC;
impl crate::RegisterSpec for FMCFLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmcflush::W](W) writer structure"]
impl crate::Writable for FMCFLUSH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCFLUSH to value 0"]
impl crate::Resettable for FMCFLUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
