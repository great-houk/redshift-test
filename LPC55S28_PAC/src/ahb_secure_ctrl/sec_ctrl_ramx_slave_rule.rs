#[doc = "Register `SEC_CTRL_RAMX_SLAVE_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_RAMX_SLAVE_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_RAMX_SLAVE_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMX_RULE` reader - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
pub type RAMX_RULE_R = crate::FieldReader<u8, RAMX_RULE_A>;
#[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMX_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<RAMX_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMX_RULE_A) -> Self {
        variant as _
    }
}
impl RAMX_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMX_RULE_A {
        match self.bits {
            0 => RAMX_RULE_A::ENUM_NS_NP,
            1 => RAMX_RULE_A::ENUM_NS_P,
            2 => RAMX_RULE_A::ENUM_S_NP,
            3 => RAMX_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RAMX_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RAMX_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RAMX_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RAMX_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `RAMX_RULE` writer - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
pub type RAMX_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_RAMX_SLAVE_RULE_SPEC, u8, RAMX_RULE_A, 2, O>;
impl<'a, const O: u8> RAMX_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RAMX_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RAMX_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RAMX_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RAMX_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[inline(always)]
    pub fn ramx_rule(&self) -> RAMX_RULE_R {
        RAMX_RULE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[inline(always)]
    #[must_use]
    pub fn ramx_rule(&mut self) -> RAMX_RULE_W<0> {
        RAMX_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for RAMX slaves.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ramx_slave_rule](index.html) module"]
pub struct SEC_CTRL_RAMX_SLAVE_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_RAMX_SLAVE_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ramx_slave_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_RAMX_SLAVE_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ramx_slave_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_RAMX_SLAVE_RULE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_RAMX_SLAVE_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_RAMX_SLAVE_RULE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
