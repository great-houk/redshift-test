#[doc = "Register `EVENT` writer"]
pub struct W(crate::W<EVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_SPEC>;
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
impl From<crate::W<EVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` writer - When bit is set, the controller and flash are reset."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `WAKEUP` writer - When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `ABORT` writer - When bit is set, a running program/erase command is aborted."]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - When bit is set, the controller and flash are reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<0> {
        RST_W::new(self)
    }
    #[doc = "Bit 1 - When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<1> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 2 - When bit is set, a running program/erase command is aborted."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<2> {
        ABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event](index.html) module"]
pub struct EVENT_SPEC;
impl crate::RegisterSpec for EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [event::W](W) writer structure"]
impl crate::Writable for EVENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EVENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
