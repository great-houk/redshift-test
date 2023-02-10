#[doc = "Register `PLL0NDEC` reader"]
pub struct R(crate::R<PLL0NDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0NDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0NDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0NDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0NDEC` writer"]
pub struct W(crate::W<PLL0NDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0NDEC_SPEC>;
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
impl From<crate::W<PLL0NDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0NDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDIV` reader - pre-divider divider ratio (N-divider)."]
pub type NDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDIV` writer - pre-divider divider ratio (N-divider)."]
pub type NDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0NDEC_SPEC, u8, u8, 8, O>;
#[doc = "Field `NREQ` reader - pre-divider ratio change request."]
pub type NREQ_R = crate::BitReader<bool>;
#[doc = "Field `NREQ` writer - pre-divider ratio change request."]
pub type NREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0NDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<0> {
        NDIV_W::new(self)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    #[must_use]
    pub fn nreq(&mut self) -> NREQ_W<8> {
        NREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 550m N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0ndec](index.html) module"]
pub struct PLL0NDEC_SPEC;
impl crate::RegisterSpec for PLL0NDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0ndec::R](R) reader structure"]
impl crate::Readable for PLL0NDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0ndec::W](W) writer structure"]
impl crate::Writable for PLL0NDEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0NDEC to value 0"]
impl crate::Resettable for PLL0NDEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
