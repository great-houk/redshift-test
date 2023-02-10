#[doc = "Register `PLL1MDEC` reader"]
pub struct R(crate::R<PLL1MDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1MDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1MDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1MDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1MDEC` writer"]
pub struct W(crate::W<PLL1MDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1MDEC_SPEC>;
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
impl From<crate::W<PLL1MDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1MDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIV` reader - feedback divider divider ratio (M-divider)."]
pub type MDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MDIV` writer - feedback divider divider ratio (M-divider)."]
pub type MDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1MDEC_SPEC, u16, u16, 16, O>;
#[doc = "Field `MREQ` reader - feedback ratio change request."]
pub type MREQ_R = crate::BitReader<bool>;
#[doc = "Field `MREQ` writer - feedback ratio change request."]
pub type MREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1MDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MDIV_W<0> {
        MDIV_W::new(self)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    #[must_use]
    pub fn mreq(&mut self) -> MREQ_W<16> {
        MREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1mdec](index.html) module"]
pub struct PLL1MDEC_SPEC;
impl crate::RegisterSpec for PLL1MDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1mdec::R](R) reader structure"]
impl crate::Readable for PLL1MDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1mdec::W](W) writer structure"]
impl crate::Writable for PLL1MDEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1MDEC to value 0"]
impl crate::Resettable for PLL1MDEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
