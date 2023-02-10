#[doc = "Register `FROHFDIV` reader"]
pub struct R(crate::R<FROHFDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROHFDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROHFDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROHFDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROHFDIV` writer"]
pub struct W(crate::W<FROHFDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROHFDIV_SPEC>;
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
impl From<crate::W<FROHFDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FROHFDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Clock divider value."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FROHFDIV_SPEC, u8, u8, 8, O>;
#[doc = "Resets the divider counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_AW {
    #[doc = "0: Divider is not reset."]
    RELEASED = 0,
    #[doc = "1: Divider is reset."]
    ASSERTED = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Resets the divider counter."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROHFDIV_SPEC, RESET_AW, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "Divider is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RESET_AW::RELEASED)
    }
    #[doc = "Divider is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RESET_AW::ASSERTED)
    }
}
#[doc = "Field `HALT` reader - Halts the divider counter."]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halts the divider counter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: Divider clock is running."]
    RUN = 0,
    #[doc = "1: Divider clock is stoped."]
    HALT = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::RUN,
            true => HALT_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == HALT_A::RUN
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == HALT_A::HALT
    }
}
#[doc = "Field `HALT` writer - Halts the divider counter."]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FROHFDIV_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Divider clock is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
    }
    #[doc = "Divider clock is stoped."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(HALT_A::HALT)
    }
}
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub type REQFLAG_R = crate::BitReader<REQFLAG_A>;
#[doc = "Divider status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQFLAG_A {
    #[doc = "0: Divider clock is stable."]
    STABLE = 0,
    #[doc = "1: Clock frequency is not stable."]
    ONGOING = 1,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl REQFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQFLAG_A {
        match self.bits {
            false => REQFLAG_A::STABLE,
            true => REQFLAG_A::ONGOING,
        }
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == REQFLAG_A::STABLE
    }
    #[doc = "Checks if the value of the field is `ONGOING`"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == REQFLAG_A::ONGOING
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<29> {
        RESET_W::new(self)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<30> {
        HALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO_HF (96MHz) clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frohfdiv](index.html) module"]
pub struct FROHFDIV_SPEC;
impl crate::RegisterSpec for FROHFDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frohfdiv::R](R) reader structure"]
impl crate::Readable for FROHFDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frohfdiv::W](W) writer structure"]
impl crate::Writable for FROHFDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FROHFDIV to value 0x4000_0000"]
impl crate::Resettable for FROHFDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
