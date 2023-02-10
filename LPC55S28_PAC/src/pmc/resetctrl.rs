#[doc = "Register `RESETCTRL` reader"]
pub struct R(crate::R<RESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETCTRL` writer"]
pub struct W(crate::W<RESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETCTRL_SPEC>;
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
impl From<crate::W<RESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` reader - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DPDWAKEUPRESETENABLE_R = crate::BitReader<DPDWAKEUPRESETENABLE_A>;
#[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPDWAKEUPRESETENABLE_A {
    #[doc = "0: Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE = 0,
    #[doc = "1: Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE = 1,
}
impl From<DPDWAKEUPRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DPDWAKEUPRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DPDWAKEUPRESETENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDWAKEUPRESETENABLE_A {
        match self.bits {
            false => DPDWAKEUPRESETENABLE_A::DISABLE,
            true => DPDWAKEUPRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLE_A::ENABLE
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` writer - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DPDWAKEUPRESETENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESETCTRL_SPEC, DPDWAKEUPRESETENABLE_A, O>;
impl<'a, const O: u8> DPDWAKEUPRESETENABLE_W<'a, O> {
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::DISABLE)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::ENABLE)
    }
}
#[doc = "Field `BODVBATRESETENABLE` reader - BOD VBAT reset enable."]
pub type BODVBATRESETENABLE_R = crate::BitReader<BODVBATRESETENABLE_A>;
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODVBATRESETENABLE_A {
    #[doc = "0: BOD VBAT reset is disable."]
    DISABLE = 0,
    #[doc = "1: BOD VBAT reset is enable."]
    ENABLE = 1,
}
impl From<BODVBATRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBATRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BODVBATRESETENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBATRESETENABLE_A {
        match self.bits {
            false => BODVBATRESETENABLE_A::DISABLE,
            true => BODVBATRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODVBATRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODVBATRESETENABLE_A::ENABLE
    }
}
#[doc = "Field `BODVBATRESETENABLE` writer - BOD VBAT reset enable."]
pub type BODVBATRESETENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESETCTRL_SPEC, BODVBATRESETENABLE_A, O>;
impl<'a, const O: u8> BODVBATRESETENABLE_W<'a, O> {
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLE_A::DISABLE)
    }
    #[doc = "BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLE_A::ENABLE)
    }
}
#[doc = "Field `SWRRESETENABLE` reader - Software reset enable."]
pub type SWRRESETENABLE_R = crate::BitReader<SWRRESETENABLE_A>;
#[doc = "Software reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRRESETENABLE_A {
    #[doc = "0: Software reset is disable."]
    DISABLE = 0,
    #[doc = "1: Software reset is enable."]
    ENABLE = 1,
}
impl From<SWRRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SWRRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRRESETENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRRESETENABLE_A {
        match self.bits {
            false => SWRRESETENABLE_A::DISABLE,
            true => SWRRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWRRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWRRESETENABLE_A::ENABLE
    }
}
#[doc = "Field `SWRRESETENABLE` writer - Software reset enable."]
pub type SWRRESETENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESETCTRL_SPEC, SWRRESETENABLE_A, O>;
impl<'a, const O: u8> SWRRESETENABLE_W<'a, O> {
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::DISABLE)
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&self) -> DPDWAKEUPRESETENABLE_R {
        DPDWAKEUPRESETENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetenable(&self) -> BODVBATRESETENABLE_R {
        BODVBATRESETENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&self) -> SWRRESETENABLE_R {
        SWRRESETENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    #[must_use]
    pub fn dpdwakeupresetenable(&mut self) -> DPDWAKEUPRESETENABLE_W<0> {
        DPDWAKEUPRESETENABLE_W::new(self)
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    #[must_use]
    pub fn bodvbatresetenable(&mut self) -> BODVBATRESETENABLE_W<1> {
        BODVBATRESETENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    #[must_use]
    pub fn swrresetenable(&mut self) -> SWRRESETENABLE_W<3> {
        SWRRESETENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctrl](index.html) module"]
pub struct RESETCTRL_SPEC;
impl crate::RegisterSpec for RESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetctrl::R](R) reader structure"]
impl crate::Readable for RESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetctrl::W](W) writer structure"]
impl crate::Writable for RESETCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETCTRL to value 0"]
impl crate::Resettable for RESETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
