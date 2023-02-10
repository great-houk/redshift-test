#[doc = "Register `PAUSE` reader"]
pub struct R(crate::R<PAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAUSE` writer"]
pub struct W(crate::W<PAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUSE_SPEC>;
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
impl From<crate::W<PAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAUSEDLY` reader - Pause Delay"]
pub type PAUSEDLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAUSEDLY` writer - Pause Delay"]
pub type PAUSEDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAUSE_SPEC, u16, u16, 9, O>;
#[doc = "Field `PAUSEEN` reader - PAUSE Option Enable"]
pub type PAUSEEN_R = crate::BitReader<PAUSEEN_A>;
#[doc = "PAUSE Option Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAUSEEN_A {
    #[doc = "0: Pause operation disabled"]
    PAUSEEN_0 = 0,
    #[doc = "1: Pause operation enabled"]
    PAUSEEN_1 = 1,
}
impl From<PAUSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAUSEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PAUSEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUSEEN_A {
        match self.bits {
            false => PAUSEEN_A::PAUSEEN_0,
            true => PAUSEEN_A::PAUSEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_0`"]
    #[inline(always)]
    pub fn is_pauseen_0(&self) -> bool {
        *self == PAUSEEN_A::PAUSEEN_0
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_1`"]
    #[inline(always)]
    pub fn is_pauseen_1(&self) -> bool {
        *self == PAUSEEN_A::PAUSEEN_1
    }
}
#[doc = "Field `PAUSEEN` writer - PAUSE Option Enable"]
pub type PAUSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAUSE_SPEC, PAUSEEN_A, O>;
impl<'a, const O: u8> PAUSEEN_W<'a, O> {
    #[doc = "Pause operation disabled"]
    #[inline(always)]
    pub fn pauseen_0(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_0)
    }
    #[doc = "Pause operation enabled"]
    #[inline(always)]
    pub fn pauseen_1(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&self) -> PAUSEDLY_R {
        PAUSEDLY_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PAUSEEN_R {
        PAUSEEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pausedly(&mut self) -> PAUSEDLY_W<0> {
        PAUSEDLY_W::new(self)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pauseen(&mut self) -> PAUSEEN_W<31> {
        PAUSEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause](index.html) module"]
pub struct PAUSE_SPEC;
impl crate::RegisterSpec for PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause::R](R) reader structure"]
impl crate::Readable for PAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pause::W](W) writer structure"]
impl crate::Writable for PAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
