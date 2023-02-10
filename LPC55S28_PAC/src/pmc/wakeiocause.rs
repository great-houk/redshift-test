#[doc = "Register `WAKEIOCAUSE` reader"]
pub struct R(crate::R<WAKEIOCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEIOCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEIOCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEIOCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEIOCAUSE` writer"]
pub struct W(crate::W<WAKEIOCAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEIOCAUSE_SPEC>;
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
impl From<crate::W<WAKEIOCAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEIOCAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP0` reader - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP0_R = crate::BitReader<WAKEUP0_A>;
#[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP0_A {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    NOEVENT = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    EVENT = 1,
}
impl From<WAKEUP0_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP0_A {
        match self.bits {
            false => WAKEUP0_A::NOEVENT,
            true => WAKEUP0_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP0_A::EVENT
    }
}
#[doc = "Field `WAKEUP1` reader - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP1_R = crate::BitReader<WAKEUP1_A>;
#[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP1_A {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    NOEVENT = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    EVENT = 1,
}
impl From<WAKEUP1_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP1_A {
        match self.bits {
            false => WAKEUP1_A::NOEVENT,
            true => WAKEUP1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP1_A::EVENT
    }
}
#[doc = "Field `WAKEUP1` writer - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEIOCAUSE_SPEC, WAKEUP1_A, O>;
impl<'a, const O: u8> WAKEUP1_W<'a, O> {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP1_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP1_A::EVENT)
    }
}
#[doc = "Field `WAKEUP2` reader - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP2_R = crate::BitReader<WAKEUP2_A>;
#[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP2_A {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    NOEVENT = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    EVENT = 1,
}
impl From<WAKEUP2_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP2_A {
        match self.bits {
            false => WAKEUP2_A::NOEVENT,
            true => WAKEUP2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP2_A::EVENT
    }
}
#[doc = "Field `WAKEUP2` writer - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEIOCAUSE_SPEC, WAKEUP2_A, O>;
impl<'a, const O: u8> WAKEUP2_W<'a, O> {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP2_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP2_A::EVENT)
    }
}
#[doc = "Field `WAKEUP3` reader - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP3_R = crate::BitReader<WAKEUP3_A>;
#[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP3_A {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    NOEVENT = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    EVENT = 1,
}
impl From<WAKEUP3_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP3_A {
        match self.bits {
            false => WAKEUP3_A::NOEVENT,
            true => WAKEUP3_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP3_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP3_A::EVENT
    }
}
#[doc = "Field `WAKEUP3` writer - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
pub type WAKEUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEIOCAUSE_SPEC, WAKEUP3_A, O>;
impl<'a, const O: u8> WAKEUP3_W<'a, O> {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP3_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP3_A::EVENT)
    }
}
impl R {
    #[doc = "Bit 0 - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup0(&self) -> WAKEUP0_R {
        WAKEUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup1(&self) -> WAKEUP1_R {
        WAKEUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup2(&self) -> WAKEUP2_R {
        WAKEUP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup3(&self) -> WAKEUP3_R {
        WAKEUP3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup1(&mut self) -> WAKEUP1_W<1> {
        WAKEUP1_W::new(self)
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup2(&mut self) -> WAKEUP2_W<2> {
        WAKEUP2_W::new(self)
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup3(&mut self) -> WAKEUP3_W<3> {
        WAKEUP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeiocause](index.html) module"]
pub struct WAKEIOCAUSE_SPEC;
impl crate::RegisterSpec for WAKEIOCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeiocause::R](R) reader structure"]
impl crate::Readable for WAKEIOCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeiocause::W](W) writer structure"]
impl crate::Writable for WAKEIOCAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKEIOCAUSE to value 0"]
impl crate::Resettable for WAKEIOCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
