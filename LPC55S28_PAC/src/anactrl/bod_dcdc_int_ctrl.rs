#[doc = "Register `BOD_DCDC_INT_CTRL` reader"]
pub struct R(crate::R<BOD_DCDC_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_DCDC_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_DCDC_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_DCDC_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD_DCDC_INT_CTRL` writer"]
pub struct W(crate::W<BOD_DCDC_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_DCDC_INT_CTRL_SPEC>;
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
impl From<crate::W<BOD_DCDC_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_DCDC_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` reader - BOD VBAT interrupt control."]
pub type BODVBAT_INT_ENABLE_R = crate::BitReader<BODVBAT_INT_ENABLE_A>;
#[doc = "BOD VBAT interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODVBAT_INT_ENABLE_A {
    #[doc = "0: BOD VBAT interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: BOD VBAT interrupt is enabled."]
    ENABLE = 1,
}
impl From<BODVBAT_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BODVBAT_INT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_INT_ENABLE_A {
        match self.bits {
            false => BODVBAT_INT_ENABLE_A::DISABLE,
            true => BODVBAT_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODVBAT_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODVBAT_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` writer - BOD VBAT interrupt control."]
pub type BODVBAT_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, BODVBAT_INT_ENABLE_A, O>;
impl<'a, const O: u8> BODVBAT_INT_ENABLE_W<'a, O> {
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `BODVBAT_INT_CLEAR` reader - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BODVBAT_INT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `BODVBAT_INT_CLEAR` writer - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BODVBAT_INT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `BODCORE_INT_ENABLE` reader - BOD CORE interrupt control."]
pub type BODCORE_INT_ENABLE_R = crate::BitReader<BODCORE_INT_ENABLE_A>;
#[doc = "BOD CORE interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODCORE_INT_ENABLE_A {
    #[doc = "0: BOD CORE interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: BOD CORE interrupt is enabled."]
    ENABLE = 1,
}
impl From<BODCORE_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BODCORE_INT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_INT_ENABLE_A {
        match self.bits {
            false => BODCORE_INT_ENABLE_A::DISABLE,
            true => BODCORE_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODCORE_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODCORE_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Field `BODCORE_INT_ENABLE` writer - BOD CORE interrupt control."]
pub type BODCORE_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, BODCORE_INT_ENABLE_A, O>;
impl<'a, const O: u8> BODCORE_INT_ENABLE_W<'a, O> {
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `BODCORE_INT_CLEAR` reader - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BODCORE_INT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `BODCORE_INT_CLEAR` writer - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BODCORE_INT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `DCDC_INT_ENABLE` reader - DCDC interrupt control."]
pub type DCDC_INT_ENABLE_R = crate::BitReader<DCDC_INT_ENABLE_A>;
#[doc = "DCDC interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_INT_ENABLE_A {
    #[doc = "0: DCDC interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: DCDC interrupt is enabled."]
    ENABLE = 1,
}
impl From<DCDC_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_INT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_INT_ENABLE_A {
        match self.bits {
            false => DCDC_INT_ENABLE_A::DISABLE,
            true => DCDC_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DCDC_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DCDC_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Field `DCDC_INT_ENABLE` writer - DCDC interrupt control."]
pub type DCDC_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, DCDC_INT_ENABLE_A, O>;
impl<'a, const O: u8> DCDC_INT_ENABLE_W<'a, O> {
    #[doc = "DCDC interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::DISABLE)
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `DCDC_INT_CLEAR` reader - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type DCDC_INT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_INT_CLEAR` writer - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type DCDC_INT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOD_DCDC_INT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&self) -> BODVBAT_INT_ENABLE_R {
        BODVBAT_INT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&self) -> BODVBAT_INT_CLEAR_R {
        BODVBAT_INT_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&self) -> BODCORE_INT_ENABLE_R {
        BODCORE_INT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&self) -> BODCORE_INT_CLEAR_R {
        BODCORE_INT_CLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&self) -> DCDC_INT_ENABLE_R {
        DCDC_INT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&self) -> DCDC_INT_CLEAR_R {
        DCDC_INT_CLEAR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    #[must_use]
    pub fn bodvbat_int_enable(&mut self) -> BODVBAT_INT_ENABLE_W<0> {
        BODVBAT_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    #[must_use]
    pub fn bodvbat_int_clear(&mut self) -> BODVBAT_INT_CLEAR_W<1> {
        BODVBAT_INT_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    #[must_use]
    pub fn bodcore_int_enable(&mut self) -> BODCORE_INT_ENABLE_W<2> {
        BODCORE_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    #[must_use]
    pub fn bodcore_int_clear(&mut self) -> BODCORE_INT_CLEAR_W<3> {
        BODCORE_INT_CLEAR_W::new(self)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_int_enable(&mut self) -> DCDC_INT_ENABLE_W<4> {
        DCDC_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_int_clear(&mut self) -> DCDC_INT_CLEAR_W<5> {
        DCDC_INT_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod_dcdc_int_ctrl](index.html) module"]
pub struct BOD_DCDC_INT_CTRL_SPEC;
impl crate::RegisterSpec for BOD_DCDC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod_dcdc_int_ctrl::R](R) reader structure"]
impl crate::Readable for BOD_DCDC_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod_dcdc_int_ctrl::W](W) writer structure"]
impl crate::Writable for BOD_DCDC_INT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD_DCDC_INT_CTRL to value 0"]
impl crate::Resettable for BOD_DCDC_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
