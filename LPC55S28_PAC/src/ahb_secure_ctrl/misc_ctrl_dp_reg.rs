#[doc = "Register `MISC_CTRL_DP_REG` reader"]
pub struct R(crate::R<MISC_CTRL_DP_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CTRL_DP_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CTRL_DP_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CTRL_DP_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CTRL_DP_REG` writer"]
pub struct W(crate::W<MISC_CTRL_DP_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CTRL_DP_REG_SPEC>;
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
impl From<crate::W<MISC_CTRL_DP_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CTRL_DP_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_LOCK` reader - Write lock."]
pub type WRITE_LOCK_R = crate::FieldReader<u8, WRITE_LOCK_A>;
#[doc = "Write lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITE_LOCK_A {
    #[doc = "1: Restricted mode."]
    RESTRICTED = 1,
    #[doc = "2: Secure control registers can be written."]
    ACCESSIBLE = 2,
}
impl From<WRITE_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITE_LOCK_A) -> Self {
        variant as _
    }
}
impl WRITE_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRITE_LOCK_A> {
        match self.bits {
            1 => Some(WRITE_LOCK_A::RESTRICTED),
            2 => Some(WRITE_LOCK_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == WRITE_LOCK_A::RESTRICTED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == WRITE_LOCK_A::ACCESSIBLE
    }
}
#[doc = "Field `WRITE_LOCK` writer - Write lock."]
pub type WRITE_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, WRITE_LOCK_A, 2, O>;
impl<'a, const O: u8> WRITE_LOCK_W<'a, O> {
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::RESTRICTED)
    }
    #[doc = "Secure control registers can be written."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::ACCESSIBLE)
    }
}
#[doc = "Field `ENABLE_SECURE_CHECKING` reader - Enable secure check for AHB matrix."]
pub type ENABLE_SECURE_CHECKING_R = crate::FieldReader<u8, ENABLE_SECURE_CHECKING_A>;
#[doc = "Enable secure check for AHB matrix.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_SECURE_CHECKING_A {
    #[doc = "1: Restricted mode."]
    ENABLE = 1,
    #[doc = "2: Disable check."]
    DISABLE = 2,
}
impl From<ENABLE_SECURE_CHECKING_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_SECURE_CHECKING_A) -> Self {
        variant as _
    }
}
impl ENABLE_SECURE_CHECKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_SECURE_CHECKING_A> {
        match self.bits {
            1 => Some(ENABLE_SECURE_CHECKING_A::ENABLE),
            2 => Some(ENABLE_SECURE_CHECKING_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::DISABLE
    }
}
#[doc = "Field `ENABLE_SECURE_CHECKING` writer - Enable secure check for AHB matrix."]
pub type ENABLE_SECURE_CHECKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, ENABLE_SECURE_CHECKING_A, 2, O>;
impl<'a, const O: u8> ENABLE_SECURE_CHECKING_W<'a, O> {
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::DISABLE)
    }
}
#[doc = "Field `ENABLE_S_PRIV_CHECK` reader - Enable secure privilege check for AHB matrix."]
pub type ENABLE_S_PRIV_CHECK_R = crate::FieldReader<u8, ENABLE_S_PRIV_CHECK_A>;
#[doc = "Enable secure privilege check for AHB matrix.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_S_PRIV_CHECK_A {
    #[doc = "1: Restricted mode."]
    ENABLE = 1,
    #[doc = "2: Disable check."]
    DISABLE = 2,
}
impl From<ENABLE_S_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_S_PRIV_CHECK_A) -> Self {
        variant as _
    }
}
impl ENABLE_S_PRIV_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_S_PRIV_CHECK_A> {
        match self.bits {
            1 => Some(ENABLE_S_PRIV_CHECK_A::ENABLE),
            2 => Some(ENABLE_S_PRIV_CHECK_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::DISABLE
    }
}
#[doc = "Field `ENABLE_S_PRIV_CHECK` writer - Enable secure privilege check for AHB matrix."]
pub type ENABLE_S_PRIV_CHECK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, ENABLE_S_PRIV_CHECK_A, 2, O>;
impl<'a, const O: u8> ENABLE_S_PRIV_CHECK_W<'a, O> {
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::DISABLE)
    }
}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` reader - Enable non-secure privilege check for AHB matrix."]
pub type ENABLE_NS_PRIV_CHECK_R = crate::FieldReader<u8, ENABLE_NS_PRIV_CHECK_A>;
#[doc = "Enable non-secure privilege check for AHB matrix.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_NS_PRIV_CHECK_A {
    #[doc = "1: Restricted mode."]
    ENABLE = 1,
    #[doc = "2: Disable check."]
    DISABLE = 2,
}
impl From<ENABLE_NS_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_NS_PRIV_CHECK_A) -> Self {
        variant as _
    }
}
impl ENABLE_NS_PRIV_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_NS_PRIV_CHECK_A> {
        match self.bits {
            1 => Some(ENABLE_NS_PRIV_CHECK_A::ENABLE),
            2 => Some(ENABLE_NS_PRIV_CHECK_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::DISABLE
    }
}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` writer - Enable non-secure privilege check for AHB matrix."]
pub type ENABLE_NS_PRIV_CHECK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, ENABLE_NS_PRIV_CHECK_A, 2, O>;
impl<'a, const O: u8> ENABLE_NS_PRIV_CHECK_W<'a, O> {
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::DISABLE)
    }
}
#[doc = "Field `DISABLE_VIOLATION_ABORT` reader - Disable secure violation abort."]
pub type DISABLE_VIOLATION_ABORT_R = crate::FieldReader<u8, DISABLE_VIOLATION_ABORT_A>;
#[doc = "Disable secure violation abort.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_VIOLATION_ABORT_A {
    #[doc = "1: Disable abort fort secure checker."]
    DISABLE = 1,
    #[doc = "2: Enable abort fort secure checker."]
    ENABLE = 2,
}
impl From<DISABLE_VIOLATION_ABORT_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_VIOLATION_ABORT_A) -> Self {
        variant as _
    }
}
impl DISABLE_VIOLATION_ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_VIOLATION_ABORT_A> {
        match self.bits {
            1 => Some(DISABLE_VIOLATION_ABORT_A::DISABLE),
            2 => Some(DISABLE_VIOLATION_ABORT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::ENABLE
    }
}
#[doc = "Field `DISABLE_VIOLATION_ABORT` writer - Disable secure violation abort."]
pub type DISABLE_VIOLATION_ABORT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, DISABLE_VIOLATION_ABORT_A, 2, O>;
impl<'a, const O: u8> DISABLE_VIOLATION_ABORT_W<'a, O> {
    #[doc = "Disable abort fort secure checker."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::DISABLE)
    }
    #[doc = "Enable abort fort secure checker."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::ENABLE)
    }
}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` reader - Disable simple master strict mode."]
pub type DISABLE_SIMPLE_MASTER_STRICT_MODE_R =
    crate::FieldReader<u8, DISABLE_SIMPLE_MASTER_STRICT_MODE_A>;
#[doc = "Disable simple master strict mode.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_SIMPLE_MASTER_STRICT_MODE_A {
    #[doc = "1: Simple master in tier mode."]
    TIER_MODE = 1,
    #[doc = "2: Simple master in strict mode."]
    STRICT_MODE = 2,
}
impl From<DISABLE_SIMPLE_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SIMPLE_MASTER_STRICT_MODE_A) -> Self {
        variant as _
    }
}
impl DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_SIMPLE_MASTER_STRICT_MODE_A> {
        match self.bits {
            1 => Some(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE),
            2 => Some(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE
    }
}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` writer - Disable simple master strict mode."]
pub type DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    MISC_CTRL_DP_REG_SPEC,
    u8,
    DISABLE_SIMPLE_MASTER_STRICT_MODE_A,
    2,
    O,
>;
impl<'a, const O: u8> DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a, O> {
    #[doc = "Simple master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE)
    }
    #[doc = "Simple master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE)
    }
}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` reader - Disable smart master strict mode."]
pub type DISABLE_SMART_MASTER_STRICT_MODE_R =
    crate::FieldReader<u8, DISABLE_SMART_MASTER_STRICT_MODE_A>;
#[doc = "Disable smart master strict mode.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISABLE_SMART_MASTER_STRICT_MODE_A {
    #[doc = "1: Smart master in tier mode."]
    TIER_MODE = 1,
    #[doc = "2: Smart master in strict mode."]
    STRICT_MODE = 2,
}
impl From<DISABLE_SMART_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SMART_MASTER_STRICT_MODE_A) -> Self {
        variant as _
    }
}
impl DISABLE_SMART_MASTER_STRICT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISABLE_SMART_MASTER_STRICT_MODE_A> {
        match self.bits {
            1 => Some(DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE),
            2 => Some(DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE
    }
}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` writer - Disable smart master strict mode."]
pub type DISABLE_SMART_MASTER_STRICT_MODE_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    MISC_CTRL_DP_REG_SPEC,
    u8,
    DISABLE_SMART_MASTER_STRICT_MODE_A,
    2,
    O,
>;
impl<'a, const O: u8> DISABLE_SMART_MASTER_STRICT_MODE_W<'a, O> {
    #[doc = "Smart master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE)
    }
    #[doc = "Smart master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE)
    }
}
#[doc = "Field `IDAU_ALL_NS` reader - Disable IDAU."]
pub type IDAU_ALL_NS_R = crate::FieldReader<u8, IDAU_ALL_NS_A>;
#[doc = "Disable IDAU.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDAU_ALL_NS_A {
    #[doc = "1: IDAU is disable."]
    DISABLE = 1,
    #[doc = "2: IDAU is enabled."]
    ENABLE = 2,
}
impl From<IDAU_ALL_NS_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAU_ALL_NS_A) -> Self {
        variant as _
    }
}
impl IDAU_ALL_NS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDAU_ALL_NS_A> {
        match self.bits {
            1 => Some(IDAU_ALL_NS_A::DISABLE),
            2 => Some(IDAU_ALL_NS_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDAU_ALL_NS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IDAU_ALL_NS_A::ENABLE
    }
}
#[doc = "Field `IDAU_ALL_NS` writer - Disable IDAU."]
pub type IDAU_ALL_NS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_CTRL_DP_REG_SPEC, u8, IDAU_ALL_NS_A, 2, O>;
impl<'a, const O: u8> IDAU_ALL_NS_W<'a, O> {
    #[doc = "IDAU is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::DISABLE)
    }
    #[doc = "IDAU is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    pub fn write_lock(&self) -> WRITE_LOCK_R {
        WRITE_LOCK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enable secure check for AHB matrix."]
    #[inline(always)]
    pub fn enable_secure_checking(&self) -> ENABLE_SECURE_CHECKING_R {
        ENABLE_SECURE_CHECKING_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_s_priv_check(&self) -> ENABLE_S_PRIV_CHECK_R {
        ENABLE_S_PRIV_CHECK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_ns_priv_check(&self) -> ENABLE_NS_PRIV_CHECK_R {
        ENABLE_NS_PRIV_CHECK_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    pub fn disable_violation_abort(&self) -> DISABLE_VIOLATION_ABORT_R {
        DISABLE_VIOLATION_ABORT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    pub fn disable_simple_master_strict_mode(&self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    pub fn disable_smart_master_strict_mode(&self) -> DISABLE_SMART_MASTER_STRICT_MODE_R {
        DISABLE_SMART_MASTER_STRICT_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    pub fn idau_all_ns(&self) -> IDAU_ALL_NS_R {
        IDAU_ALL_NS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    #[must_use]
    pub fn write_lock(&mut self) -> WRITE_LOCK_W<0> {
        WRITE_LOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Enable secure check for AHB matrix."]
    #[inline(always)]
    #[must_use]
    pub fn enable_secure_checking(&mut self) -> ENABLE_SECURE_CHECKING_W<2> {
        ENABLE_SECURE_CHECKING_W::new(self)
    }
    #[doc = "Bits 4:5 - Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    #[must_use]
    pub fn enable_s_priv_check(&mut self) -> ENABLE_S_PRIV_CHECK_W<4> {
        ENABLE_S_PRIV_CHECK_W::new(self)
    }
    #[doc = "Bits 6:7 - Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    #[must_use]
    pub fn enable_ns_priv_check(&mut self) -> ENABLE_NS_PRIV_CHECK_W<6> {
        ENABLE_NS_PRIV_CHECK_W::new(self)
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    #[must_use]
    pub fn disable_violation_abort(&mut self) -> DISABLE_VIOLATION_ABORT_W<8> {
        DISABLE_VIOLATION_ABORT_W::new(self)
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    #[must_use]
    pub fn disable_simple_master_strict_mode(&mut self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_W<10> {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    #[must_use]
    pub fn disable_smart_master_strict_mode(&mut self) -> DISABLE_SMART_MASTER_STRICT_MODE_W<12> {
        DISABLE_SMART_MASTER_STRICT_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    #[must_use]
    pub fn idau_all_ns(&mut self) -> IDAU_ALL_NS_W<14> {
        IDAU_ALL_NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "secure control duplicate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctrl_dp_reg](index.html) module"]
pub struct MISC_CTRL_DP_REG_SPEC;
impl crate::RegisterSpec for MISC_CTRL_DP_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_ctrl_dp_reg::R](R) reader structure"]
impl crate::Readable for MISC_CTRL_DP_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_ctrl_dp_reg::W](W) writer structure"]
impl crate::Writable for MISC_CTRL_DP_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_CTRL_DP_REG to value 0xaaaa"]
impl crate::Resettable for MISC_CTRL_DP_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0xaaaa;
}
