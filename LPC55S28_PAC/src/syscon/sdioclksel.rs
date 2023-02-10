#[doc = "Register `SDIOCLKSEL` reader"]
pub struct R(crate::R<SDIOCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIOCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIOCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIOCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIOCLKSEL` writer"]
pub struct W(crate::W<SDIOCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCLKSEL_SPEC>;
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
impl From<crate::W<SDIOCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - SDIO clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "SDIO clock source select.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock."]
    ENUM_0X0 = 0,
    #[doc = "1: PLL0 clock."]
    ENUM_0X1 = 1,
    #[doc = "2: No clock."]
    ENUM_0X2 = 2,
    #[doc = "3: FRO 96 MHz clock."]
    ENUM_0X3 = 3,
    #[doc = "4: No clock."]
    ENUM_0X4 = 4,
    #[doc = "5: PLL1 clock."]
    ENUM_0X5 = 5,
    #[doc = "6: No clock."]
    ENUM_0X6 = 6,
    #[doc = "7: No clock."]
    ENUM_0X7 = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::ENUM_0X0,
            1 => SEL_A::ENUM_0X1,
            2 => SEL_A::ENUM_0X2,
            3 => SEL_A::ENUM_0X3,
            4 => SEL_A::ENUM_0X4,
            5 => SEL_A::ENUM_0X5,
            6 => SEL_A::ENUM_0X6,
            7 => SEL_A::ENUM_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0X0`"]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == SEL_A::ENUM_0X0
    }
    #[doc = "Checks if the value of the field is `ENUM_0X1`"]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == SEL_A::ENUM_0X1
    }
    #[doc = "Checks if the value of the field is `ENUM_0X2`"]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == SEL_A::ENUM_0X2
    }
    #[doc = "Checks if the value of the field is `ENUM_0X3`"]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == SEL_A::ENUM_0X3
    }
    #[doc = "Checks if the value of the field is `ENUM_0X4`"]
    #[inline(always)]
    pub fn is_enum_0x4(&self) -> bool {
        *self == SEL_A::ENUM_0X4
    }
    #[doc = "Checks if the value of the field is `ENUM_0X5`"]
    #[inline(always)]
    pub fn is_enum_0x5(&self) -> bool {
        *self == SEL_A::ENUM_0X5
    }
    #[doc = "Checks if the value of the field is `ENUM_0X6`"]
    #[inline(always)]
    pub fn is_enum_0x6(&self) -> bool {
        *self == SEL_A::ENUM_0X6
    }
    #[doc = "Checks if the value of the field is `ENUM_0X7`"]
    #[inline(always)]
    pub fn is_enum_0x7(&self) -> bool {
        *self == SEL_A::ENUM_0X7
    }
}
#[doc = "Field `SEL` writer - SDIO clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDIOCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X0)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X1)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X2)
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X3)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x4(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X4)
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn enum_0x5(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X5)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x6(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X6)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x7(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X7)
    }
}
impl R {
    #[doc = "Bits 0:2 - SDIO clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDIO clock source select."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclksel](index.html) module"]
pub struct SDIOCLKSEL_SPEC;
impl crate::RegisterSpec for SDIOCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdioclksel::R](R) reader structure"]
impl crate::Readable for SDIOCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdioclksel::W](W) writer structure"]
impl crate::Writable for SDIOCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIOCLKSEL to value 0x07"]
impl crate::Resettable for SDIOCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
