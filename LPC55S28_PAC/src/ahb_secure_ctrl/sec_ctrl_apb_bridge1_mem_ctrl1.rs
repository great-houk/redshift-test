#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL1` reader"]
pub struct R(crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL1` writer"]
pub struct W(crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>;
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
impl From<crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIMER2_RULE` reader - Standard counter/Timer 2"]
pub type CTIMER2_RULE_R = crate::FieldReader<u8, CTIMER2_RULE_A>;
#[doc = "Standard counter/Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTIMER2_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER2_RULE_A) -> Self {
        variant as _
    }
}
impl CTIMER2_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER2_RULE_A {
        match self.bits {
            0 => CTIMER2_RULE_A::ENUM_NS_NP,
            1 => CTIMER2_RULE_A::ENUM_NS_P,
            2 => CTIMER2_RULE_A::ENUM_S_NP,
            3 => CTIMER2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER2_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CTIMER2_RULE` writer - Standard counter/Timer 2"]
pub type CTIMER2_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC, u8, CTIMER2_RULE_A, 2, O>;
impl<'a, const O: u8> CTIMER2_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `CTIMER3_RULE` reader - Standard counter/Timer 3"]
pub type CTIMER3_RULE_R = crate::FieldReader<u8, CTIMER3_RULE_A>;
#[doc = "Standard counter/Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTIMER3_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER3_RULE_A) -> Self {
        variant as _
    }
}
impl CTIMER3_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER3_RULE_A {
        match self.bits {
            0 => CTIMER3_RULE_A::ENUM_NS_NP,
            1 => CTIMER3_RULE_A::ENUM_NS_P,
            2 => CTIMER3_RULE_A::ENUM_S_NP,
            3 => CTIMER3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER3_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CTIMER3_RULE` writer - Standard counter/Timer 3"]
pub type CTIMER3_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC, u8, CTIMER3_RULE_A, 2, O>;
impl<'a, const O: u8> CTIMER3_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `CTIMER4_RULE` reader - Standard counter/Timer 4"]
pub type CTIMER4_RULE_R = crate::FieldReader<u8, CTIMER4_RULE_A>;
#[doc = "Standard counter/Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTIMER4_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CTIMER4_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTIMER4_RULE_A) -> Self {
        variant as _
    }
}
impl CTIMER4_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER4_RULE_A {
        match self.bits {
            0 => CTIMER4_RULE_A::ENUM_NS_NP,
            1 => CTIMER4_RULE_A::ENUM_NS_P,
            2 => CTIMER4_RULE_A::ENUM_S_NP,
            3 => CTIMER4_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER4_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CTIMER4_RULE` writer - Standard counter/Timer 4"]
pub type CTIMER4_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC, u8, CTIMER4_RULE_A, 2, O>;
impl<'a, const O: u8> CTIMER4_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `RTC_RULE` reader - Real Time Counter"]
pub type RTC_RULE_R = crate::FieldReader<u8, RTC_RULE_A>;
#[doc = "Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RTC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_RULE_A) -> Self {
        variant as _
    }
}
impl RTC_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_RULE_A {
        match self.bits {
            0 => RTC_RULE_A::ENUM_NS_NP,
            1 => RTC_RULE_A::ENUM_NS_P,
            2 => RTC_RULE_A::ENUM_S_NP,
            3 => RTC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RTC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RTC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RTC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RTC_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `RTC_RULE` writer - Real Time Counter"]
pub type RTC_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC, u8, RTC_RULE_A, 2, O>;
impl<'a, const O: u8> RTC_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RTC_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `OSEVENT_RULE` reader - OS Event Timer"]
pub type OSEVENT_RULE_R = crate::FieldReader<u8, OSEVENT_RULE_A>;
#[doc = "OS Event Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEVENT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<OSEVENT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEVENT_RULE_A) -> Self {
        variant as _
    }
}
impl OSEVENT_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEVENT_RULE_A {
        match self.bits {
            0 => OSEVENT_RULE_A::ENUM_NS_NP,
            1 => OSEVENT_RULE_A::ENUM_NS_P,
            2 => OSEVENT_RULE_A::ENUM_S_NP,
            3 => OSEVENT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == OSEVENT_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `OSEVENT_RULE` writer - OS Event Timer"]
pub type OSEVENT_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC, u8, OSEVENT_RULE_A, 2, O>;
impl<'a, const O: u8> OSEVENT_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    pub fn ctimer2_rule(&self) -> CTIMER2_RULE_R {
        CTIMER2_RULE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    pub fn ctimer3_rule(&self) -> CTIMER3_RULE_R {
        CTIMER3_RULE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    pub fn ctimer4_rule(&self) -> CTIMER4_RULE_R {
        CTIMER4_RULE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    pub fn rtc_rule(&self) -> RTC_RULE_R {
        RTC_RULE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    pub fn osevent_rule(&self) -> OSEVENT_RULE_R {
        OSEVENT_RULE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctimer2_rule(&mut self) -> CTIMER2_RULE_W<0> {
        CTIMER2_RULE_W::new(self)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctimer3_rule(&mut self) -> CTIMER3_RULE_W<4> {
        CTIMER3_RULE_W::new(self)
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctimer4_rule(&mut self) -> CTIMER4_RULE_W<8> {
        CTIMER4_RULE_W::new(self)
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_rule(&mut self) -> RTC_RULE_W<16> {
        RTC_RULE_W::new(self)
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_rule(&mut self) -> OSEVENT_RULE_W<20> {
        OSEVENT_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_apb_bridge1_mem_ctrl1](index.html) module"]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_apb_bridge1_mem_ctrl1::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_apb_bridge1_mem_ctrl1::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 to value 0"]
impl crate::Resettable for SEC_CTRL_APB_BRIDGE1_MEM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
