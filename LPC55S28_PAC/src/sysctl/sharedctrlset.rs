#[doc = "Register `SHAREDCTRLSET%s` reader"]
pub struct R(crate::R<SHAREDCTRLSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAREDCTRLSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAREDCTRLSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAREDCTRLSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHAREDCTRLSET%s` writer"]
pub struct W(crate::W<SHAREDCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHAREDCTRLSET_SPEC>;
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
impl From<crate::W<SHAREDCTRLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHAREDCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAREDSCKSEL` reader - Selects the source for SCK of this shared signal set."]
pub type SHAREDSCKSEL_R = crate::FieldReader<u8, SHAREDSCKSEL_A>;
#[doc = "Selects the source for SCK of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDSCKSEL_A {
    #[doc = "0: SCK for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: SCK for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: SCK for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: SCK for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: SCK for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: SCK for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: SCK for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: SCK for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDSCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDSCKSEL_A) -> Self {
        variant as _
    }
}
impl SHAREDSCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDSCKSEL_A {
        match self.bits {
            0 => SHAREDSCKSEL_A::FLEXCOMM0,
            1 => SHAREDSCKSEL_A::FLEXCOMM1,
            2 => SHAREDSCKSEL_A::FLEXCOMM2,
            3 => SHAREDSCKSEL_A::FLEXCOMM3,
            4 => SHAREDSCKSEL_A::FLEXCOMM4,
            5 => SHAREDSCKSEL_A::FLEXCOMM5,
            6 => SHAREDSCKSEL_A::FLEXCOMM6,
            7 => SHAREDSCKSEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM7
    }
}
#[doc = "Field `SHAREDSCKSEL` writer - Selects the source for SCK of this shared signal set."]
pub type SHAREDSCKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDSCKSEL_A, 3, O>;
impl<'a, const O: u8> SHAREDSCKSEL_W<'a, O> {
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM0)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM1)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM2)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM3)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM4)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM5)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM6)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM7)
    }
}
#[doc = "Field `SHAREDWSSEL` reader - Selects the source for WS of this shared signal set."]
pub type SHAREDWSSEL_R = crate::FieldReader<u8, SHAREDWSSEL_A>;
#[doc = "Selects the source for WS of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDWSSEL_A {
    #[doc = "0: WS for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: WS for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: WS for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: WS for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: WS for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: WS for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: WS for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: WS for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDWSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDWSSEL_A) -> Self {
        variant as _
    }
}
impl SHAREDWSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDWSSEL_A {
        match self.bits {
            0 => SHAREDWSSEL_A::FLEXCOMM0,
            1 => SHAREDWSSEL_A::FLEXCOMM1,
            2 => SHAREDWSSEL_A::FLEXCOMM2,
            3 => SHAREDWSSEL_A::FLEXCOMM3,
            4 => SHAREDWSSEL_A::FLEXCOMM4,
            5 => SHAREDWSSEL_A::FLEXCOMM5,
            6 => SHAREDWSSEL_A::FLEXCOMM6,
            7 => SHAREDWSSEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM7
    }
}
#[doc = "Field `SHAREDWSSEL` writer - Selects the source for WS of this shared signal set."]
pub type SHAREDWSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDWSSEL_A, 3, O>;
impl<'a, const O: u8> SHAREDWSSEL_W<'a, O> {
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM0)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM1)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM2)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM3)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM4)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM5)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM6)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM7)
    }
}
#[doc = "Field `SHAREDDATASEL` reader - Selects the source for DATA input for this shared signal set."]
pub type SHAREDDATASEL_R = crate::FieldReader<u8, SHAREDDATASEL_A>;
#[doc = "Selects the source for DATA input for this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDDATASEL_A {
    #[doc = "0: DATA input for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: DATA input for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: DATA input for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: DATA input for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: DATA input for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: DATA input for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: DATA input for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: DATA input for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDDATASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDDATASEL_A) -> Self {
        variant as _
    }
}
impl SHAREDDATASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDDATASEL_A {
        match self.bits {
            0 => SHAREDDATASEL_A::FLEXCOMM0,
            1 => SHAREDDATASEL_A::FLEXCOMM1,
            2 => SHAREDDATASEL_A::FLEXCOMM2,
            3 => SHAREDDATASEL_A::FLEXCOMM3,
            4 => SHAREDDATASEL_A::FLEXCOMM4,
            5 => SHAREDDATASEL_A::FLEXCOMM5,
            6 => SHAREDDATASEL_A::FLEXCOMM6,
            7 => SHAREDDATASEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM7
    }
}
#[doc = "Field `SHAREDDATASEL` writer - Selects the source for DATA input for this shared signal set."]
pub type SHAREDDATASEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDDATASEL_A, 3, O>;
impl<'a, const O: u8> SHAREDDATASEL_W<'a, O> {
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM0)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM1)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM2)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM3)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM4)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM5)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM6)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM7)
    }
}
#[doc = "Field `FC0DATAOUTEN` reader - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
pub type FC0DATAOUTEN_R = crate::BitReader<FC0DATAOUTEN_A>;
#[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC0DATAOUTEN_A {
    #[doc = "0: Data output from FC0 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC0 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC0DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC0DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC0DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0DATAOUTEN_A {
        match self.bits {
            false => FC0DATAOUTEN_A::INPUT,
            true => FC0DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC0DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC0DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC0DATAOUTEN` writer - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
pub type FC0DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC0DATAOUTEN_A, O>;
impl<'a, const O: u8> FC0DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC0 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC1DATAOUTEN` reader - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
pub type FC1DATAOUTEN_R = crate::BitReader<FC1DATAOUTEN_A>;
#[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC1DATAOUTEN_A {
    #[doc = "0: Data output from FC1 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC1 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC1DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC1DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC1DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1DATAOUTEN_A {
        match self.bits {
            false => FC1DATAOUTEN_A::INPUT,
            true => FC1DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC1DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC1DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC1DATAOUTEN` writer - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
pub type FC1DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC1DATAOUTEN_A, O>;
impl<'a, const O: u8> FC1DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC1 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC2DATAOUTEN` reader - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
pub type FC2DATAOUTEN_R = crate::BitReader<FC2DATAOUTEN_A>;
#[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC2DATAOUTEN_A {
    #[doc = "0: Data output from FC2 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC2 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC2DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC2DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC2DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2DATAOUTEN_A {
        match self.bits {
            false => FC2DATAOUTEN_A::INPUT,
            true => FC2DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC2DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC2DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC2DATAOUTEN` writer - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
pub type FC2DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC2DATAOUTEN_A, O>;
impl<'a, const O: u8> FC2DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC2 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC4DATAOUTEN` reader - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
pub type FC4DATAOUTEN_R = crate::BitReader<FC4DATAOUTEN_A>;
#[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC4DATAOUTEN_A {
    #[doc = "0: Data output from FC4 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC4 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC4DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC4DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC4DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4DATAOUTEN_A {
        match self.bits {
            false => FC4DATAOUTEN_A::INPUT,
            true => FC4DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC4DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC4DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC4DATAOUTEN` writer - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
pub type FC4DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC4DATAOUTEN_A, O>;
impl<'a, const O: u8> FC4DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC4 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC5DATAOUTEN` reader - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
pub type FC5DATAOUTEN_R = crate::BitReader<FC5DATAOUTEN_A>;
#[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC5DATAOUTEN_A {
    #[doc = "0: Data output from FC5 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC5 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC5DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC5DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC5DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5DATAOUTEN_A {
        match self.bits {
            false => FC5DATAOUTEN_A::INPUT,
            true => FC5DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC5DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC5DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC5DATAOUTEN` writer - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
pub type FC5DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC5DATAOUTEN_A, O>;
impl<'a, const O: u8> FC5DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC5 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC6DATAOUTEN` reader - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
pub type FC6DATAOUTEN_R = crate::BitReader<FC6DATAOUTEN_A>;
#[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC6DATAOUTEN_A {
    #[doc = "0: Data output from FC6 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC6 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC6DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC6DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC6DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6DATAOUTEN_A {
        match self.bits {
            false => FC6DATAOUTEN_A::INPUT,
            true => FC6DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC6DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC6DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC6DATAOUTEN` writer - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
pub type FC6DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC6DATAOUTEN_A, O>;
impl<'a, const O: u8> FC6DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC6 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::OUTPUT)
    }
}
#[doc = "Field `FC7DATAOUTEN` reader - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
pub type FC7DATAOUTEN_R = crate::BitReader<FC7DATAOUTEN_A>;
#[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC7DATAOUTEN_A {
    #[doc = "0: Data output from FC7 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC7 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC7DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC7DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC7DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7DATAOUTEN_A {
        match self.bits {
            false => FC7DATAOUTEN_A::INPUT,
            true => FC7DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC7DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC7DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Field `FC7DATAOUTEN` writer - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
pub type FC7DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC7DATAOUTEN_A, O>;
impl<'a, const O: u8> FC7DATAOUTEN_W<'a, O> {
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC7 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub fn sharedscksel(&self) -> SHAREDSCKSEL_R {
        SHAREDSCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub fn sharedwssel(&self) -> SHAREDWSSEL_R {
        SHAREDWSSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub fn shareddatasel(&self) -> SHAREDDATASEL_R {
        SHAREDDATASEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc0dataouten(&self) -> FC0DATAOUTEN_R {
        FC0DATAOUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc1dataouten(&self) -> FC1DATAOUTEN_R {
        FC1DATAOUTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc2dataouten(&self) -> FC2DATAOUTEN_R {
        FC2DATAOUTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc4dataouten(&self) -> FC4DATAOUTEN_R {
        FC4DATAOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc5dataouten(&self) -> FC5DATAOUTEN_R {
        FC5DATAOUTEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc6dataouten(&self) -> FC6DATAOUTEN_R {
        FC6DATAOUTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc7dataouten(&self) -> FC7DATAOUTEN_R {
        FC7DATAOUTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    #[must_use]
    pub fn sharedscksel(&mut self) -> SHAREDSCKSEL_W<0> {
        SHAREDSCKSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    #[must_use]
    pub fn sharedwssel(&mut self) -> SHAREDWSSEL_W<4> {
        SHAREDWSSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    #[must_use]
    pub fn shareddatasel(&mut self) -> SHAREDDATASEL_W<8> {
        SHAREDDATASEL_W::new(self)
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc0dataouten(&mut self) -> FC0DATAOUTEN_W<16> {
        FC0DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc1dataouten(&mut self) -> FC1DATAOUTEN_W<17> {
        FC1DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc2dataouten(&mut self) -> FC2DATAOUTEN_W<18> {
        FC2DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc4dataouten(&mut self) -> FC4DATAOUTEN_W<20> {
        FC4DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc5dataouten(&mut self) -> FC5DATAOUTEN_W<21> {
        FC5DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc6dataouten(&mut self) -> FC6DATAOUTEN_W<22> {
        FC6DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    #[must_use]
    pub fn fc7dataouten(&mut self) -> FC7DATAOUTEN_W<23> {
        FC7DATAOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects sources and data combinations for shared signal set index.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sharedctrlset](index.html) module"]
pub struct SHAREDCTRLSET_SPEC;
impl crate::RegisterSpec for SHAREDCTRLSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sharedctrlset::R](R) reader structure"]
impl crate::Readable for SHAREDCTRLSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sharedctrlset::W](W) writer structure"]
impl crate::Writable for SHAREDCTRLSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHAREDCTRLSET%s to value 0"]
impl crate::Resettable for SHAREDCTRLSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
