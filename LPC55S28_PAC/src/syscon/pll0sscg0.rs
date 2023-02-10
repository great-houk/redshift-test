#[doc = "Register `PLL0SSCG0` reader"]
pub struct R(crate::R<PLL0SSCG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0SSCG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0SSCG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0SSCG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0SSCG0` writer"]
pub struct W(crate::W<PLL0SSCG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0SSCG0_SPEC>;
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
impl From<crate::W<PLL0SSCG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0SSCG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD_LBS` reader - input word of the wrapper bit 31 to 0."]
pub type MD_LBS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MD_LBS` writer - input word of the wrapper bit 31 to 0."]
pub type MD_LBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0SSCG0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub fn md_lbs(&self) -> MD_LBS_R {
        MD_LBS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn md_lbs(&mut self) -> MD_LBS_W<0> {
        MD_LBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0sscg0](index.html) module"]
pub struct PLL0SSCG0_SPEC;
impl crate::RegisterSpec for PLL0SSCG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0sscg0::R](R) reader structure"]
impl crate::Readable for PLL0SSCG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0sscg0::W](W) writer structure"]
impl crate::Writable for PLL0SSCG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0SSCG0 to value 0"]
impl crate::Resettable for PLL0SSCG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
