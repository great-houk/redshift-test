#[doc = "Register `SWTRIG` reader"]
pub struct R(crate::R<SWTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIG` writer"]
pub struct W(crate::W<SWTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIG_SPEC>;
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
impl From<crate::W<SWTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWT0` reader - Software trigger 0 event"]
pub type SWT0_R = crate::BitReader<SWT0_A>;
#[doc = "Software trigger 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT0_A {
    #[doc = "0: No trigger 0 event generated."]
    SWT0_0 = 0,
    #[doc = "1: Trigger 0 event generated."]
    SWT0_1 = 1,
}
impl From<SWT0_A> for bool {
    #[inline(always)]
    fn from(variant: SWT0_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT0_A {
        match self.bits {
            false => SWT0_A::SWT0_0,
            true => SWT0_A::SWT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT0_0`"]
    #[inline(always)]
    pub fn is_swt0_0(&self) -> bool {
        *self == SWT0_A::SWT0_0
    }
    #[doc = "Checks if the value of the field is `SWT0_1`"]
    #[inline(always)]
    pub fn is_swt0_1(&self) -> bool {
        *self == SWT0_A::SWT0_1
    }
}
#[doc = "Field `SWT0` writer - Software trigger 0 event"]
pub type SWT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT0_A, O>;
impl<'a, const O: u8> SWT0_W<'a, O> {
    #[doc = "No trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_0(self) -> &'a mut W {
        self.variant(SWT0_A::SWT0_0)
    }
    #[doc = "Trigger 0 event generated."]
    #[inline(always)]
    pub fn swt0_1(self) -> &'a mut W {
        self.variant(SWT0_A::SWT0_1)
    }
}
#[doc = "Field `SWT1` reader - Software trigger 1 event"]
pub type SWT1_R = crate::BitReader<SWT1_A>;
#[doc = "Software trigger 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT1_A {
    #[doc = "0: No trigger 1 event generated."]
    SWT1_0 = 0,
    #[doc = "1: Trigger 1 event generated."]
    SWT1_1 = 1,
}
impl From<SWT1_A> for bool {
    #[inline(always)]
    fn from(variant: SWT1_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT1_A {
        match self.bits {
            false => SWT1_A::SWT1_0,
            true => SWT1_A::SWT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT1_0`"]
    #[inline(always)]
    pub fn is_swt1_0(&self) -> bool {
        *self == SWT1_A::SWT1_0
    }
    #[doc = "Checks if the value of the field is `SWT1_1`"]
    #[inline(always)]
    pub fn is_swt1_1(&self) -> bool {
        *self == SWT1_A::SWT1_1
    }
}
#[doc = "Field `SWT1` writer - Software trigger 1 event"]
pub type SWT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT1_A, O>;
impl<'a, const O: u8> SWT1_W<'a, O> {
    #[doc = "No trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_0(self) -> &'a mut W {
        self.variant(SWT1_A::SWT1_0)
    }
    #[doc = "Trigger 1 event generated."]
    #[inline(always)]
    pub fn swt1_1(self) -> &'a mut W {
        self.variant(SWT1_A::SWT1_1)
    }
}
#[doc = "Field `SWT2` reader - Software trigger 2 event"]
pub type SWT2_R = crate::BitReader<SWT2_A>;
#[doc = "Software trigger 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT2_A {
    #[doc = "0: No trigger 2 event generated."]
    SWT2_0 = 0,
    #[doc = "1: Trigger 2 event generated."]
    SWT2_1 = 1,
}
impl From<SWT2_A> for bool {
    #[inline(always)]
    fn from(variant: SWT2_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT2_A {
        match self.bits {
            false => SWT2_A::SWT2_0,
            true => SWT2_A::SWT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT2_0`"]
    #[inline(always)]
    pub fn is_swt2_0(&self) -> bool {
        *self == SWT2_A::SWT2_0
    }
    #[doc = "Checks if the value of the field is `SWT2_1`"]
    #[inline(always)]
    pub fn is_swt2_1(&self) -> bool {
        *self == SWT2_A::SWT2_1
    }
}
#[doc = "Field `SWT2` writer - Software trigger 2 event"]
pub type SWT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT2_A, O>;
impl<'a, const O: u8> SWT2_W<'a, O> {
    #[doc = "No trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_0(self) -> &'a mut W {
        self.variant(SWT2_A::SWT2_0)
    }
    #[doc = "Trigger 2 event generated."]
    #[inline(always)]
    pub fn swt2_1(self) -> &'a mut W {
        self.variant(SWT2_A::SWT2_1)
    }
}
#[doc = "Field `SWT3` reader - Software trigger 3 event"]
pub type SWT3_R = crate::BitReader<SWT3_A>;
#[doc = "Software trigger 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT3_A {
    #[doc = "0: No trigger 3 event generated."]
    SWT3_0 = 0,
    #[doc = "1: Trigger 3 event generated."]
    SWT3_1 = 1,
}
impl From<SWT3_A> for bool {
    #[inline(always)]
    fn from(variant: SWT3_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT3_A {
        match self.bits {
            false => SWT3_A::SWT3_0,
            true => SWT3_A::SWT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT3_0`"]
    #[inline(always)]
    pub fn is_swt3_0(&self) -> bool {
        *self == SWT3_A::SWT3_0
    }
    #[doc = "Checks if the value of the field is `SWT3_1`"]
    #[inline(always)]
    pub fn is_swt3_1(&self) -> bool {
        *self == SWT3_A::SWT3_1
    }
}
#[doc = "Field `SWT3` writer - Software trigger 3 event"]
pub type SWT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT3_A, O>;
impl<'a, const O: u8> SWT3_W<'a, O> {
    #[doc = "No trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_0(self) -> &'a mut W {
        self.variant(SWT3_A::SWT3_0)
    }
    #[doc = "Trigger 3 event generated."]
    #[inline(always)]
    pub fn swt3_1(self) -> &'a mut W {
        self.variant(SWT3_A::SWT3_1)
    }
}
#[doc = "Field `SWT4` reader - Software trigger 4 event"]
pub type SWT4_R = crate::BitReader<SWT4_A>;
#[doc = "Software trigger 4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT4_A {
    #[doc = "0: No trigger 4 event generated."]
    SWT4_0 = 0,
    #[doc = "1: Trigger 4 event generated."]
    SWT4_1 = 1,
}
impl From<SWT4_A> for bool {
    #[inline(always)]
    fn from(variant: SWT4_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT4_A {
        match self.bits {
            false => SWT4_A::SWT4_0,
            true => SWT4_A::SWT4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT4_0`"]
    #[inline(always)]
    pub fn is_swt4_0(&self) -> bool {
        *self == SWT4_A::SWT4_0
    }
    #[doc = "Checks if the value of the field is `SWT4_1`"]
    #[inline(always)]
    pub fn is_swt4_1(&self) -> bool {
        *self == SWT4_A::SWT4_1
    }
}
#[doc = "Field `SWT4` writer - Software trigger 4 event"]
pub type SWT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT4_A, O>;
impl<'a, const O: u8> SWT4_W<'a, O> {
    #[doc = "No trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_0(self) -> &'a mut W {
        self.variant(SWT4_A::SWT4_0)
    }
    #[doc = "Trigger 4 event generated."]
    #[inline(always)]
    pub fn swt4_1(self) -> &'a mut W {
        self.variant(SWT4_A::SWT4_1)
    }
}
#[doc = "Field `SWT5` reader - Software trigger 5 event"]
pub type SWT5_R = crate::BitReader<SWT5_A>;
#[doc = "Software trigger 5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT5_A {
    #[doc = "0: No trigger 5 event generated."]
    SWT5_0 = 0,
    #[doc = "1: Trigger 5 event generated."]
    SWT5_1 = 1,
}
impl From<SWT5_A> for bool {
    #[inline(always)]
    fn from(variant: SWT5_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT5_A {
        match self.bits {
            false => SWT5_A::SWT5_0,
            true => SWT5_A::SWT5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT5_0`"]
    #[inline(always)]
    pub fn is_swt5_0(&self) -> bool {
        *self == SWT5_A::SWT5_0
    }
    #[doc = "Checks if the value of the field is `SWT5_1`"]
    #[inline(always)]
    pub fn is_swt5_1(&self) -> bool {
        *self == SWT5_A::SWT5_1
    }
}
#[doc = "Field `SWT5` writer - Software trigger 5 event"]
pub type SWT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT5_A, O>;
impl<'a, const O: u8> SWT5_W<'a, O> {
    #[doc = "No trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_0(self) -> &'a mut W {
        self.variant(SWT5_A::SWT5_0)
    }
    #[doc = "Trigger 5 event generated."]
    #[inline(always)]
    pub fn swt5_1(self) -> &'a mut W {
        self.variant(SWT5_A::SWT5_1)
    }
}
#[doc = "Field `SWT6` reader - Software trigger 6 event"]
pub type SWT6_R = crate::BitReader<SWT6_A>;
#[doc = "Software trigger 6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT6_A {
    #[doc = "0: No trigger 6 event generated."]
    SWT6_0 = 0,
    #[doc = "1: Trigger 6 event generated."]
    SWT6_1 = 1,
}
impl From<SWT6_A> for bool {
    #[inline(always)]
    fn from(variant: SWT6_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT6_A {
        match self.bits {
            false => SWT6_A::SWT6_0,
            true => SWT6_A::SWT6_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT6_0`"]
    #[inline(always)]
    pub fn is_swt6_0(&self) -> bool {
        *self == SWT6_A::SWT6_0
    }
    #[doc = "Checks if the value of the field is `SWT6_1`"]
    #[inline(always)]
    pub fn is_swt6_1(&self) -> bool {
        *self == SWT6_A::SWT6_1
    }
}
#[doc = "Field `SWT6` writer - Software trigger 6 event"]
pub type SWT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT6_A, O>;
impl<'a, const O: u8> SWT6_W<'a, O> {
    #[doc = "No trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_0(self) -> &'a mut W {
        self.variant(SWT6_A::SWT6_0)
    }
    #[doc = "Trigger 6 event generated."]
    #[inline(always)]
    pub fn swt6_1(self) -> &'a mut W {
        self.variant(SWT6_A::SWT6_1)
    }
}
#[doc = "Field `SWT7` reader - Software trigger 7 event"]
pub type SWT7_R = crate::BitReader<SWT7_A>;
#[doc = "Software trigger 7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT7_A {
    #[doc = "0: No trigger 7 event generated."]
    SWT7_0 = 0,
    #[doc = "1: Trigger 7 event generated."]
    SWT7_1 = 1,
}
impl From<SWT7_A> for bool {
    #[inline(always)]
    fn from(variant: SWT7_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT7_A {
        match self.bits {
            false => SWT7_A::SWT7_0,
            true => SWT7_A::SWT7_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT7_0`"]
    #[inline(always)]
    pub fn is_swt7_0(&self) -> bool {
        *self == SWT7_A::SWT7_0
    }
    #[doc = "Checks if the value of the field is `SWT7_1`"]
    #[inline(always)]
    pub fn is_swt7_1(&self) -> bool {
        *self == SWT7_A::SWT7_1
    }
}
#[doc = "Field `SWT7` writer - Software trigger 7 event"]
pub type SWT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT7_A, O>;
impl<'a, const O: u8> SWT7_W<'a, O> {
    #[doc = "No trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_0(self) -> &'a mut W {
        self.variant(SWT7_A::SWT7_0)
    }
    #[doc = "Trigger 7 event generated."]
    #[inline(always)]
    pub fn swt7_1(self) -> &'a mut W {
        self.variant(SWT7_A::SWT7_1)
    }
}
#[doc = "Field `SWT8` reader - Software trigger 8 event"]
pub type SWT8_R = crate::BitReader<SWT8_A>;
#[doc = "Software trigger 8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT8_A {
    #[doc = "0: No trigger 8 event generated."]
    SWT8_0 = 0,
    #[doc = "1: Trigger 8 event generated."]
    SWT8_1 = 1,
}
impl From<SWT8_A> for bool {
    #[inline(always)]
    fn from(variant: SWT8_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT8_A {
        match self.bits {
            false => SWT8_A::SWT8_0,
            true => SWT8_A::SWT8_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT8_0`"]
    #[inline(always)]
    pub fn is_swt8_0(&self) -> bool {
        *self == SWT8_A::SWT8_0
    }
    #[doc = "Checks if the value of the field is `SWT8_1`"]
    #[inline(always)]
    pub fn is_swt8_1(&self) -> bool {
        *self == SWT8_A::SWT8_1
    }
}
#[doc = "Field `SWT8` writer - Software trigger 8 event"]
pub type SWT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT8_A, O>;
impl<'a, const O: u8> SWT8_W<'a, O> {
    #[doc = "No trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_0(self) -> &'a mut W {
        self.variant(SWT8_A::SWT8_0)
    }
    #[doc = "Trigger 8 event generated."]
    #[inline(always)]
    pub fn swt8_1(self) -> &'a mut W {
        self.variant(SWT8_A::SWT8_1)
    }
}
#[doc = "Field `SWT9` reader - Software trigger 9 event"]
pub type SWT9_R = crate::BitReader<SWT9_A>;
#[doc = "Software trigger 9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT9_A {
    #[doc = "0: No trigger 9 event generated."]
    SWT9_0 = 0,
    #[doc = "1: Trigger 9 event generated."]
    SWT9_1 = 1,
}
impl From<SWT9_A> for bool {
    #[inline(always)]
    fn from(variant: SWT9_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT9_A {
        match self.bits {
            false => SWT9_A::SWT9_0,
            true => SWT9_A::SWT9_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT9_0`"]
    #[inline(always)]
    pub fn is_swt9_0(&self) -> bool {
        *self == SWT9_A::SWT9_0
    }
    #[doc = "Checks if the value of the field is `SWT9_1`"]
    #[inline(always)]
    pub fn is_swt9_1(&self) -> bool {
        *self == SWT9_A::SWT9_1
    }
}
#[doc = "Field `SWT9` writer - Software trigger 9 event"]
pub type SWT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT9_A, O>;
impl<'a, const O: u8> SWT9_W<'a, O> {
    #[doc = "No trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_0(self) -> &'a mut W {
        self.variant(SWT9_A::SWT9_0)
    }
    #[doc = "Trigger 9 event generated."]
    #[inline(always)]
    pub fn swt9_1(self) -> &'a mut W {
        self.variant(SWT9_A::SWT9_1)
    }
}
#[doc = "Field `SWT10` reader - Software trigger 10 event"]
pub type SWT10_R = crate::BitReader<SWT10_A>;
#[doc = "Software trigger 10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT10_A {
    #[doc = "0: No trigger 10 event generated."]
    SWT10_0 = 0,
    #[doc = "1: Trigger 10 event generated."]
    SWT10_1 = 1,
}
impl From<SWT10_A> for bool {
    #[inline(always)]
    fn from(variant: SWT10_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT10_A {
        match self.bits {
            false => SWT10_A::SWT10_0,
            true => SWT10_A::SWT10_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT10_0`"]
    #[inline(always)]
    pub fn is_swt10_0(&self) -> bool {
        *self == SWT10_A::SWT10_0
    }
    #[doc = "Checks if the value of the field is `SWT10_1`"]
    #[inline(always)]
    pub fn is_swt10_1(&self) -> bool {
        *self == SWT10_A::SWT10_1
    }
}
#[doc = "Field `SWT10` writer - Software trigger 10 event"]
pub type SWT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT10_A, O>;
impl<'a, const O: u8> SWT10_W<'a, O> {
    #[doc = "No trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_0(self) -> &'a mut W {
        self.variant(SWT10_A::SWT10_0)
    }
    #[doc = "Trigger 10 event generated."]
    #[inline(always)]
    pub fn swt10_1(self) -> &'a mut W {
        self.variant(SWT10_A::SWT10_1)
    }
}
#[doc = "Field `SWT11` reader - Software trigger 11 event"]
pub type SWT11_R = crate::BitReader<SWT11_A>;
#[doc = "Software trigger 11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT11_A {
    #[doc = "0: No trigger 11 event generated."]
    SWT11_0 = 0,
    #[doc = "1: Trigger 11 event generated."]
    SWT11_1 = 1,
}
impl From<SWT11_A> for bool {
    #[inline(always)]
    fn from(variant: SWT11_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT11_A {
        match self.bits {
            false => SWT11_A::SWT11_0,
            true => SWT11_A::SWT11_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT11_0`"]
    #[inline(always)]
    pub fn is_swt11_0(&self) -> bool {
        *self == SWT11_A::SWT11_0
    }
    #[doc = "Checks if the value of the field is `SWT11_1`"]
    #[inline(always)]
    pub fn is_swt11_1(&self) -> bool {
        *self == SWT11_A::SWT11_1
    }
}
#[doc = "Field `SWT11` writer - Software trigger 11 event"]
pub type SWT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT11_A, O>;
impl<'a, const O: u8> SWT11_W<'a, O> {
    #[doc = "No trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_0(self) -> &'a mut W {
        self.variant(SWT11_A::SWT11_0)
    }
    #[doc = "Trigger 11 event generated."]
    #[inline(always)]
    pub fn swt11_1(self) -> &'a mut W {
        self.variant(SWT11_A::SWT11_1)
    }
}
#[doc = "Field `SWT12` reader - Software trigger 12 event"]
pub type SWT12_R = crate::BitReader<SWT12_A>;
#[doc = "Software trigger 12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT12_A {
    #[doc = "0: No trigger 12 event generated."]
    SWT12_0 = 0,
    #[doc = "1: Trigger 12 event generated."]
    SWT12_1 = 1,
}
impl From<SWT12_A> for bool {
    #[inline(always)]
    fn from(variant: SWT12_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT12_A {
        match self.bits {
            false => SWT12_A::SWT12_0,
            true => SWT12_A::SWT12_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT12_0`"]
    #[inline(always)]
    pub fn is_swt12_0(&self) -> bool {
        *self == SWT12_A::SWT12_0
    }
    #[doc = "Checks if the value of the field is `SWT12_1`"]
    #[inline(always)]
    pub fn is_swt12_1(&self) -> bool {
        *self == SWT12_A::SWT12_1
    }
}
#[doc = "Field `SWT12` writer - Software trigger 12 event"]
pub type SWT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT12_A, O>;
impl<'a, const O: u8> SWT12_W<'a, O> {
    #[doc = "No trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_0(self) -> &'a mut W {
        self.variant(SWT12_A::SWT12_0)
    }
    #[doc = "Trigger 12 event generated."]
    #[inline(always)]
    pub fn swt12_1(self) -> &'a mut W {
        self.variant(SWT12_A::SWT12_1)
    }
}
#[doc = "Field `SWT13` reader - Software trigger 13 event"]
pub type SWT13_R = crate::BitReader<SWT13_A>;
#[doc = "Software trigger 13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT13_A {
    #[doc = "0: No trigger 13 event generated."]
    SWT13_0 = 0,
    #[doc = "1: Trigger 13 event generated."]
    SWT13_1 = 1,
}
impl From<SWT13_A> for bool {
    #[inline(always)]
    fn from(variant: SWT13_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT13_A {
        match self.bits {
            false => SWT13_A::SWT13_0,
            true => SWT13_A::SWT13_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT13_0`"]
    #[inline(always)]
    pub fn is_swt13_0(&self) -> bool {
        *self == SWT13_A::SWT13_0
    }
    #[doc = "Checks if the value of the field is `SWT13_1`"]
    #[inline(always)]
    pub fn is_swt13_1(&self) -> bool {
        *self == SWT13_A::SWT13_1
    }
}
#[doc = "Field `SWT13` writer - Software trigger 13 event"]
pub type SWT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT13_A, O>;
impl<'a, const O: u8> SWT13_W<'a, O> {
    #[doc = "No trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_0(self) -> &'a mut W {
        self.variant(SWT13_A::SWT13_0)
    }
    #[doc = "Trigger 13 event generated."]
    #[inline(always)]
    pub fn swt13_1(self) -> &'a mut W {
        self.variant(SWT13_A::SWT13_1)
    }
}
#[doc = "Field `SWT14` reader - Software trigger 14 event"]
pub type SWT14_R = crate::BitReader<SWT14_A>;
#[doc = "Software trigger 14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT14_A {
    #[doc = "0: No trigger 14 event generated."]
    SWT14_0 = 0,
    #[doc = "1: Trigger 14 event generated."]
    SWT14_1 = 1,
}
impl From<SWT14_A> for bool {
    #[inline(always)]
    fn from(variant: SWT14_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT14_A {
        match self.bits {
            false => SWT14_A::SWT14_0,
            true => SWT14_A::SWT14_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT14_0`"]
    #[inline(always)]
    pub fn is_swt14_0(&self) -> bool {
        *self == SWT14_A::SWT14_0
    }
    #[doc = "Checks if the value of the field is `SWT14_1`"]
    #[inline(always)]
    pub fn is_swt14_1(&self) -> bool {
        *self == SWT14_A::SWT14_1
    }
}
#[doc = "Field `SWT14` writer - Software trigger 14 event"]
pub type SWT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT14_A, O>;
impl<'a, const O: u8> SWT14_W<'a, O> {
    #[doc = "No trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_0(self) -> &'a mut W {
        self.variant(SWT14_A::SWT14_0)
    }
    #[doc = "Trigger 14 event generated."]
    #[inline(always)]
    pub fn swt14_1(self) -> &'a mut W {
        self.variant(SWT14_A::SWT14_1)
    }
}
#[doc = "Field `SWT15` reader - Software trigger 15 event"]
pub type SWT15_R = crate::BitReader<SWT15_A>;
#[doc = "Software trigger 15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT15_A {
    #[doc = "0: No trigger 15 event generated."]
    SWT15_0 = 0,
    #[doc = "1: Trigger 15 event generated."]
    SWT15_1 = 1,
}
impl From<SWT15_A> for bool {
    #[inline(always)]
    fn from(variant: SWT15_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT15_A {
        match self.bits {
            false => SWT15_A::SWT15_0,
            true => SWT15_A::SWT15_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWT15_0`"]
    #[inline(always)]
    pub fn is_swt15_0(&self) -> bool {
        *self == SWT15_A::SWT15_0
    }
    #[doc = "Checks if the value of the field is `SWT15_1`"]
    #[inline(always)]
    pub fn is_swt15_1(&self) -> bool {
        *self == SWT15_A::SWT15_1
    }
}
#[doc = "Field `SWT15` writer - Software trigger 15 event"]
pub type SWT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT15_A, O>;
impl<'a, const O: u8> SWT15_W<'a, O> {
    #[doc = "No trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_0(self) -> &'a mut W {
        self.variant(SWT15_A::SWT15_0)
    }
    #[doc = "Trigger 15 event generated."]
    #[inline(always)]
    pub fn swt15_1(self) -> &'a mut W {
        self.variant(SWT15_A::SWT15_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    pub fn swt0(&self) -> SWT0_R {
        SWT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    pub fn swt1(&self) -> SWT1_R {
        SWT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    pub fn swt2(&self) -> SWT2_R {
        SWT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    pub fn swt3(&self) -> SWT3_R {
        SWT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    pub fn swt4(&self) -> SWT4_R {
        SWT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    pub fn swt5(&self) -> SWT5_R {
        SWT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    pub fn swt6(&self) -> SWT6_R {
        SWT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    pub fn swt7(&self) -> SWT7_R {
        SWT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    pub fn swt8(&self) -> SWT8_R {
        SWT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    pub fn swt9(&self) -> SWT9_R {
        SWT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    pub fn swt10(&self) -> SWT10_R {
        SWT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    pub fn swt11(&self) -> SWT11_R {
        SWT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    pub fn swt12(&self) -> SWT12_R {
        SWT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    pub fn swt13(&self) -> SWT13_R {
        SWT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    pub fn swt14(&self) -> SWT14_R {
        SWT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    pub fn swt15(&self) -> SWT15_R {
        SWT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger 0 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt0(&mut self) -> SWT0_W<0> {
        SWT0_W::new(self)
    }
    #[doc = "Bit 1 - Software trigger 1 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt1(&mut self) -> SWT1_W<1> {
        SWT1_W::new(self)
    }
    #[doc = "Bit 2 - Software trigger 2 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt2(&mut self) -> SWT2_W<2> {
        SWT2_W::new(self)
    }
    #[doc = "Bit 3 - Software trigger 3 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt3(&mut self) -> SWT3_W<3> {
        SWT3_W::new(self)
    }
    #[doc = "Bit 4 - Software trigger 4 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt4(&mut self) -> SWT4_W<4> {
        SWT4_W::new(self)
    }
    #[doc = "Bit 5 - Software trigger 5 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt5(&mut self) -> SWT5_W<5> {
        SWT5_W::new(self)
    }
    #[doc = "Bit 6 - Software trigger 6 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt6(&mut self) -> SWT6_W<6> {
        SWT6_W::new(self)
    }
    #[doc = "Bit 7 - Software trigger 7 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt7(&mut self) -> SWT7_W<7> {
        SWT7_W::new(self)
    }
    #[doc = "Bit 8 - Software trigger 8 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt8(&mut self) -> SWT8_W<8> {
        SWT8_W::new(self)
    }
    #[doc = "Bit 9 - Software trigger 9 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt9(&mut self) -> SWT9_W<9> {
        SWT9_W::new(self)
    }
    #[doc = "Bit 10 - Software trigger 10 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt10(&mut self) -> SWT10_W<10> {
        SWT10_W::new(self)
    }
    #[doc = "Bit 11 - Software trigger 11 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt11(&mut self) -> SWT11_W<11> {
        SWT11_W::new(self)
    }
    #[doc = "Bit 12 - Software trigger 12 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt12(&mut self) -> SWT12_W<12> {
        SWT12_W::new(self)
    }
    #[doc = "Bit 13 - Software trigger 13 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt13(&mut self) -> SWT13_W<13> {
        SWT13_W::new(self)
    }
    #[doc = "Bit 14 - Software trigger 14 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt14(&mut self) -> SWT14_W<14> {
        SWT14_W::new(self)
    }
    #[doc = "Bit 15 - Software trigger 15 event"]
    #[inline(always)]
    #[must_use]
    pub fn swt15(&mut self) -> SWT15_W<15> {
        SWT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](index.html) module"]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrig::R](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrig::W](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
