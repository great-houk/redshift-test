#[doc = "Register `CDETECT` reader"]
pub struct R(crate::R<CDETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDETECT` writer"]
pub struct W(crate::W<CDETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDETECT_SPEC>;
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
impl From<crate::W<CDETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD0_DETECT` reader - Card 0 detect"]
pub type CARD0_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `CARD0_DETECT` writer - Card 0 detect"]
pub type CARD0_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDETECT_SPEC, bool, O>;
#[doc = "Field `CARD1_DETECT` reader - Card 1 detect"]
pub type CARD1_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `CARD1_DETECT` writer - Card 1 detect"]
pub type CARD1_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDETECT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    pub fn card0_detect(&self) -> CARD0_DETECT_R {
        CARD0_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    pub fn card1_detect(&self) -> CARD1_DETECT_R {
        CARD1_DETECT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    #[must_use]
    pub fn card0_detect(&mut self) -> CARD0_DETECT_W<0> {
        CARD0_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    #[must_use]
    pub fn card1_detect(&mut self) -> CARD1_DETECT_W<1> {
        CARD1_DETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card Detect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdetect](index.html) module"]
pub struct CDETECT_SPEC;
impl crate::RegisterSpec for CDETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdetect::R](R) reader structure"]
impl crate::Readable for CDETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdetect::W](W) writer structure"]
impl crate::Writable for CDETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDETECT to value 0"]
impl crate::Resettable for CDETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
