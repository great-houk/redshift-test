#[doc = "Register `OUTPUTDIRCTRL` reader"]
pub struct R(crate::R<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUTDIRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUTDIRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUTDIRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUTDIRCTRL` writer"]
pub struct W(crate::W<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUTDIRCTRL_SPEC>;
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
impl From<crate::W<OUTPUTDIRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUTDIRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETCLR0` reader - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR0_R = crate::FieldReader<u8, SETCLR0_A>;
#[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR0_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR0_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR0_A) -> Self {
        variant as _
    }
}
impl SETCLR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR0_A> {
        match self.bits {
            0 => Some(SETCLR0_A::INDEPENDENT),
            1 => Some(SETCLR0_A::L_REVERSED),
            2 => Some(SETCLR0_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR0_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR0_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR0` writer - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR0_A, 2, O>;
impl<'a, const O: u8> SETCLR0_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR1` reader - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR1_R = crate::FieldReader<u8, SETCLR1_A>;
#[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR1_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR1_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR1_A) -> Self {
        variant as _
    }
}
impl SETCLR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR1_A> {
        match self.bits {
            0 => Some(SETCLR1_A::INDEPENDENT),
            1 => Some(SETCLR1_A::L_REVERSED),
            2 => Some(SETCLR1_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR1_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR1_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR1` writer - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR1_A, 2, O>;
impl<'a, const O: u8> SETCLR1_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR2` reader - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR2_R = crate::FieldReader<u8, SETCLR2_A>;
#[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR2_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR2_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR2_A) -> Self {
        variant as _
    }
}
impl SETCLR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR2_A> {
        match self.bits {
            0 => Some(SETCLR2_A::INDEPENDENT),
            1 => Some(SETCLR2_A::L_REVERSED),
            2 => Some(SETCLR2_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR2_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR2_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR2` writer - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR2_A, 2, O>;
impl<'a, const O: u8> SETCLR2_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR3` reader - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR3_R = crate::FieldReader<u8, SETCLR3_A>;
#[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR3_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR3_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR3_A) -> Self {
        variant as _
    }
}
impl SETCLR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR3_A> {
        match self.bits {
            0 => Some(SETCLR3_A::INDEPENDENT),
            1 => Some(SETCLR3_A::L_REVERSED),
            2 => Some(SETCLR3_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR3_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR3_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR3` writer - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR3_A, 2, O>;
impl<'a, const O: u8> SETCLR3_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR4` reader - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR4_R = crate::FieldReader<u8, SETCLR4_A>;
#[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR4_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR4_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR4_A) -> Self {
        variant as _
    }
}
impl SETCLR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR4_A> {
        match self.bits {
            0 => Some(SETCLR4_A::INDEPENDENT),
            1 => Some(SETCLR4_A::L_REVERSED),
            2 => Some(SETCLR4_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR4_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR4_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR4_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR4` writer - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR4_A, 2, O>;
impl<'a, const O: u8> SETCLR4_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR4_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR4_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR4_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR5` reader - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR5_R = crate::FieldReader<u8, SETCLR5_A>;
#[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR5_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR5_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR5_A) -> Self {
        variant as _
    }
}
impl SETCLR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR5_A> {
        match self.bits {
            0 => Some(SETCLR5_A::INDEPENDENT),
            1 => Some(SETCLR5_A::L_REVERSED),
            2 => Some(SETCLR5_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR5_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR5_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR5_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR5` writer - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR5_A, 2, O>;
impl<'a, const O: u8> SETCLR5_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR5_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR5_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR5_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR6` reader - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR6_R = crate::FieldReader<u8, SETCLR6_A>;
#[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR6_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR6_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR6_A) -> Self {
        variant as _
    }
}
impl SETCLR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR6_A> {
        match self.bits {
            0 => Some(SETCLR6_A::INDEPENDENT),
            1 => Some(SETCLR6_A::L_REVERSED),
            2 => Some(SETCLR6_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR6_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR6_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR6_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR6` writer - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR6_A, 2, O>;
impl<'a, const O: u8> SETCLR6_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR6_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR6_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR6_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR7` reader - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR7_R = crate::FieldReader<u8, SETCLR7_A>;
#[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR7_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR7_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR7_A) -> Self {
        variant as _
    }
}
impl SETCLR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR7_A> {
        match self.bits {
            0 => Some(SETCLR7_A::INDEPENDENT),
            1 => Some(SETCLR7_A::L_REVERSED),
            2 => Some(SETCLR7_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR7_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR7_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR7_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR7` writer - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR7_A, 2, O>;
impl<'a, const O: u8> SETCLR7_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR7_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR7_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR7_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR8` reader - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR8_R = crate::FieldReader<u8, SETCLR8_A>;
#[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR8_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR8_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR8_A) -> Self {
        variant as _
    }
}
impl SETCLR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR8_A> {
        match self.bits {
            0 => Some(SETCLR8_A::INDEPENDENT),
            1 => Some(SETCLR8_A::L_REVERSED),
            2 => Some(SETCLR8_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR8_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR8_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR8_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR8` writer - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR8_A, 2, O>;
impl<'a, const O: u8> SETCLR8_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR8_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR8_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR8_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR9` reader - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR9_R = crate::FieldReader<u8, SETCLR9_A>;
#[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR9_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR9_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR9_A) -> Self {
        variant as _
    }
}
impl SETCLR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR9_A> {
        match self.bits {
            0 => Some(SETCLR9_A::INDEPENDENT),
            1 => Some(SETCLR9_A::L_REVERSED),
            2 => Some(SETCLR9_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR9_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR9_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR9_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR9` writer - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR9_A, 2, O>;
impl<'a, const O: u8> SETCLR9_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR9_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR9_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR9_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR10` reader - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR10_R = crate::FieldReader<u8, SETCLR10_A>;
#[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR10_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR10_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR10_A) -> Self {
        variant as _
    }
}
impl SETCLR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR10_A> {
        match self.bits {
            0 => Some(SETCLR10_A::INDEPENDENT),
            1 => Some(SETCLR10_A::L_REVERSED),
            2 => Some(SETCLR10_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR10_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR10_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR10_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR10` writer - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR10_A, 2, O>;
impl<'a, const O: u8> SETCLR10_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR10_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR10_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR10_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR11` reader - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR11_R = crate::FieldReader<u8, SETCLR11_A>;
#[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR11_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR11_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR11_A) -> Self {
        variant as _
    }
}
impl SETCLR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR11_A> {
        match self.bits {
            0 => Some(SETCLR11_A::INDEPENDENT),
            1 => Some(SETCLR11_A::L_REVERSED),
            2 => Some(SETCLR11_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR11_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR11_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR11_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR11` writer - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR11_A, 2, O>;
impl<'a, const O: u8> SETCLR11_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR11_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR11_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR11_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR12` reader - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR12_R = crate::FieldReader<u8, SETCLR12_A>;
#[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR12_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR12_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR12_A) -> Self {
        variant as _
    }
}
impl SETCLR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR12_A> {
        match self.bits {
            0 => Some(SETCLR12_A::INDEPENDENT),
            1 => Some(SETCLR12_A::L_REVERSED),
            2 => Some(SETCLR12_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR12_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR12_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR12_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR12` writer - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR12_A, 2, O>;
impl<'a, const O: u8> SETCLR12_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR12_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR12_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR12_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR13` reader - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR13_R = crate::FieldReader<u8, SETCLR13_A>;
#[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR13_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR13_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR13_A) -> Self {
        variant as _
    }
}
impl SETCLR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR13_A> {
        match self.bits {
            0 => Some(SETCLR13_A::INDEPENDENT),
            1 => Some(SETCLR13_A::L_REVERSED),
            2 => Some(SETCLR13_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR13_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR13_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR13_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR13` writer - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR13_A, 2, O>;
impl<'a, const O: u8> SETCLR13_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR13_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR13_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR13_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR14` reader - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR14_R = crate::FieldReader<u8, SETCLR14_A>;
#[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR14_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR14_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR14_A) -> Self {
        variant as _
    }
}
impl SETCLR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR14_A> {
        match self.bits {
            0 => Some(SETCLR14_A::INDEPENDENT),
            1 => Some(SETCLR14_A::L_REVERSED),
            2 => Some(SETCLR14_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR14_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR14_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR14_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR14` writer - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR14_A, 2, O>;
impl<'a, const O: u8> SETCLR14_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR14_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR14_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR14_A::H_REVERSED)
    }
}
#[doc = "Field `SETCLR15` reader - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR15_R = crate::FieldReader<u8, SETCLR15_A>;
#[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETCLR15_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR15_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR15_A) -> Self {
        variant as _
    }
}
impl SETCLR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR15_A> {
        match self.bits {
            0 => Some(SETCLR15_A::INDEPENDENT),
            1 => Some(SETCLR15_A::L_REVERSED),
            2 => Some(SETCLR15_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR15_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR15_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR15_A::H_REVERSED
    }
}
#[doc = "Field `SETCLR15` writer - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub type SETCLR15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIRCTRL_SPEC, u8, SETCLR15_A, 2, O>;
impl<'a, const O: u8> SETCLR15_W<'a, O> {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR15_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR15_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR15_A::H_REVERSED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> SETCLR0_R {
        SETCLR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> SETCLR1_R {
        SETCLR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> SETCLR2_R {
        SETCLR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> SETCLR3_R {
        SETCLR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&self) -> SETCLR4_R {
        SETCLR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&self) -> SETCLR5_R {
        SETCLR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&self) -> SETCLR6_R {
        SETCLR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&self) -> SETCLR7_R {
        SETCLR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&self) -> SETCLR8_R {
        SETCLR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&self) -> SETCLR9_R {
        SETCLR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&self) -> SETCLR10_R {
        SETCLR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&self) -> SETCLR11_R {
        SETCLR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&self) -> SETCLR12_R {
        SETCLR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&self) -> SETCLR13_R {
        SETCLR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&self) -> SETCLR14_R {
        SETCLR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&self) -> SETCLR15_R {
        SETCLR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr0(&mut self) -> SETCLR0_W<0> {
        SETCLR0_W::new(self)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr1(&mut self) -> SETCLR1_W<2> {
        SETCLR1_W::new(self)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr2(&mut self) -> SETCLR2_W<4> {
        SETCLR2_W::new(self)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr3(&mut self) -> SETCLR3_W<6> {
        SETCLR3_W::new(self)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr4(&mut self) -> SETCLR4_W<8> {
        SETCLR4_W::new(self)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr5(&mut self) -> SETCLR5_W<10> {
        SETCLR5_W::new(self)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr6(&mut self) -> SETCLR6_W<12> {
        SETCLR6_W::new(self)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr7(&mut self) -> SETCLR7_W<14> {
        SETCLR7_W::new(self)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr8(&mut self) -> SETCLR8_W<16> {
        SETCLR8_W::new(self)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr9(&mut self) -> SETCLR9_W<18> {
        SETCLR9_W::new(self)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr10(&mut self) -> SETCLR10_W<20> {
        SETCLR10_W::new(self)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr11(&mut self) -> SETCLR11_W<22> {
        SETCLR11_W::new(self)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr12(&mut self) -> SETCLR12_W<24> {
        SETCLR12_W::new(self)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr13(&mut self) -> SETCLR13_W<26> {
        SETCLR13_W::new(self)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr14(&mut self) -> SETCLR14_W<28> {
        SETCLR14_W::new(self)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    #[must_use]
    pub fn setclr15(&mut self) -> SETCLR15_W<30> {
        SETCLR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output counter direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdirctrl](index.html) module"]
pub struct OUTPUTDIRCTRL_SPEC;
impl crate::RegisterSpec for OUTPUTDIRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outputdirctrl::R](R) reader structure"]
impl crate::Readable for OUTPUTDIRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outputdirctrl::W](W) writer structure"]
impl crate::Writable for OUTPUTDIRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTPUTDIRCTRL to value 0"]
impl crate::Resettable for OUTPUTDIRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
