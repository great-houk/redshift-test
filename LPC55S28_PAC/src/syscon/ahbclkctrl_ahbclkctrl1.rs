#[doc = "Register `AHBCLKCTRL1` reader"]
pub struct R(crate::R<AHBCLKCTRL_AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL_AHBCLKCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL_AHBCLKCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL_AHBCLKCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL1` writer"]
pub struct W(crate::W<AHBCLKCTRL_AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL_AHBCLKCTRL1_SPEC>;
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
impl From<crate::W<AHBCLKCTRL_AHBCLKCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL_AHBCLKCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT` reader - Enables the clock for the MRT."]
pub type MRT_R = crate::BitReader<MRT_A>;
#[doc = "Enables the clock for the MRT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MRT_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::DISABLE,
            true => MRT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MRT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MRT_A::ENABLE
    }
}
#[doc = "Field `MRT` writer - Enables the clock for the MRT."]
pub type MRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, MRT_A, O>;
impl<'a, const O: u8> MRT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT_A::ENABLE)
    }
}
#[doc = "Field `OSTIMER` reader - Enables the clock for the OS Event Timer."]
pub type OSTIMER_R = crate::BitReader<OSTIMER_A>;
#[doc = "Enables the clock for the OS Event Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTIMER_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<OSTIMER_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER_A {
        match self.bits {
            false => OSTIMER_A::DISABLE,
            true => OSTIMER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OSTIMER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OSTIMER_A::ENABLE
    }
}
#[doc = "Field `OSTIMER` writer - Enables the clock for the OS Event Timer."]
pub type OSTIMER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, OSTIMER_A, O>;
impl<'a, const O: u8> OSTIMER_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSTIMER_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OSTIMER_A::ENABLE)
    }
}
#[doc = "Field `SCT` reader - Enables the clock for the SCT."]
pub type SCT_R = crate::BitReader<SCT_A>;
#[doc = "Enables the clock for the SCT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SCT_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::DISABLE,
            true => SCT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT_A::ENABLE
    }
}
#[doc = "Field `SCT` writer - Enables the clock for the SCT."]
pub type SCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, SCT_A, O>;
impl<'a, const O: u8> SCT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_A::ENABLE)
    }
}
#[doc = "Field `UTICK` reader - Enables the clock for the UTICK."]
pub type UTICK_R = crate::BitReader<UTICK_A>;
#[doc = "Enables the clock for the UTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<UTICK_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_A {
        match self.bits {
            false => UTICK_A::DISABLE,
            true => UTICK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTICK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTICK_A::ENABLE
    }
}
#[doc = "Field `UTICK` writer - Enables the clock for the UTICK."]
pub type UTICK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, UTICK_A, O>;
impl<'a, const O: u8> UTICK_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK_A::ENABLE)
    }
}
#[doc = "Field `FC0` reader - Enables the clock for the FC0."]
pub type FC0_R = crate::BitReader<FC0_A>;
#[doc = "Enables the clock for the FC0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC0_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_A) -> Self {
        variant as u8 != 0
    }
}
impl FC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_A {
        match self.bits {
            false => FC0_A::DISABLE,
            true => FC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC0_A::ENABLE
    }
}
#[doc = "Field `FC0` writer - Enables the clock for the FC0."]
pub type FC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC0_A, O>;
impl<'a, const O: u8> FC0_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC0_A::ENABLE)
    }
}
#[doc = "Field `FC1` reader - Enables the clock for the FC1."]
pub type FC1_R = crate::BitReader<FC1_A>;
#[doc = "Enables the clock for the FC1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC1_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_A) -> Self {
        variant as u8 != 0
    }
}
impl FC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_A {
        match self.bits {
            false => FC1_A::DISABLE,
            true => FC1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC1_A::ENABLE
    }
}
#[doc = "Field `FC1` writer - Enables the clock for the FC1."]
pub type FC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC1_A, O>;
impl<'a, const O: u8> FC1_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC1_A::ENABLE)
    }
}
#[doc = "Field `FC2` reader - Enables the clock for the FC2."]
pub type FC2_R = crate::BitReader<FC2_A>;
#[doc = "Enables the clock for the FC2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC2_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_A) -> Self {
        variant as u8 != 0
    }
}
impl FC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_A {
        match self.bits {
            false => FC2_A::DISABLE,
            true => FC2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC2_A::ENABLE
    }
}
#[doc = "Field `FC2` writer - Enables the clock for the FC2."]
pub type FC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC2_A, O>;
impl<'a, const O: u8> FC2_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC2_A::ENABLE)
    }
}
#[doc = "Field `FC3` reader - Enables the clock for the FC3."]
pub type FC3_R = crate::BitReader<FC3_A>;
#[doc = "Enables the clock for the FC3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC3_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_A) -> Self {
        variant as u8 != 0
    }
}
impl FC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_A {
        match self.bits {
            false => FC3_A::DISABLE,
            true => FC3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC3_A::ENABLE
    }
}
#[doc = "Field `FC3` writer - Enables the clock for the FC3."]
pub type FC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC3_A, O>;
impl<'a, const O: u8> FC3_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC3_A::ENABLE)
    }
}
#[doc = "Field `FC4` reader - Enables the clock for the FC4."]
pub type FC4_R = crate::BitReader<FC4_A>;
#[doc = "Enables the clock for the FC4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC4_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_A) -> Self {
        variant as u8 != 0
    }
}
impl FC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_A {
        match self.bits {
            false => FC4_A::DISABLE,
            true => FC4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC4_A::ENABLE
    }
}
#[doc = "Field `FC4` writer - Enables the clock for the FC4."]
pub type FC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC4_A, O>;
impl<'a, const O: u8> FC4_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC4_A::ENABLE)
    }
}
#[doc = "Field `FC5` reader - Enables the clock for the FC5."]
pub type FC5_R = crate::BitReader<FC5_A>;
#[doc = "Enables the clock for the FC5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC5_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC5_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_A) -> Self {
        variant as u8 != 0
    }
}
impl FC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_A {
        match self.bits {
            false => FC5_A::DISABLE,
            true => FC5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC5_A::ENABLE
    }
}
#[doc = "Field `FC5` writer - Enables the clock for the FC5."]
pub type FC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC5_A, O>;
impl<'a, const O: u8> FC5_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC5_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC5_A::ENABLE)
    }
}
#[doc = "Field `FC6` reader - Enables the clock for the FC6."]
pub type FC6_R = crate::BitReader<FC6_A>;
#[doc = "Enables the clock for the FC6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC6_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC6_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_A) -> Self {
        variant as u8 != 0
    }
}
impl FC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_A {
        match self.bits {
            false => FC6_A::DISABLE,
            true => FC6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC6_A::ENABLE
    }
}
#[doc = "Field `FC6` writer - Enables the clock for the FC6."]
pub type FC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC6_A, O>;
impl<'a, const O: u8> FC6_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC6_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC6_A::ENABLE)
    }
}
#[doc = "Field `FC7` reader - Enables the clock for the FC7."]
pub type FC7_R = crate::BitReader<FC7_A>;
#[doc = "Enables the clock for the FC7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC7_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FC7_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_A) -> Self {
        variant as u8 != 0
    }
}
impl FC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_A {
        match self.bits {
            false => FC7_A::DISABLE,
            true => FC7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC7_A::ENABLE
    }
}
#[doc = "Field `FC7` writer - Enables the clock for the FC7."]
pub type FC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, FC7_A, O>;
impl<'a, const O: u8> FC7_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC7_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC7_A::ENABLE)
    }
}
#[doc = "Field `TIMER2` reader - Enables the clock for the Timer 2."]
pub type TIMER2_R = crate::BitReader<TIMER2_A>;
#[doc = "Enables the clock for the Timer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_A {
        match self.bits {
            false => TIMER2_A::DISABLE,
            true => TIMER2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER2_A::ENABLE
    }
}
#[doc = "Field `TIMER2` writer - Enables the clock for the Timer 2."]
pub type TIMER2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, TIMER2_A, O>;
impl<'a, const O: u8> TIMER2_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER2_A::ENABLE)
    }
}
#[doc = "Field `USB0_DEV` reader - Enables the clock for the USB0 DEV."]
pub type USB0_DEV_R = crate::BitReader<USB0_DEV_A>;
#[doc = "Enables the clock for the USB0 DEV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_DEV_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB0_DEV_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_DEV_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_DEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_DEV_A {
        match self.bits {
            false => USB0_DEV_A::DISABLE,
            true => USB0_DEV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_DEV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_DEV_A::ENABLE
    }
}
#[doc = "Field `USB0_DEV` writer - Enables the clock for the USB0 DEV."]
pub type USB0_DEV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, USB0_DEV_A, O>;
impl<'a, const O: u8> USB0_DEV_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::ENABLE)
    }
}
#[doc = "Field `TIMER0` reader - Enables the clock for the Timer 0."]
pub type TIMER0_R = crate::BitReader<TIMER0_A>;
#[doc = "Enables the clock for the Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_A {
        match self.bits {
            false => TIMER0_A::DISABLE,
            true => TIMER0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER0_A::ENABLE
    }
}
#[doc = "Field `TIMER0` writer - Enables the clock for the Timer 0."]
pub type TIMER0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, TIMER0_A, O>;
impl<'a, const O: u8> TIMER0_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER0_A::ENABLE)
    }
}
#[doc = "Field `TIMER1` reader - Enables the clock for the Timer 1."]
pub type TIMER1_R = crate::BitReader<TIMER1_A>;
#[doc = "Enables the clock for the Timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_A {
        match self.bits {
            false => TIMER1_A::DISABLE,
            true => TIMER1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER1_A::ENABLE
    }
}
#[doc = "Field `TIMER1` writer - Enables the clock for the Timer 1."]
pub type TIMER1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL1_SPEC, TIMER1_A, O>;
impl<'a, const O: u8> TIMER1_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER1_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub fn ostimer(&self) -> OSTIMER_R {
        OSTIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&self) -> FC1_R {
        FC1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&self) -> FC3_R {
        FC3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&self) -> FC4_R {
        FC4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&self) -> FC5_R {
        FC5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&self) -> FC6_R {
        FC6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&self) -> FC7_R {
        FC7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub fn usb0_dev(&self) -> USB0_DEV_R {
        USB0_DEV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    #[must_use]
    pub fn mrt(&mut self) -> MRT_W<0> {
        MRT_W::new(self)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    #[must_use]
    pub fn ostimer(&mut self) -> OSTIMER_W<1> {
        OSTIMER_W::new(self)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<2> {
        SCT_W::new(self)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    #[must_use]
    pub fn utick(&mut self) -> UTICK_W<10> {
        UTICK_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    #[must_use]
    pub fn fc0(&mut self) -> FC0_W<11> {
        FC0_W::new(self)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    #[must_use]
    pub fn fc1(&mut self) -> FC1_W<12> {
        FC1_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    #[must_use]
    pub fn fc2(&mut self) -> FC2_W<13> {
        FC2_W::new(self)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    #[must_use]
    pub fn fc3(&mut self) -> FC3_W<14> {
        FC3_W::new(self)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    #[must_use]
    pub fn fc4(&mut self) -> FC4_W<15> {
        FC4_W::new(self)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    #[must_use]
    pub fn fc5(&mut self) -> FC5_W<16> {
        FC5_W::new(self)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    #[must_use]
    pub fn fc6(&mut self) -> FC6_W<17> {
        FC6_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    #[must_use]
    pub fn fc7(&mut self) -> FC7_W<18> {
        FC7_W::new(self)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<22> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_dev(&mut self) -> USB0_DEV_W<25> {
        USB0_DEV_W::new(self)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<26> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<27> {
        TIMER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl_ahbclkctrl1](index.html) module"]
pub struct AHBCLKCTRL_AHBCLKCTRL1_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL_AHBCLKCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl_ahbclkctrl1::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL_AHBCLKCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl_ahbclkctrl1::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL_AHBCLKCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBCLKCTRL1 to value 0"]
impl crate::Resettable for AHBCLKCTRL_AHBCLKCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
