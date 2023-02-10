#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD` writer"]
pub struct W(crate::W<MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_SPEC>;
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
impl From<crate::W<MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDEN` reader - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
pub type WDEN_R = crate::BitReader<WDEN_A>;
#[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDEN_A {
    #[doc = "0: Stop. The watchdog timer is stopped."]
    STOP = 0,
    #[doc = "1: Run. The watchdog timer is running."]
    RUN = 1,
}
impl From<WDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEN_A {
        match self.bits {
            false => WDEN_A::STOP,
            true => WDEN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDEN_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDEN_A::RUN
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
pub type WDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDEN_A, O>;
impl<'a, const O: u8> WDEN_W<'a, O> {
    #[doc = "Stop. The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(WDEN_A::STOP)
    }
    #[doc = "Run. The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WDEN_A::RUN)
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
pub type WDRESET_R = crate::BitReader<WDRESET_A>;
#[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDRESET_A {
    #[doc = "0: Interrupt. A watchdog time-out will not cause a chip reset."]
    INTERRUPT = 0,
    #[doc = "1: Reset. A watchdog time-out will cause a chip reset."]
    RESET = 1,
}
impl From<WDRESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRESET_A {
        match self.bits {
            false => WDRESET_A::INTERRUPT,
            true => WDRESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESET_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESET_A::RESET
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
pub type WDRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDRESET_A, O>;
impl<'a, const O: u8> WDRESET_W<'a, O> {
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESET_A::INTERRUPT)
    }
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESET_A::RESET)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
pub type WDTOF_R = crate::BitReader<bool>;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
pub type WDTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, bool, O>;
#[doc = "Field `WDINT` reader - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
pub type WDINT_R = crate::BitReader<bool>;
#[doc = "Field `WDINT` writer - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
pub type WDINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, bool, O>;
#[doc = "Field `WDPROTECT` reader - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
pub type WDPROTECT_R = crate::BitReader<WDPROTECT_A>;
#[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDPROTECT_A {
    #[doc = "0: Flexible. The watchdog time-out value (TC) can be changed at any time."]
    FLEXIBLE = 0,
    #[doc = "1: Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    THRESHOLD = 1,
}
impl From<WDPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPROTECT_A {
        match self.bits {
            false => WDPROTECT_A::FLEXIBLE,
            true => WDPROTECT_A::THRESHOLD,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIBLE`"]
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        *self == WDPROTECT_A::FLEXIBLE
    }
    #[doc = "Checks if the value of the field is `THRESHOLD`"]
    #[inline(always)]
    pub fn is_threshold(&self) -> bool {
        *self == WDPROTECT_A::THRESHOLD
    }
}
#[doc = "Field `WDPROTECT` writer - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
pub type WDPROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDPROTECT_A, O>;
impl<'a, const O: u8> WDPROTECT_W<'a, O> {
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    #[inline(always)]
    pub fn flexible(self) -> &'a mut W {
        self.variant(WDPROTECT_A::FLEXIBLE)
    }
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline(always)]
    pub fn threshold(self) -> &'a mut W {
        self.variant(WDPROTECT_A::THRESHOLD)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WDEN_W<0> {
        WDEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    #[must_use]
    pub fn wdreset(&mut self) -> WDRESET_W<1> {
        WDRESET_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdtof(&mut self) -> WDTOF_W<2> {
        WDTOF_W::new(self)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    #[must_use]
    pub fn wdint(&mut self) -> WDINT_W<3> {
        WDINT_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdprotect(&mut self) -> WDPROTECT_W<4> {
        WDPROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_::W](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
