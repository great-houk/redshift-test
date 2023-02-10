#[doc = "Register `SEC_CPU_INT_MASK0` reader"]
pub struct R(crate::R<SEC_CPU_INT_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CPU_INT_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CPU_INT_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CPU_INT_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CPU_INT_MASK0` writer"]
pub struct W(crate::W<SEC_CPU_INT_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CPU_INT_MASK0_SPEC>;
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
impl From<crate::W<SEC_CPU_INT_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CPU_INT_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS_IRQ` reader - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub type SYS_IRQ_R = crate::BitReader<SYS_IRQ_A>;
#[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYS_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SYS_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SYS_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_IRQ_A {
        match self.bits {
            false => SYS_IRQ_A::INVISIBLE,
            true => SYS_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SYS_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SYS_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SYS_IRQ` writer - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub type SYS_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, SYS_IRQ_A, O>;
impl<'a, const O: u8> SYS_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SYS_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SYS_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SDMA0_IRQ` reader - System DMA 0 (non-secure) interrupt."]
pub type SDMA0_IRQ_R = crate::BitReader<SDMA0_IRQ_A>;
#[doc = "System DMA 0 (non-secure) interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SDMA0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMA0_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_IRQ_A {
        match self.bits {
            false => SDMA0_IRQ_A::INVISIBLE,
            true => SDMA0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SDMA0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SDMA0_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SDMA0_IRQ` writer - System DMA 0 (non-secure) interrupt."]
pub type SDMA0_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, SDMA0_IRQ_A, O>;
impl<'a, const O: u8> SDMA0_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA0_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` reader - GPIO Group 0 interrupt."]
pub type GPIO_GLOBALINT0_IRQ_R = crate::BitReader<GPIO_GLOBALINT0_IRQ_A>;
#[doc = "GPIO Group 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_GLOBALINT0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_GLOBALINT0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_GLOBALINT0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_GLOBALINT0_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_GLOBALINT0_IRQ_A {
        match self.bits {
            false => GPIO_GLOBALINT0_IRQ_A::INVISIBLE,
            true => GPIO_GLOBALINT0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_GLOBALINT0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_GLOBALINT0_IRQ_A::VISIBLE
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` writer - GPIO Group 0 interrupt."]
pub type GPIO_GLOBALINT0_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_GLOBALINT0_IRQ_A, O>;
impl<'a, const O: u8> GPIO_GLOBALINT0_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` reader - GPIO Group 1 interrupt."]
pub type GPIO_GLOBALINT1_IRQ_R = crate::BitReader<GPIO_GLOBALINT1_IRQ_A>;
#[doc = "GPIO Group 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_GLOBALINT1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_GLOBALINT1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_GLOBALINT1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_GLOBALINT1_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_GLOBALINT1_IRQ_A {
        match self.bits {
            false => GPIO_GLOBALINT1_IRQ_A::INVISIBLE,
            true => GPIO_GLOBALINT1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_GLOBALINT1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_GLOBALINT1_IRQ_A::VISIBLE
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` writer - GPIO Group 1 interrupt."]
pub type GPIO_GLOBALINT1_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_GLOBALINT1_IRQ_A, O>;
impl<'a, const O: u8> GPIO_GLOBALINT1_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type GPIO_INT0_IRQ0_R = crate::BitReader<GPIO_INT0_IRQ0_A>;
#[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ0_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ0_A {
        match self.bits {
            false => GPIO_INT0_IRQ0_A::INVISIBLE,
            true => GPIO_INT0_IRQ0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type GPIO_INT0_IRQ0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_INT0_IRQ0_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ0_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type GPIO_INT0_IRQ1_R = crate::BitReader<GPIO_INT0_IRQ1_A>;
#[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ1_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ1_A {
        match self.bits {
            false => GPIO_INT0_IRQ1_A::INVISIBLE,
            true => GPIO_INT0_IRQ1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type GPIO_INT0_IRQ1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_INT0_IRQ1_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ1_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub type GPIO_INT0_IRQ2_R = crate::BitReader<GPIO_INT0_IRQ2_A>;
#[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ2_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ2_A {
        match self.bits {
            false => GPIO_INT0_IRQ2_A::INVISIBLE,
            true => GPIO_INT0_IRQ2_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub type GPIO_INT0_IRQ2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_INT0_IRQ2_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ2_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::VISIBLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub type GPIO_INT0_IRQ3_R = crate::BitReader<GPIO_INT0_IRQ3_A>;
#[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ3_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ3_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ3_A {
        match self.bits {
            false => GPIO_INT0_IRQ3_A::INVISIBLE,
            true => GPIO_INT0_IRQ3_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::VISIBLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub type GPIO_INT0_IRQ3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, GPIO_INT0_IRQ3_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ3_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::VISIBLE)
    }
}
#[doc = "Field `UTICK_IRQ` reader - Micro Tick Timer interrupt."]
pub type UTICK_IRQ_R = crate::BitReader<UTICK_IRQ_A>;
#[doc = "Micro Tick Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<UTICK_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_IRQ_A {
        match self.bits {
            false => UTICK_IRQ_A::INVISIBLE,
            true => UTICK_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == UTICK_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == UTICK_IRQ_A::VISIBLE
    }
}
#[doc = "Field `UTICK_IRQ` writer - Micro Tick Timer interrupt."]
pub type UTICK_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, UTICK_IRQ_A, O>;
impl<'a, const O: u8> UTICK_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(UTICK_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(UTICK_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `MRT_IRQ` reader - Multi-Rate Timer interrupt."]
pub type MRT_IRQ_R = crate::BitReader<MRT_IRQ_A>;
#[doc = "Multi-Rate Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<MRT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_IRQ_A {
        match self.bits {
            false => MRT_IRQ_A::INVISIBLE,
            true => MRT_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == MRT_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == MRT_IRQ_A::VISIBLE
    }
}
#[doc = "Field `MRT_IRQ` writer - Multi-Rate Timer interrupt."]
pub type MRT_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, MRT_IRQ_A, O>;
impl<'a, const O: u8> MRT_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MRT_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(MRT_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `CTIMER0_IRQ` reader - Standard counter/timer 0 interrupt."]
pub type CTIMER0_IRQ_R = crate::BitReader<CTIMER0_IRQ_A>;
#[doc = "Standard counter/timer 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMER0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMER0_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER0_IRQ_A {
        match self.bits {
            false => CTIMER0_IRQ_A::INVISIBLE,
            true => CTIMER0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER0_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CTIMER0_IRQ` writer - Standard counter/timer 0 interrupt."]
pub type CTIMER0_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, CTIMER0_IRQ_A, O>;
impl<'a, const O: u8> CTIMER0_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `CTIMER1_IRQ` reader - Standard counter/timer 1 interrupt."]
pub type CTIMER1_IRQ_R = crate::BitReader<CTIMER1_IRQ_A>;
#[doc = "Standard counter/timer 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMER1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMER1_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER1_IRQ_A {
        match self.bits {
            false => CTIMER1_IRQ_A::INVISIBLE,
            true => CTIMER1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER1_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CTIMER1_IRQ` writer - Standard counter/timer 1 interrupt."]
pub type CTIMER1_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, CTIMER1_IRQ_A, O>;
impl<'a, const O: u8> CTIMER1_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `SCT_IRQ` reader - SCTimer/PWM interrupt."]
pub type SCT_IRQ_R = crate::BitReader<SCT_IRQ_A>;
#[doc = "SCTimer/PWM interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SCT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_IRQ_A {
        match self.bits {
            false => SCT_IRQ_A::INVISIBLE,
            true => SCT_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SCT_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SCT_IRQ_A::VISIBLE
    }
}
#[doc = "Field `SCT_IRQ` writer - SCTimer/PWM interrupt."]
pub type SCT_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, SCT_IRQ_A, O>;
impl<'a, const O: u8> SCT_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SCT_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SCT_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `CTIMER3_IRQ` reader - Standard counter/timer 3 interrupt."]
pub type CTIMER3_IRQ_R = crate::BitReader<CTIMER3_IRQ_A>;
#[doc = "Standard counter/timer 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMER3_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER3_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER3_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTIMER3_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER3_IRQ_A {
        match self.bits {
            false => CTIMER3_IRQ_A::INVISIBLE,
            true => CTIMER3_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER3_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER3_IRQ_A::VISIBLE
    }
}
#[doc = "Field `CTIMER3_IRQ` writer - Standard counter/timer 3 interrupt."]
pub type CTIMER3_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, CTIMER3_IRQ_A, O>;
impl<'a, const O: u8> CTIMER3_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` reader - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM0_IRQ_R = crate::BitReader<FLEXCOMM0_IRQ_A>;
#[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM0_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_IRQ_A {
        match self.bits {
            false => FLEXCOMM0_IRQ_A::INVISIBLE,
            true => FLEXCOMM0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM0_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` writer - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM0_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM0_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM0_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` reader - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM1_IRQ_R = crate::BitReader<FLEXCOMM1_IRQ_A>;
#[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM1_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_IRQ_A {
        match self.bits {
            false => FLEXCOMM1_IRQ_A::INVISIBLE,
            true => FLEXCOMM1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM1_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` writer - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM1_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM1_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM1_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` reader - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM2_IRQ_R = crate::BitReader<FLEXCOMM2_IRQ_A>;
#[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM2_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM2_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_IRQ_A {
        match self.bits {
            false => FLEXCOMM2_IRQ_A::INVISIBLE,
            true => FLEXCOMM2_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM2_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM2_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` writer - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM2_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM2_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM2_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` reader - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM3_IRQ_R = crate::BitReader<FLEXCOMM3_IRQ_A>;
#[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM3_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM3_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_IRQ_A {
        match self.bits {
            false => FLEXCOMM3_IRQ_A::INVISIBLE,
            true => FLEXCOMM3_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM3_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM3_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` writer - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM3_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM3_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM3_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` reader - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM4_IRQ_R = crate::BitReader<FLEXCOMM4_IRQ_A>;
#[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM4_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM4_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_IRQ_A {
        match self.bits {
            false => FLEXCOMM4_IRQ_A::INVISIBLE,
            true => FLEXCOMM4_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM4_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM4_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` writer - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM4_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM4_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM4_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` reader - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM5_IRQ_R = crate::BitReader<FLEXCOMM5_IRQ_A>;
#[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM5_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM5_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_IRQ_A {
        match self.bits {
            false => FLEXCOMM5_IRQ_A::INVISIBLE,
            true => FLEXCOMM5_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM5_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM5_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` writer - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM5_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM5_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM5_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` reader - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM6_IRQ_R = crate::BitReader<FLEXCOMM6_IRQ_A>;
#[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM6_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM6_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_IRQ_A {
        match self.bits {
            false => FLEXCOMM6_IRQ_A::INVISIBLE,
            true => FLEXCOMM6_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM6_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM6_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` writer - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM6_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM6_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM6_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` reader - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM7_IRQ_R = crate::BitReader<FLEXCOMM7_IRQ_A>;
#[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM7_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM7_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_IRQ_A {
        match self.bits {
            false => FLEXCOMM7_IRQ_A::INVISIBLE,
            true => FLEXCOMM7_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM7_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM7_IRQ_A::VISIBLE
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` writer - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub type FLEXCOMM7_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, FLEXCOMM7_IRQ_A, O>;
impl<'a, const O: u8> FLEXCOMM7_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `ADC_IRQ` reader - General Purpose ADC interrupt."]
pub type ADC_IRQ_R = crate::BitReader<ADC_IRQ_A>;
#[doc = "General Purpose ADC interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<ADC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IRQ_A {
        match self.bits {
            false => ADC_IRQ_A::INVISIBLE,
            true => ADC_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == ADC_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == ADC_IRQ_A::VISIBLE
    }
}
#[doc = "Field `ADC_IRQ` writer - General Purpose ADC interrupt."]
pub type ADC_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, ADC_IRQ_A, O>;
impl<'a, const O: u8> ADC_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::VISIBLE)
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
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, RESERVED0_A, O>;
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
#[doc = "Field `ACMP_IRQ` reader - Analog Comparator interrupt."]
pub type ACMP_IRQ_R = crate::BitReader<ACMP_IRQ_A>;
#[doc = "Analog Comparator interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<ACMP_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_IRQ_A {
        match self.bits {
            false => ACMP_IRQ_A::INVISIBLE,
            true => ACMP_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == ACMP_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == ACMP_IRQ_A::VISIBLE
    }
}
#[doc = "Field `ACMP_IRQ` writer - Analog Comparator interrupt."]
pub type ACMP_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, ACMP_IRQ_A, O>;
impl<'a, const O: u8> ACMP_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ACMP_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(ACMP_IRQ_A::VISIBLE)
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
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, RESERVED1_A, O>;
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
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, RESERVED2_A, O>;
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
#[doc = "Field `USB0_NEEDCLK` reader - USB Full Speed Controller Clock request interrupt."]
pub type USB0_NEEDCLK_R = crate::BitReader<USB0_NEEDCLK_A>;
#[doc = "USB Full Speed Controller Clock request interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_NEEDCLK_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB0_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_NEEDCLK_A {
        match self.bits {
            false => USB0_NEEDCLK_A::INVISIBLE,
            true => USB0_NEEDCLK_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB0_NEEDCLK_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB0_NEEDCLK_A::VISIBLE
    }
}
#[doc = "Field `USB0_NEEDCLK` writer - USB Full Speed Controller Clock request interrupt."]
pub type USB0_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, USB0_NEEDCLK_A, O>;
impl<'a, const O: u8> USB0_NEEDCLK_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::VISIBLE)
    }
}
#[doc = "Field `USB0_IRQ` reader - USB Full Speed Controller interrupt."]
pub type USB0_IRQ_R = crate::BitReader<USB0_IRQ_A>;
#[doc = "USB Full Speed Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_IRQ_A {
        match self.bits {
            false => USB0_IRQ_A::INVISIBLE,
            true => USB0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB0_IRQ_A::VISIBLE
    }
}
#[doc = "Field `USB0_IRQ` writer - USB Full Speed Controller interrupt."]
pub type USB0_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, USB0_IRQ_A, O>;
impl<'a, const O: u8> USB0_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_IRQ_A::VISIBLE)
    }
}
#[doc = "Field `RTC_IRQ` reader - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub type RTC_IRQ_R = crate::BitReader<RTC_IRQ_A>;
#[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RTC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_IRQ_A {
        match self.bits {
            false => RTC_IRQ_A::INVISIBLE,
            true => RTC_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RTC_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RTC_IRQ_A::VISIBLE
    }
}
#[doc = "Field `RTC_IRQ` writer - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub type RTC_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, RTC_IRQ_A, O>;
impl<'a, const O: u8> RTC_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RTC_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RTC_IRQ_A::VISIBLE)
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
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, RESERVED3_A, O>;
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
#[doc = "Field `MAILBOX_IRQ` reader - Mailbox interrupt."]
pub type MAILBOX_IRQ_R = crate::BitReader<MAILBOX_IRQ_A>;
#[doc = "Mailbox interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAILBOX_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<MAILBOX_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl MAILBOX_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_IRQ_A {
        match self.bits {
            false => MAILBOX_IRQ_A::INVISIBLE,
            true => MAILBOX_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == MAILBOX_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == MAILBOX_IRQ_A::VISIBLE
    }
}
#[doc = "Field `MAILBOX_IRQ` writer - Mailbox interrupt."]
pub type MAILBOX_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_CPU_INT_MASK0_SPEC, MAILBOX_IRQ_A, O>;
impl<'a, const O: u8> MAILBOX_IRQ_W<'a, O> {
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQ_A::VISIBLE)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub fn sys_irq(&self) -> SYS_IRQ_R {
        SYS_IRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub fn sdma0_irq(&self) -> SDMA0_IRQ_R {
        SDMA0_IRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint0_irq(&self) -> GPIO_GLOBALINT0_IRQ_R {
        GPIO_GLOBALINT0_IRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint1_irq(&self) -> GPIO_GLOBALINT1_IRQ_R {
        GPIO_GLOBALINT1_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&self) -> GPIO_INT0_IRQ0_R {
        GPIO_INT0_IRQ0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&self) -> GPIO_INT0_IRQ1_R {
        GPIO_INT0_IRQ1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&self) -> GPIO_INT0_IRQ2_R {
        GPIO_INT0_IRQ2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&self) -> GPIO_INT0_IRQ3_R {
        GPIO_INT0_IRQ3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    pub fn utick_irq(&self) -> UTICK_IRQ_R {
        UTICK_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub fn mrt_irq(&self) -> MRT_IRQ_R {
        MRT_IRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub fn ctimer0_irq(&self) -> CTIMER0_IRQ_R {
        CTIMER0_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub fn ctimer1_irq(&self) -> CTIMER1_IRQ_R {
        CTIMER1_IRQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    pub fn sct_irq(&self) -> SCT_IRQ_R {
        SCT_IRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub fn ctimer3_irq(&self) -> CTIMER3_IRQ_R {
        CTIMER3_IRQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm0_irq(&self) -> FLEXCOMM0_IRQ_R {
        FLEXCOMM0_IRQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm1_irq(&self) -> FLEXCOMM1_IRQ_R {
        FLEXCOMM1_IRQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm2_irq(&self) -> FLEXCOMM2_IRQ_R {
        FLEXCOMM2_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm3_irq(&self) -> FLEXCOMM3_IRQ_R {
        FLEXCOMM3_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm4_irq(&self) -> FLEXCOMM4_IRQ_R {
        FLEXCOMM4_IRQ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm5_irq(&self) -> FLEXCOMM5_IRQ_R {
        FLEXCOMM5_IRQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm6_irq(&self) -> FLEXCOMM6_IRQ_R {
        FLEXCOMM6_IRQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm7_irq(&self) -> FLEXCOMM7_IRQ_R {
        FLEXCOMM7_IRQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    pub fn acmp_irq(&self) -> ACMP_IRQ_R {
        ACMP_IRQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb0_irq(&self) -> USB0_IRQ_R {
        USB0_IRQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub fn rtc_irq(&self) -> RTC_IRQ_R {
        RTC_IRQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    pub fn mailbox_irq(&self) -> MAILBOX_IRQ_R {
        MAILBOX_IRQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn sys_irq(&mut self) -> SYS_IRQ_W<0> {
        SYS_IRQ_W::new(self)
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdma0_irq(&mut self) -> SDMA0_IRQ_W<1> {
        SDMA0_IRQ_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_globalint0_irq(&mut self) -> GPIO_GLOBALINT0_IRQ_W<2> {
        GPIO_GLOBALINT0_IRQ_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_globalint1_irq(&mut self) -> GPIO_GLOBALINT1_IRQ_W<3> {
        GPIO_GLOBALINT1_IRQ_W::new(self)
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq0(&mut self) -> GPIO_INT0_IRQ0_W<4> {
        GPIO_INT0_IRQ0_W::new(self)
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq1(&mut self) -> GPIO_INT0_IRQ1_W<5> {
        GPIO_INT0_IRQ1_W::new(self)
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq2(&mut self) -> GPIO_INT0_IRQ2_W<6> {
        GPIO_INT0_IRQ2_W::new(self)
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq3(&mut self) -> GPIO_INT0_IRQ3_W<7> {
        GPIO_INT0_IRQ3_W::new(self)
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn utick_irq(&mut self) -> UTICK_IRQ_W<8> {
        UTICK_IRQ_W::new(self)
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mrt_irq(&mut self) -> MRT_IRQ_W<9> {
        MRT_IRQ_W::new(self)
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctimer0_irq(&mut self) -> CTIMER0_IRQ_W<10> {
        CTIMER0_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctimer1_irq(&mut self) -> CTIMER1_IRQ_W<11> {
        CTIMER1_IRQ_W::new(self)
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sct_irq(&mut self) -> SCT_IRQ_W<12> {
        SCT_IRQ_W::new(self)
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctimer3_irq(&mut self) -> CTIMER3_IRQ_W<13> {
        CTIMER3_IRQ_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_irq(&mut self) -> FLEXCOMM0_IRQ_W<14> {
        FLEXCOMM0_IRQ_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_irq(&mut self) -> FLEXCOMM1_IRQ_W<15> {
        FLEXCOMM1_IRQ_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_irq(&mut self) -> FLEXCOMM2_IRQ_W<16> {
        FLEXCOMM2_IRQ_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_irq(&mut self) -> FLEXCOMM3_IRQ_W<17> {
        FLEXCOMM3_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_irq(&mut self) -> FLEXCOMM4_IRQ_W<18> {
        FLEXCOMM4_IRQ_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_irq(&mut self) -> FLEXCOMM5_IRQ_W<19> {
        FLEXCOMM5_IRQ_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_irq(&mut self) -> FLEXCOMM6_IRQ_W<20> {
        FLEXCOMM6_IRQ_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_irq(&mut self) -> FLEXCOMM7_IRQ_W<21> {
        FLEXCOMM7_IRQ_W::new(self)
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W<22> {
        ADC_IRQ_W::new(self)
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<23> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn acmp_irq(&mut self) -> ACMP_IRQ_W<24> {
        ACMP_IRQ_W::new(self)
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<25> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<26> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W<27> {
        USB0_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_irq(&mut self) -> USB0_IRQ_W<28> {
        USB0_IRQ_W::new(self)
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_irq(&mut self) -> RTC_IRQ_W<29> {
        RTC_IRQ_W::new(self)
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<30> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mailbox_irq(&mut self) -> MAILBOX_IRQ_W<31> {
        MAILBOX_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Interrupt mask for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cpu_int_mask0](index.html) module"]
pub struct SEC_CPU_INT_MASK0_SPEC;
impl crate::RegisterSpec for SEC_CPU_INT_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_cpu_int_mask0::R](R) reader structure"]
impl crate::Readable for SEC_CPU_INT_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_cpu_int_mask0::W](W) writer structure"]
impl crate::Writable for SEC_CPU_INT_MASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CPU_INT_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for SEC_CPU_INT_MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
