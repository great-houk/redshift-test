#[doc = "Register `PLL1STAT` reader"]
pub struct R(crate::R<PLL1STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1STAT` writer"]
pub struct W(crate::W<PLL1STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1STAT_SPEC>;
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
impl From<crate::W<PLL1STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\]
:100 kHz to 20 MHz."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PREDIVACK` reader - pre-divider ratio change acknowledge."]
pub type PREDIVACK_R = crate::BitReader<bool>;
#[doc = "Field `FEEDDIVACK` reader - feedback divider ratio change acknowledge."]
pub type FEEDDIVACK_R = crate::BitReader<bool>;
#[doc = "Field `POSTDIVACK` reader - post-divider ratio change acknowledge."]
pub type POSTDIVACK_R = crate::BitReader<bool>;
#[doc = "Field `FRMDET` reader - free running detector output (active high)."]
pub type FRMDET_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\]
:100 kHz to 20 MHz."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn predivack(&self) -> PREDIVACK_R {
        PREDIVACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub fn feeddivack(&self) -> FEEDDIVACK_R {
        FEEDDIVACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - post-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn postdivack(&self) -> POSTDIVACK_R {
        POSTDIVACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - free running detector output (active high)."]
    #[inline(always)]
    pub fn frmdet(&self) -> FRMDET_R {
        FRMDET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1stat](index.html) module"]
pub struct PLL1STAT_SPEC;
impl crate::RegisterSpec for PLL1STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1stat::R](R) reader structure"]
impl crate::Readable for PLL1STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1stat::W](W) writer structure"]
impl crate::Writable for PLL1STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1STAT to value 0"]
impl crate::Resettable for PLL1STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
