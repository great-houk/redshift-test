#[doc = "Register `CPPWR` reader"]
pub struct R(crate::R<CPPWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPPWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPPWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPPWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPPWR` writer"]
pub struct W(crate::W<CPPWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPPWR_SPEC>;
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
impl From<crate::W<CPPWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPPWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SU0` reader - State UNKNOWN 0."]
pub type SU0_R = crate::BitReader<SU0_A>;
#[doc = "State UNKNOWN 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU0_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU0_A> for bool {
    #[inline(always)]
    fn from(variant: SU0_A) -> Self {
        variant as u8 != 0
    }
}
impl SU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU0_A {
        match self.bits {
            false => SU0_A::UNKNOWN_NOT_PERMITTED,
            true => SU0_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU0_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU0_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU0` writer - State UNKNOWN 0."]
pub type SU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU0_A, O>;
impl<'a, const O: u8> SU0_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU0_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU0_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS0` reader - State UNKNOWN Secure only 0."]
pub type SUS0_R = crate::BitReader<SUS0_A>;
#[doc = "State UNKNOWN Secure only 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS0_A {
    #[doc = "0: The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS0_A> for bool {
    #[inline(always)]
    fn from(variant: SUS0_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS0_A {
        match self.bits {
            false => SUS0_A::SECURE_AND_NON_SECURE,
            true => SUS0_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS0_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS0_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS0` writer - State UNKNOWN Secure only 0."]
pub type SUS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS0_A, O>;
impl<'a, const O: u8> SUS0_W<'a, O> {
    #[doc = "The SU0 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS0_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU0 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS0_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU1` reader - State UNKNOWN 1."]
pub type SU1_R = crate::BitReader<SU1_A>;
#[doc = "State UNKNOWN 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU1_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU1_A> for bool {
    #[inline(always)]
    fn from(variant: SU1_A) -> Self {
        variant as u8 != 0
    }
}
impl SU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU1_A {
        match self.bits {
            false => SU1_A::UNKNOWN_NOT_PERMITTED,
            true => SU1_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU1_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU1_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU1` writer - State UNKNOWN 1."]
pub type SU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU1_A, O>;
impl<'a, const O: u8> SU1_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU1_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU1_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS1` reader - State UNKNOWN Secure only 1."]
pub type SUS1_R = crate::BitReader<SUS1_A>;
#[doc = "State UNKNOWN Secure only 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS1_A {
    #[doc = "0: The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS1_A> for bool {
    #[inline(always)]
    fn from(variant: SUS1_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS1_A {
        match self.bits {
            false => SUS1_A::SECURE_AND_NON_SECURE,
            true => SUS1_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS1_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS1_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS1` writer - State UNKNOWN Secure only 1."]
pub type SUS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS1_A, O>;
impl<'a, const O: u8> SUS1_W<'a, O> {
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS1_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS1_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU2` reader - State UNKNOWN 2."]
pub type SU2_R = crate::BitReader<SU2_A>;
#[doc = "State UNKNOWN 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU2_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU2_A> for bool {
    #[inline(always)]
    fn from(variant: SU2_A) -> Self {
        variant as u8 != 0
    }
}
impl SU2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU2_A {
        match self.bits {
            false => SU2_A::UNKNOWN_NOT_PERMITTED,
            true => SU2_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU2_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU2_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU2` writer - State UNKNOWN 2."]
pub type SU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU2_A, O>;
impl<'a, const O: u8> SU2_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU2_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU2_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS2` reader - State UNKNOWN Secure only 2."]
pub type SUS2_R = crate::BitReader<SUS2_A>;
#[doc = "State UNKNOWN Secure only 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS2_A {
    #[doc = "0: The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS2_A> for bool {
    #[inline(always)]
    fn from(variant: SUS2_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS2_A {
        match self.bits {
            false => SUS2_A::SECURE_AND_NON_SECURE,
            true => SUS2_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS2_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS2_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS2` writer - State UNKNOWN Secure only 2."]
pub type SUS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS2_A, O>;
impl<'a, const O: u8> SUS2_W<'a, O> {
    #[doc = "The SU2 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS2_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU2 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS2_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU3` reader - State UNKNOWN 3."]
pub type SU3_R = crate::BitReader<SU3_A>;
#[doc = "State UNKNOWN 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU3_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU3_A> for bool {
    #[inline(always)]
    fn from(variant: SU3_A) -> Self {
        variant as u8 != 0
    }
}
impl SU3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU3_A {
        match self.bits {
            false => SU3_A::UNKNOWN_NOT_PERMITTED,
            true => SU3_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU3_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU3_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU3` writer - State UNKNOWN 3."]
pub type SU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU3_A, O>;
impl<'a, const O: u8> SU3_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU3_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU3_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS3` reader - State UNKNOWN Secure only 3."]
pub type SUS3_R = crate::BitReader<SUS3_A>;
#[doc = "State UNKNOWN Secure only 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS3_A {
    #[doc = "0: The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS3_A> for bool {
    #[inline(always)]
    fn from(variant: SUS3_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS3_A {
        match self.bits {
            false => SUS3_A::SECURE_AND_NON_SECURE,
            true => SUS3_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS3_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS3_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS3` writer - State UNKNOWN Secure only 3."]
pub type SUS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS3_A, O>;
impl<'a, const O: u8> SUS3_W<'a, O> {
    #[doc = "The SU3 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS3_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU3 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS3_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU4` reader - State UNKNOWN 4."]
pub type SU4_R = crate::BitReader<SU4_A>;
#[doc = "State UNKNOWN 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU4_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU4_A> for bool {
    #[inline(always)]
    fn from(variant: SU4_A) -> Self {
        variant as u8 != 0
    }
}
impl SU4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU4_A {
        match self.bits {
            false => SU4_A::UNKNOWN_NOT_PERMITTED,
            true => SU4_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU4_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU4_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU4` writer - State UNKNOWN 4."]
pub type SU4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU4_A, O>;
impl<'a, const O: u8> SU4_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU4_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU4_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS4` reader - State UNKNOWN Secure only 4."]
pub type SUS4_R = crate::BitReader<SUS4_A>;
#[doc = "State UNKNOWN Secure only 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS4_A {
    #[doc = "0: The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS4_A> for bool {
    #[inline(always)]
    fn from(variant: SUS4_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS4_A {
        match self.bits {
            false => SUS4_A::SECURE_AND_NON_SECURE,
            true => SUS4_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS4_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS4_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS4` writer - State UNKNOWN Secure only 4."]
pub type SUS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS4_A, O>;
impl<'a, const O: u8> SUS4_W<'a, O> {
    #[doc = "The SU4 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS4_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU4 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS4_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU5` reader - State UNKNOWN 5."]
pub type SU5_R = crate::BitReader<SU5_A>;
#[doc = "State UNKNOWN 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU5_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU5_A> for bool {
    #[inline(always)]
    fn from(variant: SU5_A) -> Self {
        variant as u8 != 0
    }
}
impl SU5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU5_A {
        match self.bits {
            false => SU5_A::UNKNOWN_NOT_PERMITTED,
            true => SU5_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU5_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU5_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU5` writer - State UNKNOWN 5."]
pub type SU5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU5_A, O>;
impl<'a, const O: u8> SU5_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU5_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU5_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS5` reader - State UNKNOWN Secure only 5."]
pub type SUS5_R = crate::BitReader<SUS5_A>;
#[doc = "State UNKNOWN Secure only 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS5_A {
    #[doc = "0: The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS5_A> for bool {
    #[inline(always)]
    fn from(variant: SUS5_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS5_A {
        match self.bits {
            false => SUS5_A::SECURE_AND_NON_SECURE,
            true => SUS5_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS5_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS5_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS5` writer - State UNKNOWN Secure only 5."]
pub type SUS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS5_A, O>;
impl<'a, const O: u8> SUS5_W<'a, O> {
    #[doc = "The SU5 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS5_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU5 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS5_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU6` reader - State UNKNOWN 6."]
pub type SU6_R = crate::BitReader<SU6_A>;
#[doc = "State UNKNOWN 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU6_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU6_A> for bool {
    #[inline(always)]
    fn from(variant: SU6_A) -> Self {
        variant as u8 != 0
    }
}
impl SU6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU6_A {
        match self.bits {
            false => SU6_A::UNKNOWN_NOT_PERMITTED,
            true => SU6_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU6_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU6_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU6` writer - State UNKNOWN 6."]
pub type SU6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU6_A, O>;
impl<'a, const O: u8> SU6_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU6_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU6_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS6` reader - State UNKNOWN Secure only 6."]
pub type SUS6_R = crate::BitReader<SUS6_A>;
#[doc = "State UNKNOWN Secure only 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS6_A {
    #[doc = "0: The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS6_A> for bool {
    #[inline(always)]
    fn from(variant: SUS6_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS6_A {
        match self.bits {
            false => SUS6_A::SECURE_AND_NON_SECURE,
            true => SUS6_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS6_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS6_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS6` writer - State UNKNOWN Secure only 6."]
pub type SUS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS6_A, O>;
impl<'a, const O: u8> SUS6_W<'a, O> {
    #[doc = "The SU6 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS6_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU6 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS6_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU7` reader - State UNKNOWN 7."]
pub type SU7_R = crate::BitReader<SU7_A>;
#[doc = "State UNKNOWN 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU7_A {
    #[doc = "0: The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU7_A> for bool {
    #[inline(always)]
    fn from(variant: SU7_A) -> Self {
        variant as u8 != 0
    }
}
impl SU7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU7_A {
        match self.bits {
            false => SU7_A::UNKNOWN_NOT_PERMITTED,
            true => SU7_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU7_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU7_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU7` writer - State UNKNOWN 7."]
pub type SU7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU7_A, O>;
impl<'a, const O: u8> SU7_W<'a, O> {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU7_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU7_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS7` reader - State UNKNOWN Secure only 7."]
pub type SUS7_R = crate::BitReader<SUS7_A>;
#[doc = "State UNKNOWN Secure only 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS7_A {
    #[doc = "0: The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS7_A> for bool {
    #[inline(always)]
    fn from(variant: SUS7_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS7_A {
        match self.bits {
            false => SUS7_A::SECURE_AND_NON_SECURE,
            true => SUS7_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS7_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS7_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS7` writer - State UNKNOWN Secure only 7."]
pub type SUS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS7_A, O>;
impl<'a, const O: u8> SUS7_W<'a, O> {
    #[doc = "The SU7 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS7_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU7 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS7_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU10` reader - State UNKNOWN 10."]
pub type SU10_R = crate::BitReader<SU10_A>;
#[doc = "State UNKNOWN 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SU10_A {
    #[doc = "0: The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0,
    #[doc = "1: The floating-point state is permitted to become UNKNOWN"]
    UNKNOWN_PERMITTED = 1,
}
impl From<SU10_A> for bool {
    #[inline(always)]
    fn from(variant: SU10_A) -> Self {
        variant as u8 != 0
    }
}
impl SU10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SU10_A {
        match self.bits {
            false => SU10_A::UNKNOWN_NOT_PERMITTED,
            true => SU10_A::UNKNOWN_PERMITTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_NOT_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_not_permitted(&self) -> bool {
        *self == SU10_A::UNKNOWN_NOT_PERMITTED
    }
    #[doc = "Checks if the value of the field is `UNKNOWN_PERMITTED`"]
    #[inline(always)]
    pub fn is_unknown_permitted(&self) -> bool {
        *self == SU10_A::UNKNOWN_PERMITTED
    }
}
#[doc = "Field `SU10` writer - State UNKNOWN 10."]
pub type SU10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SU10_A, O>;
impl<'a, const O: u8> SU10_W<'a, O> {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    #[inline(always)]
    pub fn unknown_not_permitted(self) -> &'a mut W {
        self.variant(SU10_A::UNKNOWN_NOT_PERMITTED)
    }
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    #[inline(always)]
    pub fn unknown_permitted(self) -> &'a mut W {
        self.variant(SU10_A::UNKNOWN_PERMITTED)
    }
}
#[doc = "Field `SUS10` reader - State UNKNOWN Secure only 10."]
pub type SUS10_R = crate::BitReader<SUS10_A>;
#[doc = "State UNKNOWN Secure only 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS10_A {
    #[doc = "0: The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0,
    #[doc = "1: The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY = 1,
}
impl From<SUS10_A> for bool {
    #[inline(always)]
    fn from(variant: SUS10_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS10_A {
        match self.bits {
            false => SUS10_A::SECURE_AND_NON_SECURE,
            true => SUS10_A::SECURE_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE_AND_NON_SECURE`"]
    #[inline(always)]
    pub fn is_secure_and_non_secure(&self) -> bool {
        *self == SUS10_A::SECURE_AND_NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE_ONLY`"]
    #[inline(always)]
    pub fn is_secure_only(&self) -> bool {
        *self == SUS10_A::SECURE_ONLY
    }
}
#[doc = "Field `SUS10` writer - State UNKNOWN Secure only 10."]
pub type SUS10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, SUS10_A, O>;
impl<'a, const O: u8> SUS10_W<'a, O> {
    #[doc = "The SU10 field is accessible from both Security states."]
    #[inline(always)]
    pub fn secure_and_non_secure(self) -> &'a mut W {
        self.variant(SUS10_A::SECURE_AND_NON_SECURE)
    }
    #[doc = "The SU10 field is only accessible from the Secure state."]
    #[inline(always)]
    pub fn secure_only(self) -> &'a mut W {
        self.variant(SUS10_A::SECURE_ONLY)
    }
}
#[doc = "Field `SU11` reader - State UNKNOWN 11."]
pub type SU11_R = crate::BitReader<bool>;
#[doc = "Field `SU11` writer - State UNKNOWN 11."]
pub type SU11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, bool, O>;
#[doc = "Field `SUS11` reader - State UNKNOWN Secure only 11."]
pub type SUS11_R = crate::BitReader<bool>;
#[doc = "Field `SUS11` writer - State UNKNOWN Secure only 11."]
pub type SUS11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPWR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline(always)]
    pub fn su0(&self) -> SU0_R {
        SU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub fn sus0(&self) -> SUS0_R {
        SUS0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline(always)]
    pub fn su1(&self) -> SU1_R {
        SU1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub fn sus1(&self) -> SUS1_R {
        SUS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline(always)]
    pub fn su2(&self) -> SU2_R {
        SU2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub fn sus2(&self) -> SUS2_R {
        SUS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline(always)]
    pub fn su3(&self) -> SU3_R {
        SU3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub fn sus3(&self) -> SUS3_R {
        SUS3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline(always)]
    pub fn su4(&self) -> SU4_R {
        SU4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub fn sus4(&self) -> SUS4_R {
        SUS4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline(always)]
    pub fn su5(&self) -> SU5_R {
        SU5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub fn sus5(&self) -> SUS5_R {
        SUS5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline(always)]
    pub fn su6(&self) -> SU6_R {
        SU6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub fn sus6(&self) -> SUS6_R {
        SUS6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline(always)]
    pub fn su7(&self) -> SU7_R {
        SU7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub fn sus7(&self) -> SUS7_R {
        SUS7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline(always)]
    pub fn su10(&self) -> SU10_R {
        SU10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub fn sus10(&self) -> SUS10_R {
        SUS10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline(always)]
    pub fn su11(&self) -> SU11_R {
        SU11_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub fn sus11(&self) -> SUS11_R {
        SUS11_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - State UNKNOWN 0."]
    #[inline(always)]
    #[must_use]
    pub fn su0(&mut self) -> SU0_W<0> {
        SU0_W::new(self)
    }
    #[doc = "Bit 1 - State UNKNOWN Secure only 0."]
    #[inline(always)]
    #[must_use]
    pub fn sus0(&mut self) -> SUS0_W<1> {
        SUS0_W::new(self)
    }
    #[doc = "Bit 2 - State UNKNOWN 1."]
    #[inline(always)]
    #[must_use]
    pub fn su1(&mut self) -> SU1_W<2> {
        SU1_W::new(self)
    }
    #[doc = "Bit 3 - State UNKNOWN Secure only 1."]
    #[inline(always)]
    #[must_use]
    pub fn sus1(&mut self) -> SUS1_W<3> {
        SUS1_W::new(self)
    }
    #[doc = "Bit 4 - State UNKNOWN 2."]
    #[inline(always)]
    #[must_use]
    pub fn su2(&mut self) -> SU2_W<4> {
        SU2_W::new(self)
    }
    #[doc = "Bit 5 - State UNKNOWN Secure only 2."]
    #[inline(always)]
    #[must_use]
    pub fn sus2(&mut self) -> SUS2_W<5> {
        SUS2_W::new(self)
    }
    #[doc = "Bit 6 - State UNKNOWN 3."]
    #[inline(always)]
    #[must_use]
    pub fn su3(&mut self) -> SU3_W<6> {
        SU3_W::new(self)
    }
    #[doc = "Bit 7 - State UNKNOWN Secure only 3."]
    #[inline(always)]
    #[must_use]
    pub fn sus3(&mut self) -> SUS3_W<7> {
        SUS3_W::new(self)
    }
    #[doc = "Bit 8 - State UNKNOWN 4."]
    #[inline(always)]
    #[must_use]
    pub fn su4(&mut self) -> SU4_W<8> {
        SU4_W::new(self)
    }
    #[doc = "Bit 9 - State UNKNOWN Secure only 4."]
    #[inline(always)]
    #[must_use]
    pub fn sus4(&mut self) -> SUS4_W<9> {
        SUS4_W::new(self)
    }
    #[doc = "Bit 10 - State UNKNOWN 5."]
    #[inline(always)]
    #[must_use]
    pub fn su5(&mut self) -> SU5_W<10> {
        SU5_W::new(self)
    }
    #[doc = "Bit 11 - State UNKNOWN Secure only 5."]
    #[inline(always)]
    #[must_use]
    pub fn sus5(&mut self) -> SUS5_W<11> {
        SUS5_W::new(self)
    }
    #[doc = "Bit 12 - State UNKNOWN 6."]
    #[inline(always)]
    #[must_use]
    pub fn su6(&mut self) -> SU6_W<12> {
        SU6_W::new(self)
    }
    #[doc = "Bit 13 - State UNKNOWN Secure only 6."]
    #[inline(always)]
    #[must_use]
    pub fn sus6(&mut self) -> SUS6_W<13> {
        SUS6_W::new(self)
    }
    #[doc = "Bit 14 - State UNKNOWN 7."]
    #[inline(always)]
    #[must_use]
    pub fn su7(&mut self) -> SU7_W<14> {
        SU7_W::new(self)
    }
    #[doc = "Bit 15 - State UNKNOWN Secure only 7."]
    #[inline(always)]
    #[must_use]
    pub fn sus7(&mut self) -> SUS7_W<15> {
        SUS7_W::new(self)
    }
    #[doc = "Bit 20 - State UNKNOWN 10."]
    #[inline(always)]
    #[must_use]
    pub fn su10(&mut self) -> SU10_W<20> {
        SU10_W::new(self)
    }
    #[doc = "Bit 21 - State UNKNOWN Secure only 10."]
    #[inline(always)]
    #[must_use]
    pub fn sus10(&mut self) -> SUS10_W<21> {
        SUS10_W::new(self)
    }
    #[doc = "Bit 22 - State UNKNOWN 11."]
    #[inline(always)]
    #[must_use]
    pub fn su11(&mut self) -> SU11_W<22> {
        SU11_W::new(self)
    }
    #[doc = "Bit 23 - State UNKNOWN Secure only 11."]
    #[inline(always)]
    #[must_use]
    pub fn sus11(&mut self) -> SUS11_W<23> {
        SUS11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppwr](index.html) module"]
pub struct CPPWR_SPEC;
impl crate::RegisterSpec for CPPWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cppwr::R](R) reader structure"]
impl crate::Readable for CPPWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cppwr::W](W) writer structure"]
impl crate::Writable for CPPWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPPWR to value 0"]
impl crate::Resettable for CPPWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
