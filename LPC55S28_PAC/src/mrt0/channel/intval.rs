#[doc = "Register `INTVAL` reader"]
pub struct R(crate::R<INTVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTVAL` writer"]
pub struct W(crate::W<INTVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTVAL_SPEC>;
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
impl From<crate::W<INTVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVALUE` reader - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
pub type IVALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IVALUE` writer - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
pub type IVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTVAL_SPEC, u32, u32, 24, O>;
#[doc = "Field `LOAD` reader - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
pub type LOAD_R = crate::BitReader<LOAD_A>;
#[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOAD_A {
    #[doc = "0: No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD = 0,
    #[doc = "1: Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD = 1,
}
impl From<LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl LOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            false => LOAD_A::NO_FORCE_LOAD,
            true => LOAD_A::FORCE_LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_no_force_load(&self) -> bool {
        *self == LOAD_A::NO_FORCE_LOAD
    }
    #[doc = "Checks if the value of the field is `FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_force_load(&self) -> bool {
        *self == LOAD_A::FORCE_LOAD
    }
}
#[doc = "Field `LOAD` writer - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTVAL_SPEC, LOAD_A, O>;
impl<'a, const O: u8> LOAD_W<'a, O> {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline(always)]
    pub fn no_force_load(self) -> &'a mut W {
        self.variant(LOAD_A::NO_FORCE_LOAD)
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline(always)]
    pub fn force_load(self) -> &'a mut W {
        self.variant(LOAD_A::FORCE_LOAD)
    }
}
impl R {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&self) -> IVALUE_R {
        IVALUE_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    #[must_use]
    pub fn ivalue(&mut self) -> IVALUE_W<0> {
        IVALUE_W::new(self)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<31> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intval](index.html) module"]
pub struct INTVAL_SPEC;
impl crate::RegisterSpec for INTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intval::R](R) reader structure"]
impl crate::Readable for INTVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intval::W](W) writer structure"]
impl crate::Writable for INTVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTVAL to value 0"]
impl crate::Resettable for INTVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
