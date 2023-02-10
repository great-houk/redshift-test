#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` reader"]
pub struct R(crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` writer"]
pub struct W(crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>;
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
impl From<crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIMER0_RULE` reader - Standard counter/Timer 0"]
pub type CTIMER0_RULE_R = crate::FieldReader<u8, CTIMER0_RULE_A>;
#[doc = "Standard counter/Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTIMER0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER0_RULE_A) -> Self {
        variant as _
    }
}
impl CTIMER0_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER0_RULE_A {
        match self.bits {
            0 => CTIMER0_RULE_A::ENUM_NS_NP,
            1 => CTIMER0_RULE_A::ENUM_NS_P,
            2 => CTIMER0_RULE_A::ENUM_S_NP,
            3 => CTIMER0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER0_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CTIMER0_RULE` writer - Standard counter/Timer 0"]
pub type CTIMER0_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC, u8, CTIMER0_RULE_A, 2, O>;
impl<'a, const O: u8> CTIMER0_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER0_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `CTIMER1_RULE` reader - Standard counter/Timer 1"]
pub type CTIMER1_RULE_R = crate::FieldReader<u8, CTIMER1_RULE_A>;
#[doc = "Standard counter/Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTIMER1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER1_RULE_A) -> Self {
        variant as _
    }
}
impl CTIMER1_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER1_RULE_A {
        match self.bits {
            0 => CTIMER1_RULE_A::ENUM_NS_NP,
            1 => CTIMER1_RULE_A::ENUM_NS_P,
            2 => CTIMER1_RULE_A::ENUM_S_NP,
            3 => CTIMER1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER1_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CTIMER1_RULE` writer - Standard counter/Timer 1"]
pub type CTIMER1_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC, u8, CTIMER1_RULE_A, 2, O>;
impl<'a, const O: u8> CTIMER1_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER1_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `WWDT_RULE` reader - Windiwed wtachdog Timer"]
pub type WWDT_RULE_R = crate::FieldReader<u8, WWDT_RULE_A>;
#[doc = "Windiwed wtachdog Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WWDT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<WWDT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDT_RULE_A) -> Self {
        variant as _
    }
}
impl WWDT_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_RULE_A {
        match self.bits {
            0 => WWDT_RULE_A::ENUM_NS_NP,
            1 => WWDT_RULE_A::ENUM_NS_P,
            2 => WWDT_RULE_A::ENUM_S_NP,
            3 => WWDT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == WWDT_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `WWDT_RULE` writer - Windiwed wtachdog Timer"]
pub type WWDT_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC, u8, WWDT_RULE_A, 2, O>;
impl<'a, const O: u8> WWDT_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(WWDT_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `MRT_RULE` reader - Multi-rate Timer"]
pub type MRT_RULE_R = crate::FieldReader<u8, MRT_RULE_A>;
#[doc = "Multi-rate Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MRT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<MRT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: MRT_RULE_A) -> Self {
        variant as _
    }
}
impl MRT_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RULE_A {
        match self.bits {
            0 => MRT_RULE_A::ENUM_NS_NP,
            1 => MRT_RULE_A::ENUM_NS_P,
            2 => MRT_RULE_A::ENUM_S_NP,
            3 => MRT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MRT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MRT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MRT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MRT_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `MRT_RULE` writer - Multi-rate Timer"]
pub type MRT_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC, u8, MRT_RULE_A, 2, O>;
impl<'a, const O: u8> MRT_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MRT_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `UTICK_RULE` reader - Micro-Timer"]
pub type UTICK_RULE_R = crate::FieldReader<u8, UTICK_RULE_A>;
#[doc = "Micro-Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTICK_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<UTICK_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: UTICK_RULE_A) -> Self {
        variant as _
    }
}
impl UTICK_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_RULE_A {
        match self.bits {
            0 => UTICK_RULE_A::ENUM_NS_NP,
            1 => UTICK_RULE_A::ENUM_NS_P,
            2 => UTICK_RULE_A::ENUM_S_NP,
            3 => UTICK_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UTICK_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `UTICK_RULE` writer - Micro-Timer"]
pub type UTICK_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC, u8, UTICK_RULE_A, 2, O>;
impl<'a, const O: u8> UTICK_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(UTICK_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    pub fn ctimer0_rule(&self) -> CTIMER0_RULE_R {
        CTIMER0_RULE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    pub fn ctimer1_rule(&self) -> CTIMER1_RULE_R {
        CTIMER1_RULE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    pub fn wwdt_rule(&self) -> WWDT_RULE_R {
        WWDT_RULE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    pub fn mrt_rule(&self) -> MRT_RULE_R {
        MRT_RULE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    pub fn utick_rule(&self) -> UTICK_RULE_R {
        UTICK_RULE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctimer0_rule(&mut self) -> CTIMER0_RULE_W<0> {
        CTIMER0_RULE_W::new(self)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctimer1_rule(&mut self) -> CTIMER1_RULE_W<4> {
        CTIMER1_RULE_W::new(self)
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_rule(&mut self) -> WWDT_RULE_W<16> {
        WWDT_RULE_W::new(self)
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    #[must_use]
    pub fn mrt_rule(&mut self) -> MRT_RULE_W<20> {
        MRT_RULE_W::new(self)
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    #[must_use]
    pub fn utick_rule(&mut self) -> UTICK_RULE_W<24> {
        UTICK_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge0_mem_ctrl1](index.html) module"]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_apb_bridge0_mem_ctrl1::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge0_mem_ctrl1::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 to value 0"]
impl crate::Resettable for SEC_CTRL_APB_BRIDGE0_MEM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
