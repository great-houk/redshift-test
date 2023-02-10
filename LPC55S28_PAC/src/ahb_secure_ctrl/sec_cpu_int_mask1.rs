#[doc = "Register `SEC_CPU_INT_MASK1` reader"]
pub struct R(crate::R<SEC_CPU_INT_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CPU_INT_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CPU_INT_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CPU_INT_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CPU_INT_MASK1` writer"]
pub struct W(crate::W<SEC_CPU_INT_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CPU_INT_MASK1_SPEC>;
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
impl From<crate::W<SEC_CPU_INT_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CPU_INT_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` reader - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
pub type GPIO_INT0_IRQ4_R = crate::BitReader<GPIO_INT0_IRQ4_A>;
#[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ4_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ4_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ4_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ4_A {
        match self.bits {
            false => GPIO_INT0_IRQ4_A::INVISIBLE,
            true => GPIO_INT0_IRQ4_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` writer - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
pub type GPIO_INT0_IRQ4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, GPIO_INT0_IRQ4_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ4_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` reader - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
pub type GPIO_INT0_IRQ5_R = crate::BitReader<GPIO_INT0_IRQ5_A>;
#[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ5_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ5_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ5_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ5_A {
        match self.bits {
            false => GPIO_INT0_IRQ5_A::INVISIBLE,
            true => GPIO_INT0_IRQ5_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` writer - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
pub type GPIO_INT0_IRQ5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, GPIO_INT0_IRQ5_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ5_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` reader - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
pub type GPIO_INT0_IRQ6_R = crate::BitReader<GPIO_INT0_IRQ6_A>;
#[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ6_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ6_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ6_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ6_A {
        match self.bits {
            false => GPIO_INT0_IRQ6_A::INVISIBLE,
            true => GPIO_INT0_IRQ6_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` writer - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
pub type GPIO_INT0_IRQ6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, GPIO_INT0_IRQ6_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ6_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` reader - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
pub type GPIO_INT0_IRQ7_R = crate::BitReader<GPIO_INT0_IRQ7_A>;
#[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ7_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ7_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ7_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ7_A {
        match self.bits {
            false => GPIO_INT0_IRQ7_A::INVISIBLE,
            true => GPIO_INT0_IRQ7_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` writer - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
pub type GPIO_INT0_IRQ7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, GPIO_INT0_IRQ7_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ7_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::VISIBLE)
    }
}
#[doc = "Field `CTIMER2_IRQ` reader - Standard counter/timer 2 interrupt."]
pub type CTIMER2_IRQ_R = crate::BitReader<CTIMER2_IRQ_A>;
#[doc = "Standard counter/timer 2 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMER2_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER2_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER2_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMER2_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER2_IRQ_A {
        match self.bits {
            false => CTIMER2_IRQ_A::INVISIBLE,
            true => CTIMER2_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER2_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER2_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CTIMER2_IRQ` writer - Standard counter/timer 2 interrupt."]
pub type CTIMER2_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, CTIMER2_IRQ_A, O>;
impl<'a, const O: u8> CTIMER2_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `CTIMER4_IRQ` reader - Standard counter/timer 4 interrupt."]
pub type CTIMER4_IRQ_R = crate::BitReader<CTIMER4_IRQ_A>;
#[doc = "Standard counter/timer 4 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMER4_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER4_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER4_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMER4_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER4_IRQ_A {
        match self.bits {
            false => CTIMER4_IRQ_A::INVISIBLE,
            true => CTIMER4_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER4_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER4_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CTIMER4_IRQ` writer - Standard counter/timer 4 interrupt."]
pub type CTIMER4_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, CTIMER4_IRQ_A, O>;
impl<'a, const O: u8> CTIMER4_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `OS_EVENT_TIMER_IRQ` reader - OS Event Timer and OS Event Timer Wakeup interrupts"]
pub type OS_EVENT_TIMER_IRQ_R = crate::BitReader<OS_EVENT_TIMER_IRQ_A>;
#[doc = "OS Event Timer and OS Event Timer Wakeup interrupts\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OS_EVENT_TIMER_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<OS_EVENT_TIMER_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: OS_EVENT_TIMER_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl OS_EVENT_TIMER_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_TIMER_IRQ_A {
        match self.bits {
            false => OS_EVENT_TIMER_IRQ_A::INVISIBLE,
            true => OS_EVENT_TIMER_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQ_A::VISIBLE
    }
}
#[doc = "Field `OS_EVENT_TIMER_IRQ` writer - OS Event Timer and OS Event Timer Wakeup interrupts"]
pub type OS_EVENT_TIMER_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, OS_EVENT_TIMER_IRQ_A, O>;
impl<'a, const O: u8> OS_EVENT_TIMER_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED0` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED0_R = crate::BitReader<RESERVED0_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED0_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED0_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED0_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED0_A {
        match self.bits {
            false => RESERVED0_A::INVISIBLE,
            true => RESERVED0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED0_A::VISIBLE
    }
}
#[doc = "Field `RESERVED0` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED0_A, O>;
impl<'a, const O: u8> RESERVED0_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED0_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED1` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED1_R = crate::BitReader<RESERVED1_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED1_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED1_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED1_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED1_A {
        match self.bits {
            false => RESERVED1_A::INVISIBLE,
            true => RESERVED1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED1_A::VISIBLE
    }
}
#[doc = "Field `RESERVED1` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED1_A, O>;
impl<'a, const O: u8> RESERVED1_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED1_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED2` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED2_R = crate::BitReader<RESERVED2_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED2_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED2_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED2_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED2_A {
        match self.bits {
            false => RESERVED2_A::INVISIBLE,
            true => RESERVED2_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED2_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED2_A::VISIBLE
    }
}
#[doc = "Field `RESERVED2` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED2_A, O>;
impl<'a, const O: u8> RESERVED2_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED2_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED2_A::VISIBLE)
    }
}
#[doc = "Field `SDIO_IRQ` reader - SDIO Controller interrupt."]
pub type SDIO_IRQ_R = crate::BitReader<SDIO_IRQ_A>;
#[doc = "SDIO Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SDIO_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_IRQ_A {
        match self.bits {
            false => SDIO_IRQ_A::INVISIBLE,
            true => SDIO_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SDIO_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SDIO_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SDIO_IRQ` writer - SDIO Controller interrupt."]
pub type SDIO_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SDIO_IRQ_A, O>;
impl<'a, const O: u8> SDIO_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDIO_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDIO_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED3` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED3_R = crate::BitReader<RESERVED3_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED3_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED3_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED3_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED3_A {
        match self.bits {
            false => RESERVED3_A::INVISIBLE,
            true => RESERVED3_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED3_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED3_A::VISIBLE
    }
}
#[doc = "Field `RESERVED3` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED3_A, O>;
impl<'a, const O: u8> RESERVED3_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED3_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED3_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED4` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED4_R = crate::BitReader<RESERVED4_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED4_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED4_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED4_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED4_A {
        match self.bits {
            false => RESERVED4_A::INVISIBLE,
            true => RESERVED4_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED4_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED4_A::VISIBLE
    }
}
#[doc = "Field `RESERVED4` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED4_A, O>;
impl<'a, const O: u8> RESERVED4_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED4_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED4_A::VISIBLE)
    }
}
#[doc = "Field `RESERVED5` reader - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED5_R = crate::BitReader<RESERVED5_A>;
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESERVED5_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED5_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED5_A) -> Self {
        variant as u8 != 0
    }
}
impl RESERVED5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED5_A {
        match self.bits {
            false => RESERVED5_A::INVISIBLE,
            true => RESERVED5_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED5_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED5_A::VISIBLE
    }
}
#[doc = "Field `RESERVED5` writer - Reserved. Read value is undefined, only zero should be written."]
pub type RESERVED5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, RESERVED5_A, O>;
impl<'a, const O: u8> RESERVED5_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED5_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED5_A::VISIBLE)
    }
}
#[doc = "Field `USB1_PHY_IRQ` reader - USB High Speed PHY Controller interrupt."]
pub type USB1_PHY_IRQ_R = crate::BitReader<USB1_PHY_IRQ_A>;
#[doc = "USB High Speed PHY Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB1_PHY_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB1_PHY_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USB1_PHY_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_IRQ_A {
        match self.bits {
            false => USB1_PHY_IRQ_A::INVISIBLE,
            true => USB1_PHY_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_PHY_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_PHY_IRQ_A::VISIBLE
    }
}
#[doc = "Field `USB1_PHY_IRQ` writer - USB High Speed PHY Controller interrupt."]
pub type USB1_PHY_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, USB1_PHY_IRQ_A, O>;
impl<'a, const O: u8> USB1_PHY_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_PHY_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_PHY_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `USB1_IRQ` reader - USB High Speed Controller interrupt."]
pub type USB1_IRQ_R = crate::BitReader<USB1_IRQ_A>;
#[doc = "USB High Speed Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USB1_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_IRQ_A {
        match self.bits {
            false => USB1_IRQ_A::INVISIBLE,
            true => USB1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_IRQ_A::VISIBLE
    }
}
#[doc = "Field `USB1_IRQ` writer - USB High Speed Controller interrupt."]
pub type USB1_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, USB1_IRQ_A, O>;
impl<'a, const O: u8> USB1_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `USB1_NEEDCLK` reader - USB High Speed Controller Clock request interrupt."]
pub type USB1_NEEDCLK_R = crate::BitReader<USB1_NEEDCLK_A>;
#[doc = "USB High Speed Controller Clock request interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB1_NEEDCLK_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB1_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USB1_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_NEEDCLK_A {
        match self.bits {
            false => USB1_NEEDCLK_A::INVISIBLE,
            true => USB1_NEEDCLK_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_NEEDCLK_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_NEEDCLK_A::VISIBLE
    }
}
#[doc = "Field `USB1_NEEDCLK` writer - USB High Speed Controller Clock request interrupt."]
pub type USB1_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, USB1_NEEDCLK_A, O>;
impl<'a, const O: u8> USB1_NEEDCLK_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::VISIBLE)
    }
}
#[doc = "Field `SEC_HYPERVISOR_CALL_IRQ` reader - Secure fault Hyper Visor call interrupt."]
pub type SEC_HYPERVISOR_CALL_IRQ_R = crate::BitReader<SEC_HYPERVISOR_CALL_IRQ_A>;
#[doc = "Secure fault Hyper Visor call interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_HYPERVISOR_CALL_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SEC_HYPERVISOR_CALL_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_HYPERVISOR_CALL_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_HYPERVISOR_CALL_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_HYPERVISOR_CALL_IRQ_A {
        match self.bits {
            false => SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE,
            true => SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SEC_HYPERVISOR_CALL_IRQ` writer - Secure fault Hyper Visor call interrupt."]
pub type SEC_HYPERVISOR_CALL_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SEC_HYPERVISOR_CALL_IRQ_A, O>;
impl<'a, const O: u8> SEC_HYPERVISOR_CALL_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ0` reader - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type SEC_GPIO_INT0_IRQ0_R = crate::BitReader<SEC_GPIO_INT0_IRQ0_A>;
#[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_GPIO_INT0_IRQ0_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SEC_GPIO_INT0_IRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT0_IRQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_GPIO_INT0_IRQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT0_IRQ0_A {
        match self.bits {
            false => SEC_GPIO_INT0_IRQ0_A::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0_A::VISIBLE
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ0` writer - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type SEC_GPIO_INT0_IRQ0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SEC_GPIO_INT0_IRQ0_A, O>;
impl<'a, const O: u8> SEC_GPIO_INT0_IRQ0_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0_A::VISIBLE)
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ1` reader - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type SEC_GPIO_INT0_IRQ1_R = crate::BitReader<SEC_GPIO_INT0_IRQ1_A>;
#[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_GPIO_INT0_IRQ1_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SEC_GPIO_INT0_IRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT0_IRQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_GPIO_INT0_IRQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT0_IRQ1_A {
        match self.bits {
            false => SEC_GPIO_INT0_IRQ1_A::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1_A::VISIBLE
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ1` writer - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type SEC_GPIO_INT0_IRQ1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SEC_GPIO_INT0_IRQ1_A, O>;
impl<'a, const O: u8> SEC_GPIO_INT0_IRQ1_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1_A::VISIBLE)
    }
}
#[doc = "Field `PLU_IRQ` reader - Programmable Look-Up Controller interrupt."]
pub type PLU_IRQ_R = crate::BitReader<PLU_IRQ_A>;
#[doc = "Programmable Look-Up Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLU_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<PLU_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PLU_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl PLU_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_IRQ_A {
        match self.bits {
            false => PLU_IRQ_A::INVISIBLE,
            true => PLU_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PLU_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PLU_IRQ_A::VISIBLE
    }
}
#[doc = "Field `PLU_IRQ` writer - Programmable Look-Up Controller interrupt."]
pub type PLU_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, PLU_IRQ_A, O>;
impl<'a, const O: u8> PLU_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PLU_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PLU_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SEC_VIO_IRQ` reader - Security Violation interrupt."]
pub type SEC_VIO_IRQ_R = crate::BitReader<SEC_VIO_IRQ_A>;
#[doc = "Security Violation interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_VIO_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SEC_VIO_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_VIO_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_IRQ_A {
        match self.bits {
            false => SEC_VIO_IRQ_A::INVISIBLE,
            true => SEC_VIO_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_VIO_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_VIO_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SEC_VIO_IRQ` writer - Security Violation interrupt."]
pub type SEC_VIO_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SEC_VIO_IRQ_A, O>;
impl<'a, const O: u8> SEC_VIO_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SHA_IRQ` reader - HASH-AES interrupt."]
pub type SHA_IRQ_R = crate::BitReader<SHA_IRQ_A>;
#[doc = "HASH-AES interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHA_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SHA_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SHA_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_IRQ_A {
        match self.bits {
            false => SHA_IRQ_A::INVISIBLE,
            true => SHA_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SHA_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SHA_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SHA_IRQ` writer - HASH-AES interrupt."]
pub type SHA_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SHA_IRQ_A, O>;
impl<'a, const O: u8> SHA_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SHA_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SHA_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `CASPER_IRQ` reader - CASPER interrupt."]
pub type CASPER_IRQ_R = crate::BitReader<CASPER_IRQ_A>;
#[doc = "CASPER interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CASPER_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_IRQ_A {
        match self.bits {
            false => CASPER_IRQ_A::INVISIBLE,
            true => CASPER_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CASPER_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CASPER_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CASPER_IRQ` writer - CASPER interrupt."]
pub type CASPER_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, CASPER_IRQ_A, O>;
impl<'a, const O: u8> CASPER_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CASPER_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CASPER_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `PUFKEY_IRQ` reader - PUF interrupt."]
pub type PUFKEY_IRQ_R = crate::BitReader<PUFKEY_IRQ_A>;
#[doc = "PUF interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUFKEY_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<PUFKEY_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PUFKEY_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl PUFKEY_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUFKEY_IRQ_A {
        match self.bits {
            false => PUFKEY_IRQ_A::INVISIBLE,
            true => PUFKEY_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PUFKEY_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PUFKEY_IRQ_A::VISIBLE
    }
}
#[doc = "Field `PUFKEY_IRQ` writer - PUF interrupt."]
pub type PUFKEY_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, PUFKEY_IRQ_A, O>;
impl<'a, const O: u8> PUFKEY_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PUFKEY_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PUFKEY_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `PQ_IRQ` reader - Power Quad interrupt."]
pub type PQ_IRQ_R = crate::BitReader<PQ_IRQ_A>;
#[doc = "Power Quad interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PQ_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<PQ_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl PQ_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_IRQ_A {
        match self.bits {
            false => PQ_IRQ_A::INVISIBLE,
            true => PQ_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PQ_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PQ_IRQ_A::VISIBLE
    }
}
#[doc = "Field `PQ_IRQ` writer - Power Quad interrupt."]
pub type PQ_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, PQ_IRQ_A, O>;
impl<'a, const O: u8> PQ_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PQ_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PQ_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SDMA1_IRQ` reader - System DMA 1 (Secure) interrupt"]
pub type SDMA1_IRQ_R = crate::BitReader<SDMA1_IRQ_A>;
#[doc = "System DMA 1 (Secure) interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SDMA1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMA1_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_IRQ_A {
        match self.bits {
            false => SDMA1_IRQ_A::INVISIBLE,
            true => SDMA1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SDMA1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SDMA1_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SDMA1_IRQ` writer - System DMA 1 (Secure) interrupt"]
pub type SDMA1_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, SDMA1_IRQ_A, O>;
impl<'a, const O: u8> SDMA1_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA1_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `LSPI_HS_IRQ` reader - High Speed SPI interrupt"]
pub type LSPI_HS_IRQ_R = crate::BitReader<LSPI_HS_IRQ_A>;
#[doc = "High Speed SPI interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSPI_HS_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<LSPI_HS_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: LSPI_HS_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LSPI_HS_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPI_HS_IRQ_A {
        match self.bits {
            false => LSPI_HS_IRQ_A::INVISIBLE,
            true => LSPI_HS_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == LSPI_HS_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == LSPI_HS_IRQ_A::VISIBLE
    }
}
#[doc = "Field `LSPI_HS_IRQ` writer - High Speed SPI interrupt"]
pub type LSPI_HS_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK1_SPEC, LSPI_HS_IRQ_A, O>;
impl<'a, const O: u8> LSPI_HS_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQ_A::VISIBLE)
    }
}
impl R {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&self) -> GPIO_INT0_IRQ4_R {
        GPIO_INT0_IRQ4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&self) -> GPIO_INT0_IRQ5_R {
        GPIO_INT0_IRQ5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&self) -> GPIO_INT0_IRQ6_R {
        GPIO_INT0_IRQ6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&self) -> GPIO_INT0_IRQ7_R {
        GPIO_INT0_IRQ7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub fn ctimer2_irq(&self) -> CTIMER2_IRQ_R {
        CTIMER2_IRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub fn ctimer4_irq(&self) -> CTIMER4_IRQ_R {
        CTIMER4_IRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub fn os_event_timer_irq(&self) -> OS_EVENT_TIMER_IRQ_R {
        OS_EVENT_TIMER_IRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    pub fn sdio_irq(&self) -> SDIO_IRQ_R {
        SDIO_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub fn usb1_phy_irq(&self) -> USB1_PHY_IRQ_R {
        USB1_PHY_IRQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb1_irq(&self) -> USB1_IRQ_R {
        USB1_IRQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb1_needclk(&self) -> USB1_NEEDCLK_R {
        USB1_NEEDCLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub fn sec_hypervisor_call_irq(&self) -> SEC_HYPERVISOR_CALL_IRQ_R {
        SEC_HYPERVISOR_CALL_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq0(&self) -> SEC_GPIO_INT0_IRQ0_R {
        SEC_GPIO_INT0_IRQ0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq1(&self) -> SEC_GPIO_INT0_IRQ1_R {
        SEC_GPIO_INT0_IRQ1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub fn plu_irq(&self) -> PLU_IRQ_R {
        PLU_IRQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    pub fn sec_vio_irq(&self) -> SEC_VIO_IRQ_R {
        SEC_VIO_IRQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    pub fn sha_irq(&self) -> SHA_IRQ_R {
        SHA_IRQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    pub fn casper_irq(&self) -> CASPER_IRQ_R {
        CASPER_IRQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    pub fn pufkey_irq(&self) -> PUFKEY_IRQ_R {
        PUFKEY_IRQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    pub fn pq_irq(&self) -> PQ_IRQ_R {
        PQ_IRQ_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub fn sdma1_irq(&self) -> SDMA1_IRQ_R {
        SDMA1_IRQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    pub fn lspi_hs_irq(&self) -> LSPI_HS_IRQ_R {
        LSPI_HS_IRQ_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq4(&mut self) -> GPIO_INT0_IRQ4_W<0> {
        GPIO_INT0_IRQ4_W::new(self)
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq5(&mut self) -> GPIO_INT0_IRQ5_W<1> {
        GPIO_INT0_IRQ5_W::new(self)
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq6(&mut self) -> GPIO_INT0_IRQ6_W<2> {
        GPIO_INT0_IRQ6_W::new(self)
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq7(&mut self) -> GPIO_INT0_IRQ7_W<3> {
        GPIO_INT0_IRQ7_W::new(self)
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctimer2_irq(&mut self) -> CTIMER2_IRQ_W<4> {
        CTIMER2_IRQ_W::new(self)
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctimer4_irq(&mut self) -> CTIMER4_IRQ_W<5> {
        CTIMER4_IRQ_W::new(self)
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn os_event_timer_irq(&mut self) -> OS_EVENT_TIMER_IRQ_W<6> {
        OS_EVENT_TIMER_IRQ_W::new(self)
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<7> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<8> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<9> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_irq(&mut self) -> SDIO_IRQ_W<10> {
        SDIO_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<11> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<12> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<13> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb1_phy_irq(&mut self) -> USB1_PHY_IRQ_W<14> {
        USB1_PHY_IRQ_W::new(self)
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb1_irq(&mut self) -> USB1_IRQ_W<15> {
        USB1_IRQ_W::new(self)
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb1_needclk(&mut self) -> USB1_NEEDCLK_W<16> {
        USB1_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sec_hypervisor_call_irq(&mut self) -> SEC_HYPERVISOR_CALL_IRQ_W<17> {
        SEC_HYPERVISOR_CALL_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_int0_irq0(&mut self) -> SEC_GPIO_INT0_IRQ0_W<18> {
        SEC_GPIO_INT0_IRQ0_W::new(self)
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sec_gpio_int0_irq1(&mut self) -> SEC_GPIO_INT0_IRQ1_W<19> {
        SEC_GPIO_INT0_IRQ1_W::new(self)
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn plu_irq(&mut self) -> PLU_IRQ_W<20> {
        PLU_IRQ_W::new(self)
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sec_vio_irq(&mut self) -> SEC_VIO_IRQ_W<21> {
        SEC_VIO_IRQ_W::new(self)
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sha_irq(&mut self) -> SHA_IRQ_W<22> {
        SHA_IRQ_W::new(self)
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn casper_irq(&mut self) -> CASPER_IRQ_W<23> {
        CASPER_IRQ_W::new(self)
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pufkey_irq(&mut self) -> PUFKEY_IRQ_W<24> {
        PUFKEY_IRQ_W::new(self)
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pq_irq(&mut self) -> PQ_IRQ_W<25> {
        PQ_IRQ_W::new(self)
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdma1_irq(&mut self) -> SDMA1_IRQ_W<26> {
        SDMA1_IRQ_W::new(self)
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lspi_hs_irq(&mut self) -> LSPI_HS_IRQ_W<27> {
        LSPI_HS_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Interrupt mask for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cpu_int_mask1](index.html) module"]
pub struct SEC_CPU_INT_MASK1_SPEC;
impl crate::RegisterSpec for SEC_CPU_INT_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_cpu_int_mask1::R](R) reader structure"]
impl crate::Readable for SEC_CPU_INT_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_cpu_int_mask1::W](W) writer structure"]
impl crate::Writable for SEC_CPU_INT_MASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CPU_INT_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for SEC_CPU_INT_MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
