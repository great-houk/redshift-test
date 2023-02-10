#[doc = "Register `PLL0PDEC` reader"]
pub struct R(crate::R<PLL0PDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0PDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0PDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0PDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0PDEC` writer"]
pub struct W(crate::W<PLL0PDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0PDEC_SPEC>;
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
impl From<crate::W<PLL0PDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0PDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIV` reader - post-divider divider ratio (P-divider)"]
pub type PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDIV` writer - post-divider divider ratio (P-divider)"]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0PDEC_SPEC, u8, u8, 5, O>;
#[doc = "Field `PREQ` reader - feedback ratio change request."]
pub type PREQ_R = crate::BitReader<bool>;
#[doc = "Field `PREQ` writer - feedback ratio change request."]
pub type PREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0PDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<0> {
        PDIV_W::new(self)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    #[must_use]
    pub fn preq(&mut self) -> PREQ_W<5> {
        PREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 550m P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0pdec](index.html) module"]
pub struct PLL0PDEC_SPEC;
impl crate::RegisterSpec for PLL0PDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0pdec::R](R) reader structure"]
impl crate::Readable for PLL0PDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0pdec::W](W) writer structure"]
impl crate::Writable for PLL0PDEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0PDEC to value 0"]
impl crate::Resettable for PLL0PDEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
