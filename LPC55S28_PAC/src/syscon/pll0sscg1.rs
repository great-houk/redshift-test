#[doc = "Register `PLL0SSCG1` reader"]
pub struct R(crate::R<PLL0SSCG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0SSCG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0SSCG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0SSCG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0SSCG1` writer"]
pub struct W(crate::W<PLL0SSCG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0SSCG1_SPEC>;
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
impl From<crate::W<PLL0SSCG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0SSCG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD_MBS` reader - input word of the wrapper bit 32."]
pub type MD_MBS_R = crate::BitReader<bool>;
#[doc = "Field `MD_MBS` writer - input word of the wrapper bit 32."]
pub type MD_MBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0SSCG1_SPEC, bool, O>;
#[doc = "Field `MD_REQ` reader - md change request."]
pub type MD_REQ_R = crate::BitReader<bool>;
#[doc = "Field `MD_REQ` writer - md change request."]
pub type MD_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0SSCG1_SPEC, bool, O>;
#[doc = "Field `MF` reader - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
pub type MF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MF` writer - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
pub type MF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0SSCG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MR` reader - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
pub type MR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MR` writer - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
pub type MR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0SSCG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MC` reader - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub type MC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MC` writer - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub type MC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0SSCG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `MDIV_EXT` reader - to select an external mdiv value."]
pub type MDIV_EXT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MDIV_EXT` writer - to select an external mdiv value."]
pub type MDIV_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL0SSCG1_SPEC, u16, u16, 16, O>;
#[doc = "Field `MREQ` reader - to select an external mreq value."]
pub type MREQ_R = crate::BitReader<bool>;
#[doc = "Field `MREQ` writer - to select an external mreq value."]
pub type MREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0SSCG1_SPEC, bool, O>;
#[doc = "Field `DITHER` reader - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub type DITHER_R = crate::BitReader<bool>;
#[doc = "Field `DITHER` writer - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub type DITHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0SSCG1_SPEC, bool, O>;
#[doc = "Field `SEL_EXT` reader - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub type SEL_EXT_R = crate::BitReader<bool>;
#[doc = "Field `SEL_EXT` writer - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub type SEL_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL0SSCG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    pub fn md_mbs(&self) -> MD_MBS_R {
        MD_MBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    pub fn md_req(&self) -> MD_REQ_R {
        MD_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    pub fn mdiv_ext(&self) -> MDIV_EXT_R {
        MDIV_EXT_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    #[must_use]
    pub fn md_mbs(&mut self) -> MD_MBS_W<0> {
        MD_MBS_W::new(self)
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    #[must_use]
    pub fn md_req(&mut self) -> MD_REQ_W<1> {
        MD_REQ_W::new(self)
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MF_W<2> {
        MF_W::new(self)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<5> {
        MR_W::new(self)
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<8> {
        MC_W::new(self)
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    #[must_use]
    pub fn mdiv_ext(&mut self) -> MDIV_EXT_W<10> {
        MDIV_EXT_W::new(self)
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    #[must_use]
    pub fn mreq(&mut self) -> MREQ_W<26> {
        MREQ_W::new(self)
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<27> {
        DITHER_W::new(self)
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    #[must_use]
    pub fn sel_ext(&mut self) -> SEL_EXT_W<28> {
        SEL_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0sscg1](index.html) module"]
pub struct PLL0SSCG1_SPEC;
impl crate::RegisterSpec for PLL0SSCG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0sscg1::R](R) reader structure"]
impl crate::Readable for PLL0SSCG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0sscg1::W](W) writer structure"]
impl crate::Writable for PLL0SSCG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0SSCG1 to value 0"]
impl crate::Resettable for PLL0SSCG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
