#[doc = "Register `PRESETCTRL1` reader"]
pub struct R(crate::R<PRESETCTRL_PRESETCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_PRESETCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_PRESETCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL1` writer"]
pub struct W(crate::W<PRESETCTRL_PRESETCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_PRESETCTRL1_SPEC>;
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
impl From<crate::W<PRESETCTRL_PRESETCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_PRESETCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT_RST` reader - MRT reset control."]
pub type MRT_RST_R = crate::BitReader<MRT_RST_A>;
#[doc = "MRT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<MRT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RST_A {
        match self.bits {
            false => MRT_RST_A::RELEASED,
            true => MRT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MRT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MRT_RST_A::ASSERTED
    }
}
#[doc = "Field `MRT_RST` writer - MRT reset control."]
pub type MRT_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, MRT_RST_A, O>;
impl<'a, const O: u8> MRT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MRT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MRT_RST_A::ASSERTED)
    }
}
#[doc = "Field `OSTIMER_RST` reader - OS Event Timer reset control."]
pub type OSTIMER_RST_R = crate::BitReader<OSTIMER_RST_A>;
#[doc = "OS Event Timer reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTIMER_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<OSTIMER_RST_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTIMER_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER_RST_A {
        match self.bits {
            false => OSTIMER_RST_A::RELEASED,
            true => OSTIMER_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == OSTIMER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == OSTIMER_RST_A::ASSERTED
    }
}
#[doc = "Field `OSTIMER_RST` writer - OS Event Timer reset control."]
pub type OSTIMER_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, OSTIMER_RST_A, O>;
impl<'a, const O: u8> OSTIMER_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(OSTIMER_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(OSTIMER_RST_A::ASSERTED)
    }
}
#[doc = "Field `SCT_RST` reader - SCT reset control."]
pub type SCT_RST_R = crate::BitReader<SCT_RST_A>;
#[doc = "SCT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SCT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RST_A {
        match self.bits {
            false => SCT_RST_A::RELEASED,
            true => SCT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SCT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SCT_RST_A::ASSERTED
    }
}
#[doc = "Field `SCT_RST` writer - SCT reset control."]
pub type SCT_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, SCT_RST_A, O>;
impl<'a, const O: u8> SCT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SCT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCT_RST_A::ASSERTED)
    }
}
#[doc = "Field `SCTIPU_RST` reader - SCTIPU reset control."]
pub type SCTIPU_RST_R = crate::BitReader<SCTIPU_RST_A>;
#[doc = "SCTIPU reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCTIPU_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SCTIPU_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SCTIPU_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SCTIPU_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTIPU_RST_A {
        match self.bits {
            false => SCTIPU_RST_A::RELEASED,
            true => SCTIPU_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SCTIPU_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SCTIPU_RST_A::ASSERTED
    }
}
#[doc = "Field `SCTIPU_RST` writer - SCTIPU reset control."]
pub type SCTIPU_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, SCTIPU_RST_A, O>;
impl<'a, const O: u8> SCTIPU_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SCTIPU_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SCTIPU_RST_A::ASSERTED)
    }
}
#[doc = "Field `UTICK_RST` reader - UTICK reset control."]
pub type UTICK_RST_R = crate::BitReader<UTICK_RST_A>;
#[doc = "UTICK reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<UTICK_RST_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_RST_A {
        match self.bits {
            false => UTICK_RST_A::RELEASED,
            true => UTICK_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == UTICK_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == UTICK_RST_A::ASSERTED
    }
}
#[doc = "Field `UTICK_RST` writer - UTICK reset control."]
pub type UTICK_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, UTICK_RST_A, O>;
impl<'a, const O: u8> UTICK_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(UTICK_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(UTICK_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC0_RST` reader - FC0 reset control."]
pub type FC0_RST_R = crate::BitReader<FC0_RST_A>;
#[doc = "FC0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_RST_A {
        match self.bits {
            false => FC0_RST_A::RELEASED,
            true => FC0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC0_RST_A::ASSERTED
    }
}
#[doc = "Field `FC0_RST` writer - FC0 reset control."]
pub type FC0_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC0_RST_A, O>;
impl<'a, const O: u8> FC0_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC0_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC1_RST` reader - FC1 reset control."]
pub type FC1_RST_R = crate::BitReader<FC1_RST_A>;
#[doc = "FC1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_RST_A {
        match self.bits {
            false => FC1_RST_A::RELEASED,
            true => FC1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC1_RST_A::ASSERTED
    }
}
#[doc = "Field `FC1_RST` writer - FC1 reset control."]
pub type FC1_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC1_RST_A, O>;
impl<'a, const O: u8> FC1_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC1_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC2_RST` reader - FC2 reset control."]
pub type FC2_RST_R = crate::BitReader<FC2_RST_A>;
#[doc = "FC2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC2_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC2_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_RST_A {
        match self.bits {
            false => FC2_RST_A::RELEASED,
            true => FC2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC2_RST_A::ASSERTED
    }
}
#[doc = "Field `FC2_RST` writer - FC2 reset control."]
pub type FC2_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC2_RST_A, O>;
impl<'a, const O: u8> FC2_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC2_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC3_RST` reader - FC3 reset control."]
pub type FC3_RST_R = crate::BitReader<FC3_RST_A>;
#[doc = "FC3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC3_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_RST_A {
        match self.bits {
            false => FC3_RST_A::RELEASED,
            true => FC3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC3_RST_A::ASSERTED
    }
}
#[doc = "Field `FC3_RST` writer - FC3 reset control."]
pub type FC3_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC3_RST_A, O>;
impl<'a, const O: u8> FC3_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC3_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC4_RST` reader - FC4 reset control."]
pub type FC4_RST_R = crate::BitReader<FC4_RST_A>;
#[doc = "FC4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC4_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC4_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_RST_A {
        match self.bits {
            false => FC4_RST_A::RELEASED,
            true => FC4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC4_RST_A::ASSERTED
    }
}
#[doc = "Field `FC4_RST` writer - FC4 reset control."]
pub type FC4_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC4_RST_A, O>;
impl<'a, const O: u8> FC4_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC4_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC5_RST` reader - FC5 reset control."]
pub type FC5_RST_R = crate::BitReader<FC5_RST_A>;
#[doc = "FC5 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC5_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC5_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC5_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_RST_A {
        match self.bits {
            false => FC5_RST_A::RELEASED,
            true => FC5_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC5_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC5_RST_A::ASSERTED
    }
}
#[doc = "Field `FC5_RST` writer - FC5 reset control."]
pub type FC5_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC5_RST_A, O>;
impl<'a, const O: u8> FC5_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC5_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC5_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC6_RST` reader - FC6 reset control."]
pub type FC6_RST_R = crate::BitReader<FC6_RST_A>;
#[doc = "FC6 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC6_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC6_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC6_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_RST_A {
        match self.bits {
            false => FC6_RST_A::RELEASED,
            true => FC6_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC6_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC6_RST_A::ASSERTED
    }
}
#[doc = "Field `FC6_RST` writer - FC6 reset control."]
pub type FC6_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC6_RST_A, O>;
impl<'a, const O: u8> FC6_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC6_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC6_RST_A::ASSERTED)
    }
}
#[doc = "Field `FC7_RST` reader - FC7 reset control."]
pub type FC7_RST_R = crate::BitReader<FC7_RST_A>;
#[doc = "FC7 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC7_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FC7_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FC7_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_RST_A {
        match self.bits {
            false => FC7_RST_A::RELEASED,
            true => FC7_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FC7_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FC7_RST_A::ASSERTED
    }
}
#[doc = "Field `FC7_RST` writer - FC7 reset control."]
pub type FC7_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, FC7_RST_A, O>;
impl<'a, const O: u8> FC7_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FC7_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FC7_RST_A::ASSERTED)
    }
}
#[doc = "Field `TIMER2_RST` reader - Timer 2 reset control."]
pub type TIMER2_RST_R = crate::BitReader<TIMER2_RST_A>;
#[doc = "Timer 2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER2_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER2_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_RST_A {
        match self.bits {
            false => TIMER2_RST_A::RELEASED,
            true => TIMER2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER2_RST_A::ASSERTED
    }
}
#[doc = "Field `TIMER2_RST` writer - Timer 2 reset control."]
pub type TIMER2_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, TIMER2_RST_A, O>;
impl<'a, const O: u8> TIMER2_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER2_RST_A::ASSERTED)
    }
}
#[doc = "Field `USB0_DEV_RST` reader - USB0 DEV reset control."]
pub type USB0_DEV_RST_R = crate::BitReader<USB0_DEV_RST_A>;
#[doc = "USB0 DEV reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_DEV_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_DEV_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_DEV_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_DEV_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_DEV_RST_A {
        match self.bits {
            false => USB0_DEV_RST_A::RELEASED,
            true => USB0_DEV_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == USB0_DEV_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == USB0_DEV_RST_A::ASSERTED
    }
}
#[doc = "Field `USB0_DEV_RST` writer - USB0 DEV reset control."]
pub type USB0_DEV_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, USB0_DEV_RST_A, O>;
impl<'a, const O: u8> USB0_DEV_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_DEV_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_DEV_RST_A::ASSERTED)
    }
}
#[doc = "Field `TIMER0_RST` reader - Timer 0 reset control."]
pub type TIMER0_RST_R = crate::BitReader<TIMER0_RST_A>;
#[doc = "Timer 0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_RST_A {
        match self.bits {
            false => TIMER0_RST_A::RELEASED,
            true => TIMER0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER0_RST_A::ASSERTED
    }
}
#[doc = "Field `TIMER0_RST` writer - Timer 0 reset control."]
pub type TIMER0_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, TIMER0_RST_A, O>;
impl<'a, const O: u8> TIMER0_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER0_RST_A::ASSERTED)
    }
}
#[doc = "Field `TIMER1_RST` reader - Timer 1 reset control."]
pub type TIMER1_RST_R = crate::BitReader<TIMER1_RST_A>;
#[doc = "Timer 1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_RST_A {
        match self.bits {
            false => TIMER1_RST_A::RELEASED,
            true => TIMER1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER1_RST_A::ASSERTED
    }
}
#[doc = "Field `TIMER1_RST` writer - Timer 1 reset control."]
pub type TIMER1_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL1_SPEC, TIMER1_RST_A, O>;
impl<'a, const O: u8> TIMER1_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER1_RST_A::ASSERTED)
    }
}
impl R {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MRT_RST_R {
        MRT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    pub fn ostimer_rst(&self) -> OSTIMER_RST_R {
        OSTIMER_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst(&self) -> SCT_RST_R {
        SCT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline(always)]
    pub fn sctipu_rst(&self) -> SCTIPU_RST_R {
        SCTIPU_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UTICK_RST_R {
        UTICK_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> FC0_RST_R {
        FC0_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> FC1_RST_R {
        FC1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> FC2_RST_R {
        FC2_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> FC3_RST_R {
        FC3_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> FC4_RST_R {
        FC4_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> FC5_RST_R {
        FC5_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> FC6_RST_R {
        FC6_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> FC7_RST_R {
        FC7_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    pub fn timer2_rst(&self) -> TIMER2_RST_R {
        TIMER2_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline(always)]
    pub fn usb0_dev_rst(&self) -> USB0_DEV_RST_R {
        USB0_DEV_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    pub fn timer0_rst(&self) -> TIMER0_RST_R {
        TIMER0_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    pub fn timer1_rst(&self) -> TIMER1_RST_R {
        TIMER1_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    #[must_use]
    pub fn mrt_rst(&mut self) -> MRT_RST_W<0> {
        MRT_RST_W::new(self)
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    #[must_use]
    pub fn ostimer_rst(&mut self) -> OSTIMER_RST_W<1> {
        OSTIMER_RST_W::new(self)
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    #[must_use]
    pub fn sct_rst(&mut self) -> SCT_RST_W<2> {
        SCT_RST_W::new(self)
    }
    #[doc = "Bit 6 - SCTIPU reset control."]
    #[inline(always)]
    #[must_use]
    pub fn sctipu_rst(&mut self) -> SCTIPU_RST_W<6> {
        SCTIPU_RST_W::new(self)
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    #[must_use]
    pub fn utick_rst(&mut self) -> UTICK_RST_W<10> {
        UTICK_RST_W::new(self)
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc0_rst(&mut self) -> FC0_RST_W<11> {
        FC0_RST_W::new(self)
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc1_rst(&mut self) -> FC1_RST_W<12> {
        FC1_RST_W::new(self)
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc2_rst(&mut self) -> FC2_RST_W<13> {
        FC2_RST_W::new(self)
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc3_rst(&mut self) -> FC3_RST_W<14> {
        FC3_RST_W::new(self)
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc4_rst(&mut self) -> FC4_RST_W<15> {
        FC4_RST_W::new(self)
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc5_rst(&mut self) -> FC5_RST_W<16> {
        FC5_RST_W::new(self)
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc6_rst(&mut self) -> FC6_RST_W<17> {
        FC6_RST_W::new(self)
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn fc7_rst(&mut self) -> FC7_RST_W<18> {
        FC7_RST_W::new(self)
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_rst(&mut self) -> TIMER2_RST_W<22> {
        TIMER2_RST_W::new(self)
    }
    #[doc = "Bit 25 - USB0 DEV reset control."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_dev_rst(&mut self) -> USB0_DEV_RST_W<25> {
        USB0_DEV_RST_W::new(self)
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_rst(&mut self) -> TIMER0_RST_W<26> {
        TIMER0_RST_W::new(self)
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_rst(&mut self) -> TIMER1_RST_W<27> {
        TIMER1_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl_presetctrl1](index.html) module"]
pub struct PRESETCTRL_PRESETCTRL1_SPEC;
impl crate::RegisterSpec for PRESETCTRL_PRESETCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl_presetctrl1::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_PRESETCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl_presetctrl1::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_PRESETCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0"]
impl crate::Resettable for PRESETCTRL_PRESETCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
