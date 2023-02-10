#[doc = "Register `RINGO0_CTRL` reader"]
pub struct R(crate::R<RINGO0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGO0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGO0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGO0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGO0_CTRL` writer"]
pub struct W(crate::W<RINGO0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGO0_CTRL_SPEC>;
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
impl From<crate::W<RINGO0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGO0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SL` reader - Select short or long ringo (for all ringos types)."]
pub type SL_R = crate::BitReader<SL_A>;
#[doc = "Select short or long ringo (for all ringos types).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SL_A {
    #[doc = "0: Select short ringo (few elements)."]
    SHORT = 0,
    #[doc = "1: Select long ringo (many elements)."]
    LONG = 1,
}
impl From<SL_A> for bool {
    #[inline(always)]
    fn from(variant: SL_A) -> Self {
        variant as u8 != 0
    }
}
impl SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SL_A {
        match self.bits {
            false => SL_A::SHORT,
            true => SL_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == SL_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == SL_A::LONG
    }
}
#[doc = "Field `SL` writer - Select short or long ringo (for all ringos types)."]
pub type SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, SL_A, O>;
impl<'a, const O: u8> SL_W<'a, O> {
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(SL_A::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(SL_A::LONG)
    }
}
#[doc = "Field `FS` reader - Ringo frequency output divider."]
pub type FS_R = crate::BitReader<FS_A>;
#[doc = "Ringo frequency output divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    #[doc = "0: High frequency output (frequency lower than 100 MHz)."]
    FAST = 0,
    #[doc = "1: Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
impl FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::FAST,
            true => FS_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == FS_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == FS_A::SLOW
    }
}
#[doc = "Field `FS` writer - Ringo frequency output divider."]
pub type FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, FS_A, O>;
impl<'a, const O: u8> FS_W<'a, O> {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(FS_A::FAST)
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(FS_A::SLOW)
    }
}
#[doc = "Field `SWN_SWP` reader - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub type SWN_SWP_R = crate::FieldReader<u8, SWN_SWP_A>;
#[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWN_SWP_A {
    #[doc = "0: Normal mode."]
    NORMAL = 0,
    #[doc = "1: P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR = 1,
    #[doc = "2: P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR = 2,
    #[doc = "3: Don't use."]
    FORBIDDEN = 3,
}
impl From<SWN_SWP_A> for u8 {
    #[inline(always)]
    fn from(variant: SWN_SWP_A) -> Self {
        variant as _
    }
}
impl SWN_SWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWN_SWP_A {
        match self.bits {
            0 => SWN_SWP_A::NORMAL,
            1 => SWN_SWP_A::P_MONITOR,
            2 => SWN_SWP_A::N_MONITOR,
            3 => SWN_SWP_A::FORBIDDEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SWN_SWP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `P_MONITOR`"]
    #[inline(always)]
    pub fn is_p_monitor(&self) -> bool {
        *self == SWN_SWP_A::P_MONITOR
    }
    #[doc = "Checks if the value of the field is `N_MONITOR`"]
    #[inline(always)]
    pub fn is_n_monitor(&self) -> bool {
        *self == SWN_SWP_A::N_MONITOR
    }
    #[doc = "Checks if the value of the field is `FORBIDDEN`"]
    #[inline(always)]
    pub fn is_forbidden(&self) -> bool {
        *self == SWN_SWP_A::FORBIDDEN
    }
}
#[doc = "Field `SWN_SWP` writer - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub type SWN_SWP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RINGO0_CTRL_SPEC, u8, SWN_SWP_A, 2, O>;
impl<'a, const O: u8> SWN_SWP_W<'a, O> {
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SWN_SWP_A::NORMAL)
    }
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    #[inline(always)]
    pub fn p_monitor(self) -> &'a mut W {
        self.variant(SWN_SWP_A::P_MONITOR)
    }
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    #[inline(always)]
    pub fn n_monitor(self) -> &'a mut W {
        self.variant(SWN_SWP_A::N_MONITOR)
    }
    #[doc = "Don't use."]
    #[inline(always)]
    pub fn forbidden(self) -> &'a mut W {
        self.variant(SWN_SWP_A::FORBIDDEN)
    }
}
#[doc = "Field `PD` reader - Ringo module Power control."]
pub type PD_R = crate::BitReader<PD_A>;
#[doc = "Ringo module Power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_A {
    #[doc = "0: The Ringo module is enabled."]
    POWERED_ON = 0,
    #[doc = "1: The Ringo module is disabled."]
    POWERED_DOWN = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_A {
        match self.bits {
            false => PD_A::POWERED_ON,
            true => PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_ON`"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == PD_A::POWERED_ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PD_A::POWERED_DOWN
    }
}
#[doc = "Field `PD` writer - Ringo module Power control."]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, PD_A, O>;
impl<'a, const O: u8> PD_W<'a, O> {
    #[doc = "The Ringo module is enabled."]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut W {
        self.variant(PD_A::POWERED_ON)
    }
    #[doc = "The Ringo module is disabled."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `E_ND0` reader - First NAND2-based ringo control."]
pub type E_ND0_R = crate::BitReader<E_ND0_A>;
#[doc = "First NAND2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_ND0_A {
    #[doc = "0: First NAND2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First NAND2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_ND0_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND0_A) -> Self {
        variant as u8 != 0
    }
}
impl E_ND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_ND0_A {
        match self.bits {
            false => E_ND0_A::DISABLE,
            true => E_ND0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_ND0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_ND0_A::ENABLE
    }
}
#[doc = "Field `E_ND0` writer - First NAND2-based ringo control."]
pub type E_ND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_ND0_A, O>;
impl<'a, const O: u8> E_ND0_W<'a, O> {
    #[doc = "First NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND0_A::DISABLE)
    }
    #[doc = "First NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND0_A::ENABLE)
    }
}
#[doc = "Field `E_ND1` reader - Second NAND2-based ringo control."]
pub type E_ND1_R = crate::BitReader<E_ND1_A>;
#[doc = "Second NAND2-based ringo control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_ND1_A {
    #[doc = "0: Second NAND2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second NAND2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_ND1_A> for bool {
    #[inline(always)]
    fn from(variant: E_ND1_A) -> Self {
        variant as u8 != 0
    }
}
impl E_ND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_ND1_A {
        match self.bits {
            false => E_ND1_A::DISABLE,
            true => E_ND1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_ND1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_ND1_A::ENABLE
    }
}
#[doc = "Field `E_ND1` writer - Second NAND2-based ringo control."]
pub type E_ND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_ND1_A, O>;
impl<'a, const O: u8> E_ND1_W<'a, O> {
    #[doc = "Second NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND1_A::DISABLE)
    }
    #[doc = "Second NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND1_A::ENABLE)
    }
}
#[doc = "Field `E_NR0` reader - First NOR2-based ringo control."]
pub type E_NR0_R = crate::BitReader<E_NR0_A>;
#[doc = "First NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_NR0_A {
    #[doc = "0: First NOR2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First NOR2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_NR0_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR0_A) -> Self {
        variant as u8 != 0
    }
}
impl E_NR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_NR0_A {
        match self.bits {
            false => E_NR0_A::DISABLE,
            true => E_NR0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_NR0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_NR0_A::ENABLE
    }
}
#[doc = "Field `E_NR0` writer - First NOR2-based ringo control."]
pub type E_NR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_NR0_A, O>;
impl<'a, const O: u8> E_NR0_W<'a, O> {
    #[doc = "First NOR2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR0_A::DISABLE)
    }
    #[doc = "First NOR2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR0_A::ENABLE)
    }
}
#[doc = "Field `E_NR1` reader - Second NOR2-based ringo control."]
pub type E_NR1_R = crate::BitReader<E_NR1_A>;
#[doc = "Second NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_NR1_A {
    #[doc = "0: Second NORD2-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second NORD2-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_NR1_A> for bool {
    #[inline(always)]
    fn from(variant: E_NR1_A) -> Self {
        variant as u8 != 0
    }
}
impl E_NR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_NR1_A {
        match self.bits {
            false => E_NR1_A::DISABLE,
            true => E_NR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_NR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_NR1_A::ENABLE
    }
}
#[doc = "Field `E_NR1` writer - Second NOR2-based ringo control."]
pub type E_NR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_NR1_A, O>;
impl<'a, const O: u8> E_NR1_W<'a, O> {
    #[doc = "Second NORD2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR1_A::DISABLE)
    }
    #[doc = "Second NORD2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR1_A::ENABLE)
    }
}
#[doc = "Field `E_IV0` reader - First Inverter-based ringo control."]
pub type E_IV0_R = crate::BitReader<E_IV0_A>;
#[doc = "First Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_IV0_A {
    #[doc = "0: First INV-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First INV-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_IV0_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV0_A) -> Self {
        variant as u8 != 0
    }
}
impl E_IV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_IV0_A {
        match self.bits {
            false => E_IV0_A::DISABLE,
            true => E_IV0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_IV0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_IV0_A::ENABLE
    }
}
#[doc = "Field `E_IV0` writer - First Inverter-based ringo control."]
pub type E_IV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_IV0_A, O>;
impl<'a, const O: u8> E_IV0_W<'a, O> {
    #[doc = "First INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV0_A::DISABLE)
    }
    #[doc = "First INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV0_A::ENABLE)
    }
}
#[doc = "Field `E_IV1` reader - Second Inverter-based ringo control."]
pub type E_IV1_R = crate::BitReader<E_IV1_A>;
#[doc = "Second Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_IV1_A {
    #[doc = "0: Second INV-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second INV-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_IV1_A> for bool {
    #[inline(always)]
    fn from(variant: E_IV1_A) -> Self {
        variant as u8 != 0
    }
}
impl E_IV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_IV1_A {
        match self.bits {
            false => E_IV1_A::DISABLE,
            true => E_IV1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_IV1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_IV1_A::ENABLE
    }
}
#[doc = "Field `E_IV1` writer - Second Inverter-based ringo control."]
pub type E_IV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_IV1_A, O>;
impl<'a, const O: u8> E_IV1_W<'a, O> {
    #[doc = "Second INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV1_A::DISABLE)
    }
    #[doc = "Second INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV1_A::ENABLE)
    }
}
#[doc = "Field `E_PN0` reader - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub type E_PN0_R = crate::BitReader<E_PN0_A>;
#[doc = "First PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_PN0_A {
    #[doc = "0: First PN-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: First PN-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_PN0_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN0_A) -> Self {
        variant as u8 != 0
    }
}
impl E_PN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_PN0_A {
        match self.bits {
            false => E_PN0_A::DISABLE,
            true => E_PN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_PN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_PN0_A::ENABLE
    }
}
#[doc = "Field `E_PN0` writer - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub type E_PN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_PN0_A, O>;
impl<'a, const O: u8> E_PN0_W<'a, O> {
    #[doc = "First PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN0_A::DISABLE)
    }
    #[doc = "First PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN0_A::ENABLE)
    }
}
#[doc = "Field `E_PN1` reader - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub type E_PN1_R = crate::BitReader<E_PN1_A>;
#[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_PN1_A {
    #[doc = "0: Second PN-based ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Second PN-based ringo is enabled."]
    ENABLE = 1,
}
impl From<E_PN1_A> for bool {
    #[inline(always)]
    fn from(variant: E_PN1_A) -> Self {
        variant as u8 != 0
    }
}
impl E_PN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_PN1_A {
        match self.bits {
            false => E_PN1_A::DISABLE,
            true => E_PN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_PN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_PN1_A::ENABLE
    }
}
#[doc = "Field `E_PN1` writer - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub type E_PN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO0_CTRL_SPEC, E_PN1_A, O>;
impl<'a, const O: u8> E_PN1_W<'a, O> {
    #[doc = "Second PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN1_A::DISABLE)
    }
    #[doc = "Second PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN1_A::ENABLE)
    }
}
#[doc = "Field `DIVISOR` reader - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DIVISOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVISOR` writer - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DIVISOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RINGO0_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIV_UPDATE_REQ` reader - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type DIV_UPDATE_REQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn sl(&self) -> SL_R {
        SL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub fn swn_swp(&self) -> SWN_SWP_R {
        SWN_SWP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd0(&self) -> E_ND0_R {
        E_ND0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd1(&self) -> E_ND1_R {
        E_ND1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr0(&self) -> E_NR0_R {
        E_NR0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr1(&self) -> E_NR1_R {
        E_NR1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv0(&self) -> E_IV0_R {
        E_IV0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv1(&self) -> E_IV1_R {
        E_IV1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn0(&self) -> E_PN0_R {
        E_PN0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn1(&self) -> E_PN1_R {
        E_PN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn div_update_req(&self) -> DIV_UPDATE_REQ_R {
        DIV_UPDATE_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SL_W<0> {
        SL_W::new(self)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<1> {
        FS_W::new(self)
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    #[must_use]
    pub fn swn_swp(&mut self) -> SWN_SWP_W<2> {
        SWN_SWP_W::new(self)
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<4> {
        PD_W::new(self)
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_nd0(&mut self) -> E_ND0_W<5> {
        E_ND0_W::new(self)
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_nd1(&mut self) -> E_ND1_W<6> {
        E_ND1_W::new(self)
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_nr0(&mut self) -> E_NR0_W<7> {
        E_NR0_W::new(self)
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_nr1(&mut self) -> E_NR1_W<8> {
        E_NR1_W::new(self)
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_iv0(&mut self) -> E_IV0_W<9> {
        E_IV0_W::new(self)
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    #[must_use]
    pub fn e_iv1(&mut self) -> E_IV1_W<10> {
        E_IV1_W::new(self)
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_pn0(&mut self) -> E_PN0_W<11> {
        E_PN0_W::new(self)
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_pn1(&mut self) -> E_PN1_W<12> {
        E_PN1_W::new(self)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DIVISOR_W<16> {
        DIVISOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringo0_ctrl](index.html) module"]
pub struct RINGO0_CTRL_SPEC;
impl crate::RegisterSpec for RINGO0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringo0_ctrl::R](R) reader structure"]
impl crate::Readable for RINGO0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringo0_ctrl::W](W) writer structure"]
impl crate::Writable for RINGO0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RINGO0_CTRL to value 0x40"]
impl crate::Resettable for RINGO0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
