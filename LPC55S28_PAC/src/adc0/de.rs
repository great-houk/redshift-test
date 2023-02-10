#[doc = "Register `DE` reader"]
pub struct R(crate::R<DE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DE` writer"]
pub struct W(crate::W<DE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DE_SPEC>;
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
impl From<crate::W<DE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWMDE0` reader - FIFO 0 Watermark DMA Enable"]
pub type FWMDE0_R = crate::BitReader<FWMDE0_A>;
#[doc = "FIFO 0 Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWMDE0_A {
    #[doc = "0: DMA request disabled."]
    FWMDE0_0 = 0,
    #[doc = "1: DMA request enabled."]
    FWMDE0_1 = 1,
}
impl From<FWMDE0_A> for bool {
    #[inline(always)]
    fn from(variant: FWMDE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FWMDE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMDE0_A {
        match self.bits {
            false => FWMDE0_A::FWMDE0_0,
            true => FWMDE0_A::FWMDE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMDE0_0`"]
    #[inline(always)]
    pub fn is_fwmde0_0(&self) -> bool {
        *self == FWMDE0_A::FWMDE0_0
    }
    #[doc = "Checks if the value of the field is `FWMDE0_1`"]
    #[inline(always)]
    pub fn is_fwmde0_1(&self) -> bool {
        *self == FWMDE0_A::FWMDE0_1
    }
}
#[doc = "Field `FWMDE0` writer - FIFO 0 Watermark DMA Enable"]
pub type FWMDE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DE_SPEC, FWMDE0_A, O>;
impl<'a, const O: u8> FWMDE0_W<'a, O> {
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn fwmde0_0(self) -> &'a mut W {
        self.variant(FWMDE0_A::FWMDE0_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn fwmde0_1(self) -> &'a mut W {
        self.variant(FWMDE0_A::FWMDE0_1)
    }
}
#[doc = "Field `FWMDE1` reader - FIFO1 Watermark DMA Enable"]
pub type FWMDE1_R = crate::BitReader<FWMDE1_A>;
#[doc = "FIFO1 Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWMDE1_A {
    #[doc = "0: DMA request disabled."]
    FWMDE1_0 = 0,
    #[doc = "1: DMA request enabled."]
    FWMDE1_1 = 1,
}
impl From<FWMDE1_A> for bool {
    #[inline(always)]
    fn from(variant: FWMDE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FWMDE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMDE1_A {
        match self.bits {
            false => FWMDE1_A::FWMDE1_0,
            true => FWMDE1_A::FWMDE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMDE1_0`"]
    #[inline(always)]
    pub fn is_fwmde1_0(&self) -> bool {
        *self == FWMDE1_A::FWMDE1_0
    }
    #[doc = "Checks if the value of the field is `FWMDE1_1`"]
    #[inline(always)]
    pub fn is_fwmde1_1(&self) -> bool {
        *self == FWMDE1_A::FWMDE1_1
    }
}
#[doc = "Field `FWMDE1` writer - FIFO1 Watermark DMA Enable"]
pub type FWMDE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DE_SPEC, FWMDE1_A, O>;
impl<'a, const O: u8> FWMDE1_W<'a, O> {
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn fwmde1_0(self) -> &'a mut W {
        self.variant(FWMDE1_A::FWMDE1_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn fwmde1_1(self) -> &'a mut W {
        self.variant(FWMDE1_A::FWMDE1_1)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde0(&self) -> FWMDE0_R {
        FWMDE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO1 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde1(&self) -> FWMDE1_R {
        FWMDE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwmde0(&mut self) -> FWMDE0_W<0> {
        FWMDE0_W::new(self)
    }
    #[doc = "Bit 1 - FIFO1 Watermark DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwmde1(&mut self) -> FWMDE1_W<1> {
        FWMDE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [de](index.html) module"]
pub struct DE_SPEC;
impl crate::RegisterSpec for DE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [de::R](R) reader structure"]
impl crate::Readable for DE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [de::W](W) writer structure"]
impl crate::Writable for DE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DE to value 0"]
impl crate::Resettable for DE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
