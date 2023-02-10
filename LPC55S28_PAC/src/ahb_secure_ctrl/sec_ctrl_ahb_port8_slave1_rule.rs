#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM2_RULE` reader - Flexcomm interface 2"]
pub type FLEXCOMM2_RULE_R = crate::FieldReader<u8, FLEXCOMM2_RULE_A>;
#[doc = "Flexcomm interface 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM2_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM2_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM2_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_RULE_A {
        match self.bits {
            0 => FLEXCOMM2_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM2_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM2_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM2_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM2_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM2_RULE` writer - Flexcomm interface 2"]
pub type FLEXCOMM2_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC,
    u8,
    FLEXCOMM2_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM2_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM3_RULE` reader - Flexcomm interface 3"]
pub type FLEXCOMM3_RULE_R = crate::FieldReader<u8, FLEXCOMM3_RULE_A>;
#[doc = "Flexcomm interface 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM3_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM3_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_RULE_A {
        match self.bits {
            0 => FLEXCOMM3_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM3_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM3_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM3_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM3_RULE` writer - Flexcomm interface 3"]
pub type FLEXCOMM3_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC,
    u8,
    FLEXCOMM3_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM3_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM4_RULE` reader - Flexcomm interface 4"]
pub type FLEXCOMM4_RULE_R = crate::FieldReader<u8, FLEXCOMM4_RULE_A>;
#[doc = "Flexcomm interface 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM4_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM4_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM4_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_RULE_A {
        match self.bits {
            0 => FLEXCOMM4_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM4_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM4_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM4_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM4_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM4_RULE` writer - Flexcomm interface 4"]
pub type FLEXCOMM4_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC,
    u8,
    FLEXCOMM4_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM4_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `MAILBOX_RULE` reader - Inter CPU communication Mailbox"]
pub type MAILBOX_RULE_R = crate::FieldReader<u8, MAILBOX_RULE_A>;
#[doc = "Inter CPU communication Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAILBOX_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<MAILBOX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAILBOX_RULE_A) -> Self {
        variant as _
    }
}
impl MAILBOX_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_RULE_A {
        match self.bits {
            0 => MAILBOX_RULE_A::ENUM_NS_NP,
            1 => MAILBOX_RULE_A::ENUM_NS_P,
            2 => MAILBOX_RULE_A::ENUM_S_NP,
            3 => MAILBOX_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MAILBOX_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `MAILBOX_RULE` writer - Inter CPU communication Mailbox"]
pub type MAILBOX_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC, u8, MAILBOX_RULE_A, 2, O>;
impl<'a, const O: u8> MAILBOX_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MAILBOX_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `GPIO0_RULE` reader - High Speed GPIO"]
pub type GPIO0_RULE_R = crate::FieldReader<u8, GPIO0_RULE_A>;
#[doc = "High Speed GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<GPIO0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0_RULE_A) -> Self {
        variant as _
    }
}
impl GPIO0_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_RULE_A {
        match self.bits {
            0 => GPIO0_RULE_A::ENUM_NS_NP,
            1 => GPIO0_RULE_A::ENUM_NS_P,
            2 => GPIO0_RULE_A::ENUM_S_NP,
            3 => GPIO0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GPIO0_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `GPIO0_RULE` writer - High Speed GPIO"]
pub type GPIO0_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC, u8, GPIO0_RULE_A, 2, O>;
impl<'a, const O: u8> GPIO0_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GPIO0_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    pub fn flexcomm2_rule(&self) -> FLEXCOMM2_RULE_R {
        FLEXCOMM2_RULE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    pub fn flexcomm3_rule(&self) -> FLEXCOMM3_RULE_R {
        FLEXCOMM3_RULE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    pub fn flexcomm4_rule(&self) -> FLEXCOMM4_RULE_R {
        FLEXCOMM4_RULE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    pub fn mailbox_rule(&self) -> MAILBOX_RULE_R {
        MAILBOX_RULE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    pub fn gpio0_rule(&self) -> GPIO0_RULE_R {
        GPIO0_RULE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rule(&mut self) -> FLEXCOMM2_RULE_W<0> {
        FLEXCOMM2_RULE_W::new(self)
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rule(&mut self) -> FLEXCOMM3_RULE_W<4> {
        FLEXCOMM3_RULE_W::new(self)
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_rule(&mut self) -> FLEXCOMM4_RULE_W<8> {
        FLEXCOMM4_RULE_W::new(self)
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox_rule(&mut self) -> MAILBOX_RULE_W<12> {
        MAILBOX_RULE_W::new(self)
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0_rule(&mut self) -> GPIO0_RULE_W<16> {
        GPIO0_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave1_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave1_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave1_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE1_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT8_SLAVE1_RULE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
