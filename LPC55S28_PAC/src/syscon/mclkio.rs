#[doc = "Register `MCLKIO` reader"]
pub struct R(crate::R<MCLKIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKIO` writer"]
pub struct W(crate::W<MCLKIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKIO_SPEC>;
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
impl From<crate::W<MCLKIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLKIO` reader - MCLK control."]
pub type MCLKIO_R = crate::BitReader<MCLKIO_A>;
#[doc = "MCLK control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLKIO_A {
    #[doc = "0: input mode."]
    INPUT = 0,
    #[doc = "1: output mode."]
    OUTPUT = 1,
}
impl From<MCLKIO_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKIO_A) -> Self {
        variant as u8 != 0
    }
}
impl MCLKIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKIO_A {
        match self.bits {
            false => MCLKIO_A::INPUT,
            true => MCLKIO_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MCLKIO_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MCLKIO_A::OUTPUT
    }
}
#[doc = "Field `MCLKIO` writer - MCLK control."]
pub type MCLKIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCLKIO_SPEC, MCLKIO_A, O>;
impl<'a, const O: u8> MCLKIO_W<'a, O> {
    #[doc = "input mode."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MCLKIO_A::INPUT)
    }
    #[doc = "output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MCLKIO_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&self) -> MCLKIO_R {
        MCLKIO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    #[must_use]
    pub fn mclkio(&mut self) -> MCLKIO_W<0> {
        MCLKIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkio](index.html) module"]
pub struct MCLKIO_SPEC;
impl crate::RegisterSpec for MCLKIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkio::R](R) reader structure"]
impl crate::Readable for MCLKIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkio::W](W) writer structure"]
impl crate::Writable for MCLKIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKIO to value 0"]
impl crate::Resettable for MCLKIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
