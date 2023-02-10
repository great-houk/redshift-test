#[doc = "Register `CLOCKGENUPDATELOCKOUT` reader"]
pub struct R(crate::R<CLOCKGENUPDATELOCKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKGENUPDATELOCKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKGENUPDATELOCKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKGENUPDATELOCKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCKGENUPDATELOCKOUT` writer"]
pub struct W(crate::W<CLOCKGENUPDATELOCKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKGENUPDATELOCKOUT_SPEC>;
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
impl From<crate::W<CLOCKGENUPDATELOCKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKGENUPDATELOCKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` reader - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
pub type CLOCKGENUPDATELOCKOUT_R = crate::FieldReader<u32, CLOCKGENUPDATELOCKOUT_A>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLOCKGENUPDATELOCKOUT_A {
    #[doc = "0: all hardware clock configruration are freeze."]
    FREEZE = 0,
    #[doc = "1: update all clock configuration."]
    ENABLE = 1,
}
impl From<CLOCKGENUPDATELOCKOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKGENUPDATELOCKOUT_A) -> Self {
        variant as _
    }
}
impl CLOCKGENUPDATELOCKOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCKGENUPDATELOCKOUT_A> {
        match self.bits {
            0 => Some(CLOCKGENUPDATELOCKOUT_A::FREEZE),
            1 => Some(CLOCKGENUPDATELOCKOUT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUT_A::FREEZE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUT_A::ENABLE
    }
}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` writer - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
pub type CLOCKGENUPDATELOCKOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCKGENUPDATELOCKOUT_SPEC, u32, CLOCKGENUPDATELOCKOUT_A, 32, O>;
impl<'a, const O: u8> CLOCKGENUPDATELOCKOUT_W<'a, O> {
    #[doc = "all hardware clock configruration are freeze."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::FREEZE)
    }
    #[doc = "update all clock configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&self) -> CLOCKGENUPDATELOCKOUT_R {
        CLOCKGENUPDATELOCKOUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    #[must_use]
    pub fn clockgenupdatelockout(&mut self) -> CLOCKGENUPDATELOCKOUT_W<0> {
        CLOCKGENUPDATELOCKOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockgenupdatelockout](index.html) module"]
pub struct CLOCKGENUPDATELOCKOUT_SPEC;
impl crate::RegisterSpec for CLOCKGENUPDATELOCKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockgenupdatelockout::R](R) reader structure"]
impl crate::Readable for CLOCKGENUPDATELOCKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clockgenupdatelockout::W](W) writer structure"]
impl crate::Writable for CLOCKGENUPDATELOCKOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCKGENUPDATELOCKOUT to value 0"]
impl crate::Resettable for CLOCKGENUPDATELOCKOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
