#[doc = "Register `SEC_CTRL_RAM2_MEM_RULE0` reader"]
pub struct R(crate::R<SEC_CTRL_RAM2_MEM_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_RAM2_MEM_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_RAM2_MEM_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_RAM2_MEM_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_RAM2_MEM_RULE0` writer"]
pub struct W(crate::W<SEC_CTRL_RAM2_MEM_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_RAM2_MEM_RULE0_SPEC>;
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
impl From<crate::W<SEC_CTRL_RAM2_MEM_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_RAM2_MEM_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RULE0` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type RULE0_R = crate::FieldReader<u8, RULE0_A>;
#[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE0_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE0_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE0_A) -> Self {
        variant as _
    }
}
impl RULE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE0_A {
        match self.bits {
            0 => RULE0_A::ENUM_NS_NP,
            1 => RULE0_A::ENUM_NS_P,
            2 => RULE0_A::ENUM_S_NP,
            3 => RULE0_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE0_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE0_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE0_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE0_A::ENUM_S_P
    }
}
#[doc = "Field `RULE0` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type RULE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE0_A, 2, O>;
impl<'a, const O: u8> RULE0_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE0_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE1` reader - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type RULE1_R = crate::FieldReader<u8, RULE1_A>;
#[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE1_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE1_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE1_A) -> Self {
        variant as _
    }
}
impl RULE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE1_A {
        match self.bits {
            0 => RULE1_A::ENUM_NS_NP,
            1 => RULE1_A::ENUM_NS_P,
            2 => RULE1_A::ENUM_S_NP,
            3 => RULE1_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE1_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE1_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE1_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE1_A::ENUM_S_P
    }
}
#[doc = "Field `RULE1` writer - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type RULE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE1_A, 2, O>;
impl<'a, const O: u8> RULE1_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE1_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE2` reader - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub type RULE2_R = crate::FieldReader<u8, RULE2_A>;
#[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE2_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE2_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE2_A) -> Self {
        variant as _
    }
}
impl RULE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE2_A {
        match self.bits {
            0 => RULE2_A::ENUM_NS_NP,
            1 => RULE2_A::ENUM_NS_P,
            2 => RULE2_A::ENUM_S_NP,
            3 => RULE2_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE2_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE2_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE2_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE2_A::ENUM_S_P
    }
}
#[doc = "Field `RULE2` writer - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub type RULE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE2_A, 2, O>;
impl<'a, const O: u8> RULE2_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE2_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE3` reader - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub type RULE3_R = crate::FieldReader<u8, RULE3_A>;
#[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE3_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE3_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE3_A) -> Self {
        variant as _
    }
}
impl RULE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE3_A {
        match self.bits {
            0 => RULE3_A::ENUM_NS_NP,
            1 => RULE3_A::ENUM_NS_P,
            2 => RULE3_A::ENUM_S_NP,
            3 => RULE3_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE3_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE3_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE3_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE3_A::ENUM_S_P
    }
}
#[doc = "Field `RULE3` writer - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub type RULE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE3_A, 2, O>;
impl<'a, const O: u8> RULE3_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE3_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE4` reader - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type RULE4_R = crate::FieldReader<u8, RULE4_A>;
#[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE4_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE4_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE4_A) -> Self {
        variant as _
    }
}
impl RULE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE4_A {
        match self.bits {
            0 => RULE4_A::ENUM_NS_NP,
            1 => RULE4_A::ENUM_NS_P,
            2 => RULE4_A::ENUM_S_NP,
            3 => RULE4_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE4_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE4_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE4_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE4_A::ENUM_S_P
    }
}
#[doc = "Field `RULE4` writer - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type RULE4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE4_A, 2, O>;
impl<'a, const O: u8> RULE4_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE4_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE5` reader - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type RULE5_R = crate::FieldReader<u8, RULE5_A>;
#[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE5_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE5_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE5_A) -> Self {
        variant as _
    }
}
impl RULE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE5_A {
        match self.bits {
            0 => RULE5_A::ENUM_NS_NP,
            1 => RULE5_A::ENUM_NS_P,
            2 => RULE5_A::ENUM_S_NP,
            3 => RULE5_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE5_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE5_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE5_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE5_A::ENUM_S_P
    }
}
#[doc = "Field `RULE5` writer - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type RULE5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE5_A, 2, O>;
impl<'a, const O: u8> RULE5_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE5_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE6` reader - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub type RULE6_R = crate::FieldReader<u8, RULE6_A>;
#[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE6_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE6_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE6_A) -> Self {
        variant as _
    }
}
impl RULE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE6_A {
        match self.bits {
            0 => RULE6_A::ENUM_NS_NP,
            1 => RULE6_A::ENUM_NS_P,
            2 => RULE6_A::ENUM_S_NP,
            3 => RULE6_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE6_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE6_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE6_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE6_A::ENUM_S_P
    }
}
#[doc = "Field `RULE6` writer - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub type RULE6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE6_A, 2, O>;
impl<'a, const O: u8> RULE6_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE6_A::ENUM_S_P)
    }
}
#[doc = "Field `RULE7` reader - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub type RULE7_R = crate::FieldReader<u8, RULE7_A>;
#[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE7_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RULE7_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE7_A) -> Self {
        variant as _
    }
}
impl RULE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE7_A {
        match self.bits {
            0 => RULE7_A::ENUM_NS_NP,
            1 => RULE7_A::ENUM_NS_P,
            2 => RULE7_A::ENUM_S_NP,
            3 => RULE7_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RULE7_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RULE7_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RULE7_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RULE7_A::ENUM_S_P
    }
}
#[doc = "Field `RULE7` writer - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub type RULE7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAM2_MEM_RULE0_SPEC, u8, RULE7_A, 2, O>;
impl<'a, const O: u8> RULE7_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RULE7_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&self) -> RULE0_R {
        RULE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&self) -> RULE1_R {
        RULE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&self) -> RULE2_R {
        RULE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&self) -> RULE3_R {
        RULE3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&self) -> RULE4_R {
        RULE4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&self) -> RULE5_R {
        RULE5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule6(&self) -> RULE6_R {
        RULE6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule7(&self) -> RULE7_R {
        RULE7_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule0(&mut self) -> RULE0_W<0> {
        RULE0_W::new(self)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule1(&mut self) -> RULE1_W<4> {
        RULE1_W::new(self)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule2(&mut self) -> RULE2_W<8> {
        RULE2_W::new(self)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule3(&mut self) -> RULE3_W<12> {
        RULE3_W::new(self)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule4(&mut self) -> RULE4_W<16> {
        RULE4_W::new(self)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule5(&mut self) -> RULE5_W<20> {
        RULE5_W::new(self)
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule6(&mut self) -> RULE6_W<24> {
        RULE6_W::new(self)
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule7(&mut self) -> RULE7_W<28> {
        RULE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for RAM2 slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ram2_mem_rule0](index.html) module"]
pub struct SEC_CTRL_RAM2_MEM_RULE0_SPEC;
impl crate::RegisterSpec for SEC_CTRL_RAM2_MEM_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ram2_mem_rule0::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_RAM2_MEM_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ram2_mem_rule0::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_RAM2_MEM_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM2_MEM_RULE0 to value 0"]
impl crate::Resettable for SEC_CTRL_RAM2_MEM_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
