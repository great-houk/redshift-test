#[doc = "Register `STATUSCLK` reader"]
pub struct R(crate::R<STATUSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUSCLK` writer"]
pub struct W(crate::W<STATUSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUSCLK_SPEC>;
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
impl From<crate::W<STATUSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32KOK` reader - XTAL oscillator 32 K OK signal."]
pub type XTAL32KOK_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32KOSCFAILURE` reader - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub type XTAL32KOSCFAILURE_R = crate::BitReader<XTAL32KOSCFAILURE_A>;
#[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTAL32KOSCFAILURE_A {
    #[doc = "0: No oscillation failure has been detetced since the last time this bit has been cleared.."]
    NOFAIL = 0,
    #[doc = "1: At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    FAILURE = 1,
}
impl From<XTAL32KOSCFAILURE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL32KOSCFAILURE_A) -> Self {
        variant as u8 != 0
    }
}
impl XTAL32KOSCFAILURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL32KOSCFAILURE_A {
        match self.bits {
            false => XTAL32KOSCFAILURE_A::NOFAIL,
            true => XTAL32KOSCFAILURE_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAIL`"]
    #[inline(always)]
    pub fn is_nofail(&self) -> bool {
        *self == XTAL32KOSCFAILURE_A::NOFAIL
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == XTAL32KOSCFAILURE_A::FAILURE
    }
}
#[doc = "Field `XTAL32KOSCFAILURE` writer - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub type XTAL32KOSCFAILURE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATUSCLK_SPEC, XTAL32KOSCFAILURE_A, O>;
impl<'a, const O: u8> XTAL32KOSCFAILURE_W<'a, O> {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn nofail(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::NOFAIL)
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::FAILURE)
    }
}
impl R {
    #[doc = "Bit 0 - XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub fn xtal32kok(&self) -> XTAL32KOK_R {
        XTAL32KOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&self) -> XTAL32KOSCFAILURE_R {
        XTAL32KOSCFAILURE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32koscfailure(&mut self) -> XTAL32KOSCFAILURE_W<2> {
        XTAL32KOSCFAILURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusclk](index.html) module"]
pub struct STATUSCLK_SPEC;
impl crate::RegisterSpec for STATUSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusclk::R](R) reader structure"]
impl crate::Readable for STATUSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statusclk::W](W) writer structure"]
impl crate::Writable for STATUSCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUSCLK to value 0x06"]
impl crate::Resettable for STATUSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
